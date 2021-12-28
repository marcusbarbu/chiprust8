use simple_error::{SimpleError, simple_error};

pub const CHIP8_CLEAR_RET_FIRST_NIBBLE: u8 = 0;
pub const CHIP8_JUMP_FIRST_NIBBLE: u8 = 1;
pub const CHIP8_CALL_FIRST_NIBBLE: u8 = 2;
pub const CHIP8_IMM_EQ_FIRST_NIBBLE: u8 = 3;
pub const CHIP8_IMM_NE_FIRST_NIBBLE: u8 = 4;
pub const CHIP8_REG_EQ_FIRST_NIBBLE: u8 = 5;
pub const CHIP8_REG_ASSIGN_FIRST_NIBBLE: u8 = 6;
pub const CHIP8_REG_INCR_FIRST_NIBBLE: u8 = 7;
pub const CHIP8_MATH_FIRST_NIBBLE: u8 = 8;
pub const CHIP8_REG_NE_FIRST_NIBBLE: u8 = 9;
pub const CHIP8_INDEX_SET_FIRST_NIBBLE: u8 = 0xA;
pub const CHIP8_REL_JUMP_FIRST_NIBBLE: u8 = 0xB;
pub const CHIP8_RAND_NUM_FIRST_NIBBLE: u8 = 0xC;
pub const CHIP8_SPRITE_OP_FIRST_NIBBLE: u8 = 0xD;
pub const CHIP8_KEY_OP_FIRST_NIBBLE: u8 = 0xE;
pub const CHIP8_EXTRA_OPS_FIRST_NIBBLE: u8 = 0xF;

#[derive(Debug)]
pub struct Chip8NoArgsOp {}
#[derive(Debug)]
pub struct Chip8LongImmOp {
    pub imm: u16
}

#[derive(Debug)]
pub struct Chip8SingleRegOp {
    pub reg: u8
}

#[derive(Debug)]
pub struct Chip8DoubleRegOp {
    pub a: u8,
    pub b: u8
}

#[derive(Debug)]
pub struct Chip8SingleRegImmOp {
    pub reg: u8,
    pub imm: u8
}

#[derive(Debug)]
pub struct Chip8DoubleRegImmOp {
    pub a: u8,
    pub b: u8,
    pub imm: u8
}

#[derive(Debug)]
pub enum Chip8MathInstr {
    Assign(Chip8DoubleRegOp),
    Or(Chip8DoubleRegOp),
    And(Chip8DoubleRegOp),
    Xor(Chip8DoubleRegOp),
    IncrBy(Chip8DoubleRegOp),
    DecrBy(Chip8DoubleRegOp),
    RightShift(Chip8DoubleRegOp),
    InvDecrBy(Chip8DoubleRegOp),
    LeftShift(Chip8DoubleRegOp),
}

#[derive(Debug)]
pub enum Chip8ExtraInstr {
    CheckDelay(Chip8SingleRegOp),
    WaitForKey(Chip8SingleRegOp),
    SetDelay(Chip8SingleRegOp),
    SetBuzzer(Chip8SingleRegOp),
    IncrIndex(Chip8SingleRegOp),
    SetIndexHex(Chip8SingleRegOp),
    BcdReg(Chip8SingleRegOp),
    SaveRegRange(Chip8SingleRegOp),
    LoadRegRange(Chip8SingleRegOp),
}

#[derive(Debug)]
pub enum Chip8KeyConditionalInstr {
    KeyPressed(Chip8SingleRegOp),
    KeyNotPressed(Chip8SingleRegOp),
}

#[derive(Debug)]
pub enum Chip8Instr {
    Clear(Chip8NoArgsOp),
    Return(Chip8NoArgsOp),
    Jump(Chip8LongImmOp),
    Call(Chip8LongImmOp),
    SkipImmEq(Chip8SingleRegImmOp),
    SkipImmNe(Chip8SingleRegImmOp),
    SkipRegEq(Chip8DoubleRegOp),
    RegAssign(Chip8SingleRegImmOp),
    RegIncr(Chip8SingleRegImmOp),
    Math(Chip8MathInstr),
    SkipRegNe(Chip8DoubleRegOp),
    SetIndex(Chip8LongImmOp),
    RelJump(Chip8LongImmOp),
    Random(Chip8SingleRegImmOp),
    Draw(Chip8DoubleRegImmOp),
    Key(Chip8KeyConditionalInstr),
    Extra(Chip8ExtraInstr)
}

