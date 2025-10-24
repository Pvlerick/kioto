use std::collections::VecDeque;
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::task::{Context, Poll, Waker};
use std::thread;

use crate::task_waker::{TaskWakeReceiver, TaskWaker};

pub struct Task {
    id: usize,
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl Task {
    fn new(id: usize, future: Pin<Box<dyn Future<Output = ()> + Send>>) -> Self {
        Task { id, future }
    }
}

pub struct MultiThreadedExecutor {
    tasks: Arc<Mutex<VecDeque<Task>>>,
    wake_sender: mpsc::Sender<usize>,
    wake_receiver: Arc<Mutex<TaskWakeReceiver>>,
    next_task_id: AtomicUsize,
}

impl MultiThreadedExecutor {
    pub fn new(num_threads: usize) -> Self {
        let (wake_sender, wake_receiver) = TaskWakeReceiver::new();
        let wake_receiver = Arc::new(Mutex::new(wake_receiver));

        let executor = MultiThreadedExecutor {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
            wake_sender: wake_sender.clone(),
            wake_receiver: wake_receiver.clone(),
            next_task_id: AtomicUsize::new(0),
        };

        executor.start_reordering_thread();

        executor.start_execution_threads(num_threads);

        executor
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push_front(Task::new(
            self.next_task_id.fetch_add(1, Ordering::Relaxed),
            Box::pin(future),
        ));
    }

    fn poll_future<F>(future: Pin<&mut F>, waker: Arc<TaskWaker>) -> Poll<F::Output>
    where
        F: Future + ?Sized,
    {
        let waker = Waker::from(waker);
        let mut cx = Context::from_waker(&waker);
        future.poll(&mut cx)
    }

    fn start_execution_threads(&self, num_threads: usize) {
        println!("Starting {num_threads} tasks execution threads");

        for _ in 0..num_threads {
            let tasks = self.tasks.clone();
            let wake_sender = self.wake_sender.clone();

            thread::spawn(move || {
                loop {
                    let mut tasks = tasks.lock().unwrap();

                    tasks.retain_mut(|task| {
                        let waker = TaskWaker::new(task.id, wake_sender.clone());

                        match Self::poll_future(task.future.as_mut(), waker) {
                            Poll::Ready(_) => false,
                            Poll::Pending => true,
                        }
                    });
                }
            });
        }
    }

    fn start_reordering_thread(&self) {
        println!("Starting tasks reordering thread");

        let tasks = self.tasks.clone();
        let wake_receiver = self.wake_receiver.clone();

        thread::spawn(move || {
            loop {
                // Blocks the thread while the channel is empty
                let task_id = wake_receiver.lock().unwrap().blocking_recv();

                let mut front = VecDeque::new();
                let mut back = VecDeque::new();

                let mut tasks_ref = tasks.lock().unwrap();
                let tasks = mem::take(&mut *tasks_ref);

                for task in tasks.into_iter() {
                    if task.id == task_id {
                        front.push_back(task);
                    } else {
                        back.push_back(task);
                    }
                }

                front.append(&mut back);
                *tasks_ref = front;
            }
        });
    }
}
