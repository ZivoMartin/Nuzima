use super::{
    errors::PreProcessingError,
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

#[derive(Default)]
/// Used to computed the PrePocessing, contains working field
pub struct PreProcessingCash {
    /// The current computed word
    current_word: WordBuilder,
    /// The current computed line
    current_line: Line,
    /// Each element of the vector can be either an instruction or a label def
    instructions: Vec<Line>,
    /// Link a label to its line
    labels: HashMap<String, usize>,
}

impl PreProcessingCash {
    fn consume(
        &mut self,
        c: char,
        chars: &mut impl Iterator<Item = char>,
    ) -> Result<(), PreProcessingError> {
        match self.current_word.add_char(c, chars) {
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
}

pub fn pre_processing(instruction: &str) -> Result<PreProcessing, PreProcessingError> {
    let mut pre_processor = PreProcessingCash::default();
    let mut chars = instruction.chars();
    while let Some(c) = chars.next() {
        pre_processor.consume(c, &mut chars)?;
    }
    Ok(PreProcessing::from(pre_processor))
}
