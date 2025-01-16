use std::path::PathBuf;

use clap::Parser;

use crate::client::ClientOptions;
use crate::client::WatchOptions;

#[derive(Clone, Debug, Parser)]
pub struct WatchCommand {
  /// TCP address for clients
  #[arg(short = 't', long = "tcp-address")]
  pub tcp_address: Option<String>,
  /// Filepath to watch
  pub target: PathBuf,
}

impl From<WatchCommand> for ClientOptions<String> {
  fn from(cmd: WatchCommand) -> Self {
    ClientOptions {
      tcp_address: cmd.tcp_address,
    }
  }
}

impl From<WatchCommand> for WatchOptions {
  fn from(cmd: WatchCommand) -> Self {
    WatchOptions { target: cmd.target }
  }
}
