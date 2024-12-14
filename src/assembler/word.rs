use super::{
    errors::{is_valid_label_name, SyntaxErrorKind, SyntaxResultKind},
    op_codes::OpCode,
    registers::Register,
    COMMENT_CHAR,
};

/// Word Separator are used to explicit the reason of the end of a word. For exemple a register can be ended via a space or a comma
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum WordSeparator {
    /// ','
    Comma,
    /// ' '
    Space,
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
            ':' => Self::Colon,
            _ if c.is_ascii_digit() => Self::Digit,
            _ => Self::Others,
        }
    }
}

fn get_backslash_char(c: char) -> SyntaxResultKind<char> {
    Ok(match c {
        'n' => '\n',
        't' => '\t',
        'r' => '\r',
        '0' => '\0',
        '\\' => '\\',
        '\'' => '\'',
        '\"' => '\"',
        _ => return Err(SyntaxErrorKind::InvalidBackSlash(c)),
    })
}

/// Represent the parsed content of a word
#[derive(Default, Clone, Debug)]
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
    /// Represent a litteral string (between double quotes)
    Str(String),
}

/// If s is a quote, it will replace the backslash character by its real value. It can fail if there is an invalid backslash character, but as this case is checks before, it may not.
fn trim_sep(kind: WordKind, s: &str) -> SyntaxResultKind<String> {
    Ok(if kind.is_quote() {
        let mut res = String::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            res.push(if c == '\\' {
                match chars.next() {
                    Some(c) => get_backslash_char(c)?,
                    None => return Err(SyntaxErrorKind::InvalidBackSlash('\0')),
                }
            } else {
                c
            });
        }
        res
    } else {
        if s.len() < 3 { "" } else { &s[1..s.len() - 1] }.to_string()
    })
}

impl WordContent {
    /// This function takes a string representing a trimmed quote espression, such as 'a' and cast it into the ascii value of the quote. This function assume that the given quote is valid and throw an error if not. A valid quote here is a string of three elements composed of a first quote, then a value and a quote.
    fn extract_number_from_single_quote(quotes: &str) -> SyntaxResultKind<i32> {
        let mut chars = quotes.chars();
        chars.next().unwrap();
        let value = match chars.next() {
            Some(c) => Ok(c as i32),
            None => return Err(SyntaxErrorKind::InvalidSingleQuote(quotes.to_string())),
        };

        if chars.next() != Some('\'') || chars.next().is_some() {
            Err(SyntaxErrorKind::InvalidSingleQuote(quotes.to_string()))
        } else {
            value
        }
    }

    /// Will trim the pure content before processing
    fn new(kind: WordKind, pure_content: &str) -> SyntaxResultKind<Self> {
        let pure_content = trim_sep(kind, pure_content)?;
        if pure_content.is_empty() {
            return Ok(Self::Empty);
        }
        Ok(match kind {
            WordKind::Unknown => {
                if let Ok(reg) = Register::try_from(&pure_content as &str) {
                    WordContent::Register(reg)
                } else if let Ok(opcode) = OpCode::try_from(&pure_content as &str) {
                    WordContent::OpCode(opcode)
                } else {
                    if is_valid_label_name(&pure_content) {
                        WordContent::Label(pure_content.to_string())
                    } else {
                        return Err(SyntaxErrorKind::InvalidWord(pure_content));
                    }
                }
            }
            WordKind::LabelDeclaration => {
                if is_valid_label_name(&pure_content) {
                    let label = pure_content.to_string();
                    WordContent::LabelDeclaration(label)
                } else {
                    return Err(SyntaxErrorKind::InvalidLabelName(pure_content));
                }
            }
            WordKind::Number => WordContent::Number(match pure_content.parse::<i32>() {
                Ok(x) => x,
                Err(_) => return Err(SyntaxErrorKind::InvalidNumber(pure_content)),
            }),
            WordKind::DoubleQuote => {
                let mut content = pure_content[1..pure_content.len() - 1].to_string();
                content.push('\0');
                WordContent::Str(content)
            }
            WordKind::SingleQuote => {
                WordContent::Number(Self::extract_number_from_single_quote(&pure_content)?)
            }
        })
    }
}

/// Represent a single word of a line
#[derive(Default, Clone)]
pub struct Word {
    /// The litteral content of the word, without modification
    pub pure_content: String,
    /// The important content of the word, for exemple the word '0' may be represented as WordContent::Number(48)
    pub content: WordContent,
    /// The reason why the word creation has ended. Some kind of word will be robust regarding certain separator, for exemple a word of type Str can only have a DoubleQote separator
    sep: WordSeparator,
}

use std::fmt::{Debug, Error as FmtErr, Formatter};

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), FmtErr> {
        write!(
            f,
            "Word content: {:?}, word sep: {:?}",
            self.content, self.sep
        )
    }
}

impl Word {
    pub fn new(content: WordContent, pure_content: String, sep: WordSeparator) -> Self {
        Self {
            pure_content,
            content,
            sep,
        }
    }

    pub fn get_op_code(&self) -> Option<OpCode> {
        match self.content {
            WordContent::OpCode(opcode) => Some(opcode),
            _ => None,
        }
    }

