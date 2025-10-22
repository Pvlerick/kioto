use kioto::counter_future::CounterFuture;
use kioto::executor::Executor;

fn main() {
    let mut executor = Executor::new();
    executor.spawn(async {
        let result = CounterFuture { count: 0, max: 3 }.await;
        println!("CounterFuture completed with: {}", result);
    });
    executor.run();
}
