use crate::as_number;

use super::errors::PreProcessingError;

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
    }
);

impl TryFrom<&str> for OpCode {
    type Error = PreProcessingError;

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
            _ => return Err(PreProcessingError::InvalidOpCode),
        })
    }
}
