use std::path::Path;
use std::fs::File;
use std::io::Read;

fn low_point_risk(map: &Vec<Vec<usize>>) -> usize {
    let mut risk = 0;

    for x in 0..map.len() {
        let row = &map[x];

        for y in 0..row.len() {
            let loc = map[x][y];

            let neighbors = [
                (x.checked_sub(1), Some(y)),
                (x.checked_add(1), Some(y)),
                (Some(x), y.checked_sub(1)),
                (Some(x), y.checked_add(1)),
            ];

            let is_low_point = neighbors.iter()
                .filter_map(|(x, y)| {
                    match x {
                        Some(x) if *x < map.len() => {
                            match y {
                                Some(y) if *y < row.len() => Some( (x, y) ),
                                _ => None,
                            }
                        },
                        _ => None,
                    }
                }).all(|(x, y)| {
                    loc < map[*x][*y]
                });

            if is_low_point {
                risk += loc + 1;
            }
        }
    }

    risk
}

use std::collections::VecDeque;
fn mark_basin(map: &mut Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    let mut size = 0;

    let mut q = VecDeque::new();
    q.push_back( (x, y) );

    while q.len() > 0 {
        let (x, y) = q.pop_front().unwrap();
        if map[x][y] > 8 { continue; }

        size += 1;
        map[x][y] = 10;

        let neighbors = [
            (x.checked_sub(1), Some(y)),
            (x.checked_add(1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (Some(x), y.checked_add(1)),
        ];

        q.extend(neighbors.iter()
            .filter_map(|(x, y)| {
                match x {
                    Some(x) if *x < map.len() => {
                        match y {
                            Some(y) if *y < map[0].len() => Some( (*x, *y) ),
                            _ => None,
                        }
                    },
                    _ => None,
                }
            }));
    }

    size
}

fn largest_basins(map: &Vec<Vec<usize>>) -> usize {
    let mut map = map.clone();
    let mut basin_sizes = Vec::new();

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] < 9 {
                basin_sizes.push(mark_basin(&mut map, x, y));
            }
        }
    }

    /*
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            print!("{}", if map[x][y] == 10 { '.' } else { '9' });
        }
        println!();
    }
    */

    basin_sizes.sort();

    basin_sizes.iter().rev().take(3).fold(1, |acc, x| acc * x)
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

    let map = input.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    println!("part 1: {}", low_point_risk(&map));
    println!("part 2: {}", largest_basins(&map));
}
