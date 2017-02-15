extern crate regex;

mod parse;
mod syntax;
mod is;

use std::collections::HashMap;

use parse::ParseError;
use syntax::ParsedLine;

fn main() {
    let symbol_table: HashMap<i32, i32> = HashMap::new();
    let pc = 0;

    let instruction = parse::parse("Label: ADDU FutureReference, 2, 3");
    println!("{:?}", instruction);
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

    pub fn run(lines: &[&str]) -> Result<(), ParseError> {
        let pc = 0;
        for (line_no, line) in lines.iter().enumerate() {

            let parsed_line = parse::parse(line).map_err(|err_kind| ParseError {
                kind: err_kind,
                line: line_no as u32,
            })?;

        }
        unimplemented!()
    }
}
