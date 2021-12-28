mod instrs;
use std::{fs, io::Read};
use simple_error::{SimpleError, simple_error};
use byteorder::{ByteOrder, BigEndian};
use std::collections::VecDeque;
use rand::random;
use log::{debug, error, info};
use instrs::*;

pub const PROGRAM_OFFSET: u16 = 0x200;
pub struct Chip8Regs{
    index_reg: u16,
    pc: u16,
    v_regs: [u8; 16],
}

pub struct Chip8Timers {
    delay: u8,
    sound: u8,
}

pub struct Chip8Mem {
    memspace: [u8; 4096]
}

pub struct Chip8RegularDisplay {
    _display: [[u8; 64]; 32],
}

pub struct Chip8SuperDisplay {
    _display: [[u8; 128]; 64],
}

pub enum Chip8DisplayData {
    FullSize(Chip8SuperDisplay),
    Regular(Chip8RegularDisplay)
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
        info!("Generating Chip8 Core from fname {}",prog_path);
        let mut mem: Chip8Mem = Chip8Mem{memspace: [0; 4096]};
        let timers: Chip8Timers = Chip8Timers{delay: 0, sound: 0};
        let reg_disp: Chip8RegularDisplay = Chip8RegularDisplay{_display: [[0;64]; 32]};
        let disp: Chip8DisplayData = Chip8DisplayData::Regular(reg_disp);
        let regs: Chip8Regs = Chip8Regs{ index_reg: 0, pc: 200, v_regs: [0;16]};

        let mut infile: fs::File = match fs::File::open(prog_path) {
            Ok(f) => f,
            Err(_) => panic!("ononon")
        };

        let mut prog_vec: Vec<u8> = Vec::new();

