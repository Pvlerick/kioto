use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

pub struct DelayFuture {
    max: u32,
    duration: Duration,
    count: u32,
}

impl DelayFuture {
    pub fn new(max: u32, duration: Duration) -> Self {
        Self {
            count: 0,
            max,
            duration,
        }
    }
}

impl Future for DelayFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling DelayFuture {}...", self.count);

        self.count += 1;

        if self.count >= self.max {
            Poll::Ready(())
        } else {
            thread::sleep(self.duration);
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
