#![allow(dead_code, unused)]

use std::mem::size_of;
use std::ffi::{c_char, CStr};

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
  let a: usize = 42;
  let b: &[u8; 10] = &B;
  let c: Box<[u8]> = Box::new(C);

  println!("a (unsigned integer):");
  println!("  location: {:p}", &a);
  println!("  size:     {:?} bytes", size_of::<usize>());
  println!("  value:    {:?}\n", a);

  println!("b (reference to B):");
  println!("  location: {:p}", &b);
  println!("  size:     {:?} bytes", size_of::<&[u8; 10]>());
  println!("  value:    {:p}\n", b);

  println!("c (a \"box\" for C):");
  println!("  location: {:p}", &c);
  println!("  size:     {:?} bytes", size_of::<Box<[u8]>>());
  println!("  value:    {:p}\n", c);

  println!("B (array of 10 bytes):");
  println!("  location: {:p}", &B);
  println!("  size:     {:?} bytes", size_of::<[u8; 10]>());
  println!("  value:    {:?}\n", B);

  println!("C (array of 11 bytes):");
  println!("  location: {:p}", &C);
  println!("  size:     {:?} bytes", size_of::<[u8; 11]>());
  println!("  value:    {:?}\n", C);
}
