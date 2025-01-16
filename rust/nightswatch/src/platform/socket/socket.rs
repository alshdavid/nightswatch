use std::path::PathBuf;

pub struct Socket {}

impl Socket {
  pub fn default_path() -> PathBuf {
    let exe_path = std::env::current_exe().expect("Cannot find executable path");
    let exe_path_dirname = exe_path.parent().expect("Cannot find parent of executable");
    let socket_path = exe_path_dirname.join("nightswatch.sock");
    socket_path.to_path_buf()
  }
}
