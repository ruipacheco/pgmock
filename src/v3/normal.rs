//! Messages sent by the server during the normal phase, after successful authentication.

use super::super::{Messages, Server};

pub(crate) enum Normal {}
impl Messages for Normal {}
impl Server<Normal> {}
