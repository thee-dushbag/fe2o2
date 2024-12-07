#![allow(unused)]

fn fu32(v: f32) -> u32 { unsafe { std::mem::transmute(v) } }
fn uf32(v: u32) -> f32 { unsafe { std::mem::transmute(v) } }

fn f32asu32() {
  let f_value = 100.25f32;
  let u_value = fu32(f_value);
  println!("f_value: {}", f_value);
  println!("u_value: {:08x}", u_value);
  println!("{} == {}", uf32(u_value), f_value);
  assert_eq!(u_value, 0x42C8_8000);
  assert_eq!(uf32(u_value), f_value);
}

fn overflow() {
  let mut value: u16 = 0;
  print!("{value}..");
  loop {
    value += 1;
    if value % 10_000 == 0 {
      print!("{value}..");
    }
  }
}

fn main() {
  f32asu32();
  //overflow();
}
