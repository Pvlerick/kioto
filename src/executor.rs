use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::task::{Context, Poll, Waker};

use crate::task_waker::{TaskWakeReceiver, TaskWaker};

pub struct Task {
    id: usize,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    fn new(id: usize, future: Pin<Box<dyn Future<Output = ()>>>) -> Self {
        Task { id, future }
    }
}

pub struct Executor {
    tasks: VecDeque<Task>,
    wake_sender: Sender<usize>,
    wake_receiver: TaskWakeReceiver,
    next_task_id: AtomicUsize,
}

impl Executor {
    pub fn new() -> Self {
        let (wake_sender, wake_receiver) = TaskWakeReceiver::new();

        Executor {
            tasks: VecDeque::new(),
            wake_sender,
            wake_receiver,
            next_task_id: AtomicUsize::new(0),
        }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.tasks.push_front(Task::new(
            self.next_task_id.fetch_add(1, Ordering::Relaxed),
            Box::pin(future),
        ));
    }

    pub fn run(mut self) {
        while let Some(task_id) = self.wake_receiver.try_recv() {
            let mut front = VecDeque::new();
            let mut back = VecDeque::new();

            for task in self.tasks {
                if task.id == task_id {
                    front.push_back(task);
                } else {
                    back.push_back(task);
                }
            }

            front.append(&mut back);
            self.tasks = front;
        }

        while !self.tasks.is_empty() {
            self.tasks.retain_mut(|task| {
                let waker = TaskWaker::new(task.id, self.wake_sender.clone());

                match Executor::poll_future(task.future.as_mut(), waker) {
                    Poll::Ready(_) => false,
                    Poll::Pending => true,
                }
            })
        }
    }

    fn poll_future<F>(future: Pin<&mut F>, waker: Arc<TaskWaker>) -> Poll<F::Output>
    where
        F: Future + ?Sized,
    {
        let waker = Waker::from(waker);
        let mut cx = Context::from_waker(&waker);
        future.poll(&mut cx)
    }
}
