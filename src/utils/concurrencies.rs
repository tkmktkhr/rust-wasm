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