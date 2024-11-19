use clap::{App, Arg};
use regex::Regex;
use std::{
  fs::File,
  io::{self, BufRead, BufReader},
};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
  for line in reader.lines().map(|line| line.unwrap()).enumerate() {
    match re.find(&line.1) {
      Some(_) => println!("{}: {}", line.0 + 1, line.1),
      None => (),
    }
  }
}

fn main() {
  let args = App::new("grep-lite")
    .version("0.1")
    .about("searches for patterns")
    .arg(
      Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true),
    )
    .arg(
      Arg::with_name("input")
        .help("File to search")
        .takes_value(true)
        .required(false),
    )
    .get_matches();
  let pattern = args.value_of("pattern").unwrap();
  let re = Regex::new(pattern).unwrap();

  let input = args.value_of("input").unwrap_or("-");
  if input == "-" {
    process_lines(io::stdin().lock(), re);
  } else {
    let file = File::open(input).unwrap();
    process_lines(BufReader::new(file), re);
  };
}
