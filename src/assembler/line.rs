use super::{errors::SyntaxError, word::Word};

pub struct Line {
    instruction: Vec<Word>,
}

impl Line {
    pub fn get(&self) -> &Vec<Word> {
        &self.instruction
    }
}

impl TryFrom<Vec<Word>> for Line {
    type Error = SyntaxError;
    fn try_from(instruction: Vec<Word>) -> Result<Self, Self::Error> {
        Ok(Self { instruction })
    }
}
