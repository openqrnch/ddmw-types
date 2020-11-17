use std::fmt;

#[derive(Debug)]
pub enum Error {
  BadInput(String)
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      Error::BadInput(s) => {
        write!(f, "Bad input error; {}", s)
      }
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
