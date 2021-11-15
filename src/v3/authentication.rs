//! Messages sent during the authentication handshake.

use super::super::{Messages, Server};

pub(crate) enum Authentication {}
impl Messages for Authentication {}
impl Server<Authentication> {
  /// Add a NegotiateProtocolVersion message to the buffer.
  pub fn negotiate_protocol_version(&mut self) {
    todo!()
  }

  /// Add an AuthenticationOk message to the buffer.
  pub fn authentication_ok(&mut self) {
    self.buffer.append(&mut vec![b'R']);
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_authentication_ok() {}
}
