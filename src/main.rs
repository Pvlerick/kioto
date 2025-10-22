use kioto::counter_future::CounterFuture;
use kioto::executor::Executor;

fn main() {
    let mut executor = Executor::new();
    executor.spawn(CounterFuture { count: 0, max: 3 });
    executor.run();
}
