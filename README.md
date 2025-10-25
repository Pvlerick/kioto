# Kioto: A Toy Async Runtime in Rust

**Kioto** is a toy async runtime written in Rust to explore the inner workings of async/await, futures, and executors. This project is **not production-ready** but serves as a learning tool to understand how async runtimes like Tokio or async-std work under the hood.

---

## **Purpose**
This project was created to:
- Understand how Rust’s async/await syntax desugars into state machines.
- Implement a minimal async executor from scratch.
- Explore multi-threading and task scheduling in async runtimes.
- Learn about wakers, polling, and task synchronization.

---

## **Features**
- A single-threaded executor with waker support.
- A multi-threaded executor with task reordering.
- Examples of custom futures (e.g., `DelayFuture`, `CounterFuture`).
- *No external dependencies — pure Rust and `std` only.*

---

## **How It Works**
1. **Futures**: Custom futures like `CounterFuture` and `DelayFuture` demonstrate how async code is polled and resumed.
2. **Executor**: The executor polls futures to completion, handles wakers, and manages task queues.
3. **Multi-threading**: The multi-threaded executor distributes tasks across threads and handles synchronization.

---

## **Learning Journey**
This project was developed as a learning exercise with the help of (**Le Chat**)[https://chat.mistral.ai/], an AI assistant by Mistral AI. Mistral acted as a tutor, guiding the implementation, explaining concepts, and reviewing code.

---

## **How to Run**
1. Clone the repository:
```sh
git clone https://github.com/Pvlerick/kioto.git
```

2. Run the examples:
```sh
cargo run --bin single_threaded
cargo run --bin multi_threaded
```
