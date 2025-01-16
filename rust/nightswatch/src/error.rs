use std::sync::mpsc::SendError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("std::io::Error")]
  StdIo(#[from] std::io::Error),
  #[error("GenericError")]
  Generic(String),
  #[error("BincodeError")]
  Bincode(#[from] bincode::Error),
}

impl<T> From<SendError<T>> for Error {
  fn from(value: SendError<T>) -> Self {
    Self::Generic(value.to_string())
  }
}

impl<Guard> From<std::sync::PoisonError<Guard>> for Error {
  fn from(value: std::sync::PoisonError<Guard>) -> Self {
    Self::Generic(value.to_string())
  }
}

impl From<String> for Error {
  fn from(value: String) -> Self {
    Self::Generic(value)
  }
}

impl From<&str> for Error {
  fn from(value: &str) -> Self {
    Self::Generic(value.to_string())
  }
}

pub type NwResult<T> = Result<T, Error>;
