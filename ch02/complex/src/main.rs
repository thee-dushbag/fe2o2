use num::complex::Complex;

fn main() {
  let a = Complex { re: 2.1, im: -1.2 };
  let b = Complex::new(11.1, 22.2);
  let result = a + b;

  println!("{} + {} = {}", a, b, result);

  for i in 0..5 {
    println!("i = {}", i);
  }
  for i in 5..=10 {
    println!("i = {}", i);
  }

  let values: Vec<u8> = (5u8..15u8).collect();
  println!("|values| = {}", values.len());
  for value in values {
    println!("value = {}", value);
  }
}
