#[cfg(test)]
mod tests {
    use crate::core::*;
    fn test_init() -> Chip8 {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();

        Chip8::test_core()
    }

    fn test_exec(chip8: &mut Chip8, instr: Chip8Instr) {
        match chip8.execute(instr) {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to execute {:?} with err: {:?}", instr, e);
                return;
            }
        }
    }

    #[test]
    fn test_return() {
        let mut chip8 = test_init();
        let addr: u16 = 0x234;

        chip8.stack.push_back(addr);
        let ret_instr = Chip8Instr::Return(Chip8NoArgsOp {});
        test_exec(&mut chip8, ret_instr);

        assert_eq!(chip8.regs.pc, addr);
        assert_eq!(chip8.stack.len(), 0);
    }

    #[test]
    fn test_jump() {
        let mut chip8 = test_init();
        let addr: u16 = 0x234;
        let jump_instr: Chip8Instr = Chip8Instr::Jump(Chip8LongImmOp { imm: addr });

        test_exec(&mut chip8, jump_instr);

        assert_eq!(chip8.regs.pc, addr);
    }

    #[test]
    fn test_call() {
        let addr: u16 = 0x234;
        let orig_pc: u16 = 12;
        let mut chip8 = test_init();
        chip8.regs.pc = orig_pc;
        let call_instr: Chip8Instr = Chip8Instr::Call(Chip8LongImmOp { imm: addr });
        test_exec(&mut chip8, call_instr);

        assert_eq!(chip8.regs.pc, addr, "Jump to method");
        assert_eq!(chip8.stack.len(), 1, "one item on stack _only_");
        assert_eq!(
            *chip8.stack.get(0).unwrap(),
            orig_pc + 2,
            "item on stack is original PC (incremented)"
        );
    }

    #[test]
    fn test_skip_imm_eq() {
        let val: u8 = 0x42;
        let reg: u8 = 0;
        let mut chip8 = test_init();
        let mut pc: u16 = chip8.regs.pc;

        chip8.set_reg(reg, val + 1).unwrap();

        let skip_imm_eq_instr: Chip8Instr =
            Chip8Instr::SkipImmEq(Chip8SingleRegImmOp { reg, imm: val });
        test_exec(&mut chip8, skip_imm_eq_instr);
        assert_eq!(chip8.regs.pc, pc + 2, "Negative test"); // should NOT skip.

        pc = chip8.regs.pc;
        chip8.set_reg(reg, val).unwrap();
        test_exec(&mut chip8, skip_imm_eq_instr);
        assert_eq!(chip8.regs.pc, pc + 4, "Positive Test"); // SHOULD skip
    }

    #[test]
    fn test_skip_imm_ne() {
        let val: u8 = 0x42;
        let reg: u8 = 0;
        let mut chip8 = test_init();
        let mut pc: u16 = chip8.regs.pc;

        chip8.set_reg(reg, val + 1).unwrap();

        let skip_imm_eq_instr: Chip8Instr =
            Chip8Instr::SkipImmNe(Chip8SingleRegImmOp { reg, imm: val });
        test_exec(&mut chip8, skip_imm_eq_instr);
        assert_eq!(chip8.regs.pc, pc + 4, "Positive test"); // SHOULD skip

        pc = chip8.regs.pc;
        chip8.set_reg(0, val).unwrap();
        test_exec(&mut chip8, skip_imm_eq_instr);
        assert_eq!(chip8.regs.pc, pc + 2, "Negative Test"); // should NOT skip.
    }

    #[test]
    fn test_skip_req_eq() {
        let val: u8 = 0x42;
        let reg_a: u8 = 0;
        let reg_b: u8 = 1;
        let mut chip8 = test_init();
        let mut pc: u16 = chip8.regs.pc;

        chip8.set_reg(reg_a, val).unwrap();
        chip8.set_reg(reg_b, val).unwrap();
        let skip_reg_eq_instr: Chip8Instr =
            Chip8Instr::SkipRegEq(Chip8DoubleRegOp { a: reg_a, b: reg_b });
        test_exec(&mut chip8, skip_reg_eq_instr);
        assert_eq!(chip8.regs.pc, pc + 4, "Positive test"); // SHOULD skip

        pc = chip8.regs.pc;
        chip8.set_reg(reg_b, val + 1).unwrap();
        test_exec(&mut chip8, skip_reg_eq_instr);
        assert_eq!(chip8.regs.pc, pc + 2, "Negative test"); // should NOT skip.
    }

    #[test]
    fn test_reg_assign() {
        let val: u8 = 0x42;
        let reg: u8 = 0;
        let mut chip8 = test_init();

        let set_reg_instr: Chip8Instr =
            Chip8Instr::RegAssign(Chip8SingleRegImmOp { reg, imm: val });
        test_exec(&mut chip8, set_reg_instr);

        let res: u8 = chip8.get_reg(reg).unwrap();
        assert_eq!(res, val);
    }

    #[test]
    fn test_reg_incr() {
        let val: u8 = 0x42;
        let inc: u8 = 0x17;
        let reg: u8 = 0;
        let mut chip8 = test_init();

        chip8.set_reg(reg, val).unwrap();
        let incr_reg_instr: Chip8Instr = Chip8Instr::RegIncr(Chip8SingleRegImmOp { reg, imm: inc });
        test_exec(&mut chip8, incr_reg_instr);
        let res: u8 = chip8.get_reg(reg).unwrap();
        assert_eq!(res, val + inc);
    }

    #[test]
    fn test_math_assign() {
        let val: u8 = 0x42;
        let reg_a: u8 = 0;
        let reg_b: u8 = 1;
        let mut chip8 = test_init();

        chip8.set_reg(reg_b, val).unwrap();
        let math_assign_instr: Chip8Instr = Chip8Instr::Math(Chip8MathInstr::Assign(Chip8DoubleRegOp { a: reg_a, b: reg_b}));
        test_exec(&mut chip8, math_assign_instr);

        let res: u8 = chip8.get_reg(reg_a).unwrap();
        assert_eq!(res, val);
    }
}