use std::fs::File;
use std::io::Read;

pub fn file_read() {
  let file = File::open("./src/utils/results.json"); // absolute path.
  match file {
    Ok(mut file) => {
      let mut contents = String::new();
      file.read_to_string(&mut contents);
      println!("File: {contents}");
    }
    Err(err) => {
      println!("The file could not be opened: {err}");
    }
  }
}

// Propagate Errors
// The try-operator ? is used to return errors to the caller. 
/*
  match some_expression {
    Ok(value) => value,
    Err(err) => return Err(err),
  }
*/
