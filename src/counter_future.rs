use std::task::Poll;

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
        println!(
            "Polling CounterFuture: count={}, max={}",
            self.count, self.max
        );

        self.count += 1;

        if self.count >= self.max {
            Poll::Ready(self.max)
        } else {
            Poll::Pending
        }
    }
}
