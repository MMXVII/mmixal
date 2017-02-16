extern crate regex;

mod passes;
mod is;
mod parse;
mod syntax;

fn main() {


    let lines = vec!["Label: ADDU Label, 2, 3", "AnotherLabel: CMPU 1, 2, 3"];

    let intermediate = match passes::first_pass(&lines) {
        Ok(result) => result,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    println!("{:#?}", intermediate.symbol_table);
    println!("{:#?}", intermediate.parsed);

    let final_result = match passes::second_pass(&intermediate) {
        Ok(result) => result,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    println!("{:#?}", final_result);
}
