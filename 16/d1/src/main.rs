#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[derive(Debug)]
pub enum Turn {
    Right,
    Left,
}

impl Turn {
    fn do_turn(&self, start_dir: &(isize, isize)) -> (isize, isize) {
        match *self {
            Turn::Left => {
                match *start_dir {
                    (x, 0) => (0, x),
                    (0, x) => (-x, 0),
                    _ => panic!("no such dir, 16"),
                }
            }
            Turn::Right => {
                match *start_dir {
                    (0, x) => (x, 0),
                    (x, 0) => (0, -x),
                    _ => panic!("no such dir, 23"),
                }
            }
        }
    }
}

peg_file! directions("directions.rustpeg");

const TEST: [(&'static str, isize); 3] = [
    ("R2, L3", 5),
    ("R2, R2, R2", 2),
    ("R5, L5, R5, R3", 12),
];

const FINAL: &'static str = "R1, R1, R3, R1, R1, L2, R5, L2, R5, R1, R4, L2, R3, L3, R4, L5, R4, R4, R1, L5, L4, R5, R3, L1, R4, R3, L2, L1, R3, L4, R3, L2, R5, R190, R3, R5, L5, L1, R54, L3, L4, L1, R4, R1, R3, L1, L1, R2, L2, R2, R5, L3, R4, R76, L3, R4, R191, R5, R5, L5, L4, L5, L3, R1, R3, R2, L2, L2, L4, L5, L4, R5, R4, R4, R2, R3, R4, L3, L2, R5, R3, L2, L1, R2, L3, R2, L1, L1, R1, L3, R5, L5, L1, L2, R5, R3, L3, R3, R5, R2, R5, R5, L5, L5, R2, L3, L5, L2, L1, R2, R2, L2, R2, L3, L2, R3, L5, R4, L4, L5, R3, L4, R1, R3, R2, R4, L2, L3, R2, L5, R5, R4, L2, R4, L1, L3, L1, L3, R1, R2, R1, L5, R5, R3, L3, L3, L2, R4, R2, L5, L1, L1, L5, L4, L1, L1, R1";


fn get_distance(d: &str) -> isize {
    let tokens = directions::parse(d).unwrap();
    let mut loc = (0, 0);
    let mut dir = (0, 1);
    for (turn, dist) in tokens {
        println!("performing: {:?} {}", turn, dist);
        dir = turn.do_turn(&dir);
        match dir {
            (0, x) => loc = (loc.0, loc.1 + dist * x),
            (x, 0) => loc = (loc.0 + dist * x, loc.1),
            _ => panic!("no such dir, 50"),
        }
        println!("{:?} {:?}", dir, loc);
    }
    return loc.0.abs() + loc.1.abs()
}

fn main() {
    for &(s, a) in TEST.into_iter() {
        let answer = get_distance(s);
        println!("answer {}", answer);
        assert!(answer == a);
    }
    let answer = get_distance(FINAL);
    println!("Real test: {}", answer);
}
