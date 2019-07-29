use super::structures::{Argument, Element, ByteCode};

use std::collections::HashMap;

impl Element<'_> {
    //can only use u16 for size due to 16 bit stack element and argument size
    fn bytecode_size(&self) -> u16 {
        match &self {
            Element::Label(_) => 0,
            Element::OpCode(opcode, _) => 1 + 2 * opcode.expected_args as u16,
            Element::RawData(data) => data.len() as u16,
        }
    }
}

pub fn gen_code(ast: &[Element]) -> ByteCode {
    let label_table = gen_label_table(ast);

    let mut bytecode = Vec::new();

    for element in ast {
        match element {
            Element::Label(_) => { /* Maybe add a debug tag or something here*/ }
            Element::OpCode(opcode, args) => {
                bytecode.push(opcode.code);
                for arg in args {
                    bytecode.append(&mut to_bytecode(arg, &label_table));
                }
            }
            Element::RawData(data) => {
                bytecode.extend(data);
            }
        }
    }

    ByteCode { data: bytecode }
}

fn gen_label_table<'a>(ast: &'a [Element]) -> HashMap<&'a str, u16> {
    let mut label_table = HashMap::new();

    let mut bytecode_index = 0;
    for element in ast {
        match element {
            Element::Label(name) => {
                label_table.insert(*name, bytecode_index);
            }
            _ => bytecode_index += element.bytecode_size(),
        }
    }

    label_table
}

fn to_bytecode(argument: &Argument, label_table: &HashMap<&str, u16>) -> Vec<u8> {
    match argument {
        Argument::Value(n) => to_byte_vec(*n),
        Argument::LabelAddress(name) => to_byte_vec(
            *label_table
                .get(name)
                .unwrap_or_else(|| panic!("unknwon label: {}", name)),
        ),
    }
}

fn to_byte_vec(value: u16) -> Vec<u8> {
    vec![(value >> 8) as u8, (value & 0xFF) as u8]
}