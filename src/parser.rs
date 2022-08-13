use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Peekable,
    str::FromStr,
};

use log::debug;
use regex::Regex;

pub struct Parser {
    line: Peekable<Lines<BufReader<File>>>,
    curr_command: Option<String>,

    cmd_type: Option<CommandType>,
    arg1: Option<String>,
    arg2: Option<i64>,
    trim_comment_regex: Regex,
}

use strum_macros::EnumString;
#[derive(PartialEq, Debug, EnumString)]
pub enum CommandType {
    #[strum(
        serialize = "add",
        serialize = "eq",
        serialize = "lt",
        serialize = "gt",
        serialize = "sub",
        serialize = "neg",
        serialize = "and",
        serialize = "or",
        serialize = "not"
    )]
    C_ARITHMETIC,
    #[strum(serialize = "push")]
    C_PUSH,
    #[strum(serialize = "pop")]
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
}

impl Parser {
    pub fn new(filepath: &str) -> Parser {
        let f = File::open(filepath).unwrap();
        let reader = BufReader::new(f);
        Parser {
            line: reader.lines().peekable(),
            curr_command: None,
            trim_comment_regex: Regex::new(r"(.*)\s*//.*").unwrap(),
            cmd_type: None,
            arg1: None,
            arg2: None,
        }
    }

    pub fn advance(&mut self) {
        loop {
            match self.line.next() {
                Some(l) => match l {
                    Ok(cmd_cand) => {
                        let trimmed_line = cmd_cand.trim();
                        if trimmed_line.len() > 0 && !trimmed_line.starts_with("//") {
                            let cmd = match self.trim_comment_regex.captures(trimmed_line) {
                                Some(caps) => caps.get(1).unwrap().as_str().trim(),
                                None => trimmed_line.trim(),
                            };

                            self.curr_command = Some(cmd.to_string());

                            if let Some(cmd) = &self.curr_command {
                                let vm_cmd: Vec<&str> = cmd.split(" ").collect();
                                debug!("{:?}", vm_cmd);
                                self.cmd_type = Some(CommandType::from_str(vm_cmd[0]).unwrap());
                                match self.cmd_type.as_ref().unwrap() {
                                    CommandType::C_ARITHMETIC => {
                                        self.arg1 = Some(vm_cmd[0].to_string());
                                    }
                                    CommandType::C_PUSH => {
                                        self.arg1 = Some(vm_cmd[1].to_string());
                                        self.arg2 = Some(i64::from_str(vm_cmd[2]).unwrap());
                                    }
                                    CommandType::C_POP => todo!(),
                                    CommandType::C_LABEL => todo!(),
                                    CommandType::C_GOTO => todo!(),
                                    CommandType::C_IF => todo!(),
                                    CommandType::C_FUNCTION => todo!(),
                                    CommandType::C_RETURN => todo!(),
                                    CommandType::C_CALL => todo!(),
                                }
                            }
                            break;
                        }
                    }
                    Err(e) => panic!("cannot read line. e:{}", e),
                },
                None => {
                    debug!("finish reading.");
                    self.curr_command = None;
                }
            }
        }

        debug!("curr_command: {:?}", self.curr_command);
    }

    pub fn get_command_type(&self) -> &CommandType {
        self.cmd_type.as_ref().unwrap()
    }

    pub fn get_arg_1(&self) -> &String {
        self.arg1.as_ref().unwrap()
    }

    pub fn get_arg_2(&self) -> i64 {
        self.arg2.unwrap()
    }

    pub fn has_next_cmd(&mut self) -> bool {
        match self.line.peek() {
            Some(_) => return true,
            None => return false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::CommandType;

    use super::Parser;

    #[test]
    fn work_test() {
        // env_logger::init();

        let mut parser = Parser::new("SimpleAdd.vm");
        parser.advance();
        parser.has_next_cmd();
        assert_eq!(&CommandType::C_PUSH, parser.get_command_type());
        assert_eq!("constant", parser.get_arg_1());
        assert_eq!(7, parser.get_arg_2());

        parser.advance();
        parser.has_next_cmd();
        assert_eq!(&CommandType::C_PUSH, parser.get_command_type());
        assert_eq!("constant", parser.get_arg_1());
        assert_eq!(8, parser.get_arg_2());

        parser.advance();
        parser.has_next_cmd();
        parser.get_command_type();
        assert_eq!(&CommandType::C_ARITHMETIC, parser.get_command_type());
        assert_eq!("add", parser.get_arg_1());
    }

    #[test]
    fn work_test_stack() {
        env_logger::init();

        let mut parser = Parser::new("StackTest.vm");
        parser.advance();
        parser.has_next_cmd();
        assert_eq!(&CommandType::C_PUSH, parser.get_command_type());
        assert_eq!("constant", parser.get_arg_1());
        assert_eq!(17, parser.get_arg_2());

        parser.advance();
        parser.has_next_cmd();
        assert_eq!(&CommandType::C_PUSH, parser.get_command_type());
        assert_eq!("constant", parser.get_arg_1());
        assert_eq!(17, parser.get_arg_2());

        parser.advance();
        parser.has_next_cmd();
        parser.get_command_type();
        assert_eq!(&CommandType::C_ARITHMETIC, parser.get_command_type());
        assert_eq!("eq", parser.get_arg_1());
    }
}
