// auto-generated: "lalrpop 0.19.6"
// sha3: 606359daf77150c76d4869eee73b1a1fd69be31fe842bacb2f0e05476fbb7
use crate::parser::{Arg, Op, Label, Instruction};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::parser::{Arg, Op, Label, Instruction};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(crate::lexer::Token),
        Variant1(String),
        Variant2(i64),
        Variant3(core::option::Option<crate::lexer::Token>),
        Variant4(usize),
        Variant5(Arg),
        Variant6(Instruction),
        Variant7(alloc::vec::Vec<Instruction>),
        Variant8(Op),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 20, 0, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 0, 0,
        // State 1
        0, 0, 0, 20, 0, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 0, 0,
        // State 2
        53, 54, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 3
        53, 54, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 4
        53, 54, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 5
        53, 54, 9, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 6
        53, 54, 10, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 7
        53, 54, 11, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 8
        53, 54, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 9
        53, 54, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 10
        53, 54, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 11
        53, 54, 13, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 12
        53, 54, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56,
        // State 13
        0, 0, 0, -18, 0, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, 0, 0,
        // State 14
        0, 0, 0, -15, 0, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, 0, 0,
        // State 15
        0, 0, 0, -14, 0, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, -12, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0,
        // State 18
        0, 0, 0, -13, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0,
        // State 20
        -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39,
        // State 21
        -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44,
        // State 22
        -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23,
        // State 23
        -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42,
        // State 24
        -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51,
        // State 25
        -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47,
        // State 26
        -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48,
        // State 27
        -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27,
        // State 28
        -62, -62, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62,
        // State 29
        -61, -61, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61,
        // State 30
        -60, -60, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60,
        // State 31
        -63, -63, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63,
        // State 32
        -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24,
        // State 33
        -59, -59, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59,
        // State 34
        -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49,
        // State 35
        -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50,
        // State 36
        -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43,
        // State 37
        -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41,
        // State 38
        -57, -57, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57,
        // State 39
        -52, -52, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52,
        // State 40
        -58, -58, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58,
        // State 41
        -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45,
        // State 42
        -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26,
        // State 43
        -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25,
        // State 44
        0, 0, 0, -20, 0, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 0, 0,
        // State 45
        -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40,
        // State 46
        -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46,
        // State 47
        0, 0, 0, -19, 0, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, 0, 0,
        // State 48
        -10, -10, -10, -10, 0, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, 0, -10,
        // State 49
        0, 0, 58, -22, 0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, 0,
        // State 50
        -9, -9, -9, -9, 0, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, 0, -9,
        // State 51
        -8, -8, -8, -8, 0, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, 0, -8,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0,
        // State 55
        -65, -65, -65, -65, 0, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, -65, 0, -65,
        // State 56
        0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, -21, 0, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 0, 0,
        // State 58
        -64, -64, -64, -64, 0, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, -64, 0, -64,
        // State 59
        -30, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, -30,
        // State 60
        -7, -7, -7, -7, 0, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, 0, -7,
        // State 61
        0, 0, 65, -56, 0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0,
        // State 62
        0, 0, 0, -11, 0, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, 0, 0,
        // State 63
        0, 0, 67, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, 0, 0,
        // State 64
        0, 0, 0, -55, 0, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, 0, 0,
        // State 65
        0, 0, 70, -54, 0, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, -54, 0, 0,
        // State 66
        0, 0, 0, -37, 0, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, 0, 0,
        // State 67
        0, 0, 71, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, 0,
        // State 68
        0, 0, 72, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, 0,
        // State 69
        0, 0, 0, -53, 0, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, -53, 0, 0,
        // State 70
        0, 0, 0, -35, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, 0,
        // State 71
        0, 0, 0, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, 0,
        // State 72
        0, 0, 74, -32, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, 0, 0,
        // State 73
        0, 0, 0, -31, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 34 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -28,
        // State 1
        -29,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        -18,
        // State 14
        -15,
        // State 15
        -14,
        // State 16
        -66,
        // State 17
        -12,
        // State 18
        -13,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        -20,
        // State 45
        0,
        // State 46
        0,
        // State 47
        -19,
        // State 48
        -10,
        // State 49
        -22,
        // State 50
        -9,
        // State 51
        -8,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        -65,
        // State 56
        0,
        // State 57
        -21,
        // State 58
        -64,
        // State 59
        -30,
        // State 60
        -7,
        // State 61
        -56,
        // State 62
        -11,
        // State 63
        -38,
        // State 64
        -55,
        // State 65
        -54,
        // State 66
        -37,
        // State 67
        -36,
        // State 68
        -34,
        // State 69
        -53,
        // State 70
        -35,
        // State 71
        -33,
        // State 72
        -32,
        // State 73
        -31,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            4 => 48,
            5 => match state {
                4 => 6,
                5 => 7,
                8 => 11,
                2 => 49,
                6 => 61,
                7 => 63,
                9 => 65,
                10 => 67,
                11 => 68,
                12 => 72,
                _ => 5,
            },
            6 => match state {
                1 => 47,
                _ => 13,
            },
            8 => 1,
            9 => 14,
            10 => 15,
            11 => 2,
            12 => 16,
            13 => 50,
            14 => 17,
            15 => 3,
            16 => 18,
            17 => 4,
            18 => 51,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""#""###,
            r###""%""###,
            r###"",""###,
            r###"".""###,
            r###"":""###,
            r###""add""###,
            r###""and""###,
            r###""call""###,
            r###""div""###,
            r###""eq""###,
            r###""gt""###,
            r###""gte""###,
            r###""halt""###,
            r###""jez""###,
            r###""jgz""###,
            r###""jlz""###,
            r###""jnz""###,
            r###""jump""###,
            r###""load""###,
            r###""lt""###,
            r###""lte""###,
            r###""mod""###,
            r###""mul""###,
            r###""neg""###,
            r###""neq""###,
            r###""not""###,
            r###""or""###,
            r###""pop""###,
            r###""push""###,
            r###""ret""###,
            r###""sub""###,
            r###""xor""###,
            r###"ident"###,
            r###"num"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<>
    where 
    {
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = crate::lexer::LexError;
        type Token = crate::lexer::Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = alloc::vec::Vec<Instruction>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 34 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &crate::lexer::Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            crate::lexer::Token::Hashtag if true => Some(0),
            crate::lexer::Token::Percentage if true => Some(1),
            crate::lexer::Token::Comma if true => Some(2),
            crate::lexer::Token::Dot if true => Some(3),
            crate::lexer::Token::Colon if true => Some(4),
            crate::lexer::Token::Add if true => Some(5),
            crate::lexer::Token::And if true => Some(6),
            crate::lexer::Token::Call if true => Some(7),
            crate::lexer::Token::Div if true => Some(8),
            crate::lexer::Token::Eq if true => Some(9),
            crate::lexer::Token::Gt if true => Some(10),
            crate::lexer::Token::Gte if true => Some(11),
            crate::lexer::Token::Halt if true => Some(12),
            crate::lexer::Token::Jez if true => Some(13),
            crate::lexer::Token::Jgz if true => Some(14),
            crate::lexer::Token::Jlz if true => Some(15),
            crate::lexer::Token::Jnz if true => Some(16),
            crate::lexer::Token::Jump if true => Some(17),
            crate::lexer::Token::Load if true => Some(18),
            crate::lexer::Token::Lt if true => Some(19),
            crate::lexer::Token::Lte if true => Some(20),
            crate::lexer::Token::Mod if true => Some(21),
            crate::lexer::Token::Mul if true => Some(22),
            crate::lexer::Token::Neg if true => Some(23),
            crate::lexer::Token::Neq if true => Some(24),
            crate::lexer::Token::Not if true => Some(25),
            crate::lexer::Token::Or if true => Some(26),
            crate::lexer::Token::Pop if true => Some(27),
            crate::lexer::Token::Push if true => Some(28),
            crate::lexer::Token::Ret if true => Some(29),
            crate::lexer::Token::Sub if true => Some(30),
            crate::lexer::Token::Xor if true => Some(31),
            crate::lexer::Token::Identifier(_) if true => Some(32),
            crate::lexer::Token::Number(_) if true => Some(33),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: crate::lexer::Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 => __Symbol::Variant0(__token),
            32 => match __token {
                crate::lexer::Token::Identifier(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            33 => match __token {
                crate::lexer::Token::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub(crate) struct ProgramParser {
        _priv: (),
    }

    impl ProgramParser {
        pub(crate) fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub(crate) fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<alloc::vec::Vec<Instruction>, __lalrpop_util::ParseError<usize, crate::lexer::Token, crate::lexer::LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<alloc::vec::Vec<Instruction>,__lalrpop_util::ParseError<usize, crate::lexer::Token, crate::lexer::LexError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Arg, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Op, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<crate::lexer::Token>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, crate::lexer::Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // "#"? = "#" => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // "#"? =  => ActionFn(48);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action48::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? = "," => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? =  => ActionFn(46);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action46::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(50);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action50::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Addr = ".", ident => ActionFn(89);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action89::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Argument = Val => ActionFn(28);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Argument = Reg => ActionFn(29);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Argument = Addr => ActionFn(30);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Insn = ".", ident, ":" => ActionFn(90);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action90::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Insn = ThreeArgInsn => ActionFn(38);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Insn = TwoArgInsn => ActionFn(39);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Insn = OneArgInsn => ActionFn(40);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Insn = NoArgOp => ActionFn(91);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action91::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Insn* =  => ActionFn(43);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action43::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Insn* = Insn+ => ActionFn(44);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Insn+ = Insn => ActionFn(51);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Insn+ = Insn+, Insn => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NoArgOp = "ret" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OneArgInsn = OneArgOp, Argument, "," => ActionFn(92);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action92::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OneArgInsn = OneArgOp, Argument => ActionFn(93);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action93::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OneArgOp = "call" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OneArgOp = "jump" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OneArgOp = "push" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OneArgOp = "pop" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OneArgOp = "halt" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(109);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action109::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = Insn+ => ActionFn(110);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action110::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Reg = "%", num => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action94::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgInsn = ThreeArgOp, Argument, ",", Argument, ",", Argument, "," => ActionFn(95);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant5(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action95::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (7, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgInsn = ThreeArgOp, Argument, ",", Argument, ",", Argument => ActionFn(96);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant5(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action96::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (6, 14)
    }
    pub(crate) fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgInsn = ThreeArgOp, Argument, ",", Argument, Argument, "," => ActionFn(97);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action97::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (6, 14)
    }
    pub(crate) fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgInsn = ThreeArgOp, Argument, ",", Argument, Argument => ActionFn(98);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action98::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgInsn = ThreeArgOp, Argument, Argument, ",", Argument, "," => ActionFn(99);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action99::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (6, 14)
    }
    pub(crate) fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgInsn = ThreeArgOp, Argument, Argument, ",", Argument => ActionFn(100);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action100::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgInsn = ThreeArgOp, Argument, Argument, Argument, "," => ActionFn(101);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action101::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgInsn = ThreeArgOp, Argument, Argument, Argument => ActionFn(102);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action102::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "add" => ActionFn(1);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "sub" => ActionFn(2);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "mul" => ActionFn(3);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "div" => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "mod" => ActionFn(5);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "and" => ActionFn(6);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "or" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "xor" => ActionFn(8);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "gt" => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "gte" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "lt" => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "lte" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "eq" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ThreeArgOp = "neq" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgInsn = TwoArgOp, Argument, ",", Argument, "," => ActionFn(103);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action103::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (5, 16)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgInsn = TwoArgOp, Argument, ",", Argument => ActionFn(104);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action104::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 16)
    }
    pub(crate) fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgInsn = TwoArgOp, Argument, Argument, "," => ActionFn(105);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action105::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 16)
    }
    pub(crate) fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgInsn = TwoArgOp, Argument, Argument => ActionFn(106);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action106::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgOp = "neg" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgOp = "not" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgOp = "load" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgOp = "jlz" => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgOp = "jgz" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgOp = "jez" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // TwoArgOp = "jnz" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Val = "#", num => ActionFn(107);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action107::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Val = num => ActionFn(108);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action108::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 18)
    }
}
pub(crate) use self::__parse__Program::ProgramParser;

