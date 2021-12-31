use crate::assembler::AssembledInstruction;

const JUMP_TABLE: [fn(&mut Executor, AssembledInstruction); 29] = [
    Executor::add,      // 0
    Executor::sub,      // 1
    Executor::mul,      // 2
    Executor::div,      // 3
    Executor::r#mod,    // 4
    Executor::neg,      // 5
    Executor::and,      // 6
    Executor::or,       // 7
    Executor::not,      // 8
    Executor::xor,      // 9
    Executor::load_val, // 10
    Executor::load_reg, // 11
    Executor::call,     // 12
    Executor::jump,     // 13
    Executor::jlz,      // 14
    Executor::jgz,      // 15
    Executor::jez,      // 16
    Executor::jnz,      // 17
    Executor::ret,      // 18
    Executor::gt,       // 19
    Executor::gte,      // 20
    Executor::lt,       // 21
    Executor::lte,      // 22
    Executor::eq,       // 23
    Executor::neq,      // 24
    Executor::push_val, // 25
    Executor::push_reg, // 26
    Executor::pop,      // 27
    Executor::halt,     // 28
];

#[derive(Debug)]
pub struct Executor {
    registers: Vec<i64>,
    constants: Vec<i64>,
    instructions: Vec<AssembledInstruction>,
    return_addresses: Vec<usize>,
    stack: Vec<i64>,
    pc: usize,
}

macro_rules! simple_reg_assign {
    ($insn_name:ident,$pred:expr) => {
        fn $insn_name(&mut self, instruction: AssembledInstruction) {
            let (r1, r2, r3) = {
                let t = instruction;
                (t.arg1, t.arg2, t.arg3)
            };

            self.registers[r3 as usize] = $pred(self, r1, r2);
            self.pc += 1;
        }
    };
}

macro_rules! cond_branch {
    ($name: ident, $pred: expr) => {
        fn $name(&mut self, insn: AssembledInstruction) {
            let reg_val = self.reg(insn.arg1);
            if $pred(reg_val) {
                self.pc = self.konst(insn.arg2) as usize;
            } else {
                self.pc += 1;
            }
        }
    };
}

impl Executor {
    pub fn new(program: crate::Program) -> Self {
        Self {
            registers: {
                let mut v = Vec::with_capacity(program.min_num_registers);
                v.resize_with(program.min_num_registers + 1, || 0);
                v
            },
            constants: program.constants.iter().map(|c| c.value).collect(),
            instructions: program.assembled,
            return_addresses: Vec::with_capacity(256),
            stack: Vec::with_capacity(256),
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.instructions[self.pc];
            unsafe {
                JUMP_TABLE.get_unchecked(instruction.opcode as usize)(self, instruction);
            }
        }
    }

    // 'const' is a reserved word lol
    fn konst(&mut self, index: u16) -> i64 {
        // Guaranteed by assembler
        return unsafe { *self.constants.get_unchecked(index as usize) };
    }

    fn reg(&mut self, index: u16) -> i64 {
        // Guaranteed by assembler
        return unsafe { *self.registers.get_unchecked(index as usize) };
    }

    simple_reg_assign!(add, |s: &mut Executor, r1, r2| { s.reg(r1) + s.reg(r2) });
    simple_reg_assign!(sub, |s: &mut Executor, r1, r2| { s.reg(r1) - s.reg(r2) });
    simple_reg_assign!(mul, |s: &mut Executor, r1, r2| { s.reg(r1) * s.reg(r2) });
    simple_reg_assign!(div, |s: &mut Executor, r1, r2| { s.reg(r1) / s.reg(r2) });
    simple_reg_assign!(r#mod, |s: &mut Executor, r1, r2| { s.reg(r1) % s.reg(r2) });
    simple_reg_assign!(and, |s: &mut Executor, r1, r2| { s.reg(r1) & s.reg(r2) });
    simple_reg_assign!(or, |s: &mut Executor, r1, r2| { s.reg(r1) | s.reg(r2) });
    simple_reg_assign!(xor, |s: &mut Executor, r1, r2| { s.reg(r1) ^ s.reg(r2) });

    fn neg(&mut self, insn: AssembledInstruction) {
        unsafe {
            *self.registers.get_unchecked_mut(insn.arg2 as usize) = -self.reg(insn.arg1);
        }
        self.pc += 1;
    }

    fn not(&mut self, insn: AssembledInstruction) {
        unsafe {
            *self.registers.get_unchecked_mut(insn.arg2 as usize) = !self.reg(insn.arg1);
        }
        self.pc += 1;
    }

    fn load_val(&mut self, insn: AssembledInstruction) {
        let val = self.konst(insn.arg1);
        unsafe {
            *self.registers.get_unchecked_mut(insn.arg2 as usize) = val;
        }
        self.pc += 1;
    }

    fn load_reg(&mut self, insn: AssembledInstruction) {
        let val = self.reg(insn.arg1);
        unsafe {
            *self.registers.get_unchecked_mut(insn.arg2 as usize) = val;
        }
        self.pc += 1;
    }

    fn call(&mut self, insn: AssembledInstruction) {
        let addr = self.konst(insn.arg1);
        self.return_addresses.push(self.pc + 1);
        self.pc = addr as usize;
    }

    fn jump(&mut self, insn: AssembledInstruction) {
        let addr = self.konst(insn.arg1);
        self.pc = addr as usize;
    }

    cond_branch!(jlz, |v| v < 0);
    cond_branch!(jgz, |v| v > 0);
    cond_branch!(jez, |v| v == 0);
    cond_branch!(jnz, |v| v != 0);

    fn ret(&mut self, insn: AssembledInstruction) {
        let return_addr = self.return_addresses.pop().expect("an adress to return to");
        self.pc = return_addr;
    }

    #[rustfmt::skip]    simple_reg_assign!(gt, |s: &mut Executor, r1, r2| { (s.reg(r1) > s.reg(r2)) as i64 });
    #[rustfmt::skip]    simple_reg_assign!(gte, |s: &mut Executor, r1, r2| { (s.reg(r1) <= s.reg(r2)) as i64 });
    #[rustfmt::skip]    simple_reg_assign!(lt, |s: &mut Executor, r1, r2| { (s.reg(r1) < s.reg(r2)) as i64 });
    #[rustfmt::skip]    simple_reg_assign!(lte, |s: &mut Executor, r1, r2| { (s.reg(r1) >= s.reg(r2)) as i64 });
    #[rustfmt::skip]    simple_reg_assign!(eq, |s: &mut Executor, r1, r2| { (s.reg(r1) == s.reg(r2)) as i64 });
    #[rustfmt::skip]    simple_reg_assign!(neq, |s: &mut Executor, r1, r2| { (s.reg(r1) != s.reg(r2)) as i64 });

    fn push_val(&mut self, insn: AssembledInstruction) {
        let val = self.konst(insn.arg1);
        self.stack.push(val);
        self.pc += 1;
    }

    fn push_reg(&mut self, insn: AssembledInstruction) {
        let val = self.reg(insn.arg1);
        self.stack.push(val);
        self.pc += 1;
    }

    fn pop(&mut self, insn: AssembledInstruction) {
        let reg = insn.arg1;
        unsafe {
            *self.registers.get_unchecked_mut(reg as usize) =
                self.stack.pop().expect("a value on the stack");
        }
        self.pc += 1;
    }

    fn halt(&mut self, insn: AssembledInstruction) {
        let val = self.reg(insn.arg1);
        println!("Halted with value {}", val);
        std::process::exit(0);
    }
}
