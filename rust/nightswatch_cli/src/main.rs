fn main() -> nightswatch::NwResult<()> {
  match nightswatch::cli::CliCommand::from_process().command {
    nightswatch::cli::CliCommandType::Daemon(daemon_command) => {
      nightswatch::daemon::start(daemon_command.into())
    }
    nightswatch::cli::CliCommandType::Watch(watch_command) => {
      let client = nightswatch::client::Client::connect(watch_command.clone().into())?;

      for _update in client.watch_dir(watch_command.into())? {
        println!("update");
      }

      Ok(())
    }
  }
}
