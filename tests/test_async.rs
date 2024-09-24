//! ```cargo
//! [package]
//! name = "test_async"
//! version = "0.1.0"
//! edition = "2021"
//! [dependencies]
//! tokio = "1.39.2"
//! ```
// /**
//  * Couldn't fucking run this motherfucker code
//  */
// use tokio::time::{sleep, Duration}; // Tokio is the async runtime

// async fn task_one() {
//     println!("Task 1 started");
//     sleep(Duration::from_secs(2)).await; // This will suspend the task here
//     println!("Task 1 finished");
// }

// async fn task_two() {
//     println!("Task 2 started");
//     sleep(Duration::from_secs(1)).await; // This will suspend the task here
//     println!("Task 2 finished");
// }

// #[tokio::main] // Tokio runtime entry point
// async fn main() {
//     println!("Main task started");

//     // Spawn both tasks
//     let task1 = tokio::spawn(task_one()); // First async task starts
//     let task2 = tokio::spawn(task_two()); // Second async task starts

//     // The main task will wait for both to finish
//     let _ = task1.await;
//     let _ = task2.await;

//     println!("Main task finished");
// }
