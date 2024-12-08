#![allow(unused)]

use std::rc::Rc;

#[derive(Debug)]
struct Person {
  name: &'static str,
  age: i32,
  friend: Option<Rc<Person>>,
}

impl Person {
  fn name(name: &'static str) -> Person {
    Person {
      name,
      age: -1,
      friend: None,
    }
  }
  fn age(age: i32) -> Person {
    Person {
      name: "<noname>",
      age,
      friend: None,
    }
  }
  fn new(name: &'static str, age: i32) -> Person {
    Person {
      name,
      age,
      friend: None,
    }
  }
  fn new_friend(name: &'static str, age: i32, friend: Rc<Person>) -> Person {
    Person {
      name,
      age,
      friend: Some(friend),
    }
  }
  fn greet(&self) {
    println!("Hello {}, you are {} years old!", self.name, self.age);
  }
  fn incage(&mut self) { self.age += 1; }
  fn add_friend(&mut self, friend: Rc<Person>) {
    self.friend = Some(friend);
  }
}

impl Drop for Person {
  fn drop(&mut self) {
    print!("{} died at the age of {} years old.", self.name, self.age);
    match &self.friend {
      Some(friend) => println!(" {} was very sad.", friend.name),
      None => println!(),
    }
  }
}

fn get_maintainer(friend: Option<Rc<Person>>) -> Person {
  let name = "Stephen Hillenburg";
  let age = 35;
  match &friend {
    Some(friend) => Person::new_friend(name, age, friend.clone()),
    None => Person::new(name, age),
  }
}

struct Array<const V: usize> {
  values: [i32; V]
}

fn main() {
  let values: Array<1000>;
  let nobody = Rc::new(Person::new("Nobody", 0));
  let god = Rc::new(Person::new_friend("God", 10000, nobody.clone()));
  let jesus = Rc::new(Person::new_friend("Jesus Christ", 80, god.clone()));
  let people: [Person; 3] = [
    Person::new_friend("Ariana Grande", 34, jesus.clone()),
    Person::new_friend("Jeff Bezos", 67, god.clone()),
    Person::new_friend("Megan Lopez", 28, jesus.clone()),
  ];
  for person in &people {
    person.greet();
  }
  let mut him = get_maintainer(Some(nobody.clone()));
  him.greet();
  Person::incage(&mut him);
  Person::greet(&him);
}
