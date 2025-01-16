use std::io::Write;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

use crate::platform::messages::ConnectMessage;
use crate::platform::messages::MessageKind;
use crate::platform::socket::TcpServer;
use crate::NwResult;

#[derive(Debug)]
pub struct ClientOptions<A: ToSocketAddrs> {
  pub tcp_address: Option<A>,
}

pub struct Client {}

impl Client {
  pub fn connect<A: ToSocketAddrs>(options: ClientOptions<A>) -> NwResult<()> {
    let mut server = match options.tcp_address {
      Some(address) => TcpStream::connect(address)?,
      None => TcpStream::connect(TcpServer::default_address())?,
    };

    let msg = ConnectMessage {};
    let bytes = MessageKind::Connect(msg).to_socket_message()?;
    server.write_all(&bytes)?;
    Ok(())
  }
}
