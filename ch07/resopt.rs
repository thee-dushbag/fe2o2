#![allow(unused)]

pub mod snn {
  pub enum Result<T, E> {
    Okay(T),
    Error(E),
  }

  impl<T, E> Result<T, E> {
    pub fn ok(self) -> Option<T> {
      match self {
        Result::Okay(value) => Option::Some(value),
        Result::Error(_) => Option::None,
      }
    }

    pub fn err(self) -> Option<E> {
      match self {
        Result::Okay(_) => Option::None,
        Result::Error(error) => Option::Some(error),
      }
    }
  }

  pub enum Option<T> {
    Some(T),
    None,
  }

  impl<T> Option<T> {
    pub fn unwrap(self) -> T {
      match self {
        Option::Some(value) => value,
        Option::None => panic!(),
      }
    }
  }
}

#[inline(never)]
fn get(cond: bool) -> snn::Result<i32, i32> {
  if cond {
    snn::Result::Okay(4321)
  } else {
    snn::Result::Error(1234)
  }
}

fn custom(cond: bool) -> snn::Result<(), ()> {
  if cond {
    snn::Result::Okay(())
  } else {
    snn::Result::Error(())
  }
}

fn builtin(cond: bool) -> Result<(), ()> {
  if cond {
    Ok(())
  } else {
    Err(())
  }
}

fn main() {
  let res: snn::Result<i32, i32> = snn::Result::Okay(5678);
  let value = res.ok().unwrap();
  let v = builtin(true);
  v.ok().unwrap();
  v.err().or(Some(())).unwrap();
  builtin(true).ok().unwrap();
  custom(true).ok().unwrap();
  builtin(false).err().unwrap();
  custom(false).err().unwrap();
  // let error = res.err().unwrap(); // res already moved to snn::Result::<T, E>ok

  println!("get(true): {}", get(true).ok().unwrap());
  println!("get(false): {}", get(false).err().unwrap());
}
