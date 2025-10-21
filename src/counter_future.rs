use std::{task::Poll, thread, time::Duration};

#[derive(Debug)]
pub struct CounterFuture {
    pub count: u32,
    pub max: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.count += 1;

        if self.count >= self.max {
            Poll::Ready(self.max)
        } else {
            let waker = cx.waker().clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                waker.wake_by_ref();
            });
            Poll::Pending
        }
    }
}
