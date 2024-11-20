fn use_value(_value: i32) {}

struct Demo {
  a: i32,
}

fn use_demo(_demo: Demo) -> Demo { _demo }

fn main() {
  let demo = Demo { a: 321 };
  let demo = use_demo(demo);
  println!("{}", demo.a);
  let a = 123;
  use_value(a);
  println!("{a}");
}
