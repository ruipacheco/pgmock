//! Library used to mock the PostgreSQL server. Returns messages as defined here: https://www.postgresql.org/docs/14/protocol-flow.html#id-1.10.5.7.3

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

use std::io::Write;
use std::net::TcpStream;
use std::os::unix::net::UnixStream;

use std::{net::SocketAddr, path::PathBuf};
mod server;

type GenericError = Box<dyn std::error::Error + Send + Sync>;

/// Abstraction over the type of socket to be used.
pub(crate) enum Stream {
  Tcp(TcpStream),
  Unix(UnixStream),
}

impl Stream {
  /// Create a new stream based on configuration data.
  pub(crate) fn new(tcp_socket: Option<SocketAddr>, bsd_socket: Option<PathBuf>) -> Result<Stream, GenericError> {
    let mut stream: Stream;
    if bsd_socket.is_some() {
      stream = Stream::Unix(UnixStream::connect(bsd_socket.unwrap())?);
    } else {
      stream = Stream::Tcp(TcpStream::connect(tcp_socket.unwrap())?);
    }
    Ok(stream)
  }

  pub(crate) fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
    match self {
      Stream::Tcp(value) => Ok(value.write_all(buf)?),
      Stream::Unix(value) => Ok(value.write_all(buf)?),
    }
  }
}

/// Represents a PostgreSQL server.
pub struct Server<S: Messages> {
  marker: std::marker::PhantomData<S>,
  /// Buffer used to store messages to be sent to the client.
  buffer: Vec<u8>,
  stream: Stream,
}

/// Trait used to anchor message types.
pub trait Messages {}

/// Stopped server
pub enum Stopped {}
impl Messages for Stopped {}
impl Server<Stopped> {
  /// Create a new server instance.
  /// * `configuration` - Configuration data.
  pub fn new(tcp_socket: Option<SocketAddr>, bsd_socket: Option<PathBuf>) -> Result<Self, GenericError> {
    let stream = Stream::new(tcp_socket, bsd_socket)?;
    Ok(Server {
      marker: Default::default(),
      buffer: Default::default(),
      stream,
    })
  }
}

/// Methods available to all messages
impl<S> Server<S> where S: Messages {}
