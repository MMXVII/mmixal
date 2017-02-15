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

                unimplemented!()
                //binary.push(actual_operand(&instr.x_operand, &intermediate.symbol_table, instr.command));
                //binary.push(actual_operand(&instr.y_operand, &intermediate.symbol_table, instr.command));
                //binary.push(actual_operand(&instr.z_operand, &intermediate.symbol_table, instr.command));
            }
            _ => unimplemented!(),
        }
    }
    unimplemented!()
}

fn actual_operand(operand: &Operand, symbol_table: &HashMap<String, u64>, comm: &Command)
    -> Result<u8, ParseErrorKind>
{

    match *operand {
        Operand::Value(val) => {
            Ok(val)
        }
        Operand::Label(ref label_str) => {
            let key = &label_str.clone();
            let &address = symbol_table.get(key).ok_or(ParseErrorKind::UndefinedSymbol)?;

            // TODO: special treatment for branch / jump commands
            Ok(address as u8)
        }
    }
}
