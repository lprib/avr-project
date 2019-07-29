mod assembler;
mod formatter;
mod opcodes_parser;

mod parser;
mod structures;
use assembler::gen_code;

use formatter::{format_bytecode, C_FORMAT_SPEC};
use parser::parse_program;
use std::fmt;
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

fn try_main<'a>() -> Result<(), (&'a str, impl fmt::Debug)> {
    let opt = Opt::from_args();
    let mut output: Box<Write> = match opt.output {
        Some(pathbuf) => {
            Box::new(File::create(pathbuf).err_message("unable to open or create output file")?)
        }
        None => Box::new(io::stdout()),
    };

    let input_string = fs::read_to_string(opt.input).err_message("unable to read input file")?;
    //TODO why tf err_message not working here:
    let ast = parse_program(&input_string).expect("couldn't parse");

    match opt.emit {
        Some(OutputType::Ast) => {
            //todo better error messages
            writeln!(&mut output, "{:?}", ast).err_message("Unable to write AST to output file")?;
        }

        Some(OutputType::PrettyAst) => {
            writeln!(&mut output, "{:#?}", ast)
                .err_message("Unable to write AST to output file")?;
        }
        // test on None as well since this should be the default
        Some(OutputType::ByteCode) | None => {
            let code = gen_code(&ast);
            for byte in code.data {
                write!(&mut output, "{} ", byte).err_message("unable to write to output")?;
            }
            writeln!(&mut output).err_message("unable to write to output")?;
        }

        Some(OutputType::CByteCode) => {
            let code = gen_code(&ast);
            write!(&mut output, "{}", format_bytecode(code, &C_FORMAT_SPEC))
                .err_message("unable to write to output")?;
        }
    }
    Ok(())
}

trait MessageError<T, E: fmt::Debug> {
    fn err_message(self, message: &'static str) -> Result<T, (&'static str, E)>;
}

impl<T, E: fmt::Debug> MessageError<T, E> for Result<T, E> {
    fn err_message(self, message: &'static str) -> Result<T, (&'static str, E)> {
        self.map_err(move |e| (message, e))
    }
}

fn main() {
    match try_main() {
        Ok(()) => {
            std::process::exit(0);
        }
        Err((message, err)) => {
            eprintln!("ERROR: {}", message);
            eprintln!("{:?}", err);
        }
    }
}