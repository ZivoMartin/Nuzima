pub enum Operator {
    Add,
    Sub,
}

impl Operator {
    pub fn is_operator_snippet(c: char) -> bool {
        "-+".contains(&c)
    }
}
