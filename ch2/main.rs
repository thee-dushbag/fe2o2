use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T { a + b }

fn main() {
  let (a, b) = (44, 56);
  println!("{} + {} = {}", a, b, add(a, b));
}

