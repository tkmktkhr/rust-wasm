use std::thread;
use std::time::Duration;

pub fn conc_thread() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  // Notice that the thread is stopped before it reaches 10 — the main thread is not waiting.
  // Use let handle = thread::spawn(...) and later handle.join() to wait for the thread to finish.
  // Trigger a panic in the thread, notice how this doesn’t affect main.
  // Use the Result return value from handle.join() to get access to the panic payload. This is a good time to talk about Any.
  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();
}

// Scoped threads
// NG
// Normal threads cannot borrow from their environment:
// pub fn normal_scoped_thread() {
//     let s = String::from("Hello");

//     thread::spawn(|| {
//         println!("Length: {}", s.len());
//     });
// }

// The reason for that is that when the thread::scope function completes, all the threads are guaranteed to be joined, so they can return borrowed data.
// Normal Rust borrowing rules apply: you can either borrow mutably by one thread, or immutably by any number of threads.

// pub fn normal_scoped_thread() {
//   let s = String::from("Hello");

//   thread::scope(|scope| {
//       scope.spawn(|| {
//           println!("Length: {}", s.len());
//       });
//   });
// }

// Channels
// mpsc stands for Multi-Producer, Single-Consumer. Sender and SyncSender implement Clone (so you can make multiple producers) but Receiver does not.
use std::sync::mpsc;
pub fn channel_thread() {
  let (tx, rx) = mpsc::channel();

  // send() and recv() return Result. If they return Err, it means the counterpart Sender or Receiver is dropped and the channel is closed.
  tx.send(10).unwrap();
  tx.send(20).unwrap();

  println!("Received: {:?}", rx.recv()); // Consumer is single.
  println!("Received: {:?}", rx.recv());

  let tx2 = tx.clone(); // multiple producers.
  tx2.send(30).unwrap();
  println!("Received: {:?}", rx.recv());
}

// Unbounded Channel
pub fn unbounded_channel() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let thread_id = thread::current().id();
    for i in 1..10 {
      tx.send(format!("Message {i}")).unwrap();
      println!("{thread_id:?}: sent Message {i}");
    }
    println!("{thread_id:?}: done");
  });
  thread::sleep(Duration::from_millis(100));

  for msg in rx.iter() {
    println!("Main: got {msg}");
  }
}

// Calling send will block the current thread until there is space in the channel for the new message. The thread can be blocked indefinitely if there is nobody who reads from the channel.
// A call to send will abort with an error (that is why it returns Result) if the channel is closed. A channel is closed when the receiver is dropped.
// A bounded channel with a size of zero is called a “rendezvous channel”. Every send will block the current thread until another thread calls read.

pub fn bounded_channel() {
  let (tx, rx) = mpsc::sync_channel(3);

  thread::spawn(move || {
    let thread_id = thread::current().id();
    for i in 1..10 {
      tx.send(format!("Message {i}")).unwrap();
      println!("{thread_id:?}: sent Message {i}");
    }
    println!("{thread_id:?}: done");
  });
  thread::sleep(Duration::from_millis(100));

  for msg in rx.iter() {
    println!("Main: got {msg}");
  }
}

// Send and Sync
// [unsafe trait] Send: a type T is Send if it is safe to move a T across a thread boundary.
// [unsafe trait] Sync: a type T is Sync if it is safe to move a &T across a thread boundary. // T is Sync if and only if &T is Send

// Send
// The effect of moving ownership to another thread is that destructors will run in that thread. So the question is when you can allocate a value in one thread and deallocate it in another.
// As an example, a connection to the SQLite library must only be accessed from a single thread.

// Shared State
// Rust uses the type system to enforce synchronization of shared data. This is primarily done via two types:
// Arc<T>, atomic reference counted T: handles sharing between threads and takes care to deallocate T when the last reference is dropped, a thread safe version of Rc that uses atomic operations.
// Mutex<T>: ensures mutually exclusive access to the T value. : ensures mutual exclusion and allows mutable access to T behind a read-only interface:

pub mod shared_state {
  use std::sync::{Arc, Mutex};
  use std::thread;

