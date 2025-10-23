use std::time::Duration;

use kioto::delay_future::DelayFuture;
use kioto::executor::Executor;

fn main() {
    let mut executor = Executor::new();

    executor.spawn(async {
        println!("Task 1: Starting delay...");
        DelayFuture::new(5, Duration::from_secs(1)).await;
        println!("Task 1: Delay completed!");
    });

    executor.run();
}
