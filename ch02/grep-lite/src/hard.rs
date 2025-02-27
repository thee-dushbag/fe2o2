use std::{
  fs::File,
  io::{BufRead, BufReader},
};

use clap::{App, Arg};
use regex::Regex;

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
        .required(true),
    )
    .get_matches();
  let pattern = args.value_of("pattern").unwrap();
  let re = Regex::new(pattern).unwrap();
  let file = File::open(args.value_of("input").unwrap()).unwrap();
  let content = BufReader::new(file);
  //  let content = "\
  //Every face, every shop, bedroom window,
  //public-house, and dark square is a picture
  //feverishly turned--in search of what?
  //It is the same with books. What do we seek
  //through millions of pages?";
  for line in content.lines().map(|line| line.unwrap()) {
    //let contains_substring = re.find(line);
    match re.find(&line) {
      Some(_) => println!("{}", line),
      None => (),
    }
  }
}
