use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MATCHES: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('(', ')');
        m.insert('[', ']');
        m.insert('{', '}');
        m.insert('<', '>');
        m
    };

    static ref CORRUPTED_SCORES: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert(')', 3);
        m.insert(']', 57);
        m.insert('}', 1197);
        m.insert('>', 25137);
        m
    };

    static ref INCOMPLETE_SCORES: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert(')', 1);
        m.insert(']', 2);
        m.insert('}', 3);
        m.insert('>', 4);
        m
    };
}

use std::collections::VecDeque;

fn corrupted(line: &mut VecDeque<char>) -> usize {
    if line.len() == 0 {
        return 0;
    }

    let open = line.pop_front().unwrap();

    loop {
        if let Some(peek) = line.get(0) {
            if MATCHES.keys().any(|k| k == peek) {
                let score = corrupted(line);
                if score != 0 { return score; }
            } else {
                break;
            }
        } else {
            return 0;
        }
    }

    line.pop_front()
        .map(|next| {
            if MATCHES[&open] == next {
                0
            } else {
                CORRUPTED_SCORES[&next]
            }
        })
        .unwrap_or(0)
}

fn corrupted_score(lines: &Vec<Vec<char>>) -> usize {
    lines.iter().fold(0, |acc, l| acc + corrupted(&mut l.clone().into()))
}

// Outer Option tracks whether this is incomplete (Some) or corrupted (None)
// Inner Option tracks whether we're backtracking completions (Some) or there's more left to parse (None)
fn incomplete(line: &mut VecDeque<char>) -> Option<Option<String>> {
    // unwrap: we never call this function anywhere that `line` could be empty
    let open = line.pop_front().unwrap();

    loop {
        if let Some(peek) = line.get(0) {
            if MATCHES.keys().any(|k| k == peek) {
                match incomplete(line) {
                    Some(Some(s)) => return Some(Some(format!("{}{}", s, MATCHES[&open]))),
                    Some(_) => {},
                    None => return None,
                }
            } else {
                break;
            }
        } else {
            return Some(Some(format!("{}", MATCHES[&open])));
        }
    }

    line.pop_front()
        .map(|next| {
            if MATCHES[&open] == next {
                Some(None)
            } else {
                None
            }
        })
        .unwrap_or_else(|| Some(Some(format!("{}", MATCHES[&open]))))
}

fn score(completion: String) -> usize {
    completion.chars().fold(0, |acc, c| {
        acc * 5 + INCOMPLETE_SCORES[&c]
    })
}

fn incomplete_score(lines: &Vec<Vec<char>>) -> usize {
    let mut scores = lines.iter()
        .filter_map(|l| {
            let l = &mut l.clone().into();

            loop {
                match incomplete(l) {
                    x @ Some(Some(_)) => return x,
                    None => return None,
                    Some(_) => continue,
                }
            }
        })
        .map(|c| if let Some(completion) = c { dbg!(score(completion)) } else { unreachable!() })
        .collect::<Vec<_>>();

    scores.sort();

    scores[dbg!(scores.len() / 2 /* + 1 for middle number, -1 for 0 index */)]
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
        .map(|l| l.chars().collect())
        .collect();

    let now = Instant::now();
    println!("part 1: {}", corrupted_score(&lines));
    println!("time: millis {}, nanos {}", now.elapsed().as_millis(), now.elapsed().as_nanos());

    let now = Instant::now();
    println!("part 2: {}", incomplete_score(&lines));
    println!("time: millis {}, nanos {}", now.elapsed().as_millis(), now.elapsed().as_nanos());
}
