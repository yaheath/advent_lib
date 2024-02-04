use std::fmt;

#[derive(Clone)]
pub enum Token<I: fmt::Display + fmt::LowerHex + fmt::UpperHex + TryFrom<usize> + Copy + Clone> {
    Opcode(String),
    Register(String),
    Integer(I),
    Address(I),
}

pub struct Formatter<
    I: fmt::Display + fmt::LowerHex + fmt::UpperHex + TryFrom<usize> + Copy + Clone,
> {
    tokens: Vec<Token<I>>,
}

impl<I: fmt::Display + fmt::LowerHex + fmt::UpperHex + TryFrom<usize> + Copy + Clone> Formatter<I> {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }
    pub fn add_opcode(&mut self, val: String) {
        self.tokens.push(Token::Opcode(val));
    }
    pub fn add_register(&mut self, val: String) {
        self.tokens.push(Token::Register(val));
    }
    pub fn add_integer(&mut self, val: I) {
        self.tokens.push(Token::Integer(val));
    }
    pub fn add_address(&mut self, val: I) {
        self.tokens.push(Token::Address(val));
    }
    pub fn get_tokens(&self) -> &Vec<Token<I>> {
        &self.tokens
    }
}

impl<I: fmt::Display + fmt::LowerHex + fmt::UpperHex + TryFrom<usize> + Copy + Clone> Default
    for Formatter<I>
{
    fn default() -> Self {
        Self::new()
    }
}

pub trait InstructionDisplay<
    I: fmt::Display + fmt::LowerHex + fmt::UpperHex + TryFrom<usize> + Copy + Clone,
>
{
    fn fmt(&self, fmt: &mut Formatter<I>);
}
