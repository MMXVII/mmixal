#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {
   Addu,
   Addui,
   Cmpu,
   Cmpui,
   Bz,
   Bzb,
   Bnz,
   Bnzb,
   Divu,
   Trap,
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
            "DIVU" => Some(Divu),
            "TRAP" => Some(Trap),
            "BZ" => Some(Bz),
            "BNZ" => Some(Bnz),
            "BZB" => Some(Bzb),
            "BNZB" => Some(Bnzb),
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
            Trap => 0x00,
            Bz => 0x42,
            Bnz => 0x4a,
            _ => unimplemented!(),
        }
    }

    pub fn is_relative_branch(&self) -> bool {
        use self::Command::*;
        match *self {
            Bnz | Bnzb | Bz | Bzb => true,
            _ => false,
        }
    }
}
