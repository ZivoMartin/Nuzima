use std::collections::HashMap;

pub enum Operator {
    Add,
    Sub,
}

pub enum PreProcessingError {
    InvalidSingleQuote,
    InvalidBackSlash,
    BackSlashNeeded,
    DoubleQuoteNeverEnded,
    EndOfLine,
}

pub enum Word {
    Label(String),
    Number(i32),
    Keyword(String),
    Operator(Operator),
    Str(String),
}
impl Word {
    /// Try to read a word from the iterator. Perhaps, the function will have to read multiple word for exemple, if the iterator is ["1+2", ..], then the function should return Word::Number(1), Word::Operator(Add), Word::Number(2)]. If the line does not contain word anymore the function returns an empty vec. The function can also return an error in different scenario.
    fn next<'a>(
        instruction_iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Vec<Self>, PreProcessingError> {
        let word = match instruction_iter.next() {
            Some(word) => word,
            None => return Ok(Vec::new()),
        };
        Self::from_string(word, instruction_iter)
    }

    fn from_string<'a>(
        word: &str,
        instruction_iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Vec<Self>, PreProcessingError> {
        if word.starts_with("\'") {
            Word::from_single_quotes(word, instruction_iter)
        } else if word.starts_with("\"") {
            Word::from_double_quotes(word, instruction_iter)?
        } else {
            todo!()
        }
    }

    fn from_single_quotes<'a>(
        quotes: &str,
        instruction_iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Vec<Self>, PreProcessingError> {
        if quotes == "'\'" {
            if instruction_iter.next() != Some("\'") {
                return Err(PreProcessingError::InvalidSingleQuote);
            }
            return Ok(Word::Number(' ' as i32));
        }
        let mut chars = quotes.chars();
        if chars.next() != Some('\'') {
            return Err(PreProcessingError::InvalidSingleQuote);
        }
        let first_value = match chars.next() {
            Some(c) => c,
            None => return Err(PreProcessingError::InvalidSingleQuote),
        };
        if "\'\"".contains(first_value) {
            return Err(PreProcessingError::BackSlashNeeded);
        }
        let value = if first_value == '\\' {
            match chars.next() {
                Some(second_value) => match second_value {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '0' => '\0',
                    '\\' => '\\',
                    '\'' => '\'',
                    '\"' => '\"',
                    _ => return Err(PreProcessingError::InvalidBackSlash),
                },
                None => return Err(PreProcessingError::InvalidBackSlash),
            }
        } else {
            first_value
        } as i32;

        if chars.next() != Some('\'') {
            return Err(PreProcessingError::InvalidSingleQuote);
        }
        res = Vec::new();
        Ok(Self::Number(value))
    }

    fn from_double_quotes<'a>(
        quotes: &str,
        instruction_iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Vec<Self>, PreProcessingError> {
        let mut res = String::new();
        while let Some(word) = instruction_iter.next() {}
        return Err(PreProcessingError::DoubleQuoteNeverEnded);
    }
}

pub type Line = Vec<Word>;

#[derive(Default)]
pub struct PreProcessing {
    /// Each element of the vector can be either an instruction or a label def
    instructions: Vec<Line>,
    /// Link a label to its line
    labels: HashMap<String, usize>,
}

impl PreProcessing {
    pub fn get_word() {}

    fn add_line(&mut self, line: &str) -> Result<(), PreProcessingError> {
        let mut instruction_iter = line.split_whitespace().filter(|w| !w.is_empty());
        let mut words = Vec::new();
        loop {
            match Word::next(&mut instruction_iter)? {
                Some(word) => words.push(word),
                None => break,
            }
        }
        Ok(())
    }
}

pub fn pre_processing(instruction: &str) -> Result<PreProcessing, PreProcessingError> {
    let mut res = PreProcessing::default();
    for line in instruction.lines() {
        res.add_line(line)?;
    }
    Ok(res)
}
