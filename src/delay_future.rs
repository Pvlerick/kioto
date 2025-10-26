use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

pub struct DelayFuture {
    duration: Duration,
    start: Instant,
}

impl DelayFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start: Instant::now(),
        }
    }
}

impl Future for DelayFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling DelayFuture ...");

        if Instant::now() >= self.start + self.duration {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
