use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::net::TcpListener;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use normalize_path::NormalizePath;
use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::EventKind;
use notify_debouncer_full::notify::INotifyWatcher;
use notify_debouncer_full::notify::RecursiveMode;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::Debouncer;
use notify_debouncer_full::NoCache;

use crate::error::NwResult;
use crate::platform::broadcast::channel_broadcast;
use crate::platform::broadcast::Subscribable;
use crate::platform::messages::ChangeMessage;
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

  let watched_paths = Arc::new(Mutex::new(HashMap::<
    PathBuf,
    (Debouncer<INotifyWatcher, NoCache>, Vec<Sender<MessageKind>>),
  >::new()));
  let debouncers = Arc::new(Mutex::new(
    HashMap::<PathBuf, Vec<Sender<MessageKind>>>::new(),
  ));
  let (tx_events, rx_events) = channel::<DebounceEventResult>();

  thread::spawn({
    let watched_paths = watched_paths.clone();

    move || {
      println!("stareted");

      while let Ok(mut result) = rx_events.recv().unwrap() {
        println!("update");
        let mut paths = vec![];

        while let Some(ev) = result.pop() {
          match ev.event.kind {
            EventKind::Create(_) => {}
            EventKind::Modify(_) => {}
            EventKind::Remove(_) => {}
            _ => continue,
          }
          paths.extend(ev.paths.clone());
        }

        if paths.is_empty() {
          continue;
        }

        let mut watched_paths = watched_paths.lock().unwrap();
        for (watched_path, (_, txs)) in watched_paths.iter() {
          for tx in txs {
            tx.send(MessageKind::Change(ChangeMessage {})).unwrap();
          }
        }
      }
      println!("done");
    }
  });

  while let Ok((mut socket, _)) = server.accept() {
    println!("Client Connected");

    let (tx_write, rx_write) = channel::<MessageKind>();
    let (tx_read, rx_read) = channel::<MessageKind>();

    thread::spawn::<_, NwResult<()>>({
      let mut socket = socket.try_clone()?;
      move || {
        while let Ok(msg) = rx_write.recv() {
          let bytes = msg.to_socket_message().unwrap();
          socket.write_all(&bytes).unwrap();
        }
        Ok(())
      }
    });

    thread::spawn::<_, NwResult<()>>(move || loop {
      match MessageKind::from_reader(&mut socket) {
        Ok(msg) => tx_read.send(msg).unwrap(),
        Err(_err) => panic!("Unhandled"),
      };
    });

    thread::spawn::<_, NwResult<()>>({
      let watched_paths = watched_paths.clone();
      let tx_events = tx_events.clone();

      move || {
        while let Ok(msg) = rx_read.recv() {
          match msg {
            MessageKind::Watch(msg) => {
              let target_path = msg.target.normalize();

              println!("watching {:?}", target_path);

              let mut watched_paths = watched_paths.lock().unwrap();
              if !watched_paths.contains_key(&target_path) {
                println!("watching {:?}", target_path);

                let mut debouncer: Debouncer<INotifyWatcher, NoCache> =
                  new_debouncer(Duration::from_millis(1000), None, tx_events.clone()).unwrap();

                debouncer
                  .watch(&target_path, RecursiveMode::Recursive)
                  .unwrap();
                watched_paths.insert(target_path.clone(), (debouncer, vec![]));
              }

              watched_paths
                .get_mut(&msg.target)
                .unwrap()
                .1
                .push(tx_write.clone());
            }
            MessageKind::Change(_) => {}
          };
        }

        Ok(())
      }
    });
  }

  Ok(())
}
