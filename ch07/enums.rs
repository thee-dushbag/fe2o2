#![allow(unused)]

enum Payload {
  Empty,
  Age(i32),
  Rgb {
    r: u8,
    g: u8,
    b: u8,
  },
  Name(&'static str),
  School {
    name: &'static str,
    country: &'static str,
  },
}

fn main() {
  let payloads = vec![
    Payload::Age(60),
    Payload::Name("Kenya"),
    Payload::Rgb {
      r: 19,
      g: 34,
      b: 56,
    },
    Payload::Name("Simon"),
    Payload::School {
      name: "Jomo Kenya University",
      country: "Kenya",
    },
    Payload::Name("Faith"),
    Payload::School {
      name: "Harvard University",
      country: "USA",
    },
    Payload::Name("Lydia"),
  ];

  let mut val = Payload::Name("Simon Nganga");

  if let Payload::Name(name) = &mut val {
    println!("Hello {name}?");
    *name = "Lola Bunny";
    println!("Hello {name}?");
  }

  if let Payload::Age(age) = payloads.get(0).unwrap() {
    println!(
      "You are {} to drive!",
      if *age >= 18 {
        if *age > 50 {
          "Too Old"
        } else {
          "Old Enough"
        }
      } else {
        "Too Young"
      }
    );
    //*age = 5052;
  } else {
    println!("Expected Age object.");
  }

  println!("sizeof(&str)={}", size_of::<&str>());
  println!("sizeof(Payload)={}", size_of::<Payload>());
  for payload in &payloads {
    match payload {
      Payload::Empty => continue,
      Payload::Age(age) => println!("You are {age} years old!"),
      Payload::Rgb { r, g, b } => println!("RGB(red={r}, green={g}, blue={b})"),
      Payload::Name(name) => println!("Your name is {name}!"),
      Payload::School { name, country } => println!("The school {name} can be found in {country}."),
    }
  }
}
