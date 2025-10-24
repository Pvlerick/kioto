use std::pin::Pin;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use std::task::Wake;

pub struct TaskWaker {
    id: usize,
    sender: Sender<usize>,
}

impl TaskWaker {
    pub fn new(id: usize, sender: Sender<usize>) -> Arc<Self> {
        Arc::new(TaskWaker { id, sender })
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        println!("Waker called for task {}", self.id);

        self.sender
            .send(self.id)
            .expect("Failed to send on channel");
    }
}

pub struct Task {
    pub future: Pin<Box<dyn Future<Output = ()>>>,
    pub task_id: usize,
}

pub struct TaskWakeReceiver {
    receiver: Receiver<usize>,
}

impl TaskWakeReceiver {
    pub fn new() -> (Sender<usize>, Self) {
        let (sender, receiver) = channel();
        (sender, TaskWakeReceiver { receiver })
    }

    pub fn try_recv(&self) -> Option<usize> {
        match self.receiver.try_recv() {
            Ok(task_id) => Some(task_id),
            Err(TryRecvError::Empty) => None,
            _ => panic!("Failed to receive from channel"),
        }
    }

    pub fn blocking_recv(&self) -> usize {
        self.receiver.recv().unwrap()
    }
}