        match infile.read_to_end(&mut prog_vec) {
            Ok(_) => {},
            Err(_) => error!("Failed to read file in.")
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
    
    pub fn tick(&mut self) -> Result <(), SimpleError> {
        let instr: Chip8Instr = self.fetch_decode()?;
        self.execute(instr)?;
        Ok(())
    }

    fn fetch_decode(&mut self) -> Result<Chip8Instr, SimpleError> {
        let fetch_addr: usize = self.regs.pc as usize;
        let instr: u16 = BigEndian::read_u16(&self.mem.memspace[fetch_addr .. fetch_addr + 1]);
        self.regs.pc += 2;
        return Chip8Instr::from_u16(instr);
    }

    fn clear_display(&mut self) -> Result<(), SimpleError> {
        todo!()
    }

    fn draw(&mut self, _x: u8, _y: u8, _height: u8) -> Result<(), SimpleError> {
        todo!()
    }

    fn pop(&mut self) -> Result<u16, SimpleError> {
        match self.stack.pop_back() {
            Some(val) => Ok(val),
            None => Err(simple_error!("Failed to pop from stack."))
        }
    }

    fn push(&mut self, val: u16) -> Result <(), SimpleError> {
        self.stack.push_back(val);
        Ok(())
    }

    fn get_reg(&mut self, reg: u8) -> Result<u8, SimpleError> {
        match self.regs.v_regs.get(reg as usize) {
            Some(v) => Ok(*v),
            None => Err(simple_error!("Failed to get reg: {:?}", reg))
        }
    }

    fn get_key(&mut self, key: u8) -> Result<u8, SimpleError> {
        match self.keys.get(key as usize) {
            Some(v) => Ok(*v),
            None => Err(simple_error!("Failed to get key: {:?}", key))
        }
    }

    fn set_reg(&mut self, reg: u8, val: u8) -> Result<() ,SimpleError> {
        let handle = match self.regs.v_regs.get_mut(reg as usize) {
            Some(v) => v,
            None => return Err(simple_error!("Failed to set reg {:?}", reg)),
        };
        *handle = val;
        Ok(())
    }

    fn execute(&mut self, instr: Chip8Instr) -> Result<(), SimpleError> {
        debug!("Attempting to execute instruction: {:?}", instr);
        match instr {
            Chip8Instr::Clear(_) => {
                self.clear_display()
            }
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
            Chip8Instr::RegAssign(args) => {
                self.set_reg(args.reg, args.imm)
            }
            Chip8Instr::RegIncr(args) => {
                let mut val: u8 = self.get_reg(args.reg)?;
                val = val.wrapping_add(args.imm);
                self.set_reg(args.reg, val)
            }
            Chip8Instr::Math(inner_instr) => {
                match inner_instr {
                    Chip8MathInstr::Assign(args) => {
                        let val: u8 = self.get_reg(args.b)?;
                        self.set_reg(args.a, val)
                    }
                    Chip8MathInstr::Or(args) => {
                        let mut a: u8 =  self.get_reg(args.a)?;
                        let b: u8 = self.get_reg(args.b)?;

                        a |= b;
                        self.set_reg(args.a, a)
                    }
                    Chip8MathInstr::And(args) =>{
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
                        }
                        else{
                            self.set_reg(0xF, 0)?; 
                        }
                        self.set_reg(args.a, a)
                    }
                    Chip8MathInstr::DecrBy(args) => {
                        let mut a: u8 = self.get_reg(args.a)?;
                        let b: u8 = self.get_reg(args.b)?;

                        if a > b {
                            self.set_reg(0xF, 1)?;
                        }
                        else{
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
                        }
                        else {
                            target = a
                        }
                        if target & 0x01 == 1 {
                            self.set_reg(0xF, 1)?;
                        }
                        else{
                            self.set_reg(0xF, 0)?; 
                        }

                        a = target >> 1;
                        self.set_reg(args.a, a)

                    }
                    Chip8MathInstr::InvDecrBy(args) => {
                        let mut a: u8 = self.get_reg(args.a)?;
                        let b: u8 =  self.get_reg(args.b)?;
                        if b > a {
                            self.set_reg(0xF, 1)?;
                        }
                        else{
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
                        }
                        else {
                            target = a;
                        }
                        if target & 0x80 == 1 {
                            self.set_reg(0xF, 1)?;
                        }
                        else{
                            self.set_reg(0xF, 0)?; 
                        }
                        a = target << 1;
                        self.set_reg(args.a, a)
                    }
                }
            }
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
                }
                else {
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
            Chip8Instr::Key(inner_instr) => {
                match inner_instr {
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
                }
            }
            Chip8Instr::Extra(inner_instr) => {
                match inner_instr {
                    Chip8ExtraInstr::CheckDelay(args) => {
                        self.set_reg(args.reg, self.timers.delay)
                    }
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
                        }
                        else{
                            self.set_reg(0xF, 0)?; 
                        }
                        Ok(())
                    }
                    Chip8ExtraInstr::SetIndexHex(_) => todo!(),
                    Chip8ExtraInstr::BcdReg(_) => todo!(),
                    Chip8ExtraInstr::SaveRegRange(args) => {
                        let mut addr: u16 = self.regs.index_reg;
                        let end: u8 = self.get_reg(args.reg)?;
                        for i in 0 .. end {
                            let val: u8 = self.get_reg(i)?;
                            self.mem.memspace[addr as usize] = val;
                            addr += 1
                        }
                        Ok(())
                    }
                    Chip8ExtraInstr::LoadRegRange(args) => {
                        let mut addr: u16 = self.regs.index_reg;
                        let end: u8 = self.get_reg(args.reg)?;
                        for i in 0 .. end {
                            let val: u8 = self.mem.memspace[addr as usize];
                            self.set_reg(i, val)?;
                            addr += 1
                        }
                        Ok(())
                    }
                }
            }
        }
    }

    #[cfg(test)]
    fn test_core() -> Chip8 {
        info!("Generating test core");
        let mem: Chip8Mem = Chip8Mem{memspace: [0; 4096]};
        let timers: Chip8Timers = Chip8Timers{delay: 0, sound: 0};
        let reg_disp: Chip8RegularDisplay = Chip8RegularDisplay{_display: [[0;64]; 32]};
        let disp: Chip8DisplayData = Chip8DisplayData::Regular(reg_disp);
        let regs: Chip8Regs = Chip8Regs{ index_reg: 0, pc: 200, v_regs: [0;16]};

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

#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
    }

    #[test]
    fn a() {
        init();

        info!("Running a test");
        let mut c: Chip8 = Chip8::test_core();
        let instr: Chip8Instr = Chip8Instr::Jump(Chip8LongImmOp{ imm: 123 });
        match c.execute(instr) {
            Ok(_) => {}
            Err(e) => {error!("failed with err {:?}", e)}
        }
    }

}