fn __action0<
>(
    (_, __0, _): (usize, alloc::vec::Vec<Instruction>, usize),
) -> alloc::vec::Vec<Instruction>
{
    __0
}

fn __action1<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Add
}

fn __action2<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Sub
}

fn __action3<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Mul
}

fn __action4<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Div
}

fn __action5<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Mod
}

fn __action6<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::And
}

fn __action7<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Or
}

fn __action8<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Xor
}

fn __action9<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Gt
}

fn __action10<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Gte
}

fn __action11<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Lt
}

fn __action12<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Lte
}

fn __action13<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Eq
}

fn __action14<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Neq
}

fn __action15<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Neg
}

fn __action16<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Not
}

fn __action17<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Load
}

fn __action18<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Jlz
}

fn __action19<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Jgz
}

fn __action20<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Jez
}

fn __action21<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Jnz
}

fn __action22<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Call
}

fn __action23<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Jump
}

fn __action24<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Push
}

fn __action25<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Pop
}

fn __action26<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Halt
}

fn __action27<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> Op
{
    Op::Ret
}

fn __action28<
>(
    (_, __0, _): (usize, Arg, usize),
) -> Arg
{
    __0
}

fn __action29<
>(
    (_, __0, _): (usize, Arg, usize),
) -> Arg
{
    __0
}

