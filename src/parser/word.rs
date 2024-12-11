use super::{
    errors::{is_valid_label_name, PreProcessingError, Result},
    op_codes::OpCode,
    operators::Operator,
    registers::Register,
    COMMENT_CHAR,
};

/// Word Separator are used to explicit the reason of the end of a word. For exemple a register can be ended via a space or a comma
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum WordSeparator {
    /// ','
    Comma,
    /// ' '
    Space,
    /// Any allowed operator
    Operator,
    /// '\''
    SingleQuote,
    /// To end a label declaration
    Colon,
    /// '\"'
    DoubleQuote,
    /// End of the line, can be interpreted eather as end of file or '\n'
    #[default]
    EndOfLine,
    /// A simple Digit
    Digit,
    /// Every other character
    Others,
}

impl WordSeparator {
    fn get(c: char) -> Self {
        match c {
            ',' => Self::Comma,
            ' ' => Self::Space,
            '\'' => Self::SingleQuote,
            '\"' => Self::DoubleQuote,
            '\n' => Self::EndOfLine,
            _ => {
                if Operator::is_operator_snippet(c) {
                    Self::Operator
                } else {
                    Self::Others
                }
            }
        }
    }
}

fn get_backslash_char(c: char) -> Result<char> {
    Ok(match c {
        'n' => '\n',
        't' => '\t',
        'r' => '\r',
        '0' => '\0',
        '\\' => '\\',
        '\'' => '\'',
        '\"' => '\"',
        _ => return Err(PreProcessingError::InvalidBackSlash),
    })
}

/// Represent the parsed content of a word
#[derive(Default, Clone)]
pub enum WordContent {
    /// Represents an empty word
    #[default]
    Empty,
    /// Contains the label declaration
    LabelDeclaration(String),
    /// Usage of a label in the code, exemple: "mov rax, hello". Notice that if the label doesn't exists, the pre processor does not return an error as it can't assume that the label will not be ddeclared later.
    Label(String),
    /// Represent a number, eventually negative but can't handle float yet. Character notation ('x') will be interpreted as number too
    Number(i32),
    /// Will represent a valid OpCode
    OpCode(OpCode),
    /// Will represent a valid Register
    Register(Register),
    /// Can represent an operator via the operator type
    Operator(Operator),
    /// Represent a litteral string (between double quotes)
    Str(String),
}

impl WordContent {
    /// This function takes a string representing a trimmed quote espression, such as 'a' and cast it into the ascii value of the quote. Support the backslash characters such as '\n'
    fn extract_number_from_single_quote(quotes: &str) -> Result<i32> {
        let mut chars = quotes.chars();
        chars.next().unwrap();
        let first_value = match chars.next() {
            Some(c) => c,
            None => return Err(PreProcessingError::InvalidSingleQuote),
        };
        let res = if first_value == '\\' {
            match chars.next() {
                Some(second_value) => Ok(get_backslash_char(second_value)? as i32),
                None => Err(PreProcessingError::InvalidBackSlash),
            }
        } else {
            if "\'\"".contains(first_value) {
                Err(PreProcessingError::BackSlashNeeded)
            } else {
                Ok(first_value as i32)
            }
        };
        if chars.next() != Some('\'') || chars.next().is_some() {
            Err(PreProcessingError::InvalidSingleQuote)
        } else {
            res
        }
    }

    /// Will trim the pure content before processing
    fn new(kind: WordKind, pure_content: &str) -> Result<Self> {
        let pure_content = pure_content.trim();
        if pure_content.is_empty() || pure_content == "\n" {
            return Ok(Self::Empty);
        }
        Ok(match kind {
            WordKind::Unknown => {
                if let Ok(reg) = Register::try_from(pure_content) {
                    WordContent::Register(reg)
                } else if let Ok(opcode) = OpCode::try_from(pure_content) {
                    WordContent::OpCode(opcode)
                } else {
                    if is_valid_label_name(pure_content) {
                        WordContent::Label(pure_content.to_string())
                    } else {
                        return Err(PreProcessingError::InvalidWord);
                    }
                }
            }
            WordKind::LabelDeclaration => {
                if is_valid_label_name(pure_content) {
                    let label = pure_content[..pure_content.len() - 1].to_string(); // We are removing the colon
                    WordContent::LabelDeclaration(label)
                } else {
                    return Err(PreProcessingError::InvalidLabelName);
                }
            }
            WordKind::Number => WordContent::Number(match pure_content.parse::<i32>() {
                Ok(x) => x,
                Err(_) => return Err(PreProcessingError::InvalidNumber),
            }),
            WordKind::DoubleQuote => {
                WordContent::Str(pure_content[1..pure_content.len() - 1].to_string())
            }
            WordKind::SingleQuote => {
                WordContent::Number(Self::extract_number_from_single_quote(pure_content)?)
            }
        })
    }
}

/// Represent a single word of a line
#[derive(Default, Clone)]
pub struct Word {
    /// The litteral content of the word, without modification
    pure_content: String,
    /// The important content of the word, for exemple the word '0' may be represented as WordContent::Number(48)
    content: WordContent,
    /// The reason why the word creation has ended. Some kind of word will be robust regarding certain separator, for exemple a word of type Str can only have a DoubleQote separator
    sep: WordSeparator,
}

impl Word {
    pub fn new(content: WordContent, pure_content: String, sep: WordSeparator) -> Self {
        Self {
            pure_content,
            content,
            sep,
        }
    }
}

pub enum WordRequest {
    /// Push the word in the line
    PushWord(Word),
    /// Push the word in the line and end the line
    PushLine(Word),
    /// Continue to compute current word
    Continue,
}

