use std::collections::HashMap;

use syntax::{ParsedLine, Operand, Instruction};
use parse;
use parse::{ParseError, ParseErrorKind};
use is::Command;


/// Represents the information gathered by the first assembler pass.
/// This information is needed in order to perform the second pass.
pub struct IntermediateResult {
    pub symbol_table: HashMap<String, u64>,
    pub parsed: Vec<(ParsedLine, u64)>,
}


/// The first pass goes through each line, disregards empty or comment lines,
/// trys to parse all remaining lines to `ParsedLine` and constructs the symbol table.
pub fn first_pass(lines: &[String]) -> Result<IntermediateResult, ParseError> {

    let mut pc = 0;
    let mut symbol_table = HashMap::new();
    let mut parsed = Vec::new();


    for (line_no, line) in lines.iter().enumerate() {

        let parsed_line_opt = parse::parse(line).map_err(|kind| kind.to_parse_err(line_no as u64))?;

        // If the line was not empty or only a comment, process it
        if let Some(parsed_line) = parsed_line_opt {

            match parsed_line.clone() {

                // Deal with regular instructions
                ParsedLine::RegularInstruction(instr) => {

                    // If the instruction is labeled
                    if let Some(label_str) = instr.label {

                        // Check if the label already exists
                        if symbol_table.contains_key(&label_str) {
                            return Err(ParseError{
                                kind: ParseErrorKind::LabelDoubleUse,
                                line: line_no as u64,
                            })
                        }

                        // If not, insert it into symbol table
                        symbol_table.insert(label_str, pc);
                    }

                    // Advance program counter
                    pc += 4;
                }

                // Deal with assembler directives
                _ => {
                    unimplemented!();
                }
            }

            // Save the parsed line for later use
            parsed.push((parsed_line, line_no as u64));
        }
    }


    Ok(IntermediateResult {
        symbol_table: symbol_table,
        parsed: parsed,
    })
}

/// The second pass uses the symbol table to translate
pub fn second_pass(intermediate: &IntermediateResult) -> Result<Vec<u8>, ParseError> {

    let mut binary: Vec<u8> = Vec::new();

    for &(ref line, line_no) in intermediate.parsed.iter() {

        match line {

            // Translate regular instructions to binary
            &ParsedLine::RegularInstruction(ref instr) => {

                let result = translate_instruction(instr, &(intermediate.symbol_table), binary.len() as u64);

                binary.extend(&result.map_err(|kind| kind.to_parse_err(line_no))?);
            }

            // Translate assembler directives
            _ => unimplemented!(),
        }
    }

    Ok(binary)
}

fn translate_instruction(instr: &Instruction, symbols: &HashMap<String, u64>, pc: u64)
    -> Result<Vec<u8>, ParseErrorKind>
{

    let mut binary = Vec::new();

    // Translate command to opcode
    binary.push(instr.command.opcode());

    if instr.command.is_relative_branch() {
        if let Operand::Label(ref label_str) = instr.operands[2] {
            let key = &label_str.clone();
            let absolute_address_opt = symbols.get(key);

            if absolute_address_opt.is_none() {
                return Err(ParseErrorKind::UndefinedLabel);
            }

            let absolute_address = absolute_address_opt.unwrap();
            assert_eq!(absolute_address % 4, 0);

            let difference = if command.is_forward_branch() {
                (((absolute_address - pc) / 4) - 1) as u16
            } else {
                (((pc - absolute_address) / 4) + 1) as u16
            };

            let x_op = match instr.operands[0] {
                Operand::Value(val) => val,
                Operand::Label(_) => unimplemented!()
            };

            let y_op = (difference >> 8) as u8;
            let z_op = (difference & 0xF) as u8;

            binary.push(x_op);
            binary.push(y_op);
            binary.push(z_op);

            return Ok(binary)

        } else {
            unimplemented!()
        }
    }

    // Translate operands
    for i in 0..3 {
        let effective_op: u8 = match instr.operands[i] {

            Operand::Value(val) => {
                val
            }
            Operand::Label(ref label_str) => {

                let key = &label_str.clone();
                let address = symbols.get(key);

                if address.is_none() {
                    return Err(ParseErrorKind::UndefinedLabel);
                }

                // TODO: special treatment for branch / jump commands
                *address.unwrap() as u8
            }
        };
        binary.push(effective_op);
    }

    Ok(binary)
}
