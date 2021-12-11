extern crate itertools;

use itertools::Itertools;

const SAMPLE : (&'static str, usize) = ("10000", 20);
const ANSWER : &'static str = "01100";

const REAL : &'static str = "00101000101111010";

fn extend_data(input: &str, length: usize) -> String {
    let mut input = input.to_owned();
    while input.len() < length {
        let rev = input.chars().rev().map(|c| if c == '0' { '1' } else { '0' }).collect::<String>();
        input.push('0');
        input.push_str(&rev);
    }
    input.split_at(length).0.to_owned()
}

fn checksum(filled: &str) -> String {
    let mut checksum = filled.chars().tuples().map(|(a, b)| { if a == b { '1' } else { '0' } }).collect::<String>();
    while checksum.len() % 2 == 0 {
        checksum = checksum.chars().tuples().map(|(a, b)| { if a == b { '1' } else { '0' } }).collect::<String>();
    }
    checksum
}

fn part_one(input: &str, length: usize) -> String {
    let filled = extend_data(input, length);
    checksum(&filled)
}

fn main() {
    assert_eq!(&part_one(SAMPLE.0, SAMPLE.1), ANSWER);
    println!("part one: {}", part_one(REAL, 272));
}
