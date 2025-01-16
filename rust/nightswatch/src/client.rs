use std::io::Write;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

use normalize_path::NormalizePath;

use crate::platform::broadcast::channel_broadcast;
use crate::platform::broadcast::Subscribable;
use crate::platform::messages::MessageKind;
use crate::platform::messages::WatchMessage;
use crate::platform::socket::TcpServer;
use crate::NwResult;

#[derive(Debug)]
pub struct ClientOptions<A: ToSocketAddrs> {
  pub tcp_address: Option<A>,
}

#[derive(Debug)]
pub struct WatchOptions {
  pub target: PathBuf,
}

pub struct Client {
  writer: Sender<MessageKind>,
  reader: Subscribable<MessageKind>,
}

impl Client {
  pub fn connect<A: ToSocketAddrs>(options: ClientOptions<A>) -> NwResult<Self> {
    let mut server = match options.tcp_address {
      Some(address) => TcpStream::connect(address)?,
      None => TcpStream::connect(TcpServer::default_address())?,
    };

    let (tx_write, rx_write) = channel::<MessageKind>();
    let (tx_read, rrx_read) = channel_broadcast();

    std::thread::spawn({
      let mut server = server.try_clone()?;
      move || {
        while let Ok(msg) = rx_write.recv() {
          let bytes = msg.to_socket_message().unwrap();
          server.write_all(&bytes).unwrap();
        }
      }
    });

    std::thread::spawn(move || loop {
      match MessageKind::from_reader(&mut server) {
        Ok(msg) => tx_read.send(msg).unwrap(),
        Err(_err) => panic!("Unhandled"),
      };
    });

    Ok(Self {
      writer: tx_write,
      reader: rrx_read,
    })
  }

  pub fn watch_dir(
    &self,
    mut options: WatchOptions,
  ) -> NwResult<impl Iterator<Item = ()>> {
    let (tx, rx) = std::sync::mpsc::channel::<()>();

    if options.target.is_relative() {
      options.target = std::env::current_dir()?.join(options.target)
    }
    options.target = options.target.normalize();

    std::thread::spawn({
      let rrx = self.reader.subscribe();
      move || {
        while let Ok(_msg) = rrx.recv() {
          tx.send(()).unwrap();
        }
      }
    });

    self.writer.send(MessageKind::Watch(WatchMessage {
      target: options.target,
    }))?;

    Ok(rx.into_iter())
  }
}
