#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {
   Addu,
   Addui,
   Cmpu,
   Cmpui,
}

impl Command {

    // TODO: replace this with implementation of FromStr interface
    pub fn from_str(s: &str) -> Option<Self> {
        use self::Command::*;
        match s {
            "ADDU" => Some(Addu),
            "ADDUI" => Some(Addui),
            "CMPU" => Some(Cmpu),
            "CMPUI" => Some(Cmpui),
            _ => None,
        }
    }

    pub fn opcode(&self) -> u8 {
        use self::Command::*;
        match *self {
            Addu => 0x22,
            Addui => 0x23,
            Cmpu => 0x32,
            Cmpui => 0x33,
        }
    }
}
