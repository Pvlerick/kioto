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
            self.tasks.retain_mut(|t| match Executor::poll_future(t) {
                Poll::Ready(_) => false,
                Poll::Pending => true,
            })
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
