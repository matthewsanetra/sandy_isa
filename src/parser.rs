use crate::lexer::{self, LexError, Token};

use self::argchecker::TypeErr;

pub mod argchecker;
mod grammar;

pub(crate) type Loc = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Arg {
    Val(Loc, i64),
    Reg(Loc, i64),
    Addr(Loc, Label),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgType {
    ImmediateValue,
    Register,
    Label,
}

impl Arg {
    pub(crate) fn to_argtype(&self) -> ArgType {
        match self {
            Arg::Val(_, _) => ArgType::ImmediateValue,
            Arg::Reg(_, _) => ArgType::Register,
            Arg::Addr(_, _) => ArgType::Label,
        }
    }

    pub(crate) fn loc(&self) -> Loc {
        match self {
            &Arg::Val(l, _) => l,
            &Arg::Reg(l, _) => l,
            &Arg::Addr(l, _) => l,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Label(pub(crate) String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Instruction {
    Label(Loc, Label),
    NoArg(Loc, Op),
    OneArg(Loc, Op, Arg),
    TwoArg(Loc, Op, Arg, Arg),
    ThreeArg(Loc, Op, Arg, Arg, Arg),
}

impl Instruction {
    pub(crate) fn get_op(&self) -> Op {
        match self {
            Instruction::Label(_, _) => unreachable!(),
            &Instruction::NoArg(_, o) => o,
            &Instruction::OneArg(_, o, _) => o,
            &Instruction::TwoArg(_, o, _, _) => o,
            &Instruction::ThreeArg(_, o, _, _, _) => o,
        }
    }

    pub(crate) fn deconstruct_one_arg(self) -> Arg {
        match self {
            Instruction::OneArg(_, _, a) => a,
            _ => unreachable!(),
        }
    }

    pub(crate) fn deconstruct_two_arg(self) -> (Arg, Arg) {
        match self {
            Instruction::TwoArg(_, _, a1, a2) => (a1, a2),
            _ => unreachable!(),
        }
    }

    pub(crate) fn deconstruct_three_arg(self) -> (Arg, Arg, Arg) {
        match self {
            Instruction::ThreeArg(_, _, a1, a2, a3) => (a1, a2, a3),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    And,
    Or,
    Not,
    Xor,
    Load,
    Call,
    Jump,
    Jez,
    Jnz,
    Jgz,
    Jlz,
    Ret,
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
    Push,
    Pop,
    Halt,
}

impl Op {
    pub(crate) fn first_arg_types(&self) -> Vec<ArgType> {
        use ArgType::*;
        match self {
            Op::Add => vec![Register],
            Op::Sub => vec![Register],
            Op::Mul => vec![Register],
            Op::Div => vec![Register],
            Op::Mod => vec![Register],
            Op::Neg => vec![Register],
            Op::And => vec![Register],
            Op::Or => vec![Register],
            Op::Not => vec![Register],
            Op::Xor => vec![Register],
            Op::Load => vec![ImmediateValue, Register],
            Op::Call => vec![Label],
            Op::Jump => vec![Label],
            Op::Jez => vec![Register],
            Op::Jnz => vec![Register],
            Op::Jgz => vec![Register],
            Op::Jlz => vec![Register],
            Op::Ret => vec![],
            Op::Gt => vec![Register],
            Op::Gte => vec![Register],
            Op::Lt => vec![Register],
            Op::Lte => vec![Register],
            Op::Eq => vec![Register],
            Op::Neq => vec![Register],
            Op::Push => vec![ImmediateValue, Register],
            Op::Pop => vec![Register],
            Op::Halt => vec![Register],
        }
    }

    pub(crate) fn second_arg_types(&self) -> Vec<ArgType> {
        use ArgType::*;
        match self {
            Op::Add => vec![Register],
            Op::Sub => vec![Register],
            Op::Mul => vec![Register],
            Op::Div => vec![Register],
            Op::Mod => vec![Register],
            Op::Neg => vec![Register],
            Op::And => vec![Register],
            Op::Or => vec![Register],
            Op::Not => vec![Register],
            Op::Xor => vec![Register],
            Op::Load => vec![Register],
            Op::Call => vec![Label],
            Op::Jump => vec![],
            Op::Jez => vec![Label],
            Op::Jnz => vec![Label],
            Op::Jgz => vec![Label],
            Op::Jlz => vec![Label],
            Op::Ret => vec![],
            Op::Gt => vec![Register],
            Op::Gte => vec![Register],
            Op::Lt => vec![Register],
            Op::Lte => vec![Register],
            Op::Eq => vec![Register],
            Op::Neq => vec![Register],
            Op::Push => vec![],
            Op::Pop => vec![],
            Op::Halt => vec![],
        }
    }

    pub(crate) fn third_arg_types(&self) -> Vec<ArgType> {
        use ArgType::*;
        match self {
            Op::Add => vec![Register],
            Op::Sub => vec![Register],
            Op::Mul => vec![Register],
            Op::Div => vec![Register],
            Op::Mod => vec![Register],
            Op::Neg => vec![Register],
            Op::And => vec![Register],
            Op::Or => vec![Register],
            Op::Not => vec![Register],
            Op::Xor => vec![Register],
            Op::Load => vec![],
            Op::Call => vec![],
            Op::Jump => vec![],
            Op::Jez => vec![],
            Op::Jnz => vec![],
            Op::Jgz => vec![],
            Op::Jlz => vec![],
            Op::Ret => vec![],
            Op::Gt => vec![Register],
            Op::Gte => vec![Register],
            Op::Lt => vec![Register],
            Op::Lte => vec![Register],
            Op::Eq => vec![Register],
            Op::Neq => vec![Register],
            Op::Push => vec![],
            Op::Pop => vec![],
            Op::Halt => vec![],
        }
    }

    pub(crate) fn get_arg_type(&self, i: usize) -> Vec<ArgType> {
        match i {
            1 => self.first_arg_types(),
            2 => self.second_arg_types(),
            3 => self.third_arg_types(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    SyntaxError(lalrpop_util::ParseError<usize, Token, LexError>),
    TypeError(Vec<TypeErr>),
}

pub(crate) fn parse(lines: String) -> Result<Vec<Instruction>, ParseError> {
    let lexer = lexer::Lexer::new(&lines);
    let parser = grammar::ProgramParser::new();
    let insns = match parser.parse(lexer) {
        Ok(insns) => insns,
        Err(err) => return Err(ParseError::SyntaxError(err)),
    };
    argchecker::check_instructions(&insns).map_err(|e| ParseError::TypeError(e))?;
    Ok(insns)
}
