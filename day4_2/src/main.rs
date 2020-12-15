use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use regex::Regex;

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

    let values = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut no_valid = 0;
    let hcl_re = Regex::new(r"#(\d|[[:alpha:]]){6}").unwrap();
    let pid_re = Regex::new(r"\d{9}").unwrap();

    let mut current_valid: u64 = 0;
    for line in input {
        if line.len() > 1 { //A single new line is the end of a passport input
            //hash.wrapping_add(line.split(" ").map(|s| s.split(":").map(|k| k.hash(&mut hasher))));
            for kv in line.split(" ") {
                let parts: Vec<&str> = kv.split(":").collect();
                let key = parts[0];
                let value = parts[1];
                //println!("key:{}", key);
                if key != "cid" {
                    match key {
                        "byr" => {
                            let year = value.parse::<i32>().unwrap();
                            if 1920 <= year && year <= 2002 {
                                //println!("valid birth year:{}", year);
                                current_valid += 1;
                            }
                        },
                        "iyr" => {
                            let year = value.parse::<i32>().unwrap();
                            if 2010 <= year && year <= 2020 {
                                //println!("valid issue year:{}", year);
                                current_valid += 1;
                            }
                        }
                        "eyr" => {
                            let year = value.parse::<i32>().unwrap();
                            if 2020 <= year && year <= 2030 {
                                //println!("valid exp year:{}", year);
                                current_valid += 1;
                            }
                        },
                        "hgt" => {
                            let isCm = value.find("cm");
                            if isCm != None {
                                let index = isCm.unwrap();
                                let num_str: &str = &value[0..index];
                                let num = num_str.parse::<i32>().unwrap();
                                if 150 <= num && num <= 193 {
                                    current_valid += 1;
                                }
                            } else {
                                let isIn = value.find("in");
                                if isIn != None {
                                    let index = isIn.unwrap();
                                    let num_str: &str = &value[0..index];
                                    let num = num_str.parse::<i32>().unwrap();
                                    if 59 <= num && num <= 76 {
                                        current_valid += 1;
                                    }
                                }
                            }
                        },
                        "hcl" => {
                            if hcl_re.is_match(value) {
                                current_valid += 1;
                            }
                        },
                        "ecl" => {
                            match value {
                                "amb" => current_valid += 1,
                                "blu" => current_valid += 1,
                                "brn" => current_valid += 1,
                                "gry" => current_valid += 1,
                                "grn" => current_valid += 1,
                                "hzl" => current_valid += 1,
                                "oth" => current_valid += 1,
                                &_ => (),
                            }
                        },
                        "pid" => {
                            if value.len() == 9 && pid_re.is_match(value) {
                                println!("valid pid:{}", value);
                                current_valid += 1;
                            }
                        },
                        &_ => (),
                    }
                }
            }
        } else {
            if current_valid == 7 {
                no_valid += 1;
            }
            //println!("current_valid:{:x} no_valid:{:x}", current_valid, no_valid);
            current_valid = 0;
        }
        //println!("{}", line);
    }

    println!("no_valid:{}", no_valid);
}
