use crate::as_number;

use super::{
    errors::{SyntaxErrorKind, SyntaxResultKind},
    word::Word,
};

as_number!(
    u8,
    enum JCODE {
        JMP,
        JE,
        JH,
        JL,
        JLE,
        JHE,
    },
    derive(Clone, Copy, Debug, Eq, PartialEq)
);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpCode {
    ADD,
    MUL,
    SUB,
    DIV,
    MOD,
    NEG,
    SHL,
    SHR,
    AND,
    OR,
    XOR,
    NOT,
    JMP(JCODE),
    INT,
    PUSH,
    POP,
    RET,
    CALL,
    MOV,
    READ,
    WRITE,
    CMP,
    CLEAR,
    HALT,
    TRACE,
    DUP,
    SWAP,
}

impl TryFrom<&str> for OpCode {
    type Error = SyntaxErrorKind;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "add" => OpCode::ADD,
            "mul" => OpCode::MUL,
            "sub" => OpCode::SUB,
            "div" => OpCode::DIV,
            "mod" => OpCode::MOD,
            "neg" => OpCode::NEG,
            "shl" => OpCode::SHL,
            "shr" => OpCode::SHR,
            "and" => OpCode::AND,
            "or" => OpCode::OR,
            "xor" => OpCode::XOR,
            "not" => OpCode::NOT,
            "jmp" => OpCode::JMP(JCODE::JMP),
            "je" => OpCode::JMP(JCODE::JE),
            "jl" => OpCode::JMP(JCODE::JL),
            "jh" => OpCode::JMP(JCODE::JH),
            "jhe" => OpCode::JMP(JCODE::JHE),
            "jle" => OpCode::JMP(JCODE::JLE),
            "int" => OpCode::INT,
            "push" => OpCode::PUSH,
            "pop" => OpCode::POP,
            "ret" => OpCode::RET,
            "call" => OpCode::CALL,
            "mov" => OpCode::MOV,
            "rd" => OpCode::READ,
            "wr" => OpCode::WRITE,
            "cmp" => OpCode::CMP,
            "clear" => OpCode::CLEAR,
            "halt" => OpCode::HALT,
            "trace" => OpCode::TRACE,
            "dup" => OpCode::DUP,
            "swap" => OpCode::SWAP,
            _ => return Err(SyntaxErrorKind::InvalidOpCode(s.to_string())),
        })
    }
}

impl OpCode {
    pub fn check_compatibility(self, line: &[Word]) -> SyntaxResultKind<()> {
        match self {
            Self::MOD
            | Self::ADD
            | Self::MUL
            | Self::SUB
            | Self::DIV
            | Self::SHL
            | Self::SHR
            | Self::AND
            | Self::OR
            | Self::XOR
            | Self::NOT
            | Self::CMP
            | Self::READ
            | Self::WRITE
            | Self::MOV
                if line.len() != 2 || !line[0].is_reg() || !line[1].is_reg_or_imm() =>
            {
                Err(SyntaxErrorKind::ExpectedRegImmOrReg(self))
            }

            Self::HALT
            | Self::SWAP
            | Self::DUP
            | Self::CLEAR
            | Self::TRACE
            | Self::NEG
            | Self::RET
                if line.len() != 0 =>
            {
                Err(SyntaxErrorKind::ExpectedNothing(self))
            } // Nothing

            Self::INT | Self::JMP(_) | Self::CALL | Self::PUSH
                if line.len() != 1 || !line[0].is_reg_or_imm() =>
            {
                Err(SyntaxErrorKind::ExpectedRegOrImm(self))
            } // reg || imm

            Self::POP if line.len() != 1 || !line[0].is_reg() => {
                Err(SyntaxErrorKind::ExpectedReg(self))
            } // reg
            _ => Ok(()),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> u8 {
        match op {
            OpCode::ADD => 0,
            OpCode::MUL => 1,
            OpCode::SUB => 2,
            OpCode::DIV => 3,
            OpCode::MOD => 4,
            OpCode::NEG => 5,
            OpCode::SHL => 6,
            OpCode::SHR => 7,
            OpCode::AND => 8,
            OpCode::OR => 9,
            OpCode::XOR => 10,
            OpCode::NOT => 11,
            OpCode::JMP(_) => 12,
            OpCode::INT => 13,
            OpCode::PUSH => 14,
            OpCode::POP => 15,
            OpCode::RET => 16,
            OpCode::CALL => 17,
            OpCode::MOV => 18,
            OpCode::READ => 19,
            OpCode::WRITE => 20,
            OpCode::CMP => 21,
            OpCode::CLEAR => 22,
            OpCode::HALT => 23,
            OpCode::TRACE => 24,
            OpCode::DUP => 25,
            OpCode::SWAP => 26,
        }
    }
}
