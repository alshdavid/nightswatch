use std::path::PathBuf;

#[derive(Debug)]
pub struct DaemonOptions {
  pub socket_path: PathBuf,
}

pub fn start(options: DaemonOptions) {
  dbg!(&options);
}
