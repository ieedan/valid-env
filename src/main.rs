use vnv::parsing::parse;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let path = env::args().nth(1).expect("Must provide a file path.");

    let path = Path::new(&path.trim()).canonicalize().expect("Must provide valid path.");

    let file_contents = fs::read(path).expect("Error reading file");

    let content = String::from_utf8(file_contents).unwrap();

    let result = parse(&content);

    dbg!(result);
}
