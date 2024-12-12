use libactionkv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
  akv_mem.exe FILE get KEY
  akv_mem.exe FILE delete KEY
  akv_mem.exe FILE insert KEY VALUE
  akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
  akv_mem FILE get KEY
  akv_mem FILE delete KEY
  akv_mem FILE insert KEY VALUE
  akv_mem FILE update KEY VALUE
";

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let fname = args.get(1).expect(&USAGE);
  let action = args.get(2).expect(&USAGE).as_ref();
  let key = args.get(3).expect(&USAGE).as_ref();
  let optional_value = args.get(4);

  let path = std::path::Path::new(fname);
  let mut store = ActionKV::open(path).expect("Unable to open file");
  store.load().expect("Unable to load data");

  match action {
    "get" => match store.get(key).unwrap() {
      None => eprintln!("{:?} not found", std::str::from_utf8(key).unwrap()),
      Some(value) => println!("{:?}", std::str::from_utf8(&value).unwrap()),
    },
    "delete" => store.delete(key).unwrap(),
    "insert" => store
      .insert(key, optional_value.expect(&USAGE).as_ref())
      .unwrap(),
    "update" => store
      .update(key, optional_value.expect(&USAGE).as_ref())
      .unwrap(),
    _ => eprintln!("{USAGE:?}"),
  }
}

