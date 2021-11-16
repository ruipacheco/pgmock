//! An incomplete in-memory implementation of PostgreSQL designed to mock a real PostgreSQL server in tests.
//! Each server instance will have exactly one thread responding to requests on a TCP socket with the behaviour
//! of the server being decided by the configuration struct.

#![warn(unsafe_code)]
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

mod errors;
mod stream;
mod v3;

use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process;
use tokio::net::TcpListener;
use v3::Backend;

/// Error returned by the library.
pub type GenericError = Box<dyn Error + Send + Sync + 'static>;

/// Authentication schemes supported by PostgreSQL.
#[derive(Debug, PartialEq, Clone)]
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

/// Configuration data used to define the behaviour of the server.
#[derive(Debug, Clone)]
pub struct Configuration {
  user: String,
  password: Option<String>,
  dbname: Option<String>,
  hostaddr: SocketAddr,
  authentication_type: AuthenticationType,
}

impl Configuration {
  /// Creates a configuration object with user supplied values.
  /// * `user` - Database username.
  /// * `password` - Database password.
  /// * `dbname` - Database the client will connect to.
  /// * `hostadd` - Socket the backend will listen in.
  /// * `authentication_type` - The type of authentication the backend will perform.
  pub fn new(user: String, password: Option<String>, dbname: Option<String>, hostaddr: SocketAddr, authentication_type: AuthenticationType) -> Self {
    Configuration {
      user,
      password,
      dbname,
      hostaddr,
      authentication_type,
    }
  }

  /// Database username.
  pub fn user(self) -> String {
    self.user
  }

  /// Database password.
  pub fn password(self) -> Option<String> {
    self.password
  }

  /// Database the client will connect to.
  pub fn dbname(self) -> Option<String> {
    self.dbname
  }

  /// Socket the backend will listen in.
  pub fn hostaddr(self) -> SocketAddr {
    self.hostaddr
  }

  /// The type of authentication the backend will perform.
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

/// Represents the Postmaster in the PostgreSQL architecture.
/// It spawns a number of threads, each representing a backend process that in turn handles user commands.
#[derive(Debug)]
pub struct Postmaster<'a> {
  configuration: Configuration,
  backends: Vec<Backend<'a>>,
  pid: u32,
}

impl<'a> Postmaster<'a> {
  /// Creates a new server instance with user defined settings.
  pub fn new(configuration: Configuration) -> Self {
    Postmaster {
      configuration,
      backends: Vec::new(),
      pid: process::id(),
    }
  }

  /// Starts a loop listening for messages from the client.
  /// While PostgreSQL starts a new process once a client connects, we start a new thread and store it in a vector.
  /// Reference: https://www.postgresql.org/docs/14/connect-estab.html
  #[tokio::main]
  pub async fn start(self) -> Result<(), GenericError> {
    // https://stackoverflow.com/a/55874334/70600
    let mut this = self;
    loop {
      let listener = TcpListener::bind(this.configuration.clone().hostaddr()).await?;
      match listener.accept().await {
        Ok((stream, _addr)) => {
          let id = this.pid + 1;
          let backend = Backend::new(&this.configuration, stream, id)?;
          this.backends.push(backend);
        }
        // TODO Return or log and continue?
        Err(e) => todo!("Log error accepting client connection."),
      }
    }
    unreachable!()
  }

  /// Returns the configuration used to create the server.
  pub fn configuration(self) -> Configuration {
    self.configuration
  }

  /// Returns the number of back ends available.
  pub fn number_backends(self) -> usize {
    self.backends.len()
  }

  /// Returns the process id of the Postmaster.
  pub fn pid(self) -> u32 {
    self.pid
  }
}

impl<'a> Default for Postmaster<'a> {
  /// Creates a server instance with default settings.
  fn default() -> Self {
    Postmaster {
      configuration: Configuration::default(),
      backends: Vec::new(),
      pid: std::process::id(),
    }
  }
}

#[cfg(test)]
mod tests {

  use super::{AuthenticationType, Configuration, Postmaster};

  #[test]
  fn test_default_configuration() {
    let configuration = Configuration::default();
    assert_eq!(configuration.authentication_type, AuthenticationType::Trust)
  }

  #[test]
  fn test_default_server() {
    let server = Postmaster::default();
    assert_eq!(server.pid(), std::process::id());
  }
}
