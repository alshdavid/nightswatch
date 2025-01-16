use std::net::TcpListener;
use std::net::ToSocketAddrs;
use std::thread;

use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::EventKind;
use notify_debouncer_full::notify::RecursiveMode;
use notify_debouncer_full::DebounceEventResult;

use crate::error::NwResult;
use crate::platform::messages::MessageKind;
use crate::platform::socket::TcpServer;

#[derive(Debug)]
pub struct DaemonOptions<A: ToSocketAddrs> {
  pub tcp_address: Option<A>,
}

pub fn start<A: ToSocketAddrs>(options: DaemonOptions<A>) -> NwResult<()> {
  let server = match options.tcp_address {
    Some(address) => TcpListener::bind(address)?,
    None => TcpListener::bind(TcpServer::default_address())?,
  };

  while let Ok((mut socket, _)) = server.accept() {
    thread::spawn::<_, NwResult<()>>(move || {
      println!("Client Connected");
      loop {
        match MessageKind::from_reader(&mut socket)? {
          MessageKind::Connect(connect_message) => {
            //
            dbg!(&connect_message);
          }
        };
      }
    });
  }

  Ok(())
}