/// Allow the word builder to know what it is computing currently, this type is quite the same that WordContent but does not make any difference between registers and op code. Moeover, it does not stock any parsed data on the current word
#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum WordKind {
    #[default]
    Unknown,
    LabelDeclaration,
    Number,
    DoubleQuote,
    /// Notice that if we are computing a SingleQuote, then it may become a number at the end of the computation
    SingleQuote,
}

impl WordKind {
    fn check_valid_ending_word(self) -> Result<()> {
        match self {
            Self::DoubleQuote => Err(PreProcessingError::DoubleQuoteNeverEnded),
            Self::SingleQuote => Err(PreProcessingError::SingleQuoteNeverEnded),
            _ => Ok(()),
        }
    }

    fn valid_separator_list(self) -> Vec<WordSeparator> {
        match self {
            WordKind::Unknown => vec![
                WordSeparator::Comma,
                WordSeparator::Space,
                WordSeparator::Operator,
                WordSeparator::SingleQuote,
                WordSeparator::DoubleQuote,
                WordSeparator::EndOfLine,
            ],
            WordKind::LabelDeclaration => vec![WordSeparator::Colon],
            WordKind::Number => vec![
                WordSeparator::Comma,
                WordSeparator::Space,
                WordSeparator::Operator,
                WordSeparator::SingleQuote,
                WordSeparator::DoubleQuote,
                WordSeparator::EndOfLine,
                WordSeparator::Others, // NOTE: A number is only allow to read digit
            ],
            WordKind::DoubleQuote => vec![WordSeparator::DoubleQuote],
            WordKind::SingleQuote => vec![WordSeparator::SingleQuote], // As quotes are a bit special, we have to assure before ending the computation that the character was not just after a backslash
        }
    }
}

/// The from char allow the parser to guess what kind of word it is going to parse
impl TryFrom<char> for WordKind {
    type Error = PreProcessingError;
    fn try_from(c: char) -> Result<Self> {
        Ok(match c {
            '\'' => Self::SingleQuote,
            '\"' => Self::DoubleQuote,
            _ if c.is_ascii_digit() => Self::Number,
            _ if c.is_alphabetic() => Self::Unknown,
            _ => return Err(PreProcessingError::InvalidFirstChar),
        })
    }
}

/// Charged to build a word, will eventually output a word
pub struct WordBuilder {
    /// The current litteral content of the word
    pure_content: String,
    /// The kind of the word, it is defines during the creation expect if the word is a letter, it will be set as unknown and may change if we spot a colon to become a label
    kind: WordKind,
}

impl WordBuilder {
    pub fn new(first_char: char) -> Result<Self> {
        Ok(Self {
            pure_content: String::from(first_char),
            kind: WordKind::try_from(first_char)?,
        })
    }

    fn init(&mut self, first_char: char) -> Result<()> {
        self.pure_content = String::from(first_char);
        self.kind = WordKind::try_from(first_char)?;
        Ok(())
    }

    pub fn end_of_file(&mut self) -> Result<Word> {
        self.kind.check_valid_ending_word()?;
        self.extract(WordSeparator::EndOfLine, '\0')
    }

    /// Return true if the trimed word starts with a quote, a single or double one
    pub fn is_quote(&self) -> bool {
        self.kind == WordKind::DoubleQuote || self.kind == WordKind::SingleQuote
    }

    /// This funtion extact the built word and clean the builder itself
    pub fn extract(&mut self, sep: WordSeparator, last_char: char) -> Result<Word> {
        let word = Word::new(
            WordContent::new(self.kind, &self.pure_content)?,
            self.pure_content.drain(..).collect::<String>(),
            sep,
        );
        self.init(last_char)?; // The last char of the computed word is fist of the next one
        Ok(word)
    }

    /// Takes in parameter a separator and end the construction by extracting the word if it is necessary
    fn get_request_from_sep(&mut self, c: char, sep: WordSeparator) -> Result<WordRequest> {
        Ok(if self.kind.valid_separator_list().contains(&sep) {
            let word = self.extract(sep, c)?;
            if sep == WordSeparator::EndOfLine {
                WordRequest::PushLine(word)
            } else {
                WordRequest::PushWord(word)
            }
        } else {
            WordRequest::Continue
        })
    }

    fn previous_was_backslash(&self) -> bool {
        let mut chars = self.pure_content.chars().rev();
        self.pure_content.len() != 0
            && chars.next().unwrap() == '\\'
            && (self.pure_content.len() == 1 || chars.next().unwrap() != '\\')
    }

    fn add_backslash_char_in_quote_context(&mut self, c: char) -> Result<()> {
        get_backslash_char(c)?;
        self.pure_content.push(c);
        Ok(())
    }

    pub fn add_char(
        &mut self,
        c: char,
        chars: &mut impl Iterator<Item = char>,
    ) -> Result<WordRequest> {
        let is_quote = self.is_quote();
        if c == COMMENT_CHAR && !is_quote {
            while let Some(c) = chars.next() {
                if c == '\n' {
                    return self.add_char(c, chars);
                }
            }
        }

        if is_quote && self.previous_was_backslash() {
            self.add_backslash_char_in_quote_context(c)?;
            return Ok(WordRequest::Continue);
        }
        self.pure_content.push(c);

        if self.kind == WordKind::Unknown && c == ':' {
            self.kind = WordKind::LabelDeclaration;
        }
        let sep = WordSeparator::get(c);
        self.get_request_from_sep(c, sep)
    }
}

pub type Line = Vec<Word>;
