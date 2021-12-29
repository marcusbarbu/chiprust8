mod instrs;
mod tests;
use byteorder::{BigEndian, ByteOrder};
use instrs::*;
use log::{debug, error, info};
use rand::random;
use simple_error::{simple_error, SimpleError};
use std::collections::VecDeque;
use std::{fs, io::Read};
use bitvec::prelude::*;

pub const PROGRAM_OFFSET: u16 = 0x200;

// credit to https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
const DEFAULT_FONT_MEM: [u8; 80] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
pub struct Chip8Regs {
    index_reg: u16,
    pc: u16,
    v_regs: [u8; 16],
}

pub struct Chip8Timers {
    delay: u8,
    sound: u8,
}

pub struct Chip8Mem {
    memspace: [u8; 4096],
}

pub struct Chip8DisplayData {
    _display: [[u8; 64]; 32],
}

pub struct Chip8 {
    regs: Chip8Regs,
    timers: Chip8Timers,
    _disp: Chip8DisplayData,
    mem: Chip8Mem,
    stack: VecDeque<u16>,
    keys: [u8; 16],
    cosmac: bool,
}

impl Chip8 {
    pub fn new(prog_path: &str, cosmac_compat: bool) -> Chip8 {
        info!("Generating Chip8 Core from fname {}", prog_path);
        let mut mem: Chip8Mem = Chip8Mem {
            memspace: [0; 4096],
        };
        mem.memspace[0..80].copy_from_slice(&DEFAULT_FONT_MEM[..]);
        let timers: Chip8Timers = Chip8Timers { delay: 0, sound: 0 };
        let disp: Chip8DisplayData = Chip8DisplayData{_display: [[0;64]; 32]};
        let regs: Chip8Regs = Chip8Regs {
            index_reg: 0,
            pc: 200,
            v_regs: [0; 16],
        };

        let mut infile: fs::File = match fs::File::open(prog_path) {
            Ok(f) => f,
            Err(_) => panic!("ononon"),
        };

        let mut prog_vec: Vec<u8> = Vec::new();

        match infile.read_to_end(&mut prog_vec) {
            Ok(_) => {}
            Err(_) => error!("Failed to read file in."),
        };

        mem.memspace[200..].copy_from_slice(&prog_vec[..4096]);

        Chip8 {
            regs: regs,
            timers: timers,
            _disp: disp,
            mem: mem,
            stack: VecDeque::new(),
            keys: [0; 16],
            cosmac: cosmac_compat,
        }
    }

    pub fn tick(&mut self) -> Result<(), SimpleError> {
        let instr: Chip8Instr = self.fetch_decode()?;
        self.execute(instr)?;
        Ok(())
    }

    fn fetch_decode(&mut self) -> Result<Chip8Instr, SimpleError> {
        let fetch_addr: usize = self.regs.pc as usize;
        let instr: u16 = BigEndian::read_u16(&self.mem.memspace[fetch_addr..fetch_addr + 1]);
        return Chip8Instr::from_u16(instr);
    }

    fn clear_display(&mut self) -> Result<(), SimpleError> {
        self._disp._display = [[0; 64]; 32];
        Ok(())
    }

    fn draw(&mut self, x: u8, y: u8, height: u8) -> Result<(), SimpleError> {
        let x = (x % (self._disp._display[0].len() as u8)) as usize;
        let y = (y % (self._disp._display.len() as u8)) as usize;

        for row in 0 .. height {
            let offset: usize = self.regs.index_reg as usize + row as usize;
            let val: u8 = self.mem.memspace[offset];
            let hots: BitVec::<Msb0, u8> = BitVec::<Msb0, u8>::from_element(val);
            debug!("BV: {:?} val: {:X} offset: {}", row, val, offset);
            for (col_offset, b) in hots.into_iter().enumerate() {
                let col_val: usize = x + col_offset;
                let row_val: usize = y + (row as usize);
                if row_val < self._disp._display.len() {
                    let active_row = &mut self._disp._display[row_val];
                    if col_val < active_row.len(){
                        match b {
                            true => {
                                // debug!("Row val: {} col_val {} hot", row_val, col_val );
                                active_row[col_val] ^= 1;
                            }
                            false => {
                                // debug!("Row val: {} col_val {} not hot", row_val, col_val );
                                // active_row[col_val] = 0;
                            }
                        } 
                    }
                    
                }
            }
        }
        Ok(())
    }

