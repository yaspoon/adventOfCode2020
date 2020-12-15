use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn read_input(path: &Path) -> Vec<String> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open path:{}", e),
    };

    let mut br = BufReader::new(file);
    let mut contents = String::new();
    match br.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => println!("Failed to read in string:{}", e),
    }

    //let lines: Vec<String> = contents.split("\n").filter(|s| s.len() > 1).map(|s| s.to_string()).collect();
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    return lines;
}


fn main() {
    let input = read_input(Path::new("./input"));
    //let input = read_input(Path::new("./input_test"));

    for l in input {
        println!("{}", l);
    }
}
