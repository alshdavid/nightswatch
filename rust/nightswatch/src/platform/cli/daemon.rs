use std::path::PathBuf;

use clap::Parser;

use crate::{daemon::DaemonOptions, platform::socket::Socket};

#[derive(Debug, Parser)]
pub struct DaemonCommand {
  /// Path to the socket file
  #[arg(short = 's', long = "socket-path")]
  pub socket_path: Option<PathBuf>,
}

impl From<DaemonCommand> for DaemonOptions {
  fn from(value: DaemonCommand) -> Self {
    DaemonOptions {
      socket_path: value.socket_path.unwrap_or_else(Socket::default_path)
    }
  }
}