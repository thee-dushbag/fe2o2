#![allow(unused)]

#[derive(Debug, Clone, Copy)]
struct P2 {
  x: f32,
  y: f32,
}

impl P2 {
  fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

fn sqr(p: P2) -> P2 {
  P2 {
    x: p.x * p.x,
    y: p.y * p.y,
  }
}

fn main() {
  let a = P2::new(1.5, 2.0);
  let b = P2::new(3.0, 4.0);
  let b = sqr(b);
  let b = sqr(a);
  println!("a = {a:?}");
  println!("b = {b:?}");
}