fn __action30<
>(
    (_, __0, _): (usize, Arg, usize),
) -> Arg
{
    __0
}

fn __action31<
>(
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, crate::lexer::Token, usize),
    (_, i, _): (usize, String, usize),
    (_, r, _): (usize, usize, usize),
) -> Arg
{
    Arg::Addr((l, r), Label(i))
}

fn __action32<
>(
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, core::option::Option<crate::lexer::Token>, usize),
    (_, n, _): (usize, i64, usize),
    (_, r, _): (usize, usize, usize),
) -> Arg
{
    Arg::Val((l, r), n)
}

fn __action33<
>(
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, crate::lexer::Token, usize),
    (_, n, _): (usize, i64, usize),
    (_, r, _): (usize, usize, usize),
) -> Arg
{
    Arg::Reg((l, r), n)
}

fn __action34<
>(
    (_, l, _): (usize, usize, usize),
    (_, o, _): (usize, Op, usize),
    (_, r, _): (usize, usize, usize),
    (_, a, _): (usize, Arg, usize),
    (_, _, _): (usize, core::option::Option<crate::lexer::Token>, usize),
    (_, b, _): (usize, Arg, usize),
    (_, _, _): (usize, core::option::Option<crate::lexer::Token>, usize),
    (_, c, _): (usize, Arg, usize),
    (_, _, _): (usize, core::option::Option<crate::lexer::Token>, usize),
) -> Instruction
{
    Instruction::ThreeArg((l, r), o, a, b, c)
}

