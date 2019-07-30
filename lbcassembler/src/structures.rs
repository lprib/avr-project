#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct OpCode<'a> {
    pub code: u8,
    pub name: &'a str,
    pub expected_args: usize,
}

#[derive(Debug, PartialEq)]
pub enum Element<'a> {
    Label(&'a str),
    OpCode(&'a OpCode<'a>, Vec<Argument<'a>>),
    RawData(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub enum Argument<'a> {
    Value(u16),
    LabelAddress(&'a str),
}

pub struct ByteCode {
    pub data: Vec<u8>,
}