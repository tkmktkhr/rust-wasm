// Adding context to Errors
use anyhow::{bail, Context, Result};
use std::io::Read;
use std::{fs, io};

fn read_username(path: &str) -> Result<String> {
  // type alias for Result<V, anyhow::Error>.
  println!("--- Start errors.rs");
  let mut username = String::with_capacity(100);
  fs::File::open(path)
    .with_context(|| format!("Failed to open {path}"))?
    .read_to_string(&mut username)
    .context("Failed to read")?;
  if username.is_empty() {
    bail!("Found no username in {path}");
  }
  Ok(username)
}

pub fn add_context_error() {
  // fs::write("results1.json", "Alice").unwrap();
  match read_username("./src/utils/results-no.json") {
    Ok(username) => println!("Username: {username}"),
    Err(err) => println!("Error: {err:?}"),
  }
}
