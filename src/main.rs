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

    // TODO also retrieve this name from cli
    let out_file = "test.mmo";
    if io::write_file(out_file, &final_result).is_err() {
        println!("Error when trying to write to file '{}'", out_file);
    }

    println!("{:#?}", final_result);
}
