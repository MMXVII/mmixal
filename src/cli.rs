use std::env;

pub fn get_filename() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    args.get(1).map(|name| name.to_string())
}
