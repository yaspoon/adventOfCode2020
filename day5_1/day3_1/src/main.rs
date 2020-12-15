use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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

    let lines: Vec<String> = contents.split("\n").filter(|s| s.len() > 1).map(|s| s.to_string()).collect();
    return lines;
}

fn main() {
    let input = read_input(Path::new("./input"));
    let toboggan_map = TabogganMap::new(input);

    let mut tree_count = 0;
    let mut x = 3;
    let mut y = 1;
    let mut square = toboggan_map.char_at(x, y);

    loop {
        match square {
            Some(c) => {
                if c == '#' {
                    tree_count += 1;
                }
            },
            None => break,
        }

        x += 3;
        y += 1;
        square = toboggan_map.char_at(x, y);
    }

    println!("tree_count:{}", tree_count);
}
