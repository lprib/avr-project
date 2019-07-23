use super::structures::{Argument, Element};

use std::collections::HashMap;

//TODO move opcode, arg, element into different modules
impl Element<'_> {
    fn bytecode_size(&self) -> usize {
        match &self {
            Element::Label(_) => 0,
            Element::OpCode(opcode, _) => 1 + 2 * opcode.expected_args,
        }
    }
}

pub struct ByteCode {
    pub data: Vec<u8>,
}

pub fn gen_code(ast: &[Element]) -> ByteCode {
    let label_table = gen_label_table(ast);

    let mut bytecode = Vec::new();

    for element in ast {
        match element {
            Element::Label(_) => {}
            Element::OpCode(opcode, args) => {
                bytecode.push(opcode.code);
                for arg in args {
                    bytecode.append(&mut to_bytecode(arg, &label_table));
                }
            }
        }
    }

    ByteCode { data: bytecode }
}

fn gen_label_table<'a>(ast: &'a [Element]) -> HashMap<&'a str, usize> {
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

fn to_bytecode(argument: &Argument, label_table: &HashMap<&str, usize>) -> Vec<u8> {
    match argument {
        Argument::Value(n) => to_byte_vec(*n),
        Argument::LabelAddress(name) => to_byte_vec(
            *label_table
                .get(name)
                .expect(&format!("unknown label: {}", name)) as u16,
        ),
    }
}

fn to_byte_vec(value: u16) -> Vec<u8> {
    vec![(value >> 8) as u8, (value & 0xFF) as u8]
}