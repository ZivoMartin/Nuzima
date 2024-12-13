use std::vec::IntoIter;

use super::{
    errors::{SyntaxErrorKind, SyntaxResultKind},
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
