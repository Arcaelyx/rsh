use std::io::Write;

fn main() {
  let stdin = std::io::stdin();
  let mut stdout = std::io::stdout();
  let mut buffer = String::new();

  loop {
    write!(stdout, "λ ");
    buffer.clear();
    stdout.flush().ok();
    stdin.read_line(&mut buffer).ok();
  }
}
