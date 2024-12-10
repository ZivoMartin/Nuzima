pub enum PreProcessingError {
    InvalidSingleQuote,
    InvalidBackSlash,
    BackSlashNeeded,
    DoubleQuoteNeverEnded,
    EndOfLine,
    InvalidRegister,
    InvalidOpCode,
}
