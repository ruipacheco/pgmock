//! Errors specific to the application.

use std::fmt::Display;

/// Application specific errors.
#[derive(Debug)]
pub(crate) enum Errors {
  ProtocolViolation { message: String },
}

impl Display for Errors {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      Errors::ProtocolViolation { message } => {
        write!(f, "{}", message)
      }
    }
  }
}
