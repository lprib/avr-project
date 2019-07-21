mod opcodes_parser;
mod parser;

use std::fs;

fn main() {
    let string = fs::read_to_string("/home/liam/programming/avr-project/lbcassembler/opcodes.txt")
        .expect("cant read file");
    let map = opcodes_parser::load_opcode_list(&string).unwrap();

    // let a = opcodes_parser::parse_opcode(&string);
    // println!("{:?}", a);
    println!("{:#?}", map);
}
