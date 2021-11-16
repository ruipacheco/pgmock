//! Integration tests for the library.

use rustgres::{AuthenticationType, Configuration, Postmaster};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// The default configuration uses the default PostgreSQL port so if this test fails check if the server is not running in the background.
#[test]
fn test_server_default_configuration() {
  let postmaster = Postmaster::default();
  let started = postmaster.start();
  assert!(started.is_ok());
}

#[test]
fn test_server_custom_configuration() {
  let configuration = Configuration::new(
    "postgres".to_owned(),
    None,
    None,
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000),
    AuthenticationType::Trust,
  );
  let postmaster = Postmaster::new(configuration);
  let started = postmaster.start();
  assert!(started.is_ok());
}
