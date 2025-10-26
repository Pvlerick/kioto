use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Timer<T> {
    expires_at: Instant,
    data: T,
}

impl<T> Timer<T> {
    fn new(expires_at: Instant, data: T) -> Self {
        Self { expires_at, data }
    }
}

#[derive(Debug)]
pub struct TimerWheel<T> {
    buckets: Vec<Option<VecDeque<Timer<T>>>>,
    bucket_duration: Duration,
    current_bucket: usize,
    start_time: Instant,
}

impl<T> TimerWheel<T> {
    pub fn new(num_buckets: usize, bucket_duration: Duration) -> Self {
        let mut buckets = Vec::with_capacity(num_buckets);

        for _ in 0..num_buckets {
            buckets.push(None);
        }

        TimerWheel {
            buckets,
            bucket_duration,
            current_bucket: 0,
            start_time: Instant::now(),
        }
    }

    pub fn interval(&self) -> Duration {
        self.bucket_duration
    }

    pub fn add_timer(&mut self, data: T, delay: Duration) {
        let expires_at = Instant::now() + delay;
        let bucket_index = self.calculate_bucket_index(expires_at);
        self.push_to_bucket(bucket_index, Timer::new(expires_at, data));
    }

    fn push_to_bucket(&mut self, index: usize, timer: Timer<T>) {
        match self.buckets[index].as_mut() {
            Some(bucket) => bucket.push_back(timer),
            None => {
                self.buckets[index] = {
                    let mut v = VecDeque::new();
                    v.push_back(timer);
                    Some(v)
                }
            }
        }
    }

    fn calculate_bucket_index(&self, expires_at: Instant) -> usize {
        let elapsed = expires_at.duration_since(Instant::now());
        let bucket_index = (elapsed.as_secs_f64() / self.bucket_duration.as_secs_f64()) as usize;
        (self.current_bucket + bucket_index) % self.buckets.len()
    }

    const EMPTY: VecDeque<Timer<T>> = VecDeque::new();

    pub(crate) fn tick(&mut self) -> Vec<T> {
        let mut expired = Vec::new();
        let now = Instant::now();
        let elapsed_buckets = (now.duration_since(self.start_time).as_secs_f64()
            / self.bucket_duration.as_secs_f64()) as usize;
        let elapsed_buckets = elapsed_buckets.saturating_sub(self.current_bucket);

        // If no buckets have elapsed, do nothing.
        if elapsed_buckets == 0 {
            return expired;
        }

        // Process each elapsed bucket.
        for _ in 0..elapsed_buckets {
            println!("here loop");
            let current_bucket = self.current_bucket;
            // Process all timers in the current bucket.
            while let Some(timer) = self.buckets[current_bucket]
                .as_mut()
                .map(|i| i.pop_front())
                .flatten()
            {
                println!("loop enter");
                if timer.expires_at <= now {
                    println!("epired");
                    expired.push(timer.data);
                } else {
                    // Reinsert timers that haven't expired yet.
                    let new_bucket = self.calculate_bucket_index(timer.expires_at);
                    self.push_to_bucket(new_bucket, timer);
                    println!("delayed to bucket: {new_bucket}");
                }
                println!("loop exit");
            }
            // Move to the next bucket.
            self.current_bucket = (self.current_bucket + 1) % self.buckets.len();
            println!("here loop--");
        }
        println!("end");

        expired
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn timer_one_bucket() {
        let mut sut = TimerWheel::<usize>::new(1, Duration::from_secs(1));
        sut.add_timer(42, Duration::from_secs(5));
        // for _ in 0..10 {
        //     sut.tick();
        // }
        thread::sleep(Duration::from_secs(1));
        assert_eq!(vec![42], sut.tick());
    }

    #[test]
    fn timer_two_buckets() {
        let mut sut = TimerWheel::<usize>::new(1, Duration::from_secs(1));
        sut.add_timer(42, Duration::from_millis(50));
        sut.add_timer(42, Duration::from_secs(5));
        sut.add_timer(42, Duration::from_secs(50));
        assert_eq!(false, true);
    }
}
