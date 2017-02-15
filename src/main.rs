extern crate regex;

mod passes;
mod is;
mod parse;
mod syntax;

fn main() {


    let lines = vec!["Label: ADDU FutureReference, 2, 3", "AnotherLabel: CMPU 1, 2, HOHOHO"];

    let intermediate = match passes::first_pass(&lines) {
        Ok(result) => result,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    println!("{:#?}", intermediate.symbol_table);
    println!("{:#?}", intermediate.parsed);
}
