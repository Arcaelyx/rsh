pub fn parse(buffer: &str) {
  let commands = buffer.trim().split_whitespace().collect::<Vec<_>>();
  if let [ command ] = commands[..] {
    match command {
      _ => println!("rsh: {}: command not found", command)
    }
  }
}
