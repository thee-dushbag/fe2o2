pub mod outer {
  pub mod inner {
    pub fn func() {
      println!("Hello from outer::inner::func()");
    }
  }
  mod hidden_mod {
    pub(super) fn hidden() {
      println!("Hello from outer::hidden_mod::hidden()");
    }
    pub mod private_mod {
      pub(in super::super) fn private() {
        println!("Hello from outer::hidden_mod::private_mod::private()");
      }
    }
  }
  pub fn func() {
    println!("Hello from outer::func()");
  }
  pub fn hidden() { hidden_mod::hidden(); }
  pub fn private() { hidden_mod::private_mod::private(); }
}

pub fn free() {
  println!("Hello from free()");
}

