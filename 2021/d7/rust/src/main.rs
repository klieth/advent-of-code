#![feature(int_abs_diff)]

use std::path::Path;
use std::fs::File;
use std::io::Read;

fn linear_fuel_required(positions: &Vec<usize>, position: usize) -> usize {
    positions.iter().fold(0, |acc, x| acc + x.abs_diff(position))
}

fn triangular_fuel_required(positions: &Vec<usize>, position: usize) -> usize {
    positions.iter().fold(0, |acc, x| {
        let dist = x.abs_diff(position);
        acc + ((dist + 1) * dist) / 2 
    })
}

fn best_position(positions: &Vec<usize>, fuel_required: fn(&Vec<usize>, usize) -> usize) -> usize {
    let (min, max) = positions.iter().fold((usize::MAX, 0), |acc, x| {
        let (min, max) = acc;
        (min.min(*x), max.max(*x))
    });

    (min..=max).min_by_key(|x| fuel_required(positions, *x))
        .map(|x| fuel_required(positions, x))
        .expect("iterator guaranteed non-empty, unreachable") // iterator is guaranteed non-empty, so this is safe.
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

    let line = input.lines().next().expect("couldn't retreive data from file");

    let positions = line.split(',')
        .map(|n| n.parse().expect("couldn't read fish age"))
        .collect();

    println!("part 1: {}", best_position(&positions, linear_fuel_required));
    println!("part 2: {}", best_position(&positions, triangular_fuel_required));
}
