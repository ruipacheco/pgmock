//! Implements methods related to the ErrorResponse message.

use super::{Messages, Server};

pub(crate) enum ErrorResponse {}

impl Messages for ErrorResponse {}

impl Server<ErrorResponse> {}
