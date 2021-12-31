use super::{ArgType, Instruction, Loc};

pub(crate) fn check_instructions(insns: &Vec<Instruction>) -> Result<(), Vec<TypeErr>> {
    let mut errors = vec![];
    for insn in insns {
        match check(insn) {
            Ok(()) => {}
            Err(mut errs) => errors.append(&mut errs),
        }
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TypeErr {
    pub loc: Loc,
    pub expected: Vec<ArgType>,
    pub found: ArgType,
}

fn check(insn: &Instruction) -> Result<(), Vec<TypeErr>> {
    use super::Arg::*;
    use super::Op::*;
    let (op, given) = match insn {
        Instruction::Label(_, _) => return Ok(()),
        Instruction::NoArg(_, _) => return Ok(()),
        Instruction::OneArg(_, op, arg) => (op, vec![(arg.loc(), arg.to_argtype())]),
        Instruction::TwoArg(_, op, a1, a2) => (
            op,
            vec![(a1.loc(), a1.to_argtype()), (a2.loc(), a2.to_argtype())],
        ),
        Instruction::ThreeArg(_, op, a1, a2, a3) => (
            op,
            vec![
                (a1.loc(), a1.to_argtype()),
                (a2.loc(), a2.to_argtype()),
                (a3.loc(), a3.to_argtype()),
            ],
        ),
    };
    let mut type_errors = vec![];
    for (i, (loc, arg)) in given.iter().enumerate() {
        let expected = op.get_arg_type(i + 1);
        if !expected.contains(arg) {
            type_errors.push(TypeErr {
                loc: *loc,
                expected,
                found: *arg,
            });
        }
    }
    Err(type_errors)
}
