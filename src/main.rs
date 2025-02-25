use std::env;
use std::fs::File;
use std::io::prelude::*;
mod convert;
mod preprocess;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Reading the file contents");

    let result = convert::convert(contents);
    println!("\n{}", result.expect("Unwrap result"));
}
