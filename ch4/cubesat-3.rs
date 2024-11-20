#![allow(unused)]

#[derive(Debug)]
struct CubeSat {
  id: u64,
  mailbox: MailBox,
}

impl CubeSat {
  fn recv(&self, mailbox: &mut MailBox) -> Option<Message> { mailbox.deliver(&self) }
}

#[derive(Debug)]
struct MailBox {
  messages: Vec<Message>,
}

impl MailBox {
  fn post(&mut self, msg: Message) { self.messages.push(msg); }

  fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
    for (index, message) in self.messages.iter().enumerate() {
      if message.to == recipient.id {
        return Some(self.messages.remove(index));
      }
    }
    None
  }
}

#[derive(Debug)]
struct Message {
  to: u64,
  content: String,
}

struct GroundStation;

impl GroundStation {
  fn send(&self, mailbox: &mut MailBox, msg: Message) { mailbox.post(msg); }
  fn connect(&self, sat_id: u64) -> CubeSat {
    CubeSat {
      id: sat_id,
      mailbox: MailBox { messages: vec![] },
    }
  }
}

fn fetch_sat_ids() -> [u64; 6] { [1, 2, 3, 4, 5, 6] }

fn main() {
  let base = GroundStation {};
  let mut mailbox = MailBox { messages: vec![] };
  for sat_id in fetch_sat_ids() {
    let mut sat = base.connect(sat_id);
    if sat_id & 1 == 1 {
      continue;
    }
    base.send(
      &mut mailbox,
      Message {
        to: sat.id,
        content: format!("Hello {}?", sat.id),
      },
    );
  }
  for sat_id in fetch_sat_ids() {
    let mut sat = base.connect(sat_id);
    match sat.recv(&mut mailbox) {
      Some(message) => println!("Sat({}): {:?}", sat.id, message),
      None => println!("Sat({})[NoMessage]", sat.id),
    }
  }
}
