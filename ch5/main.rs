#![allow(unused)]

struct CPU {
  registers: [u8; 16],
  memory: [u8; 0x7ff],
  position: usize,
  stack: [u16; 16],
  stack_top: usize,
}

enum Flags {
  None = 0,
  Negative = 1,
  Overflow = 2,
  Zero = 4,
  LessThan = 8,
  Above = 32,
  LessEqual = 12,
  AboveEqual = 36,
}

impl CPU {
  fn read_opcode(&self) -> u16 {
    ((self.memory[self.position] as u16) << 8) | (self.memory[self.position + 1] as u16)
  }
  fn get_reg(&self, reg: u8) -> u8 { self.registers[reg as usize] }
  fn set_reg(&mut self, reg: u8, val: u8) { self.registers[reg as usize] = val; }

  fn set_flags(&mut self, value: u8, overflow: bool) {
    let mut flags = Flags::None as u8;
    flags |= if value == 0 { Flags::Zero } else { Flags::None } as u8;
    flags |= if value & 0x80 == 0 {
      Flags::None
    } else {
      Flags::Negative
    } as u8;
    flags |= if overflow {
      Flags::Overflow
    } else {
      Flags::None
    } as u8;
    flags |= if value & 0x80 == 0x80 {
      Flags::LessThan
    } else {
      Flags::None
    } as u8;
    self.set_reg(0xf, flags);
  }
  fn sub_xy(&mut self, x: u8, y: u8) {
    let a = self.get_reg(x);
    let b = self.get_reg(y);

    let (val, overflow) = a.overflowing_sub(b);
    self.set_reg(x, val);
    self.set_flags(val, overflow);
    self.position += 2;
  }
  fn add_xy(&mut self, x: u8, y: u8) {
    let a = self.get_reg(x);
    let b = self.get_reg(y);

    let (val, overflow) = a.overflowing_add(b);
    self.set_reg(x, val);
    self.set_flags(val, overflow);
    self.position += 2;
  }
  fn mul_xy(&mut self, x: u8, y: u8) {
    let arg = (self.get_reg(x), self.get_reg(y));

    let (val, overflow) = arg.0.overflowing_mul(arg.1);
    self.set_reg(x, val);
    self.set_flags(val, overflow);
    self.position += 2;
  }
  fn mod_xy(&mut self, x: u8, y: u8) {
    let val = self.get_reg(x) % self.get_reg(y);
    self.set_reg(x, val);
    self.set_flags(val, false);
    self.position += 2;
  }
  fn and_xy(&mut self, x: u8, y: u8) {
    let val = self.get_reg(x) & self.get_reg(y);
    self.set_reg(x, val);
    self.set_flags(val, false);
    self.position += 2;
  }
  fn or_xy(&mut self, x: u8, y: u8) {
    let val = self.get_reg(x) | self.get_reg(y);
    self.set_reg(x, val);
    self.set_flags(val, false);
    self.position += 2;
  }
  fn xor_xy(&mut self, x: u8, y: u8) {
    let arg = (self.get_reg(x), self.get_reg(y));
    let val = (arg.0 & !arg.1) | (!arg.0 & arg.1);
    self.set_reg(x, val);
    self.set_flags(val, false);
    self.position += 2;
  }
  fn div_xy(&mut self, x: u8, y: u8) {
    let val = self.get_reg(x) / self.get_reg(y);
    self.set_reg(x, val);
    self.set_flags(val, false);
    self.position += 2;
  }
  fn push_xy(&mut self, x: u8, y: u8) {
    let value = ((self.get_reg(x) as u16) << 8) | self.get_reg(y) as u16;
    self.stack[self.stack_top] = value;
    self.stack_top += 1;
    if self.stack_top > 16 {
      panic!("Stack Overflow");
    }
    self.position += 2;
  }
  fn pop_xy(&mut self, x: u8, y: u8) {
    if self.stack_top == 0 {
      panic!("Stack Underflow");
    }
    self.stack_top -= 1;
    let value = self.stack[self.stack_top];
    self.set_reg(x, ((value & 0xf0) >> 8) as u8);
    self.set_reg(y, (value & 0x0f) as u8);
    self.position += 2;
  }
  fn jump(&mut self, nnn: u32) {
    let (position, overflow): (usize, bool);
    let (jump_forward, distance) = ((nnn & 0x800) == 0, (nnn & 0x7fff) as usize);
    if jump_forward {
      (position, overflow) = self.position.overflowing_add(distance);
      if overflow {
        panic!("Jump too far ahead");
      }
    } else {
      (position, overflow) = self.position.overflowing_sub(distance);
      if overflow {
        panic!("Jump too far behind");
      }
    }
    self.position = position;
  }

  fn test_flag(&self, flags: u8) -> bool { self.get_reg(0xf) & flags != 0 }

  fn jump_if(&mut self, nnn: u32, jump: bool) {
    self.position += 2;
    if jump {
      self.jump(nnn);
    }
  }

