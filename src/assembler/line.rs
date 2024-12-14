use std::{collections::HashMap, vec::IntoIter};

use super::{
    errors::{SyntaxErrorKind, SyntaxResultKind},
    op_codes::OpCode,
    word::{Word, WordContent},
};

pub struct Line {
    instruction: Vec<Word>,
}

impl Line {
    pub fn get(&self) -> &Vec<Word> {
        &self.instruction
    }
}

macro_rules! unwrap_or_ret {
    ($word:expr, $res:ident) => {
        match $word {
            Some(w) => w,
            None => return Ok($res),
        }
    };
}

fn extract(words: Vec<Word>) -> SyntaxResultKind<Vec<Word>> {
    let mut words = words.into_iter();

    let mut res = Vec::new();

    let skip_empty_words = |words: &mut IntoIter<Word>,
                            checker: Box<dyn Fn(String, bool) -> SyntaxResultKind<()>>|
     -> SyntaxResultKind<Option<Word>> {
        let mut skiped = String::new();
        while let Some(w) = words.next() {
            if let WordContent::Empty = w.content {
                skiped.push_str(&w.pure_content)
            } else {
                checker(skiped, false)?;
                return Ok(Some(w));
            }
        }
        checker(skiped, true)?;

        Ok(None)
    };

    let nb_occur_checker =
        |allowed: &'static str, n: i64| -> Box<dyn Fn(String, bool) -> SyntaxResultKind<()>> {
            Box::new(move |skiped: String, _: bool| -> SyntaxResultKind<()> {
                if skiped.len() <= 1 {
                    return Ok(());
                }
                let mut n = n;
                if skiped
                    .chars()
                    .take(skiped.len() - 1)
                    .filter(|c| !" \n".contains(*c))
                    .any(|c| {
                        n -= 1;
                        !allowed.contains(c)
                    })
                    || n != 0
                {
                    Err(SyntaxErrorKind::SyntaxError)
                } else {
                    Ok(())
                }
            })
        };

    res.push(unwrap_or_ret!(
        skip_empty_words(&mut words, nb_occur_checker("", 0))?,
        res
    ));
    while res.last().unwrap().is_label_decl() {
        res.push(unwrap_or_ret!(
            skip_empty_words(&mut words, nb_occur_checker(":", 1))?,
            res
        ));
    }
    match res.last().unwrap().content {
        WordContent::Str(_) => {
            if skip_empty_words(&mut words, nb_occur_checker("\"", 1))?.is_some() {
                return Err(SyntaxErrorKind::SyntaxError);
            }
        }
        WordContent::OpCode(op_code) => {
            let mut rest = Vec::new();
            if let Some(w) = skip_empty_words(&mut words, nb_occur_checker("", 0))? {
                rest.push(w);
                if let Some(w) = skip_empty_words(
                    &mut words,
                    Box::new(
                        |skiped, eol| match nb_occur_checker("", 0)(skiped.clone(), eol) {
                            Ok(_) => {
                                if eol {
                                    Ok(())
                                } else {
                                    Err(SyntaxErrorKind::SyntaxError)
                                }
                            }
                            Err(_) => {
                                if eol {
                                    Err(SyntaxErrorKind::SyntaxError)
                                } else {
                                    nb_occur_checker(",", 1)(skiped, eol)
                                }
                            }
                        },
                    ),
                )? {
                    rest.push(w)
                }
            }

            op_code.check_compatibility(&rest)?;
            res.append(&mut rest);
        }
        _ => return Err(SyntaxErrorKind::SyntaxError),
    }
    Ok(res)
}

impl TryFrom<Vec<Word>> for Line {
    type Error = SyntaxErrorKind;
    fn try_from(instruction: Vec<Word>) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction: extract(instruction)?,
        })
    }
}

macro_rules! inj_reg_or_imm {
    ($w:expr, $label:ident) => {
        match &$w.content {
            WordContent::Register(reg) => Into::<u8>::into(*reg) as u32,
            WordContent::Number(x) => 1 << 25 | *x as u32,
            WordContent::Label(lab) => *$label.get(lab).unwrap() as u32,
            _ => unreachable!(),
        }
    };
}

impl Line {
    fn get_binary_instruction_op_code<'a>(
        &self,
        labels: &HashMap<String, usize>,
        op_code: OpCode,
        rest_of_line: Vec<&'a Word>,
    ) -> Vec<u8> {
        let mut instr: u32 = (Into::<u8>::into(op_code) as u32) << 27;
        instr |= match rest_of_line.len() {
            1 => inj_reg_or_imm!(rest_of_line[0], labels),
            2 => {
                Into::<u8>::into(rest_of_line[0].get_reg().unwrap()) as u32
                    | inj_reg_or_imm!(rest_of_line[1], labels)
            }
            _ => 0,
        };
        println!("{instr:b}");
        instr.to_be_bytes().to_vec()
    }

    fn get_binary_instruction_str(&self, labels: &HashMap<String, usize>, s: &String) -> Vec<u8> {
        todo!()
    }

    pub fn get_binary_instruction(&self, labels: &HashMap<String, usize>) -> Vec<u8> {
        let mut words = self.instruction.iter();
        let word = loop {
            if let Some(w) = words.next() {
                if !w.is_label_decl() {
                    break w;
                }
            } else {
                return Vec::new();
            }
        };

        if let Some(s) = word.get_str() {
            self.get_binary_instruction_str(labels, s)
        } else {
            self.get_binary_instruction_op_code(
                labels,
                word.get_op_code().unwrap(),
                words.collect(),
            )
        }
    }
}
