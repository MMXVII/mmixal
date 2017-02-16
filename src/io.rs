use std::fs::File;
use std::io::{Error, BufReader, BufRead, Write};

pub fn read_file(filename: &str) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;
    BufReader::new(file).lines().collect()
}

pub fn write_file(filename: &str, binary: &[u8]) -> Result<(), Error> {
    File::create(filename)?.write_all(binary)
}
