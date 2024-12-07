#![allow(unused)]

#[derive(Debug)]
struct CubeSat {
  id: u64,
  mailbox: MailBox,
}

impl CubeSat {
  fn recv(&mut self) -> Option<Message> { self.mailbox.messages.pop() }
}

#[derive(Debug)]
struct MailBox {
  messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
  fn send(&self, to: &mut CubeSat, msg: Message) { to.mailbox.messages.push(msg); }
}

fn main() {
  let mut sat = CubeSat {
    id: 100,
    mailbox: MailBox { messages: vec![] },
  };
  let base = GroundStation {};
  base.send(&mut sat, "Hello?".to_string());
  match sat.recv() {
    Some(message) => println!("Received: {message}"),
    None => println!("No Messages Intercepted"),
  }
}
