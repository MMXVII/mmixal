use is::Command;
use syntax::{ParsedLine, Directive, Instruction, Operand};

use lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_EMPTY: Regex = {
        Regex::new("(^[[:space:]]*$)|(^[[:space:]]*;)").unwrap()
    };
    static ref REGEX: Regex = {

        let mut regex_str = String::new();

        regex_str.push_str("^[[:space:]]*");
        regex_str.push_str("(?P<label>[[:alpha:]]+:[[:space:]])?");  // Optional label
        regex_str.push_str("[[:space:]]*");
        regex_str.push_str("(?P<instr>[A-Z]+)");                     // Symbolic instruction name
        regex_str.push_str("[[:space:]]+");
        regex_str.push_str("(?P<opx>[[:alpha:]]+|[0-9]+)");          // X operand
        regex_str.push_str(",[[:space:]]");
        regex_str.push_str("(?P<opy>[[:alpha:]]+|[0-9]+)");          // Y operand
        regex_str.push_str(",[[:space:]]");
        regex_str.push_str("(?P<opz>[[:alpha:]]+|[0-9]+)");          // Z operand

        Regex::new(&regex_str).unwrap()
    };
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub line: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
    NumberTooBig,
    SyntaxError,
    UnknownSymbolic,
    LabelDoubleUse,
    UndefinedLabel,
}

impl ParseErrorKind {
    pub fn to_parse_err(self, line: u64) -> ParseError {
        ParseError {
            kind: self,
            line: line,
        }
    }
}

pub fn parse(command: &str) -> Result<Option<ParsedLine>, ParseErrorKind> {
    // disregard empty and comment lines
    if REGEX_EMPTY.is_match(command) {
        return Ok(None);
    }

    // TODO: check whether line contains normal instruction or directive and call
    // corresponding function
    parse_instruction(command).map(|instr| Some(ParsedLine::RegularInstruction(instr)))
}

pub fn parse_instruction(line: &str) -> Result<Instruction, ParseErrorKind> {

    let captures = REGEX.captures(&line).ok_or(ParseErrorKind::SyntaxError)?;

    // Construct optional label "label"-capture
    let label = if let Some(label_cap) = captures.name("label") {
        let mut label_str = String::from(label_cap.as_str());
        assert!(label_str.pop().unwrap().is_whitespace());
        assert_eq!(label_str.pop().unwrap(), ':');
        Some(label_str)
    } else {
        None
    };

    // Construct command from "instr"-capture
    let command_str = captures.name("instr").unwrap().as_str();

    let command = Command::from_str(command_str).ok_or(ParseErrorKind::UnknownSymbolic)?;

    fn construct_operand(text: &str) -> Result<Operand, ParseErrorKind> {
        if text.chars().nth(0).unwrap().is_digit(10) {
            text.parse::<u8>().map(|n| Operand::Value(n)).map_err(|_| ParseErrorKind::NumberTooBig)
        } else {
            Ok(Operand::Label(String::from(text)))
        }
    }

    let op_str = | capture_str | {
        captures.name(capture_str).unwrap().as_str()
    };

    let operands = vec![
        construct_operand(op_str("opx"))?,
        construct_operand(op_str("opy"))?,
        construct_operand(op_str("opz"))?,
    ];

    Ok(Instruction {
        label: label,
        command: command,
        operands: operands,
    })
}

fn parse_directive(_line: &str) -> Result<Directive, ParseErrorKind> {
    unimplemented!()
}
