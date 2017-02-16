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

    let (in_file, out_file) = match cli::get_filenames() {
        Some(name) => name,
        None => {
            print!("Please pass the name");
            print!("of the file you want to assemble as the first parameter,");
            println!("and the desired name of the output file as the");
            println!("second parameter to the program.");
            return;
        }
    };

    let lines = match io::read_file(&in_file) {
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

    // println!("{:#?}", intermediate.symbol_table);
    // println!("{:#?}", intermediate.parsed);

    let final_result = match passes::second_pass(&intermediate) {
        Ok(result) => result,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    if io::write_file(&out_file, &final_result).is_err() {
        println!("Error when trying to write to file '{}'", out_file);
    }

    // println!("{:#?}", final_result);
}
