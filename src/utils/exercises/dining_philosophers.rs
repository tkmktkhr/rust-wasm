use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
  name: String,
  left_fork: Arc<Mutex<Fork>>,
  right_fork: Arc<Mutex<Fork>>,
  thoughts: mpsc::SyncSender<String>,
}

impl Philosopher {
  fn think(&self) {
    self
      .thoughts
      .send(format!("Eureka! {} has a new idea!", &self.name))
      .unwrap();
  }

  fn eat(&self) {
    println!("{} is eating...", &self.name);
    thread::sleep(Duration::from_millis(10));
  }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

pub fn dining_philosophers() {
  let (tx, rx) = mpsc::sync_channel::<i32>(10);
  // Create forks
  let forks = (0..PHILOSOPHERS.len())
    .map(|_| Arc::new(Mutex::new(Fork)))
    .collect::<Vec<_>>();

  // Create philosophers

  // Make each of them think and eat 100 times

  // Output their thoughts
}
