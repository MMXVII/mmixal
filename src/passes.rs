use std::collections::HashMap;

use syntax::{ParsedLine, Operand, Instruction};
use parse;
use parse::{ParseError, ParseErrorKind};
use is::Command;

pub struct IntermediateResult {
    pub symbol_table: HashMap<String, u64>,
    pub parsed: Vec<(ParsedLine, u64)>,
}

pub fn first_pass(lines: &[&str]) -> Result<IntermediateResult, ParseError> {

    let mut pc = 0;
    let mut symbol_table = HashMap::new();
    let mut parsed = Vec::new();


    for (line_no, line) in lines.iter().enumerate() {

        let parsed_line_opt = parse::parse(line).map_err(|kind| kind.to_parse_err(line_no as u64))?;

        if let Some(parsed_line) = parsed_line_opt {
            match parsed_line.clone() {
                ParsedLine::RegularInstruction(instr) => {
                    if let Some(label_str) = instr.label {
                        if symbol_table.contains_key(&label_str) {
                            return Err(ParseError{
                                kind: ParseErrorKind::LabelDoubleUse,
                                line: line_no as u64,
                            })
                        }
                        symbol_table.insert(label_str, pc);
                    }
                    pc += 4;
                }
                _ => {
                    unimplemented!();
                }
            }
            parsed.push((parsed_line, line_no as u64));
        }
    }


    Ok(IntermediateResult {
        symbol_table: symbol_table,
        parsed: parsed,
    })
}


fn second_pass(intermediate: &IntermediateResult) -> Result<Vec<u8>, ParseError> {

    let mut binary = Vec::new();
    for &(ref line, line_no) in intermediate.parsed.iter() {
        match line {
            &ParsedLine::RegularInstruction(ref instr) => {
                binary.push(instr.command.opcode());

                for i in 0..3 {
                    let effective_op: u8 = match instr.operands[i] {
                        Operand::Value(val) => {
                            val
                        }
                        Operand::Label(ref label_str) => {
                            let key = &label_str.clone();

                            let address = intermediate.symbol_table.get(key);

                            if address.is_none() {
                                return Err(ParseError {
                                    kind: ParseErrorKind::UndefinedLabel,
                                    line: line_no,
                                });
                            }

                            // TODO: special treatment for branch / jump commands
                            *address.unwrap() as u8
                        }
                    };
                    binary.push(effective_op);
                }
            }
            _ => unimplemented!(),
        }
    }
    unimplemented!()
}
