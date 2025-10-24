use std::time::Duration;

use kioto::delay_future::DelayFuture;
use kioto::multi_threaded_executor::MultiThreadedExecutor;

fn main() {
    let executor = MultiThreadedExecutor::new(4);

    executor.spawn(async {
        println!("Task 1: Starting delay...");
        DelayFuture::new(3, Duration::from_secs(1)).await;
        println!("Task 1: Delay completed!");
    });

    executor.spawn(async {
        println!("Task 2: Starting delay...");
        DelayFuture::new(2, Duration::from_secs(1)).await;
        println!("Task 2: Delay completed!");
    });

    // Keep the main thread alive to allow tasks to complete
    std::thread::park();
}
