
use std::fs::File;
use std::io::prelude::*;

fn read_file(filename: String) -> std::io::Result<String> {
    let mut f = try!(File::open(filename));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    return Ok(s);
}

fn part_one(string: &String) -> isize {
    string.chars().fold(0, |a, i| {
        match i {
            '(' => a + 1,
            ')' => a - 1,
            _ => panic!("illegal character: {}", i)
        }
    })
}

fn part_two(string: &String) -> isize {
    0
}

fn main() {
    let filename = match std::env::args().nth(1) {
        Some(f) => f,
        None => panic!("Must specify a file to read in from")
    };
    let file = match read_file(filename) {
        Ok(f) => f,
        Err(e) => panic!("Couldn't read file: {}", e)
    };
    println!("Part one: {}", part_one(&file));
    println!("Part two: {}", part_two(&file));
}
