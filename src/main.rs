

// use tokio::runtime::Builder;
// use tokio::task::LocalSet;
// use std::time::Duration;
// use tokio::time::sleep;



// fn main() {
//     // Create a runtime and local task set for executing futures.
//     let rt = Builder::new_current_thread().enable_all().build().unwrap();
//     let local_set = LocalSet::new();

//     // Spawn a non-blocking task
//     local_set.spawn_local(async {
//         println!("1");
//         sleep(Duration::from_secs(1)).await;
//         println!("2");
//     });

//     // Simulate an external event loop and manually run the runtime
//     local_set.block_on(&rt, async {
//         let mut interval = tokio::time::interval(Duration::from_millis(100));
//         loop {
//             // Poll all tasks in the LocalSet
//             interval.tick().await;
//             // Check if tasks are still running or exit if done
//             // In a real scenario, more complex logic might be needed to determine when to exit
//         }
//     });
// }



// use futures::Future;
// use tokio::runtime::Builder;
// use tokio::time::{sleep, Duration};
// use futures::future::poll_fn;



// fn main() {
//     // Set up the Tokio runtime
//     let rt = Builder::new_current_thread().enable_all().build().unwrap();

//     // A task handle, but we do not directly poll this as we must drive the runtime instead
//     let task_handle = rt.spawn(async {
//         println!("Task started");
//         sleep(Duration::from_secs(1)).await;
//         println!("Task completed");
//     });

//     // Instead of using `poll_fn` directly on `task_handle`, use it to create a pollable task within the runtime
//     rt.block_on(async {
//         let mut task_fut = Box::pin(task_handle);

//         // Using `poll_fn` to keep trying to complete the task
//         let _resultt = poll_fn(|cx| {
//             // Properly pin and poll the future
//             task_fut.as_mut().poll(cx)
//         }).await;

//         println!("Task has been driven to completion");
//     });
// }




use futures::Future;
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use std::task::{Poll, Context};
 // For `.boxed()`

use futures::task::noop_waker_ref;

fn main() {
    // Set up the Tokio runtime
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    // Use a channel to signal the main event loop to stop
    let (tx, mut rx) = mpsc::channel::<()>(1);

    // Spawn the async task
    rt.spawn(async move {
        println!("Task started");
        time::sleep(Duration::from_secs(1)).await;
        println!("Task completed");
        tx.send(()).await.unwrap(); // Signal that the task is completed
    });

    // Simulated external event loop
    let mut future = Box::pin(rx.recv());
    let waker = noop_waker_ref();
    let mut context = Context::from_waker(&waker);

    // Non-blocking polling
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(Some(())) => {
                println!("Received completion signal, exiting loop");
                break;
            },
            Poll::Ready(None) => {
                println!("Channel closed unexpectedly");
                break;
            },
            Poll::Pending => {
                println!("No completion signal yet, doing other work...");
                // Simulate doing other work or sleeping briefly
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
