use crate::as_number;

as_number!(
    i32,
    enum SyntaxError {
        EmptyText,
        InvalidSingleQuote,
        InvalidBackSlash,
        BackSlashNeeded,
        DoubleQuoteNeverEnded,
        SingleQuoteNeverEnded,
        EndOfLine,
        InvalidRegister,
        InvalidOpCode,
        InvalidNumber,
        InvalidWord,
        InvalidLabelName,
        InvalidFirstChar,
        LabelDeclaredTwice,
        LabelIsNotDeclared,
    },
    derive(Debug)
);
pub type Result<T> = core::result::Result<T, SyntaxError>;

/// Returns true if the given name can be a label name. For now, only empty name are forbiddent.
pub fn is_valid_label_name(name: &str) -> bool {
    !name.is_empty()
}
