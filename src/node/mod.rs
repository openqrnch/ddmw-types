pub mod ddlnk;

use std::fmt;
use std::str::FromStr;

use crate::Error;

#[derive(Debug, PartialEq)]
pub enum Type {
  Sender,
  Receiver
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self {
      Type::Sender => "sender",
      Type::Receiver => "receiver"
    };
    write!(f, "{}", s)
  }
}

impl FromStr for Type {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "sender" => Ok(Type::Sender),
      "receiver" => Ok(Type::Receiver),
      _ => Err(Error::BadInput(format!("Unknown node::Type '{}'", s)))
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn type_to_string() {
    let t = Type::Sender;
    let s = format!("{}", t);
    assert_eq!(s, "sender");

    let t = Type::Receiver;
    let s = format!("{}", t);
    assert_eq!(s, "receiver");
  }

  #[test]
  fn string_to_type() {
    let t = "sender".parse::<Type>().unwrap();
    assert_eq!(t, Type::Sender);

    let t = "receiver".parse::<Type>().unwrap();
    assert_eq!(t, Type::Receiver);
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
