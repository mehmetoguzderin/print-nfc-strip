use std::env;
use std::fs;

use unicode_normalization::UnicodeNormalization;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("No file supplied");
        return;
    }
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let normalized_contents = contents.nfc().collect::<String>();
    println!("With normalized text:\n{}", normalized_contents);
}
