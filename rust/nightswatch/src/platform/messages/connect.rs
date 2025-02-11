use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WatchMessage {
  pub target: PathBuf,
}
