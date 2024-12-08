#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
  let arg1 = env::args().nth(1);
  let fname = arg1.expect("usage: fview FILENAME");

  let mut f = File::open(&fname).expect("Unable to open file.");
  let mut pos = 0;
  let mut buffer = [0u8; BYTES_PER_LINE];

  while let Ok(_) = f.read_exact(&mut buffer) {
    print!("{:08x}  ", pos);
    let mut counter = 0;
    for byte in &mut buffer {
      print!("{byte:02x} ");
      if counter == 7 {
        print!(" ");
      }
      *byte = match *byte {
        0..32 => b'.',
        127..=255 => b'.',
        32..=126 => *byte,
      };
      counter += 1;
    }

    println!(" |{}|", std::str::from_utf8(&buffer).unwrap());
    pos += BYTES_PER_LINE;
  }
  Ok(())
}
