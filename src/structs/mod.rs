use std::env;
use std::path::PathBuf;

pub struct Directory {
  pub home: PathBuf,
  pub current: PathBuf
}

impl Directory {
  pub fn new() -> Self {
    Self {
      home: env::home_dir().unwrap(),
      current: env::current_dir().unwrap()
    }
  }
}
