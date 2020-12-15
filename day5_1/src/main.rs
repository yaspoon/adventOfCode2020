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

    let lines: Vec<String> = contents.split("\n").filter(|s| s.len() > 1).map(|s| s.to_string()).collect();
    //let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    return lines;
}


fn main() {
    let input = read_input(Path::new("./input"));
    //let input = read_input(Path::new("./input_test"));

    let mut largest_seat_id = 0;
    let mut seats: Vec<Vec<bool>> = Vec::new();

    for i in 0..128 {
        seats.push(Vec::from([false; 8]));
    }

    for l in input {
        let mut current_row = 0;
        let mut current_col= 0;
        for c in l.chars() {
            match c {
                'F' => {
                    current_row = current_row * 2;
                },
                'B' => {
                    current_row = (current_row * 2) + 1;
                },
                'L' => {
                    current_col = current_col * 2;
                },
                'R' => {
                    current_col = (current_col * 2) + 1;
                },
                _ => (),
            }
        }

        seats[current_row][current_col] = true;
    }

    for (i, r) in seats.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if c == &false {
                let mut left = false;
                let mut right = false;
                let mut front = false;
                let mut back = false;

                if j > 0 {
                    if seats[i][j-1] == true {
                        left = true;
                    }
                } else {
                    left = true; //implicitly true as there is no "left"
                }

                if j < 7 {
                    if seats[i][j+1] == true {
                        right = true;
                    }
                } else {
                    right = true; //implicitly true as there is no "right"
                }

                if i > 0 {
                    if seats[i - 1][j] == true {
                        front = true;
                    }
                } else {
                    front = true; //implicitly true as there is no "front"
                }

                if i < 127 {
                    if seats[i+1][j] == true {
                        back = true;
                    }
                } else {
                    back = true; //implicity true as there is no "back"
                }

                if left && right && front && back {
                    println!("Found seat at seat_id:{}", i * 8 + j);
                }
            }
        }
    }

}
