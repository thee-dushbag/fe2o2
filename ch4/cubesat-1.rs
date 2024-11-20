#![allow(unused)]

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

#[derive(Debug)]
struct CubeSat {
  id: u64,
}

impl CubeSat {
  fn new(id: u64) -> CubeSat { CubeSat { id } }
}

fn check_status(sat_id: &CubeSat) -> StatusMessage { StatusMessage::Ok }

fn main() {
  let (sat_a, sat_b, sat_c) = (CubeSat::new(0), CubeSat::new(1), CubeSat::new(2));
  {
    let (a_stat, b_stat, c_stat) = (
      check_status(&sat_a),
      check_status(&sat_b),
      check_status(&sat_c),
    );
    println!("a: {a_stat:?}, b: {b_stat:?}, c: {c_stat:?}");
  }
  {
    let (a_stat, b_stat, c_stat) = (
      check_status(&sat_a),
      check_status(&sat_b),
      check_status(&sat_c),
    );
    println!("a: {a_stat:?}, b: {b_stat:?}, c: {c_stat:?}");
  }
}
