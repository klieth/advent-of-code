use std::path::Path;
use std::fs::File;
use std::io::Read;

fn power(lines: &Vec<Vec<usize>>) -> usize {
    let half_len = lines.len() / 2;

    let mut lines = lines.into_iter();

    // TODO: this clone should be unnecessary because we throw away this first item after it is
    // first read, but unfortunately the compiler doesn't know that.
    let first = lines.next().expect("lines was empty").clone();

    let totals = lines.fold(first, |acc, x| acc.iter().zip(x).map(|(a, b)| a + b).collect());

    let mut gamma = 0;
    let mut epsilon = 0;

    for digit in totals {
        gamma <<= 1;
        epsilon <<= 1;

        if digit > half_len {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

enum Rating {
    Oxygen,
    Co2,
}

impl Rating {
    fn get_filter(&self, idx: usize, zeroes: usize, ones: usize) -> impl FnMut(&Vec<usize>) -> bool {
        let match_digit = match self {
            Rating::Oxygen => if ones >= zeroes { 1 } else { 0 },
            Rating::Co2 => if ones < zeroes { 1 } else { 0 },
        };

        move |x| {
            x[idx] == match_digit
        }
    }
}

fn calculate(lines: Vec<Vec<usize>>, rating: Rating, idx: usize) -> usize {
    if lines.len() == 1 {
        let line = &lines[0];
        return line[1..].iter().fold(line[0], |acc, x| (acc << 1) + x)
    }

    let (zeroes, ones) = lines.iter().fold((0, 0), |(z, o), x| if x[idx] == 0 { (z + 1, o) } else { (z, o + 1) });

    // TODO: this could use drain_filter in the future when it's been stabilized for better
    // performance.
    let new_lines = lines.into_iter().filter(rating.get_filter(idx, zeroes, ones)).collect();

    calculate(new_lines, rating, idx + 1)
}

fn life_support(lines: &Vec<Vec<usize>>) -> usize {
    let oxygen = calculate(lines.clone(), Rating::Oxygen, 0);
    let co2 = calculate(lines.clone(), Rating::Co2, 0);

    oxygen * co2
}

fn main() {
    let mut args = std::env::args();

    let input_filename = args.nth(1).unwrap_or_else(|| {
        eprintln!("No input file specified");
        eprintln!("Usage: ./run <input filename>");
        std::process::exit(1);
    });

    let mut input_file = File::open(Path::new(&input_filename)).expect("failed to open input file");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("failed to read from file");

    let lines = input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(2).expect("failed to parse a digit") as usize).collect())
        .collect();

    println!("part 1: {}", power(&lines));
    println!("part 2: {}", life_support(&lines));
}
