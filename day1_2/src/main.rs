use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn read_input(path: &Path) -> Vec<i32> {
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

    let lines: Vec<i32> = contents.split("\n").filter_map(|s| s.parse::<i32>().ok()).collect();

    return lines;
}

fn main() {
    let input = read_input(Path::new("./input"));

    //input.into_iter().filter(|i| input.into_iter().filter(|j| i + j == 2020));

    for i in &input {
        for j in &input {
            for k in &input {
                if i + j + k == 2020 {
                    println!("i:{} j:{} k:{} answer:{}", i, j, k, i * j * k);
                }
            }
        }
    }
}