  pub fn arc() {
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();

    for _ in 1..5 {
      let v = Arc::clone(&v);
      handles.push(thread::spawn(move || {
        let thread_id = thread::current().id();
        println!("{thread_id:?}, {v:?}");
      }));
    }

    println!("v: {handles:?}");
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {v:?}");
  }

  pub fn mutex() {
    println!("Mutex sample");
    let v = Mutex::new(vec![10, 20, 30]);
    println!("v: {:?}", v.lock().unwrap());
    {
      let mut guard = v.lock().unwrap();
      guard.push(40);
    }
    println!("v: {:?}", v.lock().unwrap());
  }

  // v is wrapped in both Arc and Mutex, because their concerns are orthogonal.
  // Wrapping a Mutex in an Arc is a common pattern to share mutable state between threads.
  // v: Arc<_> needs to be cloned as v2 before it can be moved into another thread. Note move was added to the lambda signature.
  // Blocks are introduced to narrow the scope of the LockGuard as much as possible.
  pub fn arc_mutex() {
    let v = Arc::new(Mutex::new(vec![10, 20, 30]));

    let v2 = Arc::clone(&v);
    let handle = thread::spawn(move || {
      let mut v2 = v2.lock().unwrap();
      v2.push(10);
    });

    {
      let mut v = v.lock().unwrap();
      v.push(1000);
    }

    handle.join().unwrap();

    println!("v: {v:?}");
  }
}

// Async
// Rust’s asynchronous operation is based on “futures”, which represent work that may be completed in the future. Futures are “polled” until they signal that they are complete.
// Futures are polled by an async runtime, and several different runtimes are available.
// JavaScript’s Promise is similar, but again callback-based. The language runtime implements the event loop, so many of the details of Promise resolution are hidden.
use futures::executor::block_on;

// The “async” keyword is syntactic sugar. The compiler replaces the return type with a future.
async fn count_to(count: i32) -> i32 {
  for i in 1..=count {
    println!("Count is: {i}!");
  }
  count
}

// async fn async_main(count: i32) {
async fn async_main(count: i32) -> i32 {
  // .await asynchronously waits for the completion of another operation. Unlike block_on, .await doesn’t block the current thread.
  // .await can only be used inside an async function (or block).
  count_to(count).await
}

pub fn async_sample() {
  // You need an executor to run async code. block_on blocks the current thread until the provided future has run to completion.
  let future = block_on(async_main(3));
  println!("{:?}", future);
  block_on(async_main(5));
}

// Futures
// Future is a trait, implemented by objects that represent an operation that may not be complete yet. A future can be polled, and poll returns a Poll.
// The .await keyword, applied to a Future, causes the current async function to pause until that Future is ready, and then evaluates to its output.
// Context allows a Future to schedule itself to be polled again when an event occurs.
// Pin ensures that the Future isn’t moved in memory, so that pointers into that future remain valid. This is required to allow references to remain valid after an .await.

// Tokio
use tokio::time;

async fn tokio_count_to(count: i32) {
  for i in 1..=count {
    println!("Count in task: {i}!");
    time::sleep(time::Duration::from_millis(5)).await;
  }
}

#[tokio::main]
pub async fn tokio_sample() {
  println!("tokio sample!");
  let handle = tokio::spawn(tokio_count_to(10)); // Why does count_to not (usually) get to 10? This is an example of async cancellation. tokio::spawn returns a handle which can be awaited to wait until it finishes.

  // handle.await; // this awaits the spawn thread all(count) processes, then goes main tasks execution.

  for i in 1..5 {
    println!("Main task: {i}");
    time::sleep(time::Duration::from_millis(5)).await;
    println!("Main task done: {i}");
  }
  handle.await; // this awaits the spawn thread all(count) processes. Main and Sub tasks are executed at the same time.
}

// Tasks
// Rust has a task system, which is a form of lightweight threading.
// A task has a single top-level future which the executor polls to make progress.
// That future may have one or more nested futures that its poll method polls, corresponding loosely to a call stack. Concurrency within a task is possible by polling multiple child futures, such as racing a timer and an I/O operation.

