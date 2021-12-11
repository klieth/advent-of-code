#![feature(test)]

use std::path::Path;
use std::fs::File;
use std::io::Read;

const DEBUG: bool = false;
use std::collections::VecDeque;

fn simulate(fish: &Vec<usize>, days: usize) -> usize {
    let mut fish: VecDeque<usize> = fish.clone().into();

    if DEBUG {
        println!("initial state: {:?}", fish);
    }

    for x in 0..days {
        if let Some(new_parents) = fish.pop_front() {
            fish.resize(9, 0);

            fish[6] += new_parents;
            fish[8] = new_parents;
        }

        if DEBUG {
            println!("after {} day(s): {:?}", x + 1, fish);
        }
    }

    fish.iter().sum()
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

    let fish = line.split(',')
        .map(|n| n.parse::<usize>().expect("couldn't read fish age"))
        .fold(vec![0; 9], |acc, x| {
            let mut r = acc.clone();
            r[x] += 1;
            r
        });

    println!("part 1: {}", simulate(&fish, 80));
    println!("part 2: {}", simulate(&fish, 256));
}

#[cfg(test)]
mod tests {
    extern crate test;
    use std::collections::VecDeque;

    use test::Bencher;

    const DATA: &'static str = include_str!("../../input.txt");
    const DAYS: usize = 80;

    #[bench]
    fn simulate_pop(b: &mut Bencher) {
        let mut fish: VecDeque<usize> = DATA.trim().split(',')
            .map(|n| dbg!(n).parse::<usize>().expect("couldn't read fish age"))
            .fold(vec![0; 9], |acc, x| {
                let mut r = acc.clone();
                r[x] += 1;
                r
            }).into();

        b.iter(|| {
            for x in 0..DAYS {
                if let Some(new_parents) = fish.pop_front() {
                    fish.resize(9, 0);

                    fish[6] += new_parents;
                    fish[8] = new_parents;
                }
            }
        });

        println!("part 1: {}", fish.iter().sum::<usize>());
    }

    #[bench]
    fn simulate_rotate(b: &mut Bencher) {
        let mut fish: VecDeque<usize> = DATA.trim().split(',')
            .map(|n| n.parse::<usize>().expect("couldn't read fish age"))
            .fold(vec![0; 9], |acc, x| {
                let mut r = acc.clone();
                r[x] += 1;
                r
            }).into();

        b.iter(|| {
            for x in 0..DAYS {
                fish.rotate_left(1);
                fish[6] += fish[8];
            }
        });

        println!("part 1: {}", fish.iter().sum::<usize>());
    }
}
