use crate::as_number;

use super::errors::SyntaxError;

as_number!(
    u8,
    enum Register {
        R0,
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
        PC,
        COND,
    },
    derive(Clone, Copy, Debug)
);

impl TryFrom<&str> for Register {
    type Error = SyntaxError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "r0" => Register::R0,
            "r1" => Register::R1,
            "r2" => Register::R2,
            "r3" => Register::R3,
            "r4" => Register::R4,
            "r5" => Register::R5,
            "r6" => Register::R6,
            "r7" => Register::R7,
            "rpc" => Register::PC,
            "rcond" => Register::COND,
            _ => return Err(SyntaxError::InvalidRegister),
        })
    }
}
