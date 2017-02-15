use is::Command;

#[derive(Debug, PartialEq, Eq)]
pub enum Line {
    Data(Directive),
    Instruction(Instruction),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Label(String),
    Value(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Directive();

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub label: Option<String>,
    pub command: Command,
    pub x_operand: Operand,
    pub y_operand: Operand,
    pub z_operand: Operand,
}
