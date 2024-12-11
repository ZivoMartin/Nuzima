use super::{
    errors::{PreProcessingError, Result},
    word::{Line, WordBuilder, WordRequest},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct PreProcessing {
    /// Each element of the vector can be either an instruction or a label def
    instructions: Vec<Line>,
    /// Link a label to its line
    labels: HashMap<String, usize>,
}

impl From<PreProcessingCash> for PreProcessing {
    fn from(cash: PreProcessingCash) -> Self {
        Self {
            instructions: cash.instructions,
            labels: cash.labels,
        }
    }
}

/// Used to computed the PrePocessing, contains working field
pub struct PreProcessingCash {
    /// The current computed word
    word_builder: WordBuilder,
    /// The current computed line
    current_line: Line,
    /// Each element of the vector can be either an instruction or a label def
    instructions: Vec<Line>,
    /// Link a label to its line
    labels: HashMap<String, usize>,
}

impl PreProcessingCash {
    fn new(first_char: char) -> Result<Self> {
        Ok(PreProcessingCash {
            word_builder: WordBuilder::new(first_char)?,
            current_line: Line::new(),
            instructions: Vec::new(),
            labels: HashMap::new(),
        })
    }

    fn consume(&mut self, c: char, chars: &mut impl Iterator<Item = char>) -> Result<()> {
        match self.word_builder.add_char(c, chars)? {
            WordRequest::PushLine(word) => {
                self.current_line.push(word);
                let line = self.current_line.drain(..).collect::<Vec<_>>();
                self.instructions.push(line);
            }
            WordRequest::PushWord(word) => self.current_line.push(word),
            WordRequest::Continue => (),
        }
        Ok(())
    }

    fn end_of_file(&mut self) -> Result<()> {
        self.current_line.push(self.word_builder.end_of_file()?);
        Ok(())
    }
}

pub fn pre_processing(text: &str) -> Result<PreProcessing> {
    if text.is_empty() {
        return Err(PreProcessingError::EmptyText);
    }
    let mut chars = text.chars();
    let mut pre_processor = PreProcessingCash::new(chars.next().unwrap())?; // Unwrap can't fail as the text is not empty
    while let Some(c) = chars.next() {
        pre_processor.consume(c, &mut chars)?;
    }
    pre_processor.end_of_file()?;
    Ok(PreProcessing::from(pre_processor))
}
