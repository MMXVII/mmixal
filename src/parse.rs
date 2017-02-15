use is::Command;
use syntax::{ParsedLine, Directive, Instruction, Operand};

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub line: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
    u8ParseError,
    SyntaxError,
    UnknownSymbolic,
}



pub fn parse(command: &str) -> Result<Option<ParsedLine>, ParseErrorKind> {
    // TODO: return Ok(None) for empty lines or comment lines
    // TODO: check whether line contains normal instruction or directive and call
    // corresponding function
    parse_instruction(command).map(|instr| Some(ParsedLine::Instruction(instr)))
}

pub fn parse_instruction(line: &str) -> Result<Instruction, ParseErrorKind> {
    // TODO use lazy_static! in order to compile the regex only once
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

    let regex = Regex::new(&regex_str).unwrap();
    let captures = regex.captures(&line).ok_or(ParseErrorKind::SyntaxError)?;

    // construct optional label "label"-capture
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
            text.parse::<u8>().map(|n| Operand::Value(n)).map_err(|_| ParseErrorKind::u8ParseError)
        } else {
            Ok(Operand::Label(String::from(text)))
        }
    }

    Ok(Instruction {
        label: label,
        command: command,
        x_operand: construct_operand(captures.name("opx").unwrap().as_str())?,
        y_operand: construct_operand(captures.name("opy").unwrap().as_str())?,
        z_operand: construct_operand(captures.name("opz").unwrap().as_str())?,
    })
}

fn parse_directive(line: &str) -> Result<Directive, ParseErrorKind> {
    unimplemented!()
}
