use super::structures::ByteCode;

use lazy_static::lazy_static;
use regex::Regex;

pub struct FormatSpec<'a> {
    start: &'a str,
    instruction: &'a str,
    delimiter: &'a str,
    end: &'a str,
}

lazy_static! {
    pub static ref C_FORMAT_SPEC: FormatSpec<'static> = FormatSpec {
        start: "unsigned char bytecode[] = {\n",
        instruction: "\t0x%i",
        delimiter: ",\n",
        end: "\n};\n"
    };
}

/// formats the output bytecode for a certain format
pub fn format_bytecode(bytecode: ByteCode, spec: &FormatSpec) -> String {
    lazy_static! {
        static ref INSTRUCTION_REGEX: Regex =
            Regex::new(r#"%i"#).expect("Couldnt create regex when formatting bytecode.");
    }

    let mut output = String::from(spec.start);
    for (i, byte) in bytecode.data.iter().enumerate() {
        let code_string = &format!("{:02X}", byte)[..];
        output.push_str(&INSTRUCTION_REGEX.replace(spec.instruction, code_string));
        if i != bytecode.data.len() - 1 {
            output.push_str(spec.delimiter);
        }
    }
    output.push_str(spec.end);
    output
}