#[derive(Debug)]
struct Race {
  name: String,
  laps: Vec<i32>,
}

// method receiver.
// &self: borrows the object from the caller using a shared and immutable reference. The object can be used again afterwards.
// &mut self: borrows the object from the caller using a unique and mutable reference. The object can be used again afterwards.
// self: takes ownership of the object and moves it away from the caller. The method becomes the owner of the object. The object will be dropped (deallocated) when the method returns, unless its ownership is explicitly transmitted.
// mut self: same as above, but while the method owns the object, it can mutate it too. Complete ownership does not automatically mean mutability.
// No receiver: this becomes a static method on the struct. Typically used to create constructors which are called new by convention.

impl Race {
  fn new(name: &str) -> Race {
    // No receiver, a static method
    Race {
      name: String::from(name),
      laps: Vec::new(),
    }
  }

  fn add_lap(&mut self, lap: i32) {
    // Exclusive borrowed read-write access to self
    self.laps.push(lap);
  }

  fn print_laps(&self) {
    // Shared and read-only borrowed access to self
    println!("Recorded {} laps for {}:", self.laps.len(), self.name);
    for (idx, lap) in self.laps.iter().enumerate() {
      println!("Lap {idx}: {lap} sec.");
    }
  }

  fn finish(self) {
    // Exclusive ownership of self
    let total = self.laps.iter().sum::<i32>();
    println!("Race {} is finished, total lap time: {}", self.name, total);
  }
}

pub fn fn_race() {
  let mut race = Race::new("Monaco Grand Prix");
  race.add_lap(70);
  race.add_lap(55);
  race.print_laps();
  race.add_lap(35);
  race.print_laps();
  race.finish()
  // race.add_lap(35)
}
