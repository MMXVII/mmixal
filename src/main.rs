extern crate regex;

mod parse;
mod syntax;

fn main() {
    let instruction = parse::parse("Label: ADD FutureReference, 2, 3");
    println!("{:?}", instruction);

}

const MAPPING: [(&'static str, u8); 4] = [
    ("ADDU", 0x22),
    ("ADDUI", 0x23),
    ("CMPU", 0x32),
    ("CMPUI", 0x33),
];
