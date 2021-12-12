use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn flashes(octos: &Vec<Vec<Option<usize>>>) -> usize {
    let mut octos = octos.clone();

    let mut flashes = 0;

    for _ in 0..100 {
        // increase each energy level
        for row in octos.iter_mut() {
            for o in row {
                if let Some(o) = o {
                    *o += 1;
                } else {
                    unreachable!("no octos have flashed yet, so they should all be collecting energy");
                }
            }
        }

        // while there are new flashes:
        'flashes: loop {
            for x in 0..octos.len() {
                for y in 0..octos[x].len() {
                    if let Some(octo) = octos[x][y] {
                        // flash any above 9 that have not yet flashed
                        if octo > 9 {
                            octos[x][y].take();
                            // count flashed octos and add to `flashes`
                            flashes += 1;

                            // increment their neighbors
                            let neighbors = [
                                (x.checked_sub(1), y.checked_sub(1)),
                                (x.checked_sub(1), Some(y)),
                                (x.checked_sub(1), y.checked_add(1)),
                                (Some(x), y.checked_sub(1)),
                                (Some(x), y.checked_add(1)),
                                (x.checked_add(1), y.checked_sub(1)),
                                (x.checked_add(1), Some(y)),
                                (x.checked_add(1), y.checked_add(1)),
                            ];

                            for (nx, ny) in neighbors.into_iter().filter_map(|(nx, ny)| nx.zip(ny)) {
                                if nx >= 10 || ny >= 10 { continue; }
                                if let Some(ref mut o) = octos[nx][ny] {
                                    *o += 1;
                                }
                            }

                            continue 'flashes;
                        }
                    }
                }
            }

            break;
        }

        // reset any flashed octos to Some(0)
        for row in octos.iter_mut() {
            for o in row {
                if let None = o {
                    let _ = o.insert(0);
                }
            }
        }
    }

    flashes
}

fn sync(octos: &Vec<Vec<Option<usize>>>) -> usize {
    let mut octos = octos.clone();

    // safety: the input is guaranteed not to have a cycle
    for step in 1.. {
        // increase each energy level
        for row in octos.iter_mut() {
            for o in row {
                if let Some(o) = o {
                    *o += 1;
                } else {
                    unreachable!("no octos have flashed yet, so they should all be collecting energy");
                }
            }
        }

        // while there are new flashes:
        'flashes: loop {
            for x in 0..octos.len() {
                for y in 0..octos[x].len() {
                    if let Some(octo) = octos[x][y] {
                        // flash any above 9 that have not yet flashed
                        if octo > 9 {
                            octos[x][y].take();

                            // increment their neighbors
                            let neighbors = [
                                (x.checked_sub(1), y.checked_sub(1)),
                                (x.checked_sub(1), Some(y)),
                                (x.checked_sub(1), y.checked_add(1)),
                                (Some(x), y.checked_sub(1)),
                                (Some(x), y.checked_add(1)),
                                (x.checked_add(1), y.checked_sub(1)),
                                (x.checked_add(1), Some(y)),
                                (x.checked_add(1), y.checked_add(1)),
                            ];

                            for (nx, ny) in neighbors.into_iter().filter_map(|(nx, ny)| nx.zip(ny)) {
                                // ok, fine, hard-code the dimensions :(
                                if nx >= 10 || ny >= 10 { continue; }
                                if let Some(ref mut o) = octos[nx][ny] {
                                    *o += 1;
                                }
                            }

                            continue 'flashes;
                        }
                    }
                }
            }

            break;
        }

        if octos.iter().all(|row| row.iter().all(|octo| octo.is_none())) {
            return step;
        }

        // reset any flashed octos to Some(0)
        for row in octos.iter_mut() {
            for o in row {
                if let None = o {
                    let _ = o.insert(0);
                }
            }
        }
    }

    unreachable!()
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

    let octos = input.lines()
        .map(|l| l.chars().map(|c| (c.to_digit(10).unwrap() as usize).into()).collect())
        .collect();

    let now = Instant::now();
    println!("part 1: {}", flashes(&octos));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);

    let now = Instant::now();
    println!("part 2: {}", sync(&octos));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);
}
