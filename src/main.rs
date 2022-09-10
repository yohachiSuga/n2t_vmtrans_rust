use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use clap::Parser;
use log::{debug, info};
use writer::CodeWriter;

mod parser;
mod template;
mod writer;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short)]
    input: String,
    #[clap(short)]
    out: String,
    #[clap(short)]
    debug: bool,
}

fn compile<W: Write>(inputs: Vec<String>, output: &mut CodeWriter<W>, debug: bool) {
    // only for debugging to init Stack Pointer
    if debug {
        output.debug();
    }

    for f in inputs {
        let mut parser = parser::Parser::new(&f);

        // to remove slash
        let path = Path::new(&f);
        output.setFileName(&path.file_name().unwrap().to_str().unwrap());

        loop {
            if !parser.has_next_cmd() {
                break;
            }

            parser.advance();

            match parser.get_command_type() {
                parser::CommandType::C_ARITHMETIC => output.writeArithmetic(parser.get_arg_1()),
                parser::CommandType::C_PUSH | parser::CommandType::C_POP => output.writePushPop(
                    parser.get_command_type(),
                    parser.get_arg_1(),
                    parser.get_arg_2(),
                ),
                parser::CommandType::C_LABEL => todo!(),
                parser::CommandType::C_GOTO => todo!(),
                parser::CommandType::C_IF => todo!(),
                parser::CommandType::C_FUNCTION => todo!(),
                parser::CommandType::C_RETURN => todo!(),
                parser::CommandType::C_CALL => todo!(),
            }
        }
    }
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let mut files = vec![];
    if Path::new(&args.input).is_dir() {
        for entry in fs::read_dir(&args.input).unwrap() {
            if let Ok(e) = entry {
                let path = e.path();
                match path.extension() {
                    Some(ext) => {
                        if ext == "vm" {
                            info!("load vm file {:?}", path);
                            files.push(path.to_str().unwrap().to_string());
                        }
                    }
                    None => {
                        // do nothing for folders
                    }
                }
            } else {
                panic!("panic");
            }
        }
    } else {
        info!("load vm file {}", args.input);
        files.push(args.input);
    }

    if files.len() == 0 {
        panic!("no length");
    }

    let file = File::create(args.out).unwrap();
    let mut writer = BufWriter::new(file);
    let mut writer = writer::CodeWriter::new(writer);
    compile(files, &mut writer, args.debug);
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufWriter, Read};

    use crate::{compile, writer::CodeWriter};

    #[test]
    fn work_test() {
        let mut actual = vec![];
        {
            let mut writer = BufWriter::new(&mut actual);
            let mut writer = CodeWriter::new(writer);
            compile(vec!["SimpleAdd.vm".to_string()], &mut writer, true);
        }
    }
}
