#![feature(drain_filter)]

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

enum Direction {
    Up,
    Left,
}

impl Direction {
    fn apply(&self, (x, y): &(isize, isize), amt: isize) -> (isize, isize) {
        match self {
            Direction::Up => (*x, self.apply_internal(*y, amt)),
            Direction::Left => (self.apply_internal(*x, amt), *y),
        }
    }

    fn apply_internal(&self, v: isize, amt: isize) -> isize {
        if v > amt {
            amt - (v - amt)
        } else {
            v
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Direction {
        match s {
            "x" => Direction::Left,
            "y" => Direction::Up,
            _ => unreachable!("folds are only defined for x and y"),
        }
    }
}

fn fold(points: &[(isize, isize)], folds: &[(Direction, isize)], print: bool) -> usize {
    let mut points: std::collections::HashSet<_> = points.iter().map(|i| i.clone()).collect();

    for (dir, loc) in folds {
        points = points.iter().map(|p| dir.apply(p, *loc)).collect();
    }

    if print {
        // unwrap: assuming input data contains at least one entry
        for y in 0..=points.iter().max_by_key(|(_, y)| y).unwrap().1 {
            for x in 0..=points.iter().max_by_key(|(x, _)| x).unwrap().0 {
                if points.contains( &(x, y) ) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    points.len()
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

    let lines: Vec<&str> = input.lines().map(|l| l.trim()).collect();

    let spot = lines.binary_search_by(|x| {
        if x.contains(',') {
            std::cmp::Ordering::Less
        } else if x.contains('=') {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }).expect("couldn't find the blank line");
    let (points, folds) = lines.split_at(spot);

    let points = points.iter().map(|l| {
        let mut coords = l.split(',')
            .map(|i| i.parse::<isize>().expect("failed to parse coord number"));
        (coords.next().expect("failed to get first coord"), coords.next().expect("failed to get second coord"))
    }).collect::<Vec<_>>();

    let folds = folds[1..].iter().map(|l| {
        let word = l.split_whitespace().nth(2).expect("failed to get fold equation");
        let mut parts = word.split('=');

        (parts.next().expect("failed to get direction").into(), parts.next().expect("failed to get fold location").parse::<isize>().expect("failed to parse fold location"))
    }).collect::<Vec<_>>();

    let now = Instant::now();
    println!("part 1: {}", fold(&points, &folds[0..1], false));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);

    let now = Instant::now();
    println!("part 2: {}", fold(&points, &folds, true));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);
}
