mod errors;
mod line;
mod op_codes;
mod registers;
mod word;

pub const COMMENT_CHAR: char = ';';

use errors::{cast_result, SyntaxErrorKind, SyntaxResult};
use line::Line;
use word::{Word, WordBuilder, WordContent, WordRequest};

use std::collections::HashMap;

fn display_lines(lines: &Vec<Line>) {
    println!(
        "Pure content:\n{}\n",
        lines
            .iter()
            .map(|l| l
                .get()
                .iter()
                .map(|w| w.pure_content[..w.pure_content.len() - 1].to_string())
                .collect::<String>())
            .collect::<String>()
    );
    println!(
        "Words parsing:\n{}",
        lines
            .iter()
            .map(|l| l
                .get()
                .iter()
                .map(|w| format!("{w:?}\n"))
                .collect::<String>())
            .collect::<String>()
    )
}

/// Used to store differents word, used after to generate the biinary
pub struct Assembler {
    /// The current computed word
    word_builder: WordBuilder,
    /// The current computed line
    current_line: Vec<Word>,
    /// Each element of the vector is an instruction composed of different word
    instructions: Vec<Line>,
    /// Link a label to its line, also used to verify the existence of labels when parsing the code
    labels: HashMap<String, usize>,
}

impl Assembler {
    fn new() -> SyntaxResult<Self> {
        Ok(Self {
            word_builder: cast_result(WordBuilder::new(), 0)?,
            current_line: Vec::new(),
            instructions: Vec::new(),
            labels: HashMap::new(),
        })
    }

    fn push_word(&mut self, word: Word) -> SyntaxResult<()> {
        if let WordContent::LabelDeclaration(lab) = &word.content {
            if self
                .labels
                .insert(lab.to_string(), self.instructions.len())
                .is_some()
            {
                return cast_result(
                    Err(SyntaxErrorKind::LabelDeclaredTwice(lab.to_string())),
                    self.line(),
                );
            }
        }
        self.current_line.push(word);

        Ok(())
    }

    fn push_current_line(&mut self) -> SyntaxResult<()> {
        let line = self.current_line.drain(..).collect::<Vec<_>>();
        self.instructions
            .push(cast_result(Line::try_from(line), self.line())?);
        Ok(())
    }

    fn line(&self) -> usize {
        self.instructions.len() + 1
    }

    fn consume(&mut self, c: char, chars: &mut impl Iterator<Item = char>) -> SyntaxResult<()> {
        match cast_result(self.word_builder.add_char(c, chars), self.line())? {
            WordRequest::PushLine(word) => {
                self.push_word(word)?;
                self.push_current_line()?
            }
            WordRequest::PushWord(word) => self.push_word(word)?,
            WordRequest::Continue => (),
        }
        Ok(())
    }

    fn conclude(&mut self) -> SyntaxResult<()> {
        self.current_line
            .push(cast_result(self.word_builder.end_of_file(), self.line())?);
        self.push_current_line()?;
        for (i, line) in self.instructions.iter().enumerate() {
            if let Some(lab) = line.get().iter().find(|word| {
                if let WordContent::Label(lab) = &word.content {
                    !self.labels.contains_key(lab)
                } else {
                    false
                }
            }) {
                match &lab.content {
                    WordContent::Label(lab) => {
                        return cast_result(
                            Err(SyntaxErrorKind::LabelIsNotDeclared(lab.to_string())),
                            i + 1,
                        )
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }
}

pub fn assemble(text: &str) -> SyntaxResult<()> {
    if text.is_empty() {
        return cast_result(Err(SyntaxErrorKind::EmptyText), 0);
    }
    let mut chars = text.chars();
    let mut assembler = Assembler::new()?;
    while let Some(c) = chars.next() {
        assembler.consume(c, &mut chars)?;
    }

    assembler.conclude()?;
    display_lines(&assembler.instructions);
    Ok(())
}
