#![allow(unused, dead_code)]

use std::cell::UnsafeCell;

fn main() {
  let value: i64 = 90;
  let pointer = &value as *const i64;
  println!("value={}, *pointer={}, &value={:p}, pointer={:p}", value, unsafe { *pointer }, &value, pointer);
  let mut age: i64 = 21;
  let p_age = &mut age as *mut i64;
  println!("age={}, *p_age={}, &age={:p}, p_age={:p}", age, unsafe { *p_age }, &age, p_age);
  unsafe { *p_age = 22; }
  println!("age={}, *p_age={}, &age={:p}, p_age={:p}", age, unsafe { *p_age }, &age, p_age);
}
