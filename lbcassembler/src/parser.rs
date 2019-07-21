use super::opcodes_parser::{load_opcode_list, OpCode};
use lazy_static::lazy_static;
use nom::*;

use core::result::Result::Ok;
use std::collections::HashMap;
use std::num::ParseIntError;

lazy_static! {
    static ref OPCODES_MAP: HashMap<&'static str, OpCode<'static>> = {
        load_opcode_list("/home/liam/programming/avr-project/lbcassembler/opcodes.txt")
            .expect("Error loading opcode list...")
    };
}

#[derive(Debug)]
enum Element<'a> {
    Label(&'a str),
    OpCode(&'a OpCode<'a>, Vec<Argument<'a>>),
}

#[derive(Debug, PartialEq)]
enum Argument<'a> {
    Value(u16),
    LabelAddress(&'a str),
}

// creates a new vec, so needs to return String not &str
fn label_name(i: &str) -> IResult<&str, &str> {
    //TODO: include underscore as well
    character::complete::alphanumeric1(i)
}

fn decimal_literal(i: &str) -> IResult<&str, u16> {
    combinator::map_res(character::complete::digit1, |strn: &str| strn.parse())(i)
}

fn str_to_opcode(i: &str) -> Result<&OpCode, ()> {
    OPCODES_MAP.get(i).ok_or(())
}

fn opcode_name(i: &str) -> IResult<&str, &OpCode> {
    combinator::map_res(character::complete::alpha0, str_to_opcode)(i)
}

fn argument_decimal_literal(i: &str) -> IResult<&str, Argument> {
    combinator::map_res(decimal_literal, |num| -> Result<Argument, ()> {
        Ok(Argument::Value(num))
    })(i)
}

fn argument_label_address(i: &str) -> IResult<&str, Argument> {
    combinator::map_res(
        sequence::delimited(
            bytes::complete::tag("["),
            label_name,
            bytes::complete::tag("]"),
        ),
        |string| -> Result<Argument, ()> { Ok(Argument::LabelAddress(string)) },
    )(i)
}

fn space_and_argument(i: &str) -> IResult<&str, Argument> {
    let (i, _) = character::complete::space1(i)?;
    branch::alt((argument_label_address, argument_decimal_literal))(i)
}

fn opcode_element(i: &str) -> IResult<&str, Element> {
    let (i, opcode) = opcode_name(i)?;
    let (i, _) = character::complete::space1(i)?;
    let (i, args) = multi::many_m_n(opcode.expected_args, opcode.expected_args, space_and_argument)(i)?;

    Ok((i, Element::OpCode(opcode, args)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_name() {
        let (remaining, result) = label_name("abcDEF123abc more").unwrap();
        assert_eq!(remaining, " more");
        assert_eq!(result, "abcDEF123abc");
    }

    #[test]
    fn test_label_address() {
        let (rem, res) = argument_label_address("[LabelName32]").unwrap();
        assert_eq!(rem, "");
        assert_eq!(res, Argument::LabelAddress("LabelName32"));
    }
}