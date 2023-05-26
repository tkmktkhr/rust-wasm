use std::fs;
use std::io::{self, Read};


// Propagate Errors
// The try-operator ? is used to return errors to the caller. 
/*
  match some_expression {
    Ok(value) => value,
    Err(err) => return Err(err),
  }
*/

pub fn file_read() {
    // create new file
    // fs::write("results2.json", "alice").unwrap();
    // let username = read_username("./results2.json"); // absolute path.

    let username = read_username("./src/utils/results.json"); // absolute path.
    println!("username or error: {username:?}");
}

fn read_username(path: &str) -> Result<String, io::Error> {
  let username_file_result = fs::File::open(path);
  println!("{:?}", &username_file_result);
  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(err) => return Err(err),
  };

  let mut username = String::new();
  match username_file.read_to_string(&mut username) {
    Ok(_) =>  Ok(username),
    Err(err) => Err(err),
  }
}