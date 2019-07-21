use nom::*;
use std::collections::HashMap;

/// Format: <name> <opcode value in hex> [expected number of args]
/// if not specified, expected number of args is 0

#[derive(Debug, PartialEq)]
pub struct OpCode<'a> {
    pub name: &'a str,
    pub code: u8,
    pub expected_args: usize,
}

/// produces a HashMap<opcode_name, opcode_struct> from a string of opcode specifications
pub fn load_opcode_list(data: &str) -> Result<HashMap<&str, OpCode>, ()> {
    let (i, opcode_vec) = parse_opcode_file(&data).map_err(|_| {
        // println!("{:?}", error);
        ()
    })?;
    let mut map = HashMap::new();
    for opcode in opcode_vec {
        map.insert(opcode.name.clone(), opcode);
    }

    //TODO use all_comsuming combinator for this instead
    if !i.is_empty() {
        // all input should be consumed, else error
        Err(())
    } else {
        Ok(map)
    }
}

fn parse_ident(i: &str) -> IResult<&str, &str> {
    character::complete::alpha1(i)
}

fn hex_to_u8(digits: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(digits, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn parse_code(i: &str) -> IResult<&str, u8> {
    combinator::map_res(
        bytes::complete::take_while_m_n(2, 2, is_hex_digit),
        hex_to_u8,
    )(i)
}

fn parse_args(i: &str) -> IResult<&str, usize> {
    let (i, _) = character::complete::space0(i)?;
    let (i, num) = character::complete::digit1(i)?;
    Ok((i, num.parse().unwrap()))
}

fn parse_opcode(i: &str) -> IResult<&str, OpCode> {
    let (i, name) = parse_ident(i)?;
    let (i, _) = character::complete::space1(i)?;
    let (i, code) = parse_code(i)?;
    let (i, expected_args) = combinator::opt(parse_args)(i)?;
    Ok((
        i,
        OpCode {
            name,
            code,
            expected_args: expected_args.unwrap_or(0),
        },
    ))
}

fn parse_whitespace_and_newlines(i: &str) -> IResult<&str, Vec<&str>> {
    multi::many1(branch::alt((
        character::complete::line_ending,
        character::complete::space1,
    )))(i)
}

fn parse_opcode_file(i: &str) -> IResult<&str, Vec<OpCode>> {
    let (i, _) = combinator::opt(parse_whitespace_and_newlines)(i)?;
    let (i, res) =
        multi::separated_list(multi::many1(character::complete::newline), parse_opcode)(i)?;
    let (i, _) = combinator::opt(parse_whitespace_and_newlines)(i)?;

    Ok((i, res))
}
