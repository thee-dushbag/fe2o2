#![allow(unused)]

mod mymodule;
use std::ops::Index;

#[derive(Copy, Clone)]
struct A {
  a: i32,
}

impl Default for A {
  fn default() -> Self { Self { a: 5052 } }
}

impl std::ops::Add<u32> for A {
  type Output = ();
  fn add(self, _rhs: u32) -> Self::Output {
    println!("A + u32");
  }
}

impl std::ops::AddAssign<&u32> for A {
  fn add_assign(&mut self, _rhs: &u32) {
    println!("A += u32");
  }
}

impl std::ops::Deref for A {
  type Target = ();
  fn deref(&self) -> &Self::Target {
    println!("*A");
    &()
  }
}

impl std::ops::Mul<u32> for A {
  type Output = ();
  fn mul(self, _rhs: u32) -> Self::Output {
    println!("A * u32");
  }
}

impl std::ops::MulAssign<u32> for A {
  fn mul_assign(&mut self, _rhs: u32) {
    println!("A *= u32");
  }
}

impl std::cmp::PartialEq for A {
  fn eq(&self, _other: &Self) -> bool { self.a == _other.a }
}

impl std::cmp::PartialOrd for A {
  fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.a.partial_cmp(&_other.a)?)
  }
}

impl std::ops::Index<u32> for A {
  type Output = i32;
  fn index(&self, _index: u32) -> &Self::Output { &self.a }
}

impl std::ops::IndexMut<u32> for A {
  fn index_mut(&mut self, _index: u32) -> &mut Self::Output { &mut self.a }
}

impl std::fmt::Display for A {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "A{{ a: {} }}", self.a)?;
    Ok(())
  }
}

//impl Drop for A {
//  fn drop(&mut self) {
//    println!("Drop::drop(&mut A{{a:{}}})", self.a);
//  }
//}

struct Foo<'a> {
  val: Option<&'a Bar<'a>>,
}

struct Bar<'a> {
  val: Option<&'a Foo<'a>>,
}

impl Iterator for A {
  type Item = i32;
  fn next(&mut self) -> Option<Self::Item> {
    match self.a {
      0 | 10 => None,
      _ => {
        let res = Some(self.a);
        self.a = if self.a > 0 { self.a - 1 } else { self.a + 1 };
        res
      }
    }
  }
}

//impl<'a> Words<'a> {
//  fn new(string: &'a [u8]) -> Self { Self { string, pos: 0 } }
//}
//
//impl<'a> Iterator for Words<'a> {
//  type Item = &'a [u8];
//  fn next(&mut self) -> Option<Self::Item> {
//    for i in self.pos..self.string.len() {
//      if self.string[i] == b' ' {
//        let word = self.string.get(self.pos..i);
//        self.pos = i + 1;
//        return word;
//      }
//    }
//    let last_word = if self.pos == self.string.len() {
//      None
//    } else {
//      self.string.get(self.pos..self.string.len())
//    };
//    self.pos = self.string.len();
//    last_word
//  }
//}

struct Words<'a> {
  string: &'a str,
  pos: usize,
}

impl<'a> Words<'a> {
  fn new(string: &'a str) -> Self { Self { string, pos: 0 } }
}

impl<'a> Iterator for Words<'a> {
  type Item = &'a str;
  fn next(&mut self) -> Option<Self::Item> {
    for (i, ch) in self.string.char_indices().skip(self.pos) {
      if ch == ' ' {
        let word = self.string.get(self.pos..i);
        self.pos = i + 1;
        return word;
      }
    }
    let last_word = if self.pos == self.string.len() {
      None
    } else {
      self.string.get(self.pos..self.string.len())
    };
    self.pos = self.string.len();
    last_word
  }
}

fn main() {
  mymodule::free();
  mymodule::outer::func();
  mymodule::outer::inner::func();
  mymodule::outer::private();
  mymodule::outer::hidden();
  let v: Vec<u8>;
  let sentence = "My name is John Doe Juan and I work really hard, <3 ;) #JDJ";
  println!("WORDS: {:?}", sentence);
  for word in Words::new(sentence) {
    println!("WORD: {word:?}");
  }
  let mut a = A { a: 20 };
  for current in &mut a {
    println!("current = {current}");
  }
  println!("END");
  a.a -= 1;
  for current in &mut a {
    println!("current = {current}");
  }
  let mut name = String::from("Simon Nganga");

  {
    let sirname = &mut name;
    println!("My name is {name}!");
    //sirname.push_str(" Njoroge");
  }

  let a = A { a: 65 };
  let mut b = A { a: 66 };
  let ab: (A, A) = (a, b);
  b.a = a.a + b.a;
  println!("We have {} {}", ab.1.a, b.a);
}

#[allow(unused)]
fn main_() {
  let a = [1, 2, 3, 4, 5];

  println!("Please enter an array index.");

  let mut index = String::new();

  std::io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

  let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

  let element = a[index];

  println!("The value of the element at index {index} is: {element}");
}

#[allow(unused, non_snake_case)]
fn helloWorld() {
  let a: A = Default::default();
  a + 34u32;
  let mut a: A = Default::default();
  a += &56u32;
  *a;
  a *= 45;
  a * 9;
  let a: A = Default::default();
  let b = A { a: 6000 };
  if b == a {
    println!("{b} == {a}");
  } else {
    println!("{b} != {a}");
  }
  if b < a {
    println!("{b} < {a}");
  } else {
    println!("{b} >= {a}");
  }
  if b > a {
    println!("{b} > {a}");
  } else {
    println!("{b} <= {a}");
  }
  if b <= a {
    println!("{b} <= {a}");
  } else {
    println!("{b} > {a}");
  }
  if b >= a {
    println!("{b} >= {a}");
  } else {
    println!("{b} < {a}");
  }
}
