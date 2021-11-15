//! An incomplete in-memory implementation of PostgreSQL designed to mock a real PostgreSQL server in tests.
//! Each server instance will have exactly one thread responding to requests on a TCP socket with the behaviour
//! of the server being decided by the configuration struct.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(non_ascii_idents)]
#![deny(unreachable_pub)]
#![deny(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_results)]
#![warn(noop_method_call)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(absolute_paths_not_starting_with_crate)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
mod v3;

type GenericError = Box<dyn std::error::Error + Send + Sync>;

/// How the server expects the user to authenticate itself.
#[derive(Debug, PartialEq)]
pub enum AuthenticationType {
  /// Server requires no password
  Trust,
  /// Server requires a plain text password
  AuthenticationCleartextPassword,
  /// Server requires an MD5 password
  AuthenticationMD5Password,
  /// Server performs a SASL handshake
  AuthenticationSASL,
}

/// Holds configuration that defines the behaviour of the server.
#[derive(Debug)]
pub struct Configuration {
  user: String,
  password: Option<String>,
  dbname: Option<String>,
  hostaddr: SocketAddr,
  authentication_type: AuthenticationType,
}

impl Configuration {
  /// Creates a configuration object with user supplied values.
  pub fn new(user: String, password: Option<String>, dbname: Option<String>, hostaddr: SocketAddr, authentication_type: AuthenticationType) -> Self {
    Configuration {
      user,
      password,
      dbname,
      hostaddr,
      authentication_type,
    }
  }

  /// Returns database username.
  pub fn user(self) -> String {
    self.user
  }

  /// Returns database password.
  pub fn password(self) -> Option<String> {
    Some(self.password.unwrap_or_default())
  }

  /// Database the client will connect to.
  pub fn dbname(self) -> Option<String> {
    Some(self.dbname.unwrap_or_default())
  }

  /// Socket the database will listen in.
  pub fn hostaddr(self) -> SocketAddr {
    self.hostaddr
  }

  /// The type of authentication the database will perform.
  pub fn authentication_type(self) -> AuthenticationType {
    self.authentication_type
  }
}

impl Default for Configuration {
  /// Creates a configuration object with default settings.
  fn default() -> Self {
    Configuration {
      user: "postgres".to_owned(),
      password: None,
      dbname: None,
      hostaddr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
      authentication_type: AuthenticationType::Trust,
    }
  }
}

/// Represents the postmaster in a PostgreSQL server.
/// It spawns a number of threads, each representing a backend process in the traditional PostgreSQL architecture.
#[derive(Debug)]
pub struct Server {
  configuration: Configuration,
}

impl Server {
  /// Creates a new server instance with user defined settings.
  pub fn new(configuration: Configuration) -> Self {
    Server { configuration }
  }

  /// Returns the configuration used to create the server.
  pub fn configuration(self) -> Configuration {
    self.configuration
  }
}

impl Default for Server {
  /// Creates a server instance with default settings.
  fn default() -> Self {
    Server {
      configuration: Configuration::default(),
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_default_configuration() {
    let configuration = Configuration::default();
    assert_eq!(configuration.authentication_type, AuthenticationType::Trust)
  }

  #[test]
  fn test_default_server() {
    let server = Server::default();
    assert_eq!(server.configuration().authentication_type(), AuthenticationType::Trust)
  }
}