  fn jump_if_e(&mut self, nnn: u32) { self.jump_if(nnn, self.test_flag(Flags::Zero as u8)); }
  fn jump_if_lt(&mut self, nnn: u32) { self.jump_if(nnn, self.test_flag(Flags::LessThan as u8)); }
  fn jump_if_gt(&mut self, nnn: u32) { self.jump_if(nnn, !self.test_flag(Flags::LessEqual as u8)); }
  fn jump_if_a(&mut self, nnn: u32) { self.jump_if(nnn, self.test_flag(Flags::Above as u8)); }
  fn jump_if_ae(&mut self, nnn: u32) { self.jump_if(nnn, self.test_flag(Flags::AboveEqual as u8)); }

  fn call(&mut self, nnn: u32) {
    if self.stack_top == 16 {
      panic!("Stack Overflow");
    }
    self.position += 2;
    self.stack[self.stack_top] = self.position as u16;
    self.stack_top += 1;
    self.jump(nnn);
  }
  fn ret(&mut self) {
    if self.stack_top == 0 {
      panic!("Stack Underflow");
    }
    self.stack_top -= 1;
    self.position = self.stack[self.stack_top] as usize;
  }
  fn read_mem(&mut self, nnn: u32) {
    self.set_reg(0xf, self.memory[nnn as usize]);
    self.position += 2;
  }
  fn save_mem(&mut self, nnn: u32) {
    self.memory[nnn as usize] = self.get_reg(0xf);
    self.position += 2;
  }
  fn read_mem_dyn(&mut self, x: u8, y: u8, z: u8) {
    let reg = (self.get_reg(x), self.get_reg(y), self.get_reg(z));
    self.read_mem(((reg.0 as u32) << 16) | ((reg.1 as u32) << 8) | (reg.2 as u32));
  }
  fn save_mem_dyn(&mut self, x: u8, y: u8, z: u8) {
    let reg = (self.get_reg(x), self.get_reg(y), self.get_reg(z));
    self.save_mem(((reg.0 as u32) << 16) | ((reg.1 as u32) << 8) | (reg.2 as u32));
  }
  fn r#move(&mut self, x: u8, y: u8) {
    self.set_reg(x, self.get_reg(y));
    self.position += 2;
  }

  fn run(&mut self) {
    loop {
      let op = self.read_opcode();
      let instr = ((op & 0xf000) >> 12) as u8;
      let p1 = ((op & 0x0f00) >> 8) as u8;
      let p2 = ((op & 0x00f0) >> 4) as u8;
      let p3 = (op & 0x000f) as u8;
      let p23 = (op & 0x00ff) as u8;
      let p123 = (op & 0x0fff) as u32;
      match (instr, p1, p2, p3) {
        (0, 0, 0, 0) => break,
        (0x1, 0x0, _, _) => self.r#move(p2, p3),
        (0x1, 0x1, _, _) => self.add_xy(p2, p3),
        (0x1, 0x2, _, _) => self.sub_xy(p2, p3),
        (0x1, 0x3, _, _) => self.mul_xy(p2, p3),
        (0x1, 0x4, _, _) => self.div_xy(p2, p3),
        (0x1, 0x5, _, _) => self.mod_xy(p2, p3),
        (0x1, 0x6, _, _) => self.or_xy(p2, p3),
        (0x1, 0x7, _, _) => self.and_xy(p2, p3),
        (0x1, 0x8, _, _) => self.xor_xy(p2, p3),
        (0x2, _, _, _) => self.jump(p123),
        (0x3, _, _, _) => self.jump_if_e(p123),
        (0x4, _, _, _) => self.jump_if_lt(p123),
        (0x5, _, _, _) => self.jump_if_gt(p123),
        (0x6, _, _, _) => self.jump_if_a(p123),
        (0x7, _, _, _) => self.jump_if_ae(p123),
        (0x8, _, _, _) => self.call(p123),
        (0x9, 0, 0, 0) => self.ret(),
        (0xa, 0x1, _, _) => self.push_xy(p2, p3),
        (0xa, 0x2, _, _) => self.pop_xy(p2, p3),
        (0xb, _, _, _) => self.set_reg(p1, p23),
        (0xc, _, _, _) => self.read_mem(p123),
        (0xd, _, _, _) => self.save_mem(p123),
        (0xe, _, _, _) => self.read_mem_dyn(p1, p2, p3),
        (0xf, _, _, _) => self.save_mem_dyn(p1, p2, p3),
        _ => todo!("opcode {:04x}", op),
      }
    }
  }
}

fn main() {
  let mut cpu = CPU {
    registers: [0; 16],
    memory: [0; 0x7ff],
    position: 0,
    stack: [0; 16],
    stack_top: 0,
  };
  cpu.registers[0..4].copy_from_slice(&[5, 10, 10, 10]);
  //cpu.memory[0..6].copy_from_slice(&[0x80, 0x14, 0x80, 0x24, 0x80, 0x34]);
  cpu.memory[0..8].copy_from_slice(&0x8014_8024_8034_0000u64.to_be_bytes() as &[u8]);

  cpu.run();
  println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
