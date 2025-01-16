use clap::Parser;

use crate::client::ClientOptions;

#[derive(Debug, Parser)]
pub struct WatchCommand {
  /// TCP address for clients
  #[arg(short = 't', long = "tcp-address")]
  pub tcp_address: Option<String>,
}

impl From<WatchCommand> for ClientOptions<String> {
  fn from(cmd: WatchCommand) -> Self {
    ClientOptions {
      tcp_address: cmd.tcp_address,
    }
  }
}
