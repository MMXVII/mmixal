use std::fs::File;
use std::io::{Error, BufReader, BufRead};

pub fn read_file(filename: &str) -> Result<Vec<String>, Error> {
    let file = File::open(&filename)?;
    BufReader::new(file).lines().collect()
}
