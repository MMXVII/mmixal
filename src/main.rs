extern crate regex;

mod parse;
mod syntax;
mod is;

use std::collections::HashMap;

use parse::{ParseError, ParseErrorKind};
use syntax::ParsedLine;

fn main() {


    let lines = vec!["Label: ADDU FutureReference, 2, 3", "AnotherLabel: CMPU 1, 2, HOHOHO"];
    let mut first_pass = FirstPass::new();

    if let Err(err) = first_pass.run(&lines) {
        println!("{:?}", err);
        return;
    }

    println!("{:#?}", first_pass.symbol_table);
    println!("{:#?}", first_pass.parsed);
}

pub struct FirstPass {
    pub symbol_table: HashMap<String, u64>,
    pub parsed: Vec<(ParsedLine, u64)>,
}


impl FirstPass {

    pub fn new() -> Self {
        FirstPass {
            symbol_table: HashMap::new(),
            parsed: Vec::new(),
        }
    }

    pub fn run(&mut self, lines: &[&str]) -> Result<(), ParseError> {
        let mut pc = 0;
        for (line_no, line) in lines.iter().enumerate() {

            let parsed_line_opt = parse::parse(line).map_err(|err_kind| ParseError {
                kind: err_kind,
                line: line_no as u64,
            })?;

            if let Some(parsed_line) = parsed_line_opt {
                match parsed_line.clone() {
                    ParsedLine::RegularInstruction(instr) => {
                        if let Some(label_str) = instr.label {
                            if self.symbol_table.contains_key(&label_str) {
                                return Err(ParseError{
                                    kind: ParseErrorKind::LabelDoubleUse,
                                    line: line_no as u64,
                                })
                            }
                            self.symbol_table.insert(label_str, pc);
                        }
                        pc += 4;
                    }

                    _ => {

                    }
                }
                self.parsed.push((parsed_line, line_no as u64));
            }

        }
        Ok(())
    }
}