// Async Channels
pub mod sample_async_channels {

  use tokio::sync::mpsc::{self, Receiver};

  async fn ping_handler(mut input: Receiver<()>) {
    let mut count: usize = 0;

    while let Some(_) = input.recv().await {
      count += 1;
      println!("Received {count} pings so far.");
    }

    println!("ping_handler complete");
  }

  #[tokio::main]
  pub async fn async_channels() {
    let (sender, receiver) = mpsc::channel(32);
    let ping_handler_task = tokio::spawn(ping_handler(receiver));
    for i in 0..10 {
      sender.send(()).await.expect("Failed to send ping.");
      println!("Sent {} pings so far.", i + 1);
    }

    drop(sender);
    ping_handler_task
      .await
      .expect("Something went wrong in ping handler task.");
  }
}

// Control Flow
// Join
pub mod sample_join {
  use anyhow::Result;
  use futures::future;
  use reqwest;
  use std::collections::HashMap;

  pub async fn size_of_page(url: &str) -> Result<usize> {
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?.len())
  }

  #[tokio::main]
  pub async fn join() {
    println!("start join sample.");
    let urls: [&str; 3] = [
      // change to 4 in order to occur an error.
      "https://google.com",
      "https://httpbin.org/ip",
      "https://play.rust-lang.org/",
      // "BAD_URL",
    ];
    let futures_iter = urls.into_iter().map(size_of_page);
    let results = future::join_all(futures_iter).await;
    let page_sizes_dict: HashMap<&str, Result<usize>> =
      urls.into_iter().zip(results.into_iter()).collect();
    println!("{:?}", page_sizes_dict);
  }
}

// A select operation waits until any of a set of futures is ready, and responds to that future’s result.
// In JavaScript, this is similar to Promise.race.
pub mod sample_select {
  use tokio::sync::mpsc::{self, Receiver};
  use tokio::time::{sleep, Duration};

  #[derive(Debug, PartialEq)]
  enum Animal {
    Cat { name: String },
    Dog { name: String },
  }

  async fn first_animal_to_finish_race(
    mut cat_rcv: Receiver<String>,
    mut dog_rcv: Receiver<String>,
  ) -> Option<Animal> {
    tokio::select! {
        cat_name = cat_rcv.recv() => Some(Animal::Cat { name: cat_name? }),
        dog_name = dog_rcv.recv() => Some(Animal::Dog { name: dog_name? })
    }
  }

  #[tokio::main]
  pub async fn select() {
    let (cat_sender, cat_receiver) = mpsc::channel(32);
    let (dog_sender, dog_receiver) = mpsc::channel(32);
    tokio::spawn(async move {
      sleep(Duration::from_millis(500)).await; // changeable
      cat_sender
        .send(String::from("Felix"))
        .await
        .expect("Failed to send cat.");
    });
    tokio::spawn(async move {
      sleep(Duration::from_millis(50)).await; // changeable
      dog_sender
        .send(String::from("Rex"))
        .await
        .expect("Failed to send dog.");
    });

    let winner = first_animal_to_finish_race(cat_receiver, dog_receiver)
      .await
      .expect("Failed to receive winner");

    println!("Winner is {winner:?}");
  }
}

pub mod sample_block_executor {
  use futures::future::join_all;
  use std::time::Instant;

  async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    println!(
      "future {id} slept for {duration_ms}ms, finished after {}ms",
      start.elapsed().as_millis()
    );
  }

  #[tokio::main(flavor = "current_thread")]
  pub async fn blocking_executor() {
    println!("start blocking executor sample.");
    let start = Instant::now();
    let sleep_futures = (1..=10).map(|t| sleep_ms(&start, t, t * 10));
    join_all(sleep_futures).await;
  }
}

// Pin
// When you await a future, all local variables (that would ordinarily be stored on a stack frame) are instead stored in the Future for the current async block. If your future has pointers to data on the stack, those pointers might get invalidated. This is unsafe.
// Therefore, you must guarantee that the addresses your future points to don’t change. That is why we need to pin futures. Using the same future repeatedly in a select! often leads to issues with pinned values.
pub mod sample_pin {

}