#![allow(unused)]

use assembler::{AssembledInstruction, Constant};
use parser::ParseError;

mod assembler;
pub mod lexer;
pub mod parser;
pub mod vm;

#[derive(Debug, Clone)]
pub struct Program {
    min_num_registers: usize,
    constants: Vec<Constant>,
    assembled: Vec<AssembledInstruction>,
}

impl Program {
    pub fn new_from_source(lines: String) -> Result<Self, ParseError> {
        let parsed = parser::parse(lines)?;

        let mut assembler = assembler::Assembler::new(parsed);
        assembler.assemble();

        Ok(Self {
            min_num_registers: assembler.min_num_registers,
            constants: assembler.constants,
            assembled: assembler.assembled,
        })
    }
}
