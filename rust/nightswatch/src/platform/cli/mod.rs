mod daemon;
mod watch;

use clap::Parser;
use clap::Subcommand;

pub use self::daemon::*;
pub use self::watch::*;

#[derive(Debug, Subcommand)]
pub enum CliCommandType {
  /// Start watcher daemon
  Daemon(DaemonCommand),
  /// Listen to watch directory
  Watch(WatchCommand),
}

#[derive(Parser, Debug)]
pub struct CliCommand {
  #[clap(subcommand)]
  pub command: CliCommandType,
}

impl CliCommand {
  pub fn from_process() -> Self {
    CliCommand::parse()
  }
}
