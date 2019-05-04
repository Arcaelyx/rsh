use std::io::Write;

mod structs;

use structs::Directory;

fn main() {
  let directory = Directory::new();
  let current = directory.current.iter().last().unwrap();
  let stdin = std::io::stdin();
  let mut stdout = std::io::stdout();
  let mut buffer = String::new();

  loop {
    write!(stdout, "{} λ ", current.to_str().unwrap()).ok();
    buffer.clear();
    stdout.flush().ok();
    stdin.read_line(&mut buffer).ok();
  }
}
