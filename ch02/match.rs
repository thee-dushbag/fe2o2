use std::convert::TryInto;

fn main() {
  let a: i32 = 10;
  let b: u64 = 2u64.pow(34);

  match <u64 as TryInto<i32>>::try_into(b) {
    Ok(n) => println!("Okay: {}", n),
    Err(e) => println!("Error: {:?}", e)
  }

  if a < b.try_into().unwrap() {
    println!("a < b");
  } else {
    println!("a >= b");
  }
}

