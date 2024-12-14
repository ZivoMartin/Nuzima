use crate::as_number;

use super::{
    errors::{SyntaxErrorKind, SyntaxResultKind},
    word::Word,
};

as_number!(
    u8,
    enum OpCode {
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
        JMP,
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
    },
    derive(Clone, Copy, Debug, Eq, PartialEq)
);

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
            "jmp" => OpCode::JMP,
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

            Self::INT | Self::JMP | Self::CALL | Self::PUSH
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
