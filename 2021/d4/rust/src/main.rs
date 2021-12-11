use std::path::Path;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

fn calculate_score(board: &Vec<Vec<(usize, bool)>>) -> usize {
    let mut score = 0;

    for row in board {
        for (num, marked) in row {
            if !marked {
                score += num;
            }
        }
    }

    score
}

fn winning_board(board: &Vec<Vec<(usize, bool)>>) -> bool {
    let mut cols = vec![true; 5];

    for row in board {
        if row.iter().all(|(_, marked)| *marked) {
            return true;
        }

        for (ref mut col, (_, marked)) in cols.iter_mut().zip(row.iter()) {
            **col &= marked
        }
    }

    if cols.iter().any(|c| *c) {
        true
    } else {
        false
    }
}

fn bingo(calls: &Vec<usize>, boards: &Vec<Vec<Vec<(usize, bool)>>>) -> usize {
    let mut boards = boards.clone();

    for call in calls {
        // mark on all the boards
        for board in boards.iter_mut() {
            for row in board {
                for (num, ref mut marked) in row {
                    if num == call {
                        *marked = true
                    }
                }
            }
        }

        // check if any boards have won
        for board in boards.iter() {
            if winning_board(&board) {
                return calculate_score(&board) * call;
            }
        }
    }

    panic!("no winning boards");
}

fn lose_bingo(calls: &Vec<usize>, boards: &Vec<Vec<Vec<(usize, bool)>>>) -> usize {
    let mut boards = boards.clone();

    for call in calls {
        // mark on all the boards
        for board in boards.iter_mut() {
            for row in board {
                for (num, ref mut marked) in row {
                    if num == call {
                        *marked = true
                    }
                }
            }
        }

        if boards.len() == 1 {
            if winning_board(&boards[0]) {
                return calculate_score(&boards[0]) * call;
            }
        } else {
            // check if any boards have won
            boards = boards.into_iter().filter(|board| {
                !winning_board(&board)
            }).collect();
        }
    }

    panic!("no winning boards");
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

    let mut lines = input.lines();

    let calls = lines.next().expect("no line of calls")
        .split(',').map(|c| c.parse().expect("failed to parse a number"))
        .collect();
    let _ = lines.next().expect("no blank line");

    let boards: &Vec<Vec<Vec<(usize, bool)>>> = &lines.chunks(6).into_iter().map(|board| {
        board.take(5)
            .map(|l| l.split_ascii_whitespace().map(|n| (n.parse().expect("failed to parse board number"), false)).collect())
            .collect()
    }).collect();

    println!("part 1: {}", bingo(&calls, boards));
    println!("part 2: {}", lose_bingo(&calls, boards));
}
