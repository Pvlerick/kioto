use std::task::Poll;

pub struct CounterFuture {
    pub count: u32,
    pub max: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.count += 1;

        if self.count >= self.max {
            Poll::Ready(self.max)
        } else {
            Poll::Pending
        }
    }
}
