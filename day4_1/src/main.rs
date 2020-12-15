use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct TabogganMap {
    map: Vec<char>,
    width: usize,
    height: usize,
}

impl TabogganMap {
    fn new(lines: Vec<String>) -> TabogganMap {
        let mut map: Vec<char> = Vec::new();
        let width: usize = lines[0].chars().count();
        let mut height: usize = 0;

        for line in lines {
            map.append(&mut line.chars().collect());
            height += 1;
        }

        println!("width:{} height:{}", width, height);

        return TabogganMap { map: map, width: width, height: height};
    }

    fn char_at(self:&Self, x:usize, y:usize) -> Option<char> {

        let real_x = x % self.width;
        /*
        if (y * self.width) + real_x > self.map.len() {
            panic!("Trying to index outside of the map x:{} y:{} map.len:{}", real_x, y, self.map.len());
        }
        */

        if y > (self.height - 1) {
            return None;
        }

        let index = (y * self.width) + real_x;
        return Some(self.map[index]);
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

    //let lines: Vec<String> = contents.split("\n").filter(|s| s.len() > 1).map(|s| s.to_string()).collect();
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    return lines;
}


fn main() {
    let input = read_input(Path::new("./input"));

    let values = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut no_valid = 0;
    let mut valid_hash: u64 = 0;
    for v in &values {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        valid_hash = valid_hash.wrapping_add(hasher.finish());
    }

    let mut hash: u64 = 0;
    for line in input {
        if line.len() > 1 { //A single new line is the end of a passport input
            //hash.wrapping_add(line.split(" ").map(|s| s.split(":").map(|k| k.hash(&mut hasher))));
            for kv in line.split(" ") {
                //println!("kv:{}", kv);
                let parts: Vec<&str> = kv.split(":").collect();
                let key = parts[0];
                //println!("key:{}", key);
                if key != "cid" {
                    let mut hasher = DefaultHasher::new();
                    key.hash(&mut hasher);
                    hash = hash.wrapping_add(hasher.finish());
                }
            }
        } else {
            println!("hash:{:x} valid_hash:{:x}", hash, valid_hash);
            if hash == valid_hash {
                no_valid += 1;
            }
            hash = 0;
        }
        //println!("{}", line);
    }

    println!("no_valid:{}", no_valid);
}
