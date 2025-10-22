use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

use crate::noop_waker::NoopWaker;

pub struct Executor {
    tasks: VecDeque<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: VecDeque::new(),
        }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.tasks.push_front(Box::pin(future));
    }

    pub fn run(&mut self) {
        while !self.tasks.is_empty() {
            self.tasks.retain_mut(|task| {
                let waker = Arc::new(NoopWaker);
                match Executor::poll_future(task.as_mut(), waker) {
                    Poll::Ready(_) => false,
                    Poll::Pending => true,
                }
            })
        }
    }

    fn poll_future<F>(future: Pin<&mut F>, waker: Arc<NoopWaker>) -> Poll<F::Output>
    where
        F: Future + ?Sized,
    {
        let waker = Waker::from(waker);
        let mut cx = Context::from_waker(&waker);
        future.poll(&mut cx)
    }
}
