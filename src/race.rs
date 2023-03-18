#[derive(Debug)]
struct Race {
  name: String,
  laps: Vec<i32>,
}

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
}

