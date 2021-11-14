//! Library used to mock PostgreSQL.

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

use std::net::TcpStream;
use std::os::unix::net::UnixStream;
mod authentication;
mod error_response;

/// Struct used to define the behaviour of the library.
pub struct Configuration {
  tcp_socket: Option<TcpStream>,
  bsd_socket: Option<UnixStream>,
}

impl Configuration {
  /// Create a new configuration object.
  pub fn new(tcp_socket: Option<TcpStream>, bsd_socket: Option<UnixStream>) -> Configuration {
    Configuration { tcp_socket, bsd_socket }
  }

  /// Return the TCP socket.
  pub fn tpc_socket(&self) -> Option<&TcpStream> {
    self.tcp_socket.as_ref()
  }

  /// Return the BSD socket.
  pub fn bsd_socket(&self) -> Option<&UnixStream> {
    self.bsd_socket.as_ref()
  }
}

/// Represents a PostgreSQL server.
pub struct Server<S: Messages> {
  /// Buffer used to store messages to be sent to the client.
  buffer: Vec<u8>,
  marker: std::marker::PhantomData<S>,
}

/// Trait used to anchor message types.
pub trait Messages {}

/// Methods available to all messages
impl<S> Server<S>
where
  S: Messages,
{
  /// Tell the socket to send the buffered data to the client.
  fn flush() {}
}

/// Startup messages sent by the Postgres server.
enum Startup {}
impl Messages for Startup {}
impl Server<Startup> {
  /// Create a Startup message.
  pub fn create(&self) {}
}
