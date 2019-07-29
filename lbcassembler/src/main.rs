mod assembler;
mod formatter;
mod opcodes_parser;
mod structures;
mod parser;

use assembler::gen_code;

use formatter::{format_bytecode, C_FORMAT_SPEC};
use parser::parse_program;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum OutputType {
        Ast,
        PrettyAst,
        ByteCode,
        CByteCode,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "lbcassembler", rename_all = "kebab-case")]
struct Opt {
    #[structopt(short = "o", parse(from_os_str))]
    output: Option<PathBuf>,

    #[structopt(long, short, case_insensitive = true)]
    emit: Option<OutputType>,

    #[structopt(name = "input-file", parse(from_os_str), help = "Input File")]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let mut output: Box<Write> = match opt.output {
        Some(pathbuf) => {
            Box::new(File::create(pathbuf).expect("unable to open or create output file"))
        }
        None => Box::new(io::stdout()),
    };

    let input_string = fs::read_to_string(opt.input).expect("unable to read input file");
    let ast = parse_program(&input_string).expect("unable to parse");

    match opt.emit {
        Some(OutputType::Ast) => {
            //todo better error messages
            writeln!(&mut output, "{:?}", ast).expect("Unable to write AST to output file");
        }

        Some(OutputType::PrettyAst) => {
            writeln!(&mut output, "{:#?}", ast).expect("Unable to write AST to output file");
        }
        // test on None as well since this should be the default
        Some(OutputType::ByteCode) | None => {
            let code = gen_code(&ast);
            for byte in code.data {
                write!(&mut output, "{} ", byte).expect("unable to write to output");
            }
            writeln!(&mut output).expect("unable to write to output");
        }

        Some(OutputType::CByteCode) => {
            let code = gen_code(&ast);
            write!(&mut output, "{}", format_bytecode(code, &C_FORMAT_SPEC)).expect("unable to write to output");
        }
    }
}