impl Chip8LongImmOp {
    pub fn new(instr: &u16) -> Chip8LongImmOp {
        Chip8LongImmOp {
            imm: instr & 0xFFF
        }
    }
}

impl Chip8SingleRegOp {
    pub fn new(instr: &u16) -> Chip8SingleRegOp {
        Chip8SingleRegOp {
            reg: ((instr & 0x0F00) >> 8) as u8
        }
    }
}

impl Chip8DoubleRegOp {
    pub fn new(instr: &u16) -> Chip8DoubleRegOp {
        Chip8DoubleRegOp { 
            a: ((instr & 0xF00) >> 8) as u8,
            b: ((instr & 0xF0) >> 4) as u8
        }
    }
}

impl Chip8SingleRegImmOp {
    pub fn new(instr: &u16) -> Chip8SingleRegImmOp {
        Chip8SingleRegImmOp {
            reg: ((instr & 0xF00) >> 8) as u8,
            imm: (instr & 0xFF) as u8
        }
    }
}

impl Chip8DoubleRegImmOp {
    pub fn new(instr: &u16) -> Chip8DoubleRegImmOp {
        Chip8DoubleRegImmOp { 
            a: ((instr & 0xF00) >> 8) as u8,
            b: ((instr & 0xF0) >> 4) as u8,
            imm: (instr & 0xF) as u8
        }
    }
}