    pub fn get_str<'a>(&'a self) -> Option<&'a String> {
        match &self.content {
            WordContent::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_reg(&self) -> Option<Register> {
        match &self.content {
            WordContent::Register(r) => Some(*r),
            _ => None,
        }
    }

    pub fn is_label_decl(&self) -> bool {
        match self.content {
            WordContent::LabelDeclaration(_) => true,
            _ => false,
        }
    }

    pub fn is_reg(&self) -> bool {
        match self.content {
            WordContent::Register(_) => true,
            _ => false,
        }
    }

    pub fn is_reg_or_imm(&self) -> bool {
        match self.content {
            WordContent::Number(_) | WordContent::Label(_) | WordContent::Register(_) => true,
            _ => false,
        }
    }

    pub fn is_str(&self) -> bool {
        match self.content {
            WordContent::Str(_) => true,
            _ => false,
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
    /// Return true if the word is a between quotes
    pub fn is_quote(self) -> bool {
        self == WordKind::DoubleQuote || self == WordKind::SingleQuote
    }

    fn check_valid_ending_word(self) -> SyntaxResultKind<()> {
        match self {
            Self::DoubleQuote => Err(SyntaxErrorKind::DoubleQuoteNeverEnded),
            Self::SingleQuote => Err(SyntaxErrorKind::SingleQuoteNeverEnded),
            _ => Ok(()),
        }
    }

    fn valid_separator_list(self) -> Vec<WordSeparator> {
        match self {
            WordKind::Unknown => vec![
                WordSeparator::Comma,
                WordSeparator::Space,
                WordSeparator::SingleQuote,
                WordSeparator::DoubleQuote,
                WordSeparator::EndOfLine,
            ],
            WordKind::LabelDeclaration => vec![WordSeparator::Colon],
            WordKind::Number => vec![
                WordSeparator::Comma,
                WordSeparator::Space,
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
    type Error = SyntaxErrorKind;
    fn try_from(c: char) -> SyntaxResultKind<Self> {
        Ok(match c {
            '\'' => Self::SingleQuote,
            '\"' => Self::DoubleQuote,
            _ if c.is_ascii_digit() => Self::Number,
            _ if c.is_alphabetic() || ", :\n\0".contains(c) => Self::Unknown,
            _ => return Err(SyntaxErrorKind::InvalidFirstChar(c)),
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
    pub fn new() -> SyntaxResultKind<Self> {
        Ok(Self {
            pure_content: String::from('\0'),
            kind: WordKind::try_from('\0')?,
        })
    }

    fn init(&mut self, first_char: char) -> SyntaxResultKind<()> {
        self.pure_content = String::from(first_char);
        if self.kind.is_quote() {
            self.kind = WordKind::Unknown
        } else {
            self.kind = WordKind::try_from(first_char)?
        }
        Ok(())
    }

    pub fn end_of_file(&mut self) -> SyntaxResultKind<Word> {
        self.kind.check_valid_ending_word()?;
        self.extract(WordSeparator::EndOfLine, '\0')
    }

    /// This funtion extact the built word and clean the builder itself
    pub fn extract(&mut self, sep: WordSeparator, last_char: char) -> SyntaxResultKind<Word> {
        let word = Word::new(
            WordContent::new(self.kind, &self.pure_content)?,
            self.pure_content.drain(..).collect::<String>(),
            sep,
        );
        self.init(last_char)?; // The last char of the computed word is fist of the next one
        Ok(word)
    }

    /// Takes in parameter a separator and end the construction by extracting the word if it is necessary
    fn get_request_from_sep(
        &mut self,
        c: char,
        sep: WordSeparator,
    ) -> SyntaxResultKind<WordRequest> {
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

    fn add_backslash_char_in_quote_context(&mut self, c: char) -> SyntaxResultKind<()> {
        get_backslash_char(c)?;
        self.pure_content.push(c);
        Ok(())
    }

    fn push_char(&mut self, c: char) {
        self.pure_content.push(c);
        if c == ':' && self.kind == WordKind::Unknown {
            self.kind = WordKind::LabelDeclaration;
        }
        if self.kind == WordKind::Unknown && self.pure_content.len() == 2 && c.is_ascii_digit() {
            self.kind = WordKind::Number;
        }
    }

    fn handle_comments(
        &mut self,
        chars: &mut impl Iterator<Item = char>,
    ) -> SyntaxResultKind<WordRequest> {
        while let Some(c) = chars.next() {
            if c == '\n' {
                return self.add_char(c, chars);
            }
        }
        Ok(WordRequest::Continue) // This append only if we reach EOF
    }

    pub fn add_char(
        &mut self,
        c: char,
        chars: &mut impl Iterator<Item = char>,
    ) -> SyntaxResultKind<WordRequest> {
        let is_quote = self.kind.is_quote();
        if c == COMMENT_CHAR && !is_quote {
            return self.handle_comments(chars);
        }

        if is_quote && self.previous_was_backslash() {
            self.add_backslash_char_in_quote_context(c)?;
            return Ok(WordRequest::Continue);
        }

        self.push_char(c);
        let sep = WordSeparator::get(c);
        self.get_request_from_sep(c, sep)
    }
}
