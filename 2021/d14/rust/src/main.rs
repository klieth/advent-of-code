use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use std::collections::HashMap;

fn polymerize_alloc(init: &Vec<char>, insertions: &HashMap<(char, char), char>, steps: usize) -> usize {
    let mut polymer = init.clone();

    for _ in 0..steps {
        // unwrap: there will always be at least one character in the polymer, because `init` has at
        // least one character and we never remove characters.
        let (new_polymer, rest) = polymer.split_first().unwrap();

        let mut new_polymer = vec![*new_polymer];

        for b in rest {
            // unwrap: new_polymer is always initialized with at least a signle letter, and never
            // has letters removed.
            let a = new_polymer.last().unwrap();

            // unwrap: assuming that all possible insertion combinations are covered
            let insert = insertions.get( &(*a, *b) ).unwrap();

            new_polymer.push(*insert);
            new_polymer.push(*b);
        }

        polymer = new_polymer;
    }

    let mut chars = HashMap::new();

    for c in polymer {
        let e = chars.entry(c).or_insert(0);
        *e += 1
    }

    // unwrap: there will always be at least one character in the polymer, because `init` has at
    // least one character and we never remove characters.
    let min = chars.values().min().unwrap();
    let max = chars.values().max().unwrap();

    max - min
}

fn do_recurse(a: char, b: char, insertions: &HashMap<(char, char), char>, steps:  usize, appearances: &mut HashMap<char, usize>) {
    // unwrap: assuming that all possible insertion combinations are covered
    let c = *insertions.get( &(a, b) ).unwrap();

    let e = appearances.entry(c).or_insert(0);
    *e += 1;

    if steps > 0 {
        do_recurse(a, c, insertions, steps - 1, appearances);
        do_recurse(c, b, insertions, steps - 1, appearances);
    }
}

fn polymerize_recurse(init: &Vec<char>, insertions: &HashMap<(char, char), char>, steps: usize) -> usize {
    let mut appearances = HashMap::new();

    for c in init {
        let e = appearances.entry(*c).or_insert(0);
        *e += 1;
    }

    for window in init.windows(2) {
        do_recurse(window[0], window[1], insertions, steps - 1, &mut appearances);
    }

    // unwrap: there will always be at least one character in the polymer, because `init` has at
    // least one character and we never remove characters.
    let min = appearances.values().min().unwrap();
    let max = appearances.values().max().unwrap();

    max - min
}

fn polymerize_pair_count(init: &Vec<char>, insertions: &HashMap<(char, char), char>, steps: usize) -> usize {
    let mut pair_counts = HashMap::new();

    for window in init.windows(2) {
        *pair_counts.entry( (window[0], window[1]) ).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let old_pairs: Vec<_> = pair_counts.drain().collect();

        for ((a, b), count) in old_pairs {
            // unwrap: assuming that all possible insertion combinations are covered
            let c = *insertions.get( &(a, b) ).unwrap();
            *pair_counts.entry( (a, c) ).or_insert(0) += count;
            *pair_counts.entry( (c, b) ).or_insert(0) += count;
        }
    }

    let mut appearances = HashMap::new();

    for ((c, _), count) in pair_counts.iter() {
        let e = appearances.entry(*c).or_insert(0);
        *e += count;
    }

    // must add the last character manually, since it is never the beginning of any other pair.
    //
    // unwrap: there will always be at least one character in the polymer, because `init` has at
    // least one character and we never remove characters.
    *appearances.entry(*init.last().unwrap()).or_insert(0) += 1;

    // unwrap: there will always be at least one character in the polymer, because `init` has at
    // least one character and we never remove characters.
    let min = appearances.values().min().unwrap();
    let max = appearances.values().max().unwrap();

    max - min
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

    let mut lines: Vec<&str> = input.lines().map(|l| l.trim()).collect();

    let init = lines.drain(0..2).nth(0).expect("failed to read initial polymer").chars().collect();

    let insertions = lines.into_iter().map(|l| {
        let mut parts = l.split(" -> ");
        let mut from = parts.next().expect("failed to find 'from' side of reaction").chars();
        let to = parts.next().expect("failed to find 'to' side of reaction").chars().next().expect("'to' side of reaction was empty");
        ((from.next().expect("failed to get start of 'from'"), from.next().expect("failed to get end of 'from'")), to)
    }).collect();

    let now = Instant::now();
    println!("part 1: {}", polymerize_pair_count(&init, &insertions, 10));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);

    let now = Instant::now();
    println!("part 2: {}", polymerize_pair_count(&init, &insertions, 40));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);
}
