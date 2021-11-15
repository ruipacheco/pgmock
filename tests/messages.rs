//! Integration tests for the library.

use std::{net::SocketAddr, path::PathBuf};

use pgservermock::{Server, Stopped};

#[test]
fn test_empty_configuration() {
  let server = Server::<Stopped>::new(None, None);
}
