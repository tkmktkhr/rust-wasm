use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
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
  println!("read1: username or error: {username:?}");
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
    Ok(_) => Ok(username),
    Err(err) => Err(err),
  }
}

// Converting Error types
#[derive(Debug)]
enum ReadUsernameError {
  IoError(io::Error),
  EmptyUsername(String),
}

impl Error for ReadUsernameError {}

impl Display for ReadUsernameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {e}"),
            Self::EmptyUsername(filename) => write!(f, "Found no username in {filename}"),
        }
    }
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> ReadUsernameError {
        ReadUsernameError::IoError(err)
    }
}

fn read_username1(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    println!("before returning results. {:?}", &username);
    Ok(username)
}

pub fn file_read2() {
    //fs::write("config.dat", "").unwrap();
    let username = read_username1("./src/utils/results.json");
    println!("read2: username or error: {username:?}");
}

// `expression?` works the same as
// match expression {
//   Ok(value) => value,
//   Err(err)  => return Err(From::from(err)),
// }