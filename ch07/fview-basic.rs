#![allow(unused)]

use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main() {
  println!("Hello World!");
}
"#;

fn main() -> std::io::Result<()> {
  let mut buffer = Vec::<u8>::new();
  INPUT.read_to_end(&mut buffer)?;

  let mut position_in_input = 0;
  for line in buffer.chunks(BYTES_PER_LINE) {
    print!("[0x{:08x}] ", position_in_input);
    for byte in line {
      print!("{byte:02x} ");
    }
    for _ in 0..(BYTES_PER_LINE-line.len()) {
      print!("   ");
    }
    let string = std::str::from_utf8(line).unwrap();
    println!("|{}|", string.replace("\n", "."));
    position_in_input += BYTES_PER_LINE;
  }
  Ok(())
}
