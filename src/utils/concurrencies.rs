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