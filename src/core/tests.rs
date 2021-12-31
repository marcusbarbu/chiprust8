#[cfg(test)]
mod tests {

    use crate::core::*;
    fn test_init() -> Chip8Core {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();

        Chip8Core::test_core()
    }

    fn test_exec(chip8: &mut Chip8Core, instr: Chip8Instr) {
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
        let math_assign_instr: Chip8Instr =
            Chip8Instr::Math(Chip8MathInstr::Assign(Chip8DoubleRegOp {
                a: reg_a,
                b: reg_b,
            }));
        test_exec(&mut chip8, math_assign_instr);

        let res: u8 = chip8.get_reg(reg_a).unwrap();
        assert_eq!(res, val);
    }

    #[test]
    fn test_math_or() {
        let a_val: u8 = 0b10010010;
        let b_val: u8 = 0b01101101;

        let reg_a: u8 = 5;
        let reg_b: u8 = 6;
        let ans_val: u8 = a_val | b_val;
        let mut chip8 = test_init();

        chip8.set_reg(reg_a, a_val).unwrap();
        chip8.set_reg(reg_b, b_val).unwrap();

        let math_or_instr: Chip8Instr =
            Chip8Instr::Math(Chip8MathInstr::Or(Chip8DoubleRegOp { a: reg_a, b: reg_b }));
        test_exec(&mut chip8, math_or_instr);

        let res: u8 = chip8.get_reg(reg_a).unwrap();
        assert_eq!(res, ans_val);
    }

    #[test]
    fn test_math_and() {
        let a_val: u8 = 0b10010010;
        let b_val: u8 = 0b01101101;

        let reg_a: u8 = 5;
        let reg_b: u8 = 6;
        let ans_val: u8 = a_val & b_val;
        let mut chip8 = test_init();

        chip8.set_reg(reg_a, a_val).unwrap();
        chip8.set_reg(reg_b, b_val).unwrap();

        let math_and_instr: Chip8Instr =
            Chip8Instr::Math(Chip8MathInstr::And(Chip8DoubleRegOp { a: reg_a, b: reg_b }));
        test_exec(&mut chip8, math_and_instr);

        let res: u8 = chip8.get_reg(reg_a).unwrap();
        assert_eq!(res, ans_val);
    }

    #[test]
    fn test_math_xor() {
        let a_val: u8 = 0b10010011;
        let b_val: u8 = 0b01101101;

        let reg_a: u8 = 5;
        let reg_b: u8 = 6;
        let ans_val: u8 = a_val ^ b_val;
        let mut chip8 = test_init();

        chip8.set_reg(reg_a, a_val).unwrap();
        chip8.set_reg(reg_b, b_val).unwrap();

        let math_xor_instr: Chip8Instr =
            Chip8Instr::Math(Chip8MathInstr::Xor(Chip8DoubleRegOp { a: reg_a, b: reg_b }));
        test_exec(&mut chip8, math_xor_instr);

        let res: u8 = chip8.get_reg(reg_a).unwrap();
        assert_eq!(res, ans_val);
    }

    #[test]
    fn test_math_incr() {
        let a_val: u8 = 12;
        let b_val: u8 = 52;
        let c_val: u8 = 249;

        let reg_a: u8 = 5;
        let reg_b: u8 = 6;
        let mut ans_val: u8 = a_val + b_val;
        let mut chip8 = test_init();

        chip8.set_reg(reg_a, a_val).unwrap();
        chip8.set_reg(reg_b, b_val).unwrap();

        let math_add_instr: Chip8Instr =
            Chip8Instr::Math(Chip8MathInstr::IncrBy(Chip8DoubleRegOp {
                a: reg_a,
                b: reg_b,
            }));
        test_exec(&mut chip8, math_add_instr);
        let res: u8 = chip8.get_reg(reg_a).unwrap();
        let flag: u8 = chip8.get_reg(0xF).unwrap();
        assert_eq!(res, ans_val);
        assert_eq!(flag, 0);

        chip8.set_reg(reg_a, a_val).unwrap();
        chip8.set_reg(reg_b, c_val).unwrap();

        ans_val = a_val.wrapping_add(c_val);

        let math_add_instr: Chip8Instr =
            Chip8Instr::Math(Chip8MathInstr::IncrBy(Chip8DoubleRegOp {
                a: reg_a,
                b: reg_b,
            }));
        test_exec(&mut chip8, math_add_instr);
        let res: u8 = chip8.get_reg(reg_a).unwrap();
        let flag: u8 = chip8.get_reg(0xF).unwrap();
        assert_eq!(res, ans_val);
        assert_eq!(flag, 1);
    }