    pub fn dbg_display(&self) {
        for r in self._disp._display.into_iter() {
            let mut row_str: String = String::new();
            for c in r.into_iter() {
                match c {
                    0 => row_str.push(' '),
                    _ => row_str.push('■')
                }
            }
            println!("{}",row_str);
        }
    }

    fn pop(&mut self) -> Result<u16, SimpleError> {
        match self.stack.pop_back() {
            Some(val) => Ok(val),
            None => Err(simple_error!("Failed to pop from stack.")),
        }
    }

    fn push(&mut self, val: u16) -> Result<(), SimpleError> {
        self.stack.push_back(val);
        Ok(())
    }

    fn get_reg(&mut self, reg: u8) -> Result<u8, SimpleError> {
        match self.regs.v_regs.get(reg as usize) {
            Some(v) => Ok(*v),
            None => Err(simple_error!("Failed to get reg: {:?}", reg)),
        }
    }

    fn get_key(&mut self, key: u8) -> Result<u8, SimpleError> {
        match self.keys.get(key as usize) {
            Some(v) => Ok(*v),
            None => Err(simple_error!("Failed to get key: {:?}", key)),
        }
    }

    fn set_reg(&mut self, reg: u8, val: u8) -> Result<(), SimpleError> {
        let handle = match self.regs.v_regs.get_mut(reg as usize) {
            Some(v) => v,
            None => return Err(simple_error!("Failed to set reg {:?}", reg)),
        };
        *handle = val;
        Ok(())
    }

