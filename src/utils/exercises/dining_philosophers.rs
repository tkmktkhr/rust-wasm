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
  let (tx, rx) = mpsc::sync_channel::<String>(10);
  // Create forks
  let forks = (0..PHILOSOPHERS.len())
    .map(|_| Arc::new(Mutex::new(Fork)))
    .collect::<Vec<_>>();

  for i in 0..forks.len() {
    let tx = tx.clone();
    let mut left_fork = Arc::clone(&forks[i]);
    let mut right_fork = Arc::clone(&forks[(i + 1) % forks.len()]);

    // To avoid a deadlock, we have to break the symmetry
    // somewhere. This will swap the forks without deinitializing
    // either of them.
    if i == forks.len() - 1 {
      std::mem::swap(&mut left_fork, &mut right_fork);
    }

    let philosopher = Philosopher {
      name: PHILOSOPHERS[i].to_string(),
      thoughts: tx,
      left_fork,
      right_fork,
    };

    thread::spawn(move || {
      for _ in 0..100 {
        philosopher.eat();
        philosopher.think();
      }
    });
  }

  // Output their thoughts
}
