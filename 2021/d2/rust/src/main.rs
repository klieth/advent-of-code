use std::path::Path;
use std::fs::File;
use std::io::Read;

fn locate1(lines: &Vec<&str>) -> (usize, usize) {
    let mut position = 0;
    let mut depth = 0;

    for line in lines {
        let mut split = line.split_ascii_whitespace();
        let direction = split.next().expect("no direction");
        let amount: usize = split.next().expect("no amount").parse().expect("amount couldn't be parsed");

        match direction {
            "forward" => position += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("unknown direction: {}", direction)
        }
    }

    (position, depth)
}

fn locate2(lines: &Vec<&str>) -> (usize, usize) {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let mut split = line.split_ascii_whitespace();
        let direction = split.next().expect("no direction");
        let amount: usize = split.next().expect("no amount").parse().expect("amount couldn't be parsed");

        match direction {
            "forward" => {
                position += amount;
                depth += amount * aim;
            },
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("unknown direction: {}", direction)
        }
    }

    (position, depth)
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
        .collect();

    let (position, depth) = locate1(&lines);

    println!("part 1: position: {}, depth: {} -- multiplied: {}", position, depth, position * depth);

    let (position, depth) = locate2(&lines);

    println!("part 2: position: {}, depth: {} -- multiplied: {}", position, depth, position * depth);
}