    #[test]
    fn test_math_decr() {
        let a_val: u8 = 52;
        let b_val: u8 = 12;
        let c_val: u8 = 60;

        let reg_a: u8 = 5;
        let reg_b: u8 = 6;
        let mut ans_val: u8 = a_val - b_val;
        let mut chip8 = test_init();

        chip8.set_reg(reg_a, a_val).unwrap();
        chip8.set_reg(reg_b, b_val).unwrap();

        let math_add_instr: Chip8Instr =
            Chip8Instr::Math(Chip8MathInstr::DecrBy(Chip8DoubleRegOp {
                a: reg_a,
                b: reg_b,
            }));
        test_exec(&mut chip8, math_add_instr);
        let res: u8 = chip8.get_reg(reg_a).unwrap();
        let flag: u8 = chip8.get_reg(0xF).unwrap();
        assert_eq!(res, ans_val, "Failed 52 - 12");
        assert_eq!(flag, 1);

        chip8.set_reg(reg_a, a_val).unwrap();
        chip8.set_reg(reg_b, c_val).unwrap();

        ans_val = a_val.wrapping_sub(c_val);
        info!("Ans {}", ans_val);

        let math_add_instr: Chip8Instr =
            Chip8Instr::Math(Chip8MathInstr::DecrBy(Chip8DoubleRegOp {
                a: reg_a,
                b: reg_b,
            }));
        test_exec(&mut chip8, math_add_instr);
        let res: u8 = chip8.get_reg(reg_a).unwrap();
        let flag: u8 = chip8.get_reg(0xF).unwrap();
        assert_eq!(res, ans_val, "Failed 52 - 60");
        assert_eq!(flag, 0);
    }

    #[test]
    fn test_math_inv_decr() {
        let a_val: u8 = 52;
        let b_val: u8 = 12;
        let c_val: u8 = 60;

        let reg_a: u8 = 5;
        let reg_b: u8 = 6;
        let mut ans_val: u8 = a_val - b_val;
        let mut chip8 = test_init();

        chip8.set_reg(reg_a, b_val).unwrap();
        chip8.set_reg(reg_b, a_val).unwrap();

        let math_add_instr: Chip8Instr =
            Chip8Instr::Math(Chip8MathInstr::InvDecrBy(Chip8DoubleRegOp {
                a: reg_a,
                b: reg_b,
            }));
        test_exec(&mut chip8, math_add_instr);
        let res: u8 = chip8.get_reg(reg_a).unwrap();
        let flag: u8 = chip8.get_reg(0xF).unwrap();
        assert_eq!(res, ans_val, "Failed 52 - 12");
        assert_eq!(flag, 1);

        chip8.set_reg(reg_a, c_val).unwrap();
        chip8.set_reg(reg_b, a_val).unwrap();

        ans_val = a_val.wrapping_sub(c_val);
        info!("Ans {}", ans_val);

        let math_add_instr: Chip8Instr =
            Chip8Instr::Math(Chip8MathInstr::InvDecrBy(Chip8DoubleRegOp {
                a: reg_a,
                b: reg_b,
            }));
        test_exec(&mut chip8, math_add_instr);
        let res: u8 = chip8.get_reg(reg_a).unwrap();
        let flag: u8 = chip8.get_reg(0xF).unwrap();
        assert_eq!(res, ans_val, "Failed 52 - 60");
        assert_eq!(flag, 0);
    }

    // TODO:
    // #[test]
    // fn test_math_right_shift() {
    //     let a_val: u8 = 0b00010010;
    //     let b_val: u8 = 0b00001111;
    //     let reg_a: u8 = 5;
    //     let reg_b: u8 = 6;
    //     let mut ans_val: u8 = a_val >> 1;
    //     let mut chip8 = test_init();

    //     chip8.set_reg(reg_a, 0).unwrap();
    //     chip8.set_reg(reg_b, a_val).unwrap();

    // }

    #[test]
    fn test_draw() {
        let reg_a: u8 = 0;
        let reg_b: u8 = 1;
        let mut x: u8;
        let mut y: u8;
        let height: u8 = 5;
        let mut chip8 = test_init();

        for _ in 0..2 {
            for i in 0..0x10 {
                debug!("{}", i);
                x = 5 * i;
                y = 6 * (i / 13);
                chip8.set_reg(reg_a, x).unwrap();
                chip8.set_reg(reg_b, y).unwrap();
                chip8.regs.index_reg = (i * height) as u16;
                debug!("Trying index {}", i);
                let draw_instr: Chip8Instr = Chip8Instr::Draw(Chip8DoubleRegImmOp {
                    a: reg_a,
                    b: reg_b,
                    imm: height,
                });
                test_exec(&mut chip8, draw_instr);
                chip8.dbg_display();
            }
        }
    }
}
