use std::env;
use std::path::PathBuf;

pub struct Directory {
  pub root: PathBuf,
  pub current: PathBuf
}

impl Directory {
  pub fn new() -> Self {
    Self {
      root: env::home_dir().unwrap(),
      current: env::current_dir().unwrap()
    }
  }
}
