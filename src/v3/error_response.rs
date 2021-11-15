/*
//! Implements methods related to the ErrorResponse message.

use super::super::{Messages, Server};

pub(crate) enum ErrorResponse {}
impl Messages for ErrorResponse {}
impl Server<ErrorResponse> {
  /// Add an ErrorResponse message to the buffer.
  pub fn error_response(&mut self) {
    todo!()
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_wrong_username_error() {}
}
*/
