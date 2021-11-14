//! Methods related to Postgres authentication handshake.

use super::Messages;

enum AuthenticationOk {}

impl Messages for AuthenticationOk {}
