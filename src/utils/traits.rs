// Trait
// Rust lets you abstract over types with traits. They’re similar to interfaces:

trait Pet {
  fn name(&self) -> String;
}

struct Dog {
  name: String,
}

struct Cat;

// pub fn trait() {

impl Pet for Dog {
  fn name(&self) -> String {
    self.name.clone()
  }
}

impl Pet for Cat {
  fn name(&self) -> String {
    String::from("The cat") // No name, cats won't respond to it anyway.
  }
}

fn greet<P: Pet>(pet: &P) {
  println!("Who's a cutie? {} is!", pet.name());
}

pub fn trait_sample() {
  let fido = Dog {
    name: "Fido".into(),
  };
  greet(&fido);

  let captain_floof = Cat;
  greet(&captain_floof);

  let pets: Vec<Box<dyn Pet>> = vec![
    Box::new(Dog {
      name: String::from("Fido"),
    }),
    Box::new(Cat),
  ];
  for pet in pets {
    println!("Hello {}!", pet.name());
  }

  // Types that implement a given trait may be of different sizes. This makes it impossible to have things like Vec<Pet> in the example above.
  // dyn Pet is a way to tell the compiler about a dynamically sized type that implements Pet.
  // In the example, pets holds fat pointers to objects that implement Pet. The fat pointer consists of two components, a pointer to the actual object and a pointer to the virtual method table for the Pet implementation of that particular object.
  println!(
    "{} {}",
    std::mem::size_of::<Dog>(),
    std::mem::size_of::<Cat>()
  );
  println!(
    "{} {}",
    std::mem::size_of::<&Dog>(),
    std::mem::size_of::<&Cat>()
  );
  println!("{}", std::mem::size_of::<&dyn Pet>());
  println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
  name: String,
  strength: u8,
  hit_points: u8,
}

pub fn trait_derive() {
  let p1 = Player::default();
  let p2 = p1.clone();
  println!(
    "Is {:?}\nequal to {:?}?\nThe answer is {}!",
    &p1,
    &p2,
    if p1 == p2 { "yes" } else { "no" }
  );
}
