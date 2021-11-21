//! Holds the code necessary to manage the startup of a Backend.
//!
//! Receives the startup packet, handles protocol negotiation and authenticates the user.

use crate::errors::Errors;
use crate::stream::{slice_to_array, Stream};
use std::collections::HashMap;

static NAME_DATA_LEN: i32 = 64;
static MAXIMUM_STARTUP_PACKET_LENGTH: usize = 10000;
static SUPPORTED_PROTOCOL_VERSION: i32 = 196608;

impl Stream {
  /// PostgreSQL packets follow the TLV format except for the first packet which does not have a type.
  ///
  /// Instead of implementing a single read_all method with conditions to cater to a single packet it's easier to implement a separate method to
  /// handle that exception.
  pub(crate) async fn read_first_packet(self) -> Result<Vec<u8>, Errors> {
    todo!()
  }
}

/// Represents packets received from the client.
#[derive(Debug, PartialEq)]
pub(crate) enum FrontEndFrames {
  /// First message sent by the client when connection is opened. The only mandatory parameter is 'user' and an unknown number of parameters may be 
  /// sent so we store everything in a HashMap.
  StartupMessage { parameters: HashMap<String, String> },
}

impl TryFrom<&Vec<u8>> for FrontEndFrames {
  type Error = Errors;
  fn try_from(packet: &Vec<u8>) -> Result<Self, Self::Error> {
    if packet.len() > MAXIMUM_STARTUP_PACKET_LENGTH {
      return Err(Errors::ProtocolViolation {
        message: "invalid length of startup packet".to_owned(),
      });
    }
    
    let len: Vec<u8> = packet.clone().into_iter().take(4).collect();
    let length = i32::from_be_bytes(slice_to_array(&len[..4])) as usize;
    if length != packet.len() {
      return Err(Errors::ProtocolViolation {
        message: "incomplete startup packet".to_owned(),
      });
    }

    if packet.last().unwrap() != &0 {
      return Err(Errors::ProtocolViolation {
        message: "invalid startup packet layout: expected terminator as last byte".to_owned(),
      });
    }
    let mut parameters = HashMap::new();
    let protocol_version_barray: Vec<u8> = packet.clone().into_iter().skip(4).take(4).collect();
    let protocol_version = i32::from_be_bytes(slice_to_array(&protocol_version_barray[..4]));
    if protocol_version != SUPPORTED_PROTOCOL_VERSION {
      return Err(Errors::ProtocolViolation {
        message: "unsupported frontend protocol".to_owned(),
      });
    }
    let inserted = parameters.insert("protocol_version".to_owned(), protocol_version.to_string());

    // Vector index.
    let mut number_processed_bytes: usize = 8;
    loop {
      // Message ends with a null character so we know we're done processing by checking the length minus the last byte.
      if number_processed_bytes == length - 1 {
        break;
      }
      let mut name: Option<String> = None;
      let mut value: Option<String> = None;
      if let Some(name_terminating_byte) = packet.clone().into_iter().skip(number_processed_bytes).position(|x| x == 0) {
        let tmp: Vec<u8> = packet
          .clone()
          .into_iter()
          .skip(number_processed_bytes)
          .take(name_terminating_byte)
          .collect();
        number_processed_bytes = number_processed_bytes + tmp.len() + 1;
        name = Some(String::from_utf8(tmp).unwrap());
      }
      if let Some(value_terminating_byte) = packet.clone().into_iter().skip(number_processed_bytes).position(|x| x == 0) {
        let tmp: Vec<u8> = packet
          .clone()
          .into_iter()
          .skip(number_processed_bytes)
          .take(value_terminating_byte)
          .collect();
        number_processed_bytes = number_processed_bytes + tmp.len() + 1;
        value = Some(String::from_utf8(tmp).unwrap());
      }
      let inserted = parameters.insert(name.unwrap(), value.unwrap());
    }
    if let None = parameters.keys().into_iter().find(|&x| x == &"user".to_owned()) {
      return Err(Errors::InvalidAuthorizationSpecification {
        message: "no PostgreSQL user name specified in startup packet".to_owned(),
      });
    }
    if let None = parameters.keys().into_iter().find(|&x| x == &"database".to_owned()) {
      let user = parameters[&"user".to_owned()].clone();
      let inserted = parameters.insert("database".to_owned(), user.clone());
    }
    Ok(FrontEndFrames::StartupMessage { parameters })
  }
}

#[cfg(test)]
mod tests {

  use super::FrontEndFrames;
  use crate::errors::Errors;

