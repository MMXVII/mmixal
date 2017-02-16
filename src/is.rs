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
   Incl,
   Setl,
   Jmpb,
   Get,
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
            "JMPB" => Some(Jmpb),
            "SETL" => Some(Setl),
            "INCL" => Some(Incl),
            "GET" => Some(Get),
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
            Bnzb => 0x4b,
            Bzb => 0x43,
            Divu => 0x1e,
            Incl => 0xe7,
            Setl => 0xe3,
            Jmpb => 0xf1,
            Get => 0xfe,
        }
    }

    /// Bad last minute code
    pub fn is_relative_branch(&self) -> bool {
        use self::Command::*;
        match *self {
            Bnz | Bnzb | Bz | Bzb | Jmpb => true,
            _ => false,
        }
    }

    /// Bad last minute code
    pub fn is_forwards_branch(&self) -> bool {
        use self::Command::*;
        match *self {
            Bnz | Bz => true,
            _ => false,
        }
    }
}
