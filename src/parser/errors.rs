pub enum PreProcessingError {
    EmptyText,
    InvalidSingleQuote,
    InvalidBackSlash,
    BackSlashNeeded,
    DoubleQuoteNeverEnded,
    EndOfLine,
    InvalidRegister,
    InvalidOpCode,
    InvalidNumber,
    InvalidWord,
    InvalidLabelName,
    InvalidFirstChar,
}

pub type Result<T> = core::result::Result<T, PreProcessingError>;

/// Returns true if the given name can be a label name. For now, only empty name are forbiddent.
pub fn is_valid_label_name(name: &str) -> bool {
    !name.is_empty()
}
