use std::time::Duration;

use kioto::delay_future::DelayFuture;
use kioto::multi_threaded_executor::MultiThreadedExecutor;

fn main() {
    let executor = MultiThreadedExecutor::new(4);

    executor.spawn(async {
        delay("one", Duration::from_secs(1)).await;
        delay("two", Duration::from_millis(500)).await;
    });

    executor.spawn(async {
        delay("three", Duration::from_millis(750)).await;
        delay("four", Duration::from_millis(250)).await;
    });

    // Keep the main thread alive to allow tasks to complete
    std::thread::park();
}

async fn delay(name: &str, duration: Duration) {
    println!("{name}: starting delay");
    DelayFuture::new(5, duration).await;
    println!("{name}: finished");
}
