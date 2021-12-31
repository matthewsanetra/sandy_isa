use std::{collections::BTreeMap, convert::TryInto};

use crate::parser::{Arg, Instruction, Op};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct AssembledInstruction {
    pub(crate) opcode: u8,
    pub(crate) arg1: u16,
    pub(crate) arg2: u16,
    pub(crate) arg3: u16,
    padding: u8,
}

impl AssembledInstruction {
    pub(crate) fn new(op: u8, arg1: u16, arg2: u16, arg3: u16) -> Self {
        Self {
            opcode: op,
            arg1,
            arg2,
            arg3,
            padding: 69,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Constant {
    pub(crate) value: i64,
}

pub(crate) struct Assembler {
    pub(crate) constants: Vec<Constant>,
    pub(crate) assembled: Vec<AssembledInstruction>,
    source: Vec<Instruction>,
    labels: BTreeMap<String, u16>,
    pub(crate) min_num_registers: usize,
}

impl Assembler {
    pub(crate) fn new(source: Vec<Instruction>) -> Self {
        Self {
            constants: Vec::new(),
            assembled: Vec::new(),
            source,
            labels: BTreeMap::new(),
            min_num_registers: 0,
        }
    }

    pub(crate) fn assemble(&mut self) {
        let mut addr_with_labels = 0;
        let mut num_labels = 0;
        while addr_with_labels < self.source.len() {
            let insn = &self.source[addr_with_labels];
            let addr = addr_with_labels - num_labels;
            if let Instruction::Label(_, label) = insn {
                // if the label already exists, then update constant with the current address
                if let Some(i) = self.labels.get(&label.0) {
                    self.constants[*i as usize] = Constant { value: addr as i64 };
                } else {
                    let const_index = self.constants.len();
                    self.constants.push(Constant { value: addr as i64 });
                    self.labels.insert(
                        label.0.clone(),
                        const_index
                            .try_into()
                            .expect("too many constants; labels are also constants"),
                    );
                }
                addr_with_labels += 1;
                num_labels += 1;
                continue;
            }

            let binary_opcode = self.bin_opcode(insn.clone());

            let mut arg1 = None;
            let mut arg2 = None;
            let mut arg3 = None;

            match insn {
                Instruction::Label(_, _) => unreachable!(),
                Instruction::NoArg(_, _) => {}
                Instruction::OneArg(_, _, a) => {
                    arg1 = Some(a.clone());
                }
                Instruction::TwoArg(_, _, a1, a2) => {
                    arg1 = Some(a1.clone());
                    arg2 = Some(a2.clone());
                }
                Instruction::ThreeArg(_, _, a1, a2, a3) => {
                    arg1 = Some(a1.clone());
                    arg2 = Some(a2.clone());
                    arg3 = Some(a3.clone());
                }
            }

            let a1 = self.process_arg(arg1);
            let a2 = self.process_arg(arg2);
            let a3 = self.process_arg(arg3);

            self.assembled
                .push(AssembledInstruction::new(binary_opcode, a1, a2, a3));
            addr_with_labels += 1;
        }
    }

    fn bin_opcode(&self, insn: Instruction) -> u8 {
        match insn.get_op() {
            Op::Add => 0u8,
            Op::Sub => 1,
            Op::Mul => 2,
            Op::Div => 3,
            Op::Mod => 4,
            Op::Neg => 5,
            Op::And => 6,
            Op::Or => 7,
            Op::Not => 8,
            Op::Xor => 9,
            Op::Load => match insn.deconstruct_two_arg() {
                (Arg::Val(_, _), _) => 10,
                (Arg::Reg(_, _), _) => 11,
                _ => unreachable!(),
            },
            Op::Call => 12,
            Op::Jump => 13,
            Op::Jlz => 14,
            Op::Jgz => 15,
            Op::Jez => 16,
            Op::Jnz => 17,
            Op::Ret => 18,
            Op::Gt => 19,
            Op::Gte => 20,
            Op::Lt => 21,
            Op::Lte => 22,
            Op::Eq => 23,
            Op::Neq => 24,
            Op::Push => match insn.deconstruct_one_arg() {
                Arg::Val(_, _) => 25,
                Arg::Reg(_, _) => 26,
                _ => unreachable!(),
            },
            Op::Pop => 27,
            Op::Halt => 28,
        }
    }

    fn process_arg(&mut self, arg: Option<Arg>) -> u16 {
        match arg {
            Some(Arg::Val(_, value)) => {
                let i = self.constants.len();
                self.constants.push(Constant { value });
                i.try_into().expect("error: too many constants")
            }
            Some(Arg::Reg(_, r)) => {
                self.min_num_registers = self.min_num_registers.max(r as usize);

                r.try_into().expect("register can't fit into a u16")
            }
            Some(Arg::Addr(_, l)) => {
                let label = l.0;
                let l = self.labels.get(&label);
                match l {
                    Some(i) => *i,
                    None => {
                        let const_index = self.constants.len();
                        self.constants.push(Constant {
                            value: u64::MAX as i64,
                        });
                        self.labels.insert(label, const_index as u16);

                        const_index as u16
                    }
                }
            }
            None => 69,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn all_64_bits() {
        assert_eq!(size_of::<AssembledInstruction>(), 8);
        assert_eq!(size_of::<Constant>(), 8);
    }
}
