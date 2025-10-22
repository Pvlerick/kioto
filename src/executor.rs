use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub struct Executor {
    tasks: VecDeque<Pin<Box<dyn Future<Output = u32>>>>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: VecDeque::new(),
        }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = u32> + 'static,
    {
        self.tasks.push_front(Box::pin(future));
    }

    pub fn run(&mut self) {
        while !self.tasks.is_empty() {
            let mut tasks_to_remove = Vec::<usize>::with_capacity(self.tasks.len());

            for (i, task) in self.tasks.iter_mut().enumerate() {
                match Executor::poll_future(task) {
                    Poll::Ready(_) => {
                        tasks_to_remove.push(i);
                    }
                    Poll::Pending => {}
                }
            }

            for i in tasks_to_remove {
                self.tasks.remove(i);
            }
        }
    }

    fn poll_future<F>(future: &mut F) -> Poll<F::Output>
    where
        F: Future + Unpin,
    {
        let mut cx = Context::from_waker(Waker::noop());
        let pinned = unsafe { Pin::new_unchecked(future) };
        pinned.poll(&mut cx)
    }
}
