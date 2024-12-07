#[allow(dead_code)]
fn isupper(letter: u8) -> bool { 64 < letter && letter < 91 }

#[allow(dead_code)]
fn tolower(letter: u8) -> u8 {
  if isupper(letter) {
    letter + 32u8
  } else {
    letter
  }
}

#[allow(dead_code)]
fn islower(letter: u8) -> bool { 96 < letter && letter < 123 }

#[allow(dead_code)]
fn toupper(letter: u8) -> u8 {
  if islower(letter) {
    letter - 32u8
  } else {
    letter
  }
}

#[allow(dead_code)]
fn no_implicit_conversions(a: u16, b: u32) -> bool {
  // a < b // WRONG
  (a as u32) < b // OK
}

#[allow(dead_code)]
fn toggle(letter: u8) -> u8 {
  if isupper(letter) {
    letter + 32u8
  } else if islower(letter) {
    letter - 32u8
  } else {
    letter
  }
}

#[allow(dead_code)]
fn floats() {
  let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
  let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

  println!("abc (f32)");
  println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
  println!("        0.3: {:x}", abc.2.to_bits());

  println!("xyz (f64)");
  println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
  println!("        0.3: {:x}", xyz.2.to_bits());

  // assert!(xyz.0 + xyz.1 == xyz.2); // CRASH
  assert!(abc.0 + abc.1 == abc.2); // OK
}

#[allow(dead_code)]
fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T { x + y }

fn main() {
  println!("{} + {} = {}", 23, 34, add(23u32, 34u32));
  println!("round: {:.2}", 45.678f32);
  floats();
  let a: u8 = 65;
  println!("Letter: {}", a as char);
  println!("0.1 + 0.2 = {:.30}", 0.1 + 0.2);
  println!("0.3 = {:.30}", 0.3);
  // assert!(0.5 + 0.25 == 0.75);
  assert!(0.1f32 + 0.2f32 == 0.3f32);
}