fn __action35<
>(
    (_, l, _): (usize, usize, usize),
    (_, o, _): (usize, Op, usize),
    (_, r, _): (usize, usize, usize),
    (_, a, _): (usize, Arg, usize),
    (_, _, _): (usize, core::option::Option<crate::lexer::Token>, usize),
    (_, b, _): (usize, Arg, usize),
    (_, _, _): (usize, core::option::Option<crate::lexer::Token>, usize),
) -> Instruction
{
    Instruction::TwoArg((l, r), o, a, b)
}

fn __action36<
>(
    (_, l, _): (usize, usize, usize),
    (_, o, _): (usize, Op, usize),
    (_, r, _): (usize, usize, usize),
    (_, a, _): (usize, Arg, usize),
    (_, _, _): (usize, core::option::Option<crate::lexer::Token>, usize),
) -> Instruction
{
    Instruction::OneArg((l, r), o, a)
}

fn __action37<
>(
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, crate::lexer::Token, usize),
    (_, i, _): (usize, String, usize),
    (_, _, _): (usize, crate::lexer::Token, usize),
    (_, r, _): (usize, usize, usize),
) -> Instruction
{
    Instruction::Label((l, r), Label(i))
}

fn __action38<
>(
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    __0
}

fn __action39<
>(
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    __0
}

fn __action40<
>(
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    __0
}

fn __action41<
>(
    (_, l, _): (usize, usize, usize),
    (_, o, _): (usize, Op, usize),
    (_, r, _): (usize, usize, usize),
) -> Instruction
{
    Instruction::NoArg((l, r), o)
}

fn __action42<
>(
    (_, __0, _): (usize, alloc::vec::Vec<Instruction>, usize),
) -> alloc::vec::Vec<Instruction>
{
    __0
}

fn __action43<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Instruction>
{
    alloc::vec![]
}