  #[test]
  fn test_startup_message_try_from() {
    // 00 00 00 56 00 03 00 00 75 73 65 72 00 72 75 69  ...V....user.rui
    // 70 61 63 68 65 63 6f 00 64 61 74 61 62 61 73 65  pacheco.database
    // 00 70 6f 73 74 67 72 65 73 00 61 70 70 6c 69 63  .postgres.applic
    // 61 74 69 6f 6e 5f 6e 61 6d 65 00 70 73 71 6c 00  ation_name.psql.
    // 63 6c 69 65 6e 74 5f 65 6e 63 6f 64 69 6e 67 00  client_encoding.
    // 55 54 46 38 00 00                                UTF8..
    let packet = vec![
      0x00, 0x00, 0x00, 0x56, 0x00, 0x03, 0x00, 0x00, 0x75, 0x73, 0x65, 0x72, 0x00, 0x72, 0x75, 0x69, 0x70, 0x61, 0x63, 0x68, 0x65, 0x63, 0x6f, 0x00,
      0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x00, 0x70, 0x6f, 0x73, 0x74, 0x67, 0x72, 0x65, 0x73, 0x00, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63,
      0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x00, 0x70, 0x73, 0x71, 0x6c, 0x00, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x65,
      0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x55, 0x54, 0x46, 0x38, 0x00, 0x00,
    ];
    match FrontEndFrames::try_from(&packet) {
      Ok(FrontEndFrames::StartupMessage { parameters }) => {
        assert_eq!(parameters["user"], "ruipacheco".to_owned());
        assert_eq!(parameters["database"], "postgres".to_owned());
        assert_eq!(parameters["application_name"], "psql".to_owned());
        assert_eq!(parameters["client_encoding"], "UTF8".to_owned());
      }
      _ => {
        panic!("Wrong enum!")
      }
    }

    // TODO Truncate username and database to postgres length

    // TODO If database is missing the username becomes the database name

    /*
    // TODO Missing 'user' data - incomplete, define length as 70 bytes
    let packet = vec![
      0x00, 0x00, 0x00, 0x56, 0x00, 0x03, 0x00, 0x00, 
      0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x00, 0x70, 0x6f, 0x73, 0x74, 0x67, 0x72, 0x65, 0x73, 0x00, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63,
      0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x00, 0x70, 0x73, 0x71, 0x6c, 0x00, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x65,
      0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x55, 0x54, 0x46, 0x38, 0x00, 0x00,
    ];
    let result = FrontEndFrames::try_from(&packet);
    assert!(matches!(result, Err(Errors::InvalidAuthorizationSpecification { message })));
    */

    // Packet without null terminator as last byte.
    let packet = vec![
      0x00, 0x00, 0x00, 0x56, 0x00, 0x03, 0x00, 0x00, 0x75, 0x73, 0x65, 0x72, 0x00, 0x72, 0x75, 0x69, 0x70, 0x61, 0x63, 0x68, 0x65, 0x63, 0x6f, 0x00,
      0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x00, 0x70, 0x6f, 0x73, 0x74, 0x67, 0x72, 0x65, 0x73, 0x00, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63,
      0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x00, 0x70, 0x73, 0x71, 0x6c, 0x00, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x65,
      0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x55, 0x54, 0x46, 0x38, 0x00, 0x00, 0x71,
    ];
    let result = FrontEndFrames::try_from(&packet);
    assert!(matches!(result, Err(Errors::ProtocolViolation { message })));

    // Packet with declared length different from real length.
    let packet = vec![
      0x00, 0x00, 0x00, 0x57, 0x00, 0x03, 0x00, 0x00, 0x75, 0x73, 0x65, 0x72, 0x00, 0x72, 0x75, 0x69, 0x70, 0x61, 0x63, 0x68, 0x65, 0x63, 0x6f, 0x00,
      0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x00, 0x70, 0x6f, 0x73, 0x74, 0x67, 0x72, 0x65, 0x73, 0x00, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63,
      0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x00, 0x70, 0x73, 0x71, 0x6c, 0x00, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x65,
      0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x55, 0x54, 0x46, 0x38, 0x00, 0x00,
    ];
    let result = FrontEndFrames::try_from(&packet);
    assert!(matches!(result, Err(Errors::ProtocolViolation { message })));

    // Packet with more than 10000 bytes.
    let mut packet: Vec<u8> = Vec::new();
    loop {
      if packet.len() > 10000 {
        break;
      }
      packet.push(0x00);
    }
    let result = FrontEndFrames::try_from(&packet);
    assert!(matches!(result, Err(Errors::ProtocolViolation { message })));

    // Unsupported protocol.
    let packet = vec![
      0x00, 0x00, 0x00, 0x56, 0x00, 0x04, 0x00, 0x00, 0x75, 0x73, 0x65, 0x72, 0x00, 0x72, 0x75, 0x69, 0x70, 0x61, 0x63, 0x68, 0x65, 0x63, 0x6f, 0x00,
      0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x00, 0x70, 0x6f, 0x73, 0x74, 0x67, 0x72, 0x65, 0x73, 0x00, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63,
      0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x00, 0x70, 0x73, 0x71, 0x6c, 0x00, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x65,
      0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x55, 0x54, 0x46, 0x38, 0x00, 0x00,
    ];
    let result = FrontEndFrames::try_from(&packet);
    assert!(matches!(result, Err(Errors::ProtocolViolation { message })));
  }

  #[test]
  fn test_reading_startup_packet() {
    todo!()
  }
}