impl Chip8Instr {
    pub fn from_u16(instr: u16) -> Result<Chip8Instr, SimpleError> {
        let opcode: u8 = ((instr & 0xF000) >> 12) as u8;
        let out_instr: Chip8Instr = match opcode {
            CHIP8_CLEAR_RET_FIRST_NIBBLE => {
                Chip8Instr::Clear(Chip8NoArgsOp{})
            }
            CHIP8_JUMP_FIRST_NIBBLE => {
                Chip8Instr::Jump(Chip8LongImmOp::new(&instr))
            }
            CHIP8_CALL_FIRST_NIBBLE => {
                Chip8Instr::Call(Chip8LongImmOp::new(&instr))
            }
            CHIP8_IMM_NE_FIRST_NIBBLE => {
                Chip8Instr::SkipImmNe(Chip8SingleRegImmOp::new(&instr))
            }
            CHIP8_IMM_EQ_FIRST_NIBBLE => {
                Chip8Instr::SkipImmNe(Chip8SingleRegImmOp::new(&instr))
            }
            CHIP8_REG_NE_FIRST_NIBBLE => {
                Chip8Instr::SkipRegNe(Chip8DoubleRegOp::new(&instr))
            }
            CHIP8_REG_ASSIGN_FIRST_NIBBLE => {
                Chip8Instr::RegAssign(Chip8SingleRegImmOp::new(&instr))
            }
            CHIP8_REG_INCR_FIRST_NIBBLE => {
                Chip8Instr::RegIncr(Chip8SingleRegImmOp::new(&instr))
            }
            CHIP8_MATH_FIRST_NIBBLE => {
                let secondary_op: u8 = (instr & 0xF) as u8;
                
                let mi: Chip8MathInstr = match secondary_op {
                    0 => {
                        Chip8MathInstr::Assign(Chip8DoubleRegOp::new(&instr))
                    }
                    1 => {
                        Chip8MathInstr::Or(Chip8DoubleRegOp::new(&instr))
                    }
                    2 => {
                        Chip8MathInstr::And(Chip8DoubleRegOp::new(&instr))
                    }
                    3 => {
                        Chip8MathInstr::Xor(Chip8DoubleRegOp::new(&instr))
                    }
                    4 => {
                        Chip8MathInstr::IncrBy(Chip8DoubleRegOp::new(&instr))
                    }
                    5 => {
                        Chip8MathInstr::DecrBy(Chip8DoubleRegOp::new(&instr))
                    }
                    6 => {
                        Chip8MathInstr::RightShift(Chip8DoubleRegOp::new(&instr))
                    }
                    7 => {
                        Chip8MathInstr::InvDecrBy(Chip8DoubleRegOp::new(&instr))
                    }
                    0xE => {
                        Chip8MathInstr::LeftShift(Chip8DoubleRegOp::new(&instr))
                    }
                    _ => {
                        return Err(simple_error!("Failed to match secondary math op {:?}", secondary_op));
                    }
                };
                Chip8Instr::Math(mi)
            }
            CHIP8_REG_EQ_FIRST_NIBBLE => {
                Chip8Instr::SkipRegEq(Chip8DoubleRegOp::new(&instr))
            }
            CHIP8_INDEX_SET_FIRST_NIBBLE => {
                Chip8Instr::SetIndex(Chip8LongImmOp::new(&instr))
            }
            CHIP8_REL_JUMP_FIRST_NIBBLE => {
                Chip8Instr::RelJump(Chip8LongImmOp::new(&instr))
            }
            CHIP8_RAND_NUM_FIRST_NIBBLE => {
                Chip8Instr::Random(Chip8SingleRegImmOp::new(&instr))
            }
            CHIP8_SPRITE_OP_FIRST_NIBBLE => {
                Chip8Instr::Draw(Chip8DoubleRegImmOp::new(&instr))
            }
            CHIP8_KEY_OP_FIRST_NIBBLE => {
                let secondary_op: u8 = (instr & 0xFF) as u8;
                let mi: Chip8KeyConditionalInstr = match secondary_op {
                    0x9E => {
                        Chip8KeyConditionalInstr::KeyNotPressed(Chip8SingleRegOp::new(&instr))
                    }
                    0xA1 => {
                        Chip8KeyConditionalInstr::KeyPressed(Chip8SingleRegOp::new(&instr))
                    }
                    _ => {
                        return Err(simple_error!("Failed key conditional with secondary op {:?}", secondary_op));
                    }
                };
                Chip8Instr::Key(mi)
            }
            CHIP8_EXTRA_OPS_FIRST_NIBBLE => {
                let secondary_op: u8 = (instr & 0xFF) as u8;
                let mi: Chip8ExtraInstr = match secondary_op {
                    0x07 => {
                        Chip8ExtraInstr::CheckDelay(Chip8SingleRegOp::new(&instr))
                    }
                    0x0A => {
                        Chip8ExtraInstr::WaitForKey(Chip8SingleRegOp::new(&instr))
                    }
                    0x15 => {
                        Chip8ExtraInstr::SetDelay(Chip8SingleRegOp::new(&instr))
                    }
                    0x18 => {
                        Chip8ExtraInstr::SetBuzzer(Chip8SingleRegOp::new(&instr))
                    }
                    0x1E => {
                        Chip8ExtraInstr::IncrIndex(Chip8SingleRegOp::new(&instr))
                    }
                    0x29 => {
                        Chip8ExtraInstr::SetIndexHex(Chip8SingleRegOp::new(&instr))
                    }
                    0x33 => {
                        Chip8ExtraInstr::BcdReg(Chip8SingleRegOp::new(&instr))
                    }
                    0x55 => {
                        Chip8ExtraInstr::SaveRegRange(Chip8SingleRegOp::new(&instr))
                    }
                    0x65 => {
                        Chip8ExtraInstr::LoadRegRange(Chip8SingleRegOp::new(&instr))
                    }
                    _ => {
                        return Err(simple_error!("Failed extra instr with secondary op {:?}", secondary_op));
                    }
                };

                Chip8Instr::Extra(mi)
            }
            _ => {
                return Err(simple_error!("Could not decode instr: {:?}", instr));
            }
        };
        Ok(out_instr)
    }
}