fn __action44<
>(
    (_, v, _): (usize, alloc::vec::Vec<Instruction>, usize),
) -> alloc::vec::Vec<Instruction>
{
    v
}

fn __action45<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> core::option::Option<crate::lexer::Token>
{
    Some(__0)
}

fn __action46<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<crate::lexer::Token>
{
    None
}

fn __action47<
>(
    (_, __0, _): (usize, crate::lexer::Token, usize),
) -> core::option::Option<crate::lexer::Token>
{
    Some(__0)
}

fn __action48<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<crate::lexer::Token>
{
    None
}

fn __action49<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

fn __action50<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

fn __action51<
>(
    (_, __0, _): (usize, Instruction, usize),
) -> alloc::vec::Vec<Instruction>
{
    alloc::vec![__0]
}

fn __action52<
>(
    (_, v, _): (usize, alloc::vec::Vec<Instruction>, usize),
    (_, e, _): (usize, Instruction, usize),
) -> alloc::vec::Vec<Instruction>
{
    { let mut v = v; v.push(e); v }
}

fn __action53<
>(
    __0: (usize, usize, usize),
    __1: (usize, crate::lexer::Token, usize),
    __2: (usize, i64, usize),
    __3: (usize, usize, usize),
) -> Arg
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action47(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        __0,
        __temp0,
        __2,
        __3,
    )
}

fn __action54<
>(
    __0: (usize, usize, usize),
    __1: (usize, i64, usize),
    __2: (usize, usize, usize),
) -> Arg
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action48(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        __0,
        __temp0,
        __1,
        __2,
    )
}

fn __action55<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action45(
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

fn __action56<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

fn __action57<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
    __5: (usize, Arg, usize),
    __6: (usize, crate::lexer::Token, usize),
    __7: (usize, Arg, usize),
    __8: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __start1 = __6.0.clone();
    let __end1 = __6.2.clone();
    let __start2 = __8.0.clone();
    let __end2 = __8.2.clone();
    let __temp0 = __action45(
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action45(
        __6,
    );
    let __temp1 = (__start1, __temp1, __end1);
    let __temp2 = __action45(
        __8,
    );
    let __temp2 = (__start2, __temp2, __end2);
    __action34(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __temp1,
        __7,
        __temp2,
    )
}

fn __action58<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
    __5: (usize, Arg, usize),
    __6: (usize, crate::lexer::Token, usize),
    __7: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __start1 = __6.0.clone();
    let __end1 = __6.2.clone();
    let __start2 = __7.2.clone();
    let __end2 = __7.2.clone();
    let __temp0 = __action45(
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action45(
        __6,
    );
    let __temp1 = (__start1, __temp1, __end1);
    let __temp2 = __action46(
        &__start2,
        &__end2,
    );
    let __temp2 = (__start2, __temp2, __end2);
    __action34(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __temp1,
        __7,
        __temp2,
    )
}

fn __action59<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
    __5: (usize, Arg, usize),
    __6: (usize, Arg, usize),
    __7: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __start1 = __5.2.clone();
    let __end1 = __6.0.clone();
    let __start2 = __7.0.clone();
    let __end2 = __7.2.clone();
    let __temp0 = __action45(
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action46(
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    let __temp2 = __action45(
        __7,
    );
    let __temp2 = (__start2, __temp2, __end2);
    __action34(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __temp1,
        __6,
        __temp2,
    )
}

fn __action60<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
    __5: (usize, Arg, usize),
    __6: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __start1 = __5.2.clone();
    let __end1 = __6.0.clone();
    let __start2 = __6.2.clone();
    let __end2 = __6.2.clone();
    let __temp0 = __action45(
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action46(
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    let __temp2 = __action46(
        &__start2,
        &__end2,
    );
    let __temp2 = (__start2, __temp2, __end2);
    __action34(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __temp1,
        __6,
        __temp2,
    )
}

fn __action61<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, Arg, usize),
    __5: (usize, crate::lexer::Token, usize),
    __6: (usize, Arg, usize),
    __7: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __start1 = __5.0.clone();
    let __end1 = __5.2.clone();
    let __start2 = __7.0.clone();
    let __end2 = __7.2.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action45(
        __5,
    );
    let __temp1 = (__start1, __temp1, __end1);
    let __temp2 = __action45(
        __7,
    );
    let __temp2 = (__start2, __temp2, __end2);
    __action34(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __temp1,
        __6,
        __temp2,
    )
}

fn __action62<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, Arg, usize),
    __5: (usize, crate::lexer::Token, usize),
    __6: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __start1 = __5.0.clone();
    let __end1 = __5.2.clone();
    let __start2 = __6.2.clone();
    let __end2 = __6.2.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action45(
        __5,
    );
    let __temp1 = (__start1, __temp1, __end1);
    let __temp2 = __action46(
        &__start2,
        &__end2,
    );
    let __temp2 = (__start2, __temp2, __end2);
    __action34(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __temp1,
        __6,
        __temp2,
    )
}

