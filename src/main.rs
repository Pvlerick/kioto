use std::time::Duration;

use kioto::delay_future::DelayFuture;
use kioto::multi_threaded_executor::MultiThreadedExecutor;

fn main() {
    let executor = MultiThreadedExecutor::new(4);

    executor.spawn(async {
        DelayFuture::new(Duration::from_millis(500));
        println!("Delay expired!");
    });

    // Keep the main thread alive to allow tasks to complete
    std::thread::park();
}

// async fn delay(name: &str, duration: Duration) {
//     println!("{name}: starting delay");
//     DelayFuture::new(5, duration).await;
//     println!("{name}: finished");
// }
