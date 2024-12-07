#![allow(unused)]

use std::fmt::{Debug, Display};
use std::str::Bytes;

#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
  state: State,
}

fn write<'a>(content: &'a [u8], file: &'a mut File) -> Result<usize, &'a str> {
  if !matches!(file.state, State::OPENED) {
    return Err("File not open for writing");
  }
  file.data.extend(content);
  Ok(content.len())
}

impl File {
  fn new(name: String) -> File {
    File {
      name,
      data: Vec::new(),
      state: State::default(),
    }
  }

  fn new_with_data(name: String, data: &Vec<u8>) -> File {
    File {
      name,
      data: data.clone(),
      state: State::default(),
    }
  }

  fn open(&mut self) -> Result<(), &str> {
    if matches!(self.state, State::OPENED) {
      return Err("File already open");
    }
    self.state = State::OPENED;
    Ok(())
  }

  fn read(&self, to: &mut Vec<u8>) -> Result<usize, &str> {
    if !matches!(self.state, State::OPENED) {
      return Err("File not open for reading");
    }
    to.reserve(self.data.len());
    let mut tmp = self.data.clone();
    to.append(&mut tmp);
    Ok(self.data.len())
  }

  fn close(&mut self) -> Result<(), &str> {
    if matches!(self.state, State::OPENED) {
      self.state = State::CLOSED;
      return Ok(());
    }
    Err("File not open")
  }
}

#[derive(Debug)]
enum State {
  UNKNOWN,
  OPENED,
  CLOSED,
  LOCKED,
}

impl Default for State {
  fn default() -> Self { State::UNKNOWN }
}

impl Display for State {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      State::OPENED => write!(f, "state::opened"),
      State::CLOSED => write!(f, "state::closed"),
      State::LOCKED => write!(f, "state::locked"),
      State::UNKNOWN => write!(f, "state::unknown"),
    }
  }
}

impl Display for File {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "File({}): '{}' {:?}", self.state, self.name, self.data)
  }
}

fn main() {
  let mut f = File::new("hello.txt".to_string());
  println!("{}", f);
  f.open()
    .inspect(|x| println!("File opened successfully"))
    .unwrap();
  println!("{}", f);
  write(b"Hello World", &mut f)
    .inspect(|x| println!("File written successfully"))
    .unwrap();
  println!("{}", f);
  f.close()
    .inspect(|x| println!("File closed successfully"))
    .unwrap();
  println!("{}", f);
  let name = &f.name;
  let mut len = &mut f.data.len();
  println!("{} is {} bytes long.", name, len);
  println!("{}", f);
}
