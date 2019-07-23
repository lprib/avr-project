mod assembler;
mod opcodes_parser;
mod parser;
mod structures;


use assembler::gen_code;
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
        ByteCode
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
            write!(&mut output, "{:#?}", ast).expect("Unable to write AST to output file");
        }
        // test on None as well since this should be the default
        Some(OutputType::ByteCode) | None => {
            let code = gen_code(&ast);
            println!("{:?}", code.data);
        }
    }
}