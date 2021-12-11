extern crate regex;

use regex::Regex;

const SAMPLE : &'static str = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";
const ANSWER : usize = 5;

const REAL1 : &'static str = "Disc #1 has 17 positions; at time=0, it is at position 1.
Disc #2 has 7 positions; at time=0, it is at position 0.
Disc #3 has 19 positions; at time=0, it is at position 2.
Disc #4 has 5 positions; at time=0, it is at position 0.
Disc #5 has 3 positions; at time=0, it is at position 0.
Disc #6 has 13 positions; at time=0, it is at position 5.";

const REAL2 : &'static str = "Disc #1 has 17 positions; at time=0, it is at position 1.
Disc #2 has 7 positions; at time=0, it is at position 0.
Disc #3 has 19 positions; at time=0, it is at position 2.
Disc #4 has 5 positions; at time=0, it is at position 0.
Disc #5 has 3 positions; at time=0, it is at position 0.
Disc #6 has 13 positions; at time=0, it is at position 5.
Disc #7 has 11 positions; at time=0, it is at position 0.";

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"Disc #(\d) has (\d+) positions; at time=0, it is at position (\d+)").unwrap();
    input.lines().map(|line| match re.captures(line) {
        Some(cap) => {
            (cap.at(2).unwrap().parse().unwrap(), cap.at(3).unwrap().parse().unwrap())
        },
        None => panic!("line didn't match: {}", line),
    }).collect()
}

fn part_one(input: Vec<(usize, usize)>) -> usize {
    for time in 0.. {
        if input.iter().zip(time+1..).all(|(&(positions, start), time)| ((start + time) % positions) == 0) {
            return time;
        }
    }
    unreachable!();
}

fn main() {
    assert_eq!(part_one(parse_input(SAMPLE)), ANSWER);
    println!("part one: {}", part_one(parse_input(REAL1)));
    println!("part two: {}", part_one(parse_input(REAL2)));
}
