use std::path::Path;
use std::fs::File;
use std::io::Read;

fn count(lines: &Vec<usize>, window_size: usize) -> usize {
    let mut count = 0;

    for window in lines.windows(window_size + 1) {
        let mut res = window.windows(window_size).map(|w| w.into_iter().sum::<usize>());
        let first = res.next().unwrap();
        let second = res.next().unwrap();

        if second > first {
            count += 1;
        }
    }

    count
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
        .map(|l| l.parse().expect("failed to parse int from line"))
        .collect();

    println!("part 1: {}", count(&lines, 1));
    println!("part 2: {}", count(&lines, 3));
}
