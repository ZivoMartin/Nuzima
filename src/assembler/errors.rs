use std::fmt::{Debug, Display, Error as FmtErr, Formatter};

use super::op_codes::OpCode;

pub enum SyntaxErrorKind {
    SyntaxError,
    EmptyText,
    InvalidSingleQuote(String),
    InvalidBackSlash(char),
    BackSlashNeeded(char),
    DoubleQuoteNeverEnded,
    SingleQuoteNeverEnded,
    EndOfLine,
    InvalidRegister(String),
    InvalidOpCode(String),
    InvalidNumber(String),
    InvalidWord(String),
    InvalidLabelName(String),
    InvalidFirstChar(char),
    LabelDeclaredTwice(String),
    LabelIsNotDeclared(String),
    ExpectedRegImmOrReg(OpCode),
    ExpectedReg(OpCode),
    ExpectedRegOrImm(OpCode),
    ExpectedNothing(OpCode),
}

impl Debug for SyntaxErrorKind {
    fn fmt(&self, f: &mut Formatter) -> core::result::Result<(), FmtErr> {
        write!(
            f,
            "{}",
            match self {
                Self::SyntaxError => String::from("Syntax error"),
                Self::EmptyText => String::from("The file is empty."),
                Self::InvalidSingleQuote(pure_content) =>
                    format!("The quote is invalid. \"{pure_content}\" has been found."),
                Self::InvalidBackSlash(c) => format!("The char '{c}' has been found after a backslash, this char is not allowed as a backslash char."),
                Self::BackSlashNeeded(c) => format!("The char '{c}' has to be place right after a backslash to be well parsed."),
                Self::DoubleQuoteNeverEnded | Self::SingleQuoteNeverEnded => String::from("Quotes never ended."),
                Self::EndOfLine => String::from("Invalid word at the end of the line."),
                Self::InvalidRegister(reg) => format!("The reg \"{reg}\" does not exist."),
                Self::InvalidOpCode(op) => format!("The op code \"{op}\" does not exist."),
                Self::InvalidNumber(word) => format!("Expected a digit, found {word}."),
                Self::InvalidWord(word) => format!("Invalid token: \"{word}\"."),
                Self::InvalidLabelName(lab) => format!("This name is invalid for a label: \"{lab}\"."),
                Self::InvalidFirstChar(c) => format!("This char can't start a word: '{c}'."),
                Self::LabelDeclaredTwice(lab) => format!("The label \"{lab}\" as been declared twice."),
                Self::LabelIsNotDeclared(lab) => format!("This label has not been declared: \"{lab}\"."),
                Self::ExpectedRegImmOrReg(op) => format!("With the op code {op:?}, a register followed with an other register or an immediate value was expected."),
                Self::ExpectedReg(op) => format!("With the op code {op:?}, a single register was expected."),
                Self::ExpectedRegOrImm(op) => format!("With the op code {op:?}, a register or an immediate value was expected."),
                Self::ExpectedNothing(op) => format!("We expected nothing after the op code {op:?}."),
            }
        )
    }
}

impl Display for SyntaxErrorKind {
    fn fmt(&self, f: &mut Formatter) -> core::result::Result<(), FmtErr> {
        write!(f, "{self:?}")
    }
}

pub struct SyntaxError {
    line: usize,
    err: SyntaxErrorKind,
}

impl From<&SyntaxError> for i32 {
    fn from(err: &SyntaxError) -> i32 {
        From::from(&err.err)
    }
}

impl Debug for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> core::result::Result<(), FmtErr> {
        write!(f, "Error line {}, {:?}", self.line, self.err)
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> core::result::Result<(), FmtErr> {
        write!(f, "{self:?}")
    }
}

impl From<&SyntaxErrorKind> for i32 {
    fn from(err: &SyntaxErrorKind) -> i32 {
        match err {
            SyntaxErrorKind::EmptyText => 1,
            SyntaxErrorKind::InvalidSingleQuote(_) => 2,
            SyntaxErrorKind::InvalidBackSlash(_) => 3,
            SyntaxErrorKind::BackSlashNeeded(_) => 4,
            SyntaxErrorKind::DoubleQuoteNeverEnded => 5,
            SyntaxErrorKind::SingleQuoteNeverEnded => 6,
            SyntaxErrorKind::EndOfLine => 7,
            SyntaxErrorKind::InvalidRegister(_) => 8,
            SyntaxErrorKind::InvalidOpCode(_) => 9,
            SyntaxErrorKind::InvalidNumber(_) => 10,
            SyntaxErrorKind::InvalidWord(_) => 11,
            SyntaxErrorKind::InvalidLabelName(_) => 12,
            SyntaxErrorKind::InvalidFirstChar(_) => 13,
            SyntaxErrorKind::LabelDeclaredTwice(_) => 14,
            SyntaxErrorKind::LabelIsNotDeclared(_) => 15,
            SyntaxErrorKind::SyntaxError => 16,
            SyntaxErrorKind::ExpectedRegImmOrReg(_) => 17,
            SyntaxErrorKind::ExpectedReg(_) => 18,
            SyntaxErrorKind::ExpectedRegOrImm(_) => 19,
            SyntaxErrorKind::ExpectedNothing(_) => 20,
        }
    }
}

pub type SyntaxResultKind<T> = core::result::Result<T, SyntaxErrorKind>;
pub type SyntaxResult<T> = core::result::Result<T, SyntaxError>;

pub fn cast_result<T>(res: SyntaxResultKind<T>, line: usize) -> SyntaxResult<T> {
    match res {
        SyntaxResultKind::Ok(r) => SyntaxResult::Ok(r),
        SyntaxResultKind::Err(err) => SyntaxResult::Err(SyntaxError { line, err }),
    }
}

/// Returns true if the given name can be a label name. For now, only empty name are forbiddent.
pub fn is_valid_label_name(name: &str) -> bool {
    !name.is_empty()
}