fn __action63<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, Arg, usize),
    __5: (usize, Arg, usize),
    __6: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __start1 = __4.2.clone();
    let __end1 = __5.0.clone();
    let __start2 = __6.0.clone();
    let __end2 = __6.2.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action46(
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    let __temp2 = __action45(
        __6,
    );
    let __temp2 = (__start2, __temp2, __end2);
    __action34(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __temp1,
        __5,
        __temp2,
    )
}

fn __action64<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, Arg, usize),
    __5: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __start1 = __4.2.clone();
    let __end1 = __5.0.clone();
    let __start2 = __5.2.clone();
    let __end2 = __5.2.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action46(
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    let __temp2 = __action46(
        &__start2,
        &__end2,
    );
    let __temp2 = (__start2, __temp2, __end2);
    __action34(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __temp1,
        __5,
        __temp2,
    )
}

fn __action65<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
    __5: (usize, Arg, usize),
    __6: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __start1 = __6.0.clone();
    let __end1 = __6.2.clone();
    let __temp0 = __action45(
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action45(
        __6,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action35(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __temp1,
    )
}

fn __action66<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
    __5: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __start1 = __5.2.clone();
    let __end1 = __5.2.clone();
    let __temp0 = __action45(
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action46(
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action35(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __temp1,
    )
}

fn __action67<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, Arg, usize),
    __5: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __start1 = __5.0.clone();
    let __end1 = __5.2.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action45(
        __5,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action35(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __temp1,
    )
}

fn __action68<
>(
    __0: (usize, usize, usize),
    __1: (usize, Op, usize),
    __2: (usize, usize, usize),
    __3: (usize, Arg, usize),
    __4: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __start1 = __4.2.clone();
    let __end1 = __4.2.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action46(
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action35(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __temp1,
    )
}

fn __action69<
>(
    __0: (usize, crate::lexer::Token, usize),
    __1: (usize, String, usize),
    __2: (usize, usize, usize),
) -> Arg
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        __temp0,
        __0,
        __1,
        __2,
    )
}

fn __action70<
>(
    __0: (usize, crate::lexer::Token, usize),
    __1: (usize, String, usize),
    __2: (usize, crate::lexer::Token, usize),
    __3: (usize, usize, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action71<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        __temp0,
        __0,
        __1,
    )
}

fn __action72<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action73<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        __temp0,
        __0,
        __1,
        __2,
    )
}

fn __action74<
>(
    __0: (usize, crate::lexer::Token, usize),
    __1: (usize, i64, usize),
    __2: (usize, usize, usize),
) -> Arg
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        __temp0,
        __0,
        __1,
        __2,
    )
}

fn __action75<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, crate::lexer::Token, usize),
    __4: (usize, Arg, usize),
    __5: (usize, crate::lexer::Token, usize),
    __6: (usize, Arg, usize),
    __7: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

fn __action76<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, crate::lexer::Token, usize),
    __4: (usize, Arg, usize),
    __5: (usize, crate::lexer::Token, usize),
    __6: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

fn __action77<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, crate::lexer::Token, usize),
    __4: (usize, Arg, usize),
    __5: (usize, Arg, usize),
    __6: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

fn __action78<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, crate::lexer::Token, usize),
    __4: (usize, Arg, usize),
    __5: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

fn __action79<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
    __5: (usize, Arg, usize),
    __6: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

fn __action80<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
    __5: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

fn __action81<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, Arg, usize),
    __4: (usize, Arg, usize),
    __5: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

fn __action82<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, Arg, usize),
    __4: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action83<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, crate::lexer::Token, usize),
    __4: (usize, Arg, usize),
    __5: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

