use super::opcodes_parser::{load_opcode_list, OpCode};
use lazy_static::lazy_static;
use nom::*;

use core::result::Result::Ok;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref OPCODES_MAP_STRING: String = {
        fs::read_to_string("/home/liam/programming/avr-project/lbcassembler/opcodes.txt")
            .expect("cant read opcode list file")
    };
    static ref OPCODES_MAP: HashMap<&'static str, OpCode<'static>> =
        { load_opcode_list(&OPCODES_MAP_STRING).expect("error parsing opcode list") };
}

#[derive(Debug, PartialEq)]
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
    // TODO: include underscore as well
    character::complete::alphanumeric1(i)
    // this only takes one character...how to fold into a str:
    // character::complete::one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_")(i)
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
    let (i, args) = multi::many_m_n(
        opcode.expected_args,
        opcode.expected_args,
        space_and_argument,
    )(i)?;

    Ok((i, Element::OpCode(opcode, args)))
}

fn label_element(i: &str) -> IResult<&str, Element> {
    let (i, name) = label_name(i)?;
    let (i, _) = bytes::complete::tag(":")(i)?;

    Ok((i, Element::Label(name)))
}

fn element(i: &str) -> IResult<&str, Element> {
    branch::alt((opcode_element, label_element))(i)
}

fn comment(i: &str) -> IResult<&str, ()> {
    let (i, _) = bytes::complete::tag("#")(i)?;
    let (i, _) = character::complete::not_line_ending(i)?;
    Ok((i, ()))
}

fn line(i: &str) -> IResult<&str, Option<Element>> {
    let (i, maybe_comment) = combinator::opt(comment)(i)?;
    match maybe_comment {
        Some(_) => Ok((i, None)),
        None => combinator::map_res(element, |elem| -> Result<Option<Element>, ()> {
            Ok(Some(elem))
        })(i),
    }
}

fn newline(i: &str) -> IResult<&str, ()> {
    let (i, _) = character::complete::space0(i)?;
    let (i, _) = character::complete::line_ending(i)?;
    let (i, _) = character::complete::space0(i)?;
    Ok((i, ()))
}

// may not consume all input
fn program_maybe(i: &str) -> IResult<&str, Vec<Element>> {
    let (i, _) = character::complete::multispace0(i)?;
    let (i, element_options) = multi::separated_list(multi::many1(newline), line)(i)?;
    let (i, _) = character::complete::multispace0(i)?;

    let elements: Vec<Element> = element_options.into_iter().filter_map(|x| x).collect();

    Ok((i, elements))
}

fn program(i: &str) -> IResult<&str, Vec<Element>> {
    combinator::all_consuming(program_maybe)(i)
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

    #[test]
    fn test_space_and_arg() {
        let (_, res) = space_and_argument("    [label]").unwrap();
        assert_eq!(res, Argument::LabelAddress("label"));
    }

    #[test]
    fn test_opcode_element() {
        // NOTE relies on a working opcode parser
        // and the fact that pushconst has one bytecode argument
        let (_, res) = opcode_element("pushconst [hello]").unwrap();
        let expected_opcode = OPCODES_MAP.get("pushconst").unwrap();

        assert_eq!(
            res,
            Element::OpCode(&expected_opcode, vec![Argument::LabelAddress("hello")])
        );
    }

    #[test]
    fn test_comment() {
        let (rem, _) = comment("# Here is a comment\nmore").unwrap();
        assert_eq!(rem, "\nmore");
    }

    #[test]
    fn test_line() {
        // comments
        let (_, res) = line("# A Comment").unwrap();
        assert_eq!(res, None);

        // opcodes
        let (_, res) = line("pushconst [hello]").unwrap();
        let expected_opcode = OPCODES_MAP.get("pushconst").unwrap();
        assert_eq!(
            res,
            Some(Element::OpCode(
                &expected_opcode,
                vec![Argument::LabelAddress("hello")]
            ))
        );

        let (_, res) = line("LabelName123:").unwrap();
        assert_eq!(res, Some(Element::Label("LabelName123")));
    }

    #[test]
    fn test_program() {
        let (_rem, _res) = program(
            "

        labelName:
            pushconst 34


            pushconst [labelName]
        # Henlo

            asd123:
            pushconst [asd123]

        ",
        )
        .unwrap();
        // println!("{:#?}", res);
        //TODO assert that _res is the correct ast
    }
}