use std::sync::Arc;

pub struct NoopWaker;

impl std::task::Wake for NoopWaker {
    fn wake(self: Arc<Self>) {
        println!("Waker called!");
    }
}
