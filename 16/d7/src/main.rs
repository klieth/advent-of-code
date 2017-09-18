extern crate itertools;

use itertools::Itertools;

const SAMPLE1 : &'static str = "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";
const ANSWER1 : isize = 2;

fn load_input() -> String {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("input.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}", why.description()),
        Ok(_) => s,
    }
}

fn find_sequence(input: &str) -> bool {
    if input.len() < 4 { return false; }
    for i in 0..(input.len() - 3) {
        if input[i..i+1] == input[i+1..i+2] { continue; }
        if input[i..i+2] == input[i+2..i+4].chars().rev().collect::<String>() {
            return true;
        }
    }
    return false;
}

fn part_one(input: &str) -> isize {
    let mut count = 0;
    for line in input.lines() {
        let line = line.split(|c| c == '[' || c == ']').collect::<Vec<_>>();
        if line.iter().skip(1).step(2).all(|s| !find_sequence(s)) {
            if line.iter().step(2).any(|s| find_sequence(s)) {
                count += 1;
            }
        }
    }
    count
}

const SAMPLE2: &'static str = "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb";
const ANSWER2 : isize = 3;

struct ABAIter<'a> {
    chars: std::str::Chars<'a>,
    back1: char,
    back2: char,
}

impl<'a> ABAIter<'a> {
    fn new(input: &'a str) -> Self {
        let mut c = input.chars();
        let back2 = c.next().unwrap();
        let back1 = c.next().unwrap();
        ABAIter {
            chars: c,
            back2: back2,
            back1: back1,
        }
    }
}

impl<'a> Iterator for ABAIter<'a> {
    type Item = (char, char);
    fn next(&mut self) -> Option<(char, char)> {
        loop {
            if let Some(n) = self.chars.next() {
                let c = self.back2;
                self.back2 = self.back1;
                self.back1 = n;
                if c == n && c != self.back2 {
                    return Some( (c, self.back2) );
                }
            } else {
                return None;
            }
        }
        unreachable!();
    }
}


fn part_two(input: &str) -> isize {
    let mut count = 0;
    for line in input.lines() {
        let line = line.split(|c| c == '[' || c == ']').collect::<Vec<_>>();
        let mut aba = line.iter().step(2).map(|s| ABAIter::new(s)).flatten();
        if aba.any(|(a, b)| {
            let pat = format!("{}{}{}", b, a, b);
            line.iter().skip(1).step(2).any(|s| s.contains(&pat))
        }) {
            count += 1
        }
    }
    count
}

fn main() {
    let input = load_input();
    assert_eq!(part_one(SAMPLE1), ANSWER1);
    println!("part one: {}", part_one(&input));
    assert_eq!(part_two(SAMPLE2), ANSWER2);
    println!("part two: {}", part_two(&input));
}