fn __action84<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, crate::lexer::Token, usize),
    __4: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action85<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action86<
>(
    __0: (usize, Op, usize),
    __1: (usize, usize, usize),
    __2: (usize, Arg, usize),
    __3: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action87<
>(
    __0: (usize, crate::lexer::Token, usize),
    __1: (usize, i64, usize),
    __2: (usize, usize, usize),
) -> Arg
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        __temp0,
        __0,
        __1,
        __2,
    )
}

fn __action88<
>(
    __0: (usize, i64, usize),
    __1: (usize, usize, usize),
) -> Arg
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action50(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        __temp0,
        __0,
        __1,
    )
}

fn __action89<
>(
    __0: (usize, crate::lexer::Token, usize),
    __1: (usize, String, usize),
) -> Arg
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        __0,
        __1,
        __temp0,
    )
}

fn __action90<
>(
    __0: (usize, crate::lexer::Token, usize),
    __1: (usize, String, usize),
    __2: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action91<
>(
    __0: (usize, Op, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        __0,
        __temp0,
    )
}

fn __action92<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        __0,
        __temp0,
        __1,
        __2,
    )
}

fn __action93<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        __0,
        __temp0,
        __1,
    )
}

fn __action94<
>(
    __0: (usize, crate::lexer::Token, usize),
    __1: (usize, i64, usize),
) -> Arg
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        __0,
        __1,
        __temp0,
    )
}

fn __action95<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, crate::lexer::Token, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
    __5: (usize, Arg, usize),
    __6: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

fn __action96<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, crate::lexer::Token, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
    __5: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

fn __action97<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, crate::lexer::Token, usize),
    __3: (usize, Arg, usize),
    __4: (usize, Arg, usize),
    __5: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

fn __action98<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, crate::lexer::Token, usize),
    __3: (usize, Arg, usize),
    __4: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action99<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, Arg, usize),
    __3: (usize, crate::lexer::Token, usize),
    __4: (usize, Arg, usize),
    __5: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
        __5,
    )
}

fn __action100<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, Arg, usize),
    __3: (usize, crate::lexer::Token, usize),
    __4: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action101<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, Arg, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action102<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, Arg, usize),
    __3: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        __0,
        __temp0,
        __1,
        __2,
        __3,
    )
}

fn __action103<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, crate::lexer::Token, usize),
    __3: (usize, Arg, usize),
    __4: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action104<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, crate::lexer::Token, usize),
    __3: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        __0,
        __temp0,
        __1,
        __2,
        __3,
    )
}

fn __action105<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, Arg, usize),
    __3: (usize, crate::lexer::Token, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        __0,
        __temp0,
        __1,
        __2,
        __3,
    )
}

fn __action106<
>(
    __0: (usize, Op, usize),
    __1: (usize, Arg, usize),
    __2: (usize, Arg, usize),
) -> Instruction
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        __0,
        __temp0,
        __1,
        __2,
    )
}

fn __action107<
>(
    __0: (usize, crate::lexer::Token, usize),
    __1: (usize, i64, usize),
) -> Arg
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        __0,
        __1,
        __temp0,
    )
}

fn __action108<
>(
    __0: (usize, i64, usize),
) -> Arg
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action88(
        __0,
        __temp0,
    )
}

fn __action109<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Instruction>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        __temp0,
    )
}

fn __action110<
>(
    __0: (usize, alloc::vec::Vec<Instruction>, usize),
) -> alloc::vec::Vec<Instruction>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        __temp0,
    )
}

pub trait __ToTriple<> {
    fn to_triple(value: Self) -> Result<(usize,crate::lexer::Token,usize), __lalrpop_util::ParseError<usize, crate::lexer::Token, crate::lexer::LexError>>;
}

impl<> __ToTriple<> for (usize, crate::lexer::Token, usize) {
    fn to_triple(value: Self) -> Result<(usize,crate::lexer::Token,usize), __lalrpop_util::ParseError<usize, crate::lexer::Token, crate::lexer::LexError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, crate::lexer::Token, usize), crate::lexer::LexError> {
    fn to_triple(value: Self) -> Result<(usize,crate::lexer::Token,usize), __lalrpop_util::ParseError<usize, crate::lexer::Token, crate::lexer::LexError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
