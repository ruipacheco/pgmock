//! Implements a PostgreSQL server with version 3 of the wire protocol.

use super::Configuration;

/// Represents the backend process in the PostgreSQL architecture.
pub(crate) struct Backend {
  configuration: Configuration,
}

impl Backend {
  /// Creates a backend with user defined settings.
  pub(crate) fn new(configuration: Configuration) -> Self {
    Backend { configuration }
  }

  /// Returns the configuration used to create the server.
  pub(crate) fn configuration(self) -> Configuration {
    self.configuration
  }
}

impl Default for Backend {
  /// Creates a backend with default settings.
  fn default() -> Self {
    Backend {
      configuration: Configuration::default(),
    }
  }
}

#[cfg(test)]
mod tests {

  use super::super::*;
  use super::*;

  #[test]
  fn test_default_backend() {
    let backend = Backend::default();
    assert_eq!(backend.configuration().authentication_type(), AuthenticationType::Trust)
  }
}