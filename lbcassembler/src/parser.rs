use super::opcodes_parser::load_opcode_list;
use super::structures::{Argument, Element, OpCode};

use core::result::Result::Ok;
use lazy_static::lazy_static;
use nom::character::complete::*;
use nom::*;
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

// On error, returns (remaining_input, nom_error) tuple
pub fn parse_program(input: &str) -> Result<Vec<Element>, nom::Err<(&str, nom::error::ErrorKind)>> {
    program(input).map(|(_, vec)| vec)
}

// creates a new vec, so needs to return String not &str
fn label_name(i: &str) -> IResult<&str, &str> {
    combinator::recognize(multi::many1(branch::alt((
        bytes::complete::tag("_"),
        alphanumeric1,
    ))))(i)
}

fn decimal_literal(i: &str) -> IResult<&str, u16> {
    combinator::map_res(digit1, |strn: &str| strn.parse())(i)
}

fn str_to_opcode(i: &str) -> Result<&OpCode, ()> {
    OPCODES_MAP.get(i).ok_or(())
}

fn opcode_name(i: &str) -> IResult<&str, &OpCode> {
    combinator::map_res(alpha0, str_to_opcode)(i)
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
    let (i, _) = space1(i)?;
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

fn string_literal(i: &str) -> IResult<&str, &str> {
    sequence::delimited(
        bytes::complete::tag("\""),
        combinator::recognize(multi::many0(none_of("\""))),
        bytes::complete::tag("\""),
    )(i)
}

fn from_hex(i: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(i, 16)
}

fn is_hex_digit(i: char) -> bool {
    i.is_digit(16)
}

fn hex_literal(i: &str) -> IResult<&str, Vec<u8>> {
    multi::many1(combinator::map_res(
        bytes::complete::take_while_m_n(2, 2, is_hex_digit),
        from_hex,
    ))(i)
}

fn string_raw_value(i: &str) -> IResult<&str, Element> {
    let (i, _) = bytes::complete::tag(".string")(i)?;
    let (i, _) = space1(i)?;
    let (i, string) = string_literal(i)?;

    println!("{}", string);

    let mut out_vec = Vec::new();
    out_vec.extend_from_slice(string.as_bytes());

    Ok((i, Element::RawData(out_vec)))
}

fn hex_raw_value(i: &str) -> IResult<&str, Element> {
    let (i, _) = bytes::complete::tag(".hex")(i)?;
    let (i, _) = space1(i)?;
    let (i, out_vec) = hex_literal(i)?;

    Ok((i, Element::RawData(out_vec)))
}

fn raw_value(i: &str) -> IResult<&str, Element> {
    branch::alt((string_raw_value, hex_raw_value))(i)
}

fn element(i: &str) -> IResult<&str, Element> {
    branch::alt((opcode_element, label_element, raw_value))(i)
}

fn comment(i: &str) -> IResult<&str, ()> {
    let (i, _) = bytes::complete::tag("#")(i)?;
    let (i, _) = not_line_ending(i)?;
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
    let (i, _) = space0(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = space0(i)?;
    Ok((i, ()))
}

// may not consume all input
fn program_nonconsuming(i: &str) -> IResult<&str, Vec<Element>> {
    let (i, _) = multispace0(i)?;
    let (i, element_options) = multi::separated_list(multi::many1(newline), line)(i)?;
    let (i, _) = multispace0(i)?;

    let elements: Vec<Element> = element_options.into_iter().filter_map(|x| x).collect();

    Ok((i, elements))
}

fn program(i: &str) -> IResult<&str, Vec<Element>> {
    combinator::all_consuming(program_nonconsuming)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_name() {
        let (remaining, result) = label_name("abcDEF1_23abc more").unwrap();
        assert_eq!(remaining, " more");
        assert_eq!(result, "abcDEF1_23abc");
    }

    #[test]
    fn test_label_address() {
        let (rem, res) = argument_label_address("[Label_Name32]").unwrap();
        assert_eq!(rem, "");
        assert_eq!(res, Argument::LabelAddress("Label_Name32"));
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
    fn hex_raw() {
        let (_, res) = hex_raw_value(".hex FF00AABC").unwrap();
        assert_eq!(res, Element::RawData(vec![0xFF, 0x00, 0xAA, 0xBC]));
    }

    #[test]
    fn test_string_literal() {
        let (_, res) = string_literal("\"Hi There\"").unwrap();
        assert_eq!(res, "Hi There");
    }

    #[test]
    fn string_raw() {
        let (_, res) = string_raw_value(".string \"hello\"").expect("ohs hit");
        match res {
            Element::RawData(data) => assert_eq!(&*data, "hello".as_bytes()),
            _ => panic!(),
        }
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

            .hex 0123456789ABCDEF

            .string \"string here asd\"

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