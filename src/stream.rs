//! Abstraction between TCP and BSD sockets.

use tokio::{
  io,
  io::AsyncWriteExt,
  net::{TcpStream, UnixStream},
};

/// Converts a slice of 4 bytes to an array of 4 bytes.
#[inline]
pub(crate) fn slice_to_array(slice: &[u8]) -> [u8; 4] {
  slice.try_into().expect("Slice with incorrect length.")
}

/// Abstraction over TCP and BSD streams.
#[derive(Debug)]
pub(crate) enum Stream {
  Tcp(TcpStream),
  Unix(UnixStream),
}

impl Stream {
  pub(crate) fn try_read(&self, buf: &mut [u8]) -> io::Result<usize> {
    match self {
      Stream::Tcp(value) => value.try_read(buf),
      Stream::Unix(value) => value.try_read(buf),
    }
  }

  pub(crate) async fn readable(&self) -> io::Result<()> {
    match self {
      Stream::Tcp(value) => Ok(value.readable().await?),
      Stream::Unix(value) => Ok(value.readable().await?),
    }
  }

  pub(crate) async fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
    match self {
      Stream::Tcp(value) => Ok(value.write_all(buf).await?),
      Stream::Unix(value) => Ok(value.write_all(buf).await?),
    }
  }
}
