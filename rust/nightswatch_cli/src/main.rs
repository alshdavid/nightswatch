fn main() -> nightswatch::NwResult<()> {
  match nightswatch::cli::CliCommand::from_process().command {
    nightswatch::cli::CliCommandType::Daemon(daemon_command) => {
      nightswatch::daemon::start(daemon_command.into())
    }
    nightswatch::cli::CliCommandType::Watch(watch_command) => {
      nightswatch::client::Client::connect(watch_command.into())
    }
  }
}
