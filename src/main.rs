extern crate regex;

mod parse;
mod syntax;
mod is;

fn main() {
    let instruction = parse::parse("Label: ADD FutureReference, 2, 3");
    println!("{:?}", instruction);
}
