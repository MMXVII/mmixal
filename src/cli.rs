use std::env;

pub fn get_filenames() -> Option<(String, String)> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return None;
    }
    Some((args[1].to_string(), args[2].to_string()))
}
