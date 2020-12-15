use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct PasswordPolicy {
    min: usize,
    max: usize,
    character: String,
    password: String,
}

impl PasswordPolicy {
    fn new(line: String) -> PasswordPolicy {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() != 3 {
            panic!("Expected only 3 parts got {}", parts.len());
        }

        let minMax: Vec<usize> = parts[0].split("-").map(|s| s.parse::<usize>().unwrap()).collect();
        if minMax.len() != 2 {
            panic!("Expected only two minMaxs got {}", minMax.len());
        }
        return PasswordPolicy { min: minMax[0]-1, max: minMax[1]-1, character: parts[1].replace(":", ""), password: parts[2].to_string() };
    }
}

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
    return lines;
}

fn generate_password_policies(lines: Vec<String>) -> Vec<PasswordPolicy> {
    let mut passwordPolicies: Vec<PasswordPolicy> = Vec::new();

    for line in lines {
        passwordPolicies.push(PasswordPolicy::new(line));
    }

    return passwordPolicies;
}


fn main() {
    let input = read_input(Path::new("./input"));
    let passwordPolicies = generate_password_policies(input);

    let mut valid: usize = 0;
    for pp in passwordPolicies.into_iter() {
        if (pp.password[pp.min..pp.min+1] == pp.character && pp.password[pp.max..pp.max+1] != pp.character) || (pp.password[pp.min..pp.min+1] != pp.character && pp.password[pp.max..pp.max+1] == pp.character) {
            valid += 1;
        }
    }

    println!("valid:{}", valid);

    //input.into_iter().filter(|i| input.into_iter().filter(|j| i + j == 2020));

    /*
    for i in &input {
        for j in &input {
            for k in &input {
                if i + j + k == 2020 {
                    println!("i:{} j:{} k:{} answer:{}", i, j, k, i * j * k);
                }
            }
        }
    }
    */
}
