//! Errors specific to the application.

use std::fmt::Display;

/// Application specific errors.
#[derive(Debug)]
pub(crate) enum Errors {
  ProtocolViolation { message: String },
  InvalidAuthorizationSpecification { message: String },
  InvalidPassword { user: String }
}

impl Display for Errors {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      Errors::ProtocolViolation { message } => {
        write!(f, "{}", message)
      }
      Errors::InvalidAuthorizationSpecification { message } => {
        write!(f, "{}", message)
      }
      Errors::InvalidPassword { user } => {
        write!(f, "password authentication failed for user {}", user)
      }
    }
  }
}
