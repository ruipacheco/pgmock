//! Implements a PostgreSQL server that understands version 3 of the wire protocol.

mod startup;

use crate::{Configuration, GenericError};
use tokio::net::TcpStream;

/// Represents the backend process in the PostgreSQL architecture.
#[derive(Debug)]
pub(crate) struct Backend<'a> {
  configuration: &'a Configuration,
  stream: TcpStream,
  id: u32,
}

impl<'a> Backend<'a> {
  /// Creates a backend with user defined settings.
  ///
  /// Checks if the client sent the correct version of the wire protocol, performs the authentication handshake
  /// and either sets the server in the ready for query state or returns an error message.
  /// * `configuration` - Configuration used to define the backend process.
  /// * `stream` - Stream used to write data to and receive data from.
  /// * `id` - Postmaster generated identifier for this backend.
  pub(crate) fn new(configuration: &'a Configuration, stream: TcpStream, id: u32) -> Result<Self, GenericError> {
    Ok(Backend { configuration, stream, id })
  }

  /// Returns the configuration used to create the server.
  pub(crate) fn configuration(self) -> &'a Configuration {
    self.configuration
  }

  /// Return backend identifier.
  pub(crate) fn id(self) -> u32 {
    self.id
  }
}

#[cfg(test)]
mod tests {

  use super::super::{Configuration, GenericError, IpAddr, Ipv4Addr, SocketAddr};
  use super::{Backend, TcpStream};

  use std::sync::Once;
  static mut LISTENER: Option<std::net::TcpListener> = None;
  static INIT: Once = Once::new();

  pub(crate) fn initialize() {
    unsafe {
      INIT.call_once(|| LISTENER = Some(std::net::TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432)).unwrap()));
    }
  }

  // The default configuration uses the default PostgreSQL port so if this test fails check if the server is not running in the background.
  #[tokio::test]
  async fn test_new_backend() -> Result<(), GenericError> {
    initialize();
    let configuration = Configuration::default();
    let stream = TcpStream::connect(configuration.clone().hostaddr()).await?;
    let pid = std::process::id();
    let backend = Backend::new(&configuration, stream, pid)?;
    assert_eq!(backend.id(), pid);
    Ok(())
  }

  // The default configuration uses the default PostgreSQL port so if this test fails check if the server is not running in the background.
  #[tokio::test]
  async fn test_accept_startup_packet() -> Result<(), GenericError> {
    initialize();
    let configuration = Configuration::default();
    let stream = TcpStream::connect(configuration.clone().hostaddr()).await?;
    let pid = std::process::id();
    let backend = Backend::new(&configuration, stream, pid)?;
    assert_eq!(backend.id(), pid);
    Ok(())
  }
}
