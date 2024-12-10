pub enum PreProcessingError {
    InvalidSingleQuote,
    InvalidBackSlash,
    BackSlashNeeded,
    DoubleQuoteNeverEnded,
    EndOfLine,
    InvalidRegister,
    InvalidOpCode,
    InvalidNumber,
}

pub fn is_valid_label_name(name: &str) -> bool {
    !name.is_empty()
}
