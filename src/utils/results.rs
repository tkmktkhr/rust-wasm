use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Read};
// use std::error::Error;
use thiserror::Error;

// Propagate Errors
// The try-operator ? is used to return errors to the caller.
/*
  match some_expression {
    Ok(value) => value,
    Err(err) => return Err(err),
  }
*/

pub fn file_read1() {
  // create new file
  // fs::write("results2.json", "alice").unwrap();
  // let username = read_username("./results2.json"); // absolute path.

  let username = read_username1("./src/utils/results.json"); // absolute path.
  println!("read1: username or error: {username:?}");
}

fn read_username1(path: &str) -> Result<String, io::Error> {
  let username_file_result = fs::File::open(path);
  println!("{:?}", &username_file_result);
  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(err) => return Err(err),
  };

  let mut username = String::new();
  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(err) => Err(err),
  }
}

// Converting Error types
// and
// Deriving Error Enums: thiserror
#[derive(Debug, Error)]
enum ReadUsernameError {
  #[error("This is thiserror macro - Could not read: {0}")]
  IoError(io::Error),
  #[error("This is thiserror macro - Found no username in {0}")]
  EmptyUsername(String),
}

// thiserror’s derive macro automatically implements std::error::Error
// impl Error for ReadUsernameError {} // this is unnecessary due to thiserror.

// thiserror’s derive macro optionally Display (if the #[error(...)] attributes are provided)
// this is unnecessary due to thiserror.
// impl Display for ReadUsernameError {
//   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//     match self {
//       Self::IoError(e) => write!(f, "IO error: {e}"),
//       Self::EmptyUsername(filename) => write!(f, "Found no username in {filename}"),
//     }
//   }
// }

// and From (if the #[from] attribute is added). It also works for structs.
impl From<io::Error> for ReadUsernameError {
  fn from(err: io::Error) -> ReadUsernameError {
    ReadUsernameError::IoError(err)
  }
}

fn read_username2(path: &str) -> Result<String, ReadUsernameError> {
  let mut username = String::with_capacity(100);
  File::open(path)?.read_to_string(&mut username)?;
  if username.is_empty() {
    return Err(ReadUsernameError::EmptyUsername(String::from(path)));
  }
  println!("before returning results read_username2. {:?}", &username);
  Ok(username)
}

pub fn file_read2() {
  //fs::write("config.dat", "").unwrap();
  let username = read_username2("./src/utils/results.json");
  match username {
    Ok(username) => println!("Username: {username}"),
    Err(err) => println!("Error: {err}"),
  }
  // println!("read2: username or error: {:?}", &username);
}

// `expression?` works the same as
// match expression {
//   Ok(value) => value,
//   Err(err)  => return Err(From::from(err)),
// }
