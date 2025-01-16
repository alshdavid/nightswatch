use std::io::Read;

use bincode;

use super::ConnectMessage;
use crate::NwResult;

pub enum MessageKind {
  Connect(ConnectMessage),
}

impl MessageKind {
  pub fn to_socket_message(&self) -> NwResult<Vec<u8>> {
    let (i, bytes): (u8, Vec<u8>) = match self {
      MessageKind::Connect(msg) => (0, bincode::serialize(&msg)?),
    };

    let len = bytes.len();
    let mut header = len.to_le_bytes().to_vec();
    let message_kind = i.to_le_bytes();

    header.extend(message_kind);
    header.extend(bytes);

    Ok(header)
  }

  pub fn from_reader<R: Read>(reader: &mut R) -> NwResult<Self> {
    // Get message length
    let mut buf_header = Box::new([0; 8]);
    match reader.read_exact(&mut *buf_header) {
      Ok(_) => {}
      Err(err) => return Err(err)?,
    };
    let message_len = usize::from_le_bytes(*buf_header);

    // Get message type
    let mut buf_type = Box::new([0; 1]);
    match reader.read_exact(&mut *buf_type) {
      Ok(_) => {}
      Err(err) => return Err(err)?,
    };
    let message_type = u8::from_le_bytes(*buf_type);

    // Get message body
    let mut buf_body = vec![0 as u8; message_len].into_boxed_slice();
    match reader.read_exact(&mut buf_body) {
      Ok(_) => {}
      Err(err) => return Err(err)?,
    };

    match message_type {
      0 => Ok(Self::Connect(bincode::deserialize::<ConnectMessage>(
        &buf_body,
      )?)),
      _ => Err("Unknown message")?,
    }
  }
}
