#[macro_use]
extern crate lazy_static;
extern crate regex;

mod cli;
mod io;
mod is;
mod parse;
mod passes;
mod syntax;

fn main() {

    let filename = match cli::get_filename() {
        Some(name) => name,
        None => {
            println!("Please pass the name of the file you want to assemble as a parameter");
            return;
        }
    };

    let lines = match io::read_file(&filename) {
        Ok(lines) => lines,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };


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