    fn execute(&mut self, instr: Chip8Instr) -> Result<(), SimpleError> {
        self.regs.pc += 2;
        debug!("Attempting to execute instruction: {:?}", instr);
        match instr {
            Chip8Instr::Clear(_) => self.clear_display(),
            Chip8Instr::Return(_) => {
                let addr: u16 = self.pop()?;
                self.regs.pc = addr;
                Ok(())
            }
            Chip8Instr::Jump(args) => {
                self.regs.pc = args.imm;
                Ok(())
            }
            Chip8Instr::Call(args) => {
                self.push(self.regs.pc)?;
                self.regs.pc = args.imm;
                Ok(())
            }
            Chip8Instr::SkipImmEq(args) => {
                let a: u8 = self.get_reg(args.reg)?;
                let b: u8 = args.imm;
                if a == b {
                    self.regs.pc += 2;
                }
                Ok(())
            }
            Chip8Instr::SkipImmNe(args) => {
                let a: u8 = self.get_reg(args.reg)?;
                let b: u8 = args.imm;
                if a != b {
                    self.regs.pc += 2;
                }
                Ok(())
            }
            Chip8Instr::SkipRegEq(args) => {
                let a: u8 = self.get_reg(args.a)?;
                let b: u8 = self.get_reg(args.b)?;

                if a == b {
                    self.regs.pc += 2;
                }

                Ok(())
            }
            Chip8Instr::RegAssign(args) => self.set_reg(args.reg, args.imm),
            Chip8Instr::RegIncr(args) => {
                let mut val: u8 = self.get_reg(args.reg)?;
                val = val.wrapping_add(args.imm);
                self.set_reg(args.reg, val)
            }
            Chip8Instr::Math(inner_instr) => match inner_instr {
                Chip8MathInstr::Assign(args) => {
                    let val: u8 = self.get_reg(args.b)?;
                    self.set_reg(args.a, val)
                }
                Chip8MathInstr::Or(args) => {
                    let mut a: u8 = self.get_reg(args.a)?;
                    let b: u8 = self.get_reg(args.b)?;

                    a |= b;
                    self.set_reg(args.a, a)
                }
                Chip8MathInstr::And(args) => {
                    let mut a: u8 = self.get_reg(args.a)?;
                    let b: u8 = self.get_reg(args.b)?;

                    a &= b;
                    self.set_reg(args.a, a)
                }
                Chip8MathInstr::Xor(args) => {
                    let mut a: u8 = self.get_reg(args.a)?;
                    let b: u8 = self.get_reg(args.b)?;

                    a ^= b;
                    self.set_reg(args.a, a)
                }
                Chip8MathInstr::IncrBy(args) => {
                    let mut a: u8 = self.get_reg(args.a)?;
                    let b: u8 = self.get_reg(args.b)?;
                    a = a.wrapping_add(b);
                    if a < b {
                        self.set_reg(0xF, 1)?;
                    } else {
                        self.set_reg(0xF, 0)?;
                    }
                    self.set_reg(args.a, a)
                }
                Chip8MathInstr::DecrBy(args) => {
                    let mut a: u8 = self.get_reg(args.a)?;
                    let b: u8 = self.get_reg(args.b)?;

                    if a > b {
                        self.set_reg(0xF, 1)?;
                    } else {
                        self.set_reg(0xF, 0)?;
                    }

                    a = a.wrapping_sub(b);
                    self.set_reg(args.a, a)
                }
                Chip8MathInstr::RightShift(args) => {
                    let mut a: u8 = self.get_reg(args.a)?;
                    let b: u8 = self.get_reg(args.b)?;
                    let target: u8;
                    if self.cosmac {
                        target = b
                    } else {
                        target = a
                    }
                    if target & 0x01 == 1 {
                        self.set_reg(0xF, 1)?;
                    } else {
                        self.set_reg(0xF, 0)?;
                    }

                    a = target >> 1;
                    self.set_reg(args.a, a)
                }
                Chip8MathInstr::InvDecrBy(args) => {
                    let mut a: u8 = self.get_reg(args.a)?;
                    let b: u8 = self.get_reg(args.b)?;
                    if b > a {
                        self.set_reg(0xF, 1)?;
                    } else {
                        self.set_reg(0xF, 0)?;
                    }

                    a = b.wrapping_sub(a);
                    self.set_reg(args.a, a)
                }
                Chip8MathInstr::LeftShift(args) => {
                    let mut a: u8 = self.get_reg(args.a)?;
                    let b: u8 = self.get_reg(args.b)?;
                    let target: u8;
                    if self.cosmac {
                        target = b;
                    } else {
                        target = a;
                    }
                    if target & 0x80 == 1 {
                        self.set_reg(0xF, 1)?;
                    } else {
                        self.set_reg(0xF, 0)?;
                    }
                    a = target << 1;
                    self.set_reg(args.a, a)
                }
            },
            Chip8Instr::SkipRegNe(args) => {
                let a: u8 = self.get_reg(args.a)?;
                let b: u8 = self.get_reg(args.b)?;
                if a != b {
                    self.regs.pc += 2;
                }
                Ok(())
            }
            Chip8Instr::SetIndex(args) => {
                self.regs.index_reg = args.imm;
                Ok(())
            }
            Chip8Instr::RelJump(args) => {
                let addr: u16;
                if self.cosmac {
                    let a: u8 = self.get_reg(0)?;
                    addr = args.imm + a as u16;
                } else {
                    let reg: u8 = ((args.imm & 0xF00) >> 8) as u8;
                    let a: u8 = self.get_reg(reg)?;
                    addr = args.imm + a as u16;
                }
                self.regs.pc = addr;
                Ok(())
            }
            Chip8Instr::Random(args) => {
                let mut base_rand: u8 = random();
                base_rand &= args.imm;
                self.set_reg(args.reg, base_rand)?;
                Ok(())
            }
            Chip8Instr::Draw(args) => {
                let x: u8 = self.get_reg(args.a)?;
                let y: u8 = self.get_reg(args.b)?;
                self.draw(x, y, args.imm)
            }
            Chip8Instr::Key(inner_instr) => match inner_instr {
                Chip8KeyConditionalInstr::KeyPressed(args) => {
                    let k: u8 = self.get_reg(args.reg)?;
                    let key = self.get_key(k)?;
                    if key == 1 {
                        self.regs.pc += 2;
                    }
                    Ok(())
                }
                Chip8KeyConditionalInstr::KeyNotPressed(args) => {
                    let k: u8 = self.get_reg(args.reg)?;
                    let key = self.get_key(k)?;
                    if key != 1 {
                        self.regs.pc += 2;
                    }
                    Ok(())
                }
            },
            Chip8Instr::Extra(inner_instr) => match inner_instr {
                Chip8ExtraInstr::CheckDelay(args) => self.set_reg(args.reg, self.timers.delay),
                Chip8ExtraInstr::WaitForKey(args) => {
                    let k: u8 = self.get_reg(args.reg)?;
                    let key = self.get_key(k)?;
                    if key != 1 {
                        self.regs.pc -= 2;
                    }
                    Ok(())
                }
                Chip8ExtraInstr::SetDelay(args) => {
                    let val: u8 = self.get_reg(args.reg)?;
                    self.timers.delay = val;
                    Ok(())
                }
                Chip8ExtraInstr::SetBuzzer(args) => {
                    let val: u8 = self.get_reg(args.reg)?;
                    self.timers.sound = val;
                    Ok(())
                }
                Chip8ExtraInstr::IncrIndex(args) => {
                    let a: u8 = self.get_reg(args.reg)?;
                    let start: u16 = self.regs.index_reg;
                    self.regs.index_reg = self.regs.index_reg.wrapping_add(a as u16);
                    if self.regs.index_reg < start {
                        self.set_reg(0xF, 1)?;
                    } else {
                        self.set_reg(0xF, 0)?;
                    }
                    Ok(())
                }
                Chip8ExtraInstr::SetIndexHex(args) => {
                    let val: u8 = self.get_reg(args.reg)?;
                    self.regs.index_reg = (val * 5) as u16;
                    Ok(())
                }
                Chip8ExtraInstr::BcdReg(args) => {
                    let val: u8 = self.get_reg(args.reg)?;
                    let origin: usize = self.regs.index_reg as usize;
                    let hunds = val / 100;
                    let tens: u8 = (val - hunds) / 10;
                    let ones: u8 = val - (hunds + tens);
                    self.mem.memspace[origin] = hunds;
                    self.mem.memspace[origin+1] = tens;
                    self.mem.memspace[origin+2] = ones;
                    Ok(())
                }
                Chip8ExtraInstr::SaveRegRange(args) => {
                    let mut addr: u16 = self.regs.index_reg;
                    let end: u8 = self.get_reg(args.reg)?;
                    for i in 0..end {
                        let val: u8 = self.get_reg(i)?;
                        self.mem.memspace[addr as usize] = val;
                        addr += 1
                    }
                    Ok(())
                }
                Chip8ExtraInstr::LoadRegRange(args) => {
                    let mut addr: u16 = self.regs.index_reg;
                    let end: u8 = self.get_reg(args.reg)?;
                    for i in 0..end {
                        let val: u8 = self.mem.memspace[addr as usize];
                        self.set_reg(i, val)?;
                        addr += 1
                    }
                    Ok(())
                }
            },
        }
    }

    #[cfg(test)]
    fn test_core() -> Chip8 {
        info!("Generating test core");
        let mut mem: Chip8Mem = Chip8Mem {
            memspace: [0; 4096],
        };
        mem.memspace[0..80].copy_from_slice(&DEFAULT_FONT_MEM[..]);
        let timers: Chip8Timers = Chip8Timers { delay: 0, sound: 0 };
        let disp: Chip8DisplayData = Chip8DisplayData{_display: [[0;64]; 32]};
        let regs: Chip8Regs = Chip8Regs {
            index_reg: 0,
            pc: 200,
            v_regs: [0; 16],
        };

        Chip8 {
            regs: regs,
            timers: timers,
            _disp: disp,
            mem: mem,
            stack: VecDeque::new(),
            keys: [0; 16],
            cosmac: true,
        }
    }
}
