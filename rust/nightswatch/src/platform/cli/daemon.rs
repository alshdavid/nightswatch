use clap::Parser;

use crate::daemon::DaemonOptions;

#[derive(Clone, Debug, Parser)]
pub struct DaemonCommand {
  // /// Path to the socket file
  // #[arg(short = 's', long = "socket-path")]
  // pub socket_path: Option<PathBuf>,
  /// TCP address for clients
  #[arg(short = 't', long = "tcp-address")]
  pub tcp_address: Option<String>,
}

impl From<DaemonCommand> for DaemonOptions<String> {
  fn from(cmd: DaemonCommand) -> Self {
    DaemonOptions {
      tcp_address: cmd.tcp_address,
    }
  }
}
