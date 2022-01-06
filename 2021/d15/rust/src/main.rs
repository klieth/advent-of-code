use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

// ===== RECURSIVE =====
// While this implementation does seem to work with test data, it causes a stack overflow on the
// actual input.
fn risk(map: &Vec<Vec<usize>>, x: usize, y: usize, visited: Vec<(usize, usize)>, best: usize) -> Option<usize> {
    if x == map.len() - 1 && y == map[x].len() - 1 {
        return Some(map[x][y]);
    }

    if map[x][y] >= best {
        return None;
    }

    let neighbors = [
        (x.checked_add(1), Some(y)),
        (Some(x), y.checked_add(1)),
        (x.checked_sub(1), Some(y)),
        (Some(x), y.checked_sub(1)),
    ];

    let neighbors = neighbors.into_iter().filter_map(|coords| {
        if let (Some(x), Some(y)) = coords {
            if x < map.len() && y < map[x].len() && !visited.contains( &(x, y) ) {
                Some( (x, y) )
            } else {
                None
            }
        } else {
            None
        }
    });

    let risk = neighbors.fold(best - map[x][y], |acc, (x, y)| {
        let mut visited = visited.clone();
        visited.push((x, y));
        if let Some(risk) = risk(map, x, y, visited, acc) {
            risk
        } else {
            acc
        }
    }) + map[x][y];

    if risk < best {
        Some(risk)
    } else {
        None
    }
}

fn lowest_risk_recursive(map: &Vec<Vec<usize>>) -> usize {
    if let Some(risk) = risk(map, 0, 0, vec![(0, 0)], usize::MAX) {
        risk - map[0][0]
    } else {
        panic!("no risk calculation found; should be unreachable")
    }
}
// =====================

use std::collections::BinaryHeap;
use std::collections::HashSet;

use std::cmp::{
    Ordering,
    Reverse,
};

struct Loc {
    risk: usize,
    path_risk: Option<usize>,
    prev: Option<(usize, usize)>,
}

struct Visit {
    x: usize,
    y: usize,
    path_risk: usize,
}

impl PartialEq for Visit {
    fn eq(&self, other: &Visit) -> bool {
        self.path_risk.eq(&other.path_risk)
    }
}

impl Eq for Visit {}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Visit) -> Option<Ordering> {
        Reverse(self.path_risk).partial_cmp(&Reverse(other.path_risk))
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Visit) -> Ordering {
        Reverse(self.path_risk).cmp(&Reverse(other.path_risk))
    }
}

fn lowest_risk_dijkstra(map: &Vec<Vec<usize>>) -> usize {
    let mut map: Vec<Vec<Loc>> = map.iter()
        .map(|row| row.iter().map(|risk| Loc { risk: *risk, path_risk: None, prev: None } ).collect())
        .collect();

    let x_max = map.len() - 1;
    let y_max = map[map.len() - 1].len() - 1;

    let mut to_visit = BinaryHeap::new();
    let mut visited = HashSet::new();

    to_visit.push(Visit { x: 0, y: 0, path_risk: 0 });
    map[0][0].path_risk = Some(0);

    while to_visit.len() > 0 {
        // unwrap: we know this has values because otherwise the loop would have quit
        let Visit { x, y, .. } = to_visit.pop().unwrap();

        // if we have already visited the node, this will return false and we will continue to the
        // next node.
        if !visited.insert( (x, y) ) { continue; }

        let Loc { path_risk, .. } = map[x][y];
        let path_risk = match path_risk {
            Some(r) => r,
            None => unreachable!("path_risk will always be set if the node has been added to to_visit"),
        };

        let neighbors = [
            (x.checked_add(1), Some(y)),
            (Some(x), y.checked_add(1)),
            (x.checked_sub(1), Some(y)),
            (Some(x), y.checked_sub(1)),
        ];

        let neighbors = neighbors.into_iter().filter_map(|coords| {
            if let (Some(x), Some(y)) = coords {
                if x <= x_max && y <= y_max && !visited.contains( &(x, y) ) {
                    Some( (x, y) )
                } else {
                    None
                }
            } else {
                None
            }
        });

        for (nx, ny) in neighbors {
            let alt = path_risk + map[nx][ny].risk;

            if map[nx][ny].path_risk.map(|r| alt < r).unwrap_or(true) {
                map[nx][ny].path_risk = Some(alt);
                map[nx][ny].prev = Some( (x, y) );

                to_visit.push(Visit { x: nx, y: ny, path_risk: alt })
            }
        }
    }

    // unwrap: all path_risks will be filled in a properly implemented dijkstra's
    map[x_max][y_max].path_risk.unwrap()
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

    let map: Vec<Vec<usize>> = input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).expect("failed to parse a digit") as usize).collect())
        .collect();

    let now = Instant::now();
    println!("part 1: {}", lowest_risk_dijkstra(&map));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);

    let mut larger_prep = Vec::new();

    for _ in map.iter() {
        larger_prep.push(Vec::new());
    }

    for i in 0..5 {
        for (row, larger_row) in map.iter().zip(larger_prep.iter_mut()) {
            larger_row.extend(row.iter().map(|n| (n + i - 1) % 9 + 1));
        }
    }

    let mut larger_map: Vec<Vec<usize>> = Vec::new();

    for i in 0..5 {
        for row in larger_prep.iter() {
            larger_map.push(row.iter().map(|n| (n + i - 1) % 9 + 1).collect());
        }
    }

    let now = Instant::now();
    println!("part 2: {}", lowest_risk_dijkstra(&larger_map));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);
}
