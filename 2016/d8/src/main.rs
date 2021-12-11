#![feature(plugin)]
#![plugin(peg_syntax_ext)]

fn load_input() -> String {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("input.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}", why.description()),
        Ok(_) => s,
    }
}

#[derive(Debug)]
pub enum Command {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

peg_file! commands("grammar.rustpeg");

const SAMPLE1 : &'static str = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1
";
const ANSWER1 : isize = 6;

fn rotate_vec<T: Clone>(v: Vec<T>, amt: usize) -> Vec<T> {
    let (a, b) = v.split_at(v.len() - amt);
    let mut spun = Vec::new();
    spun.extend_from_slice(b);
    spun.extend_from_slice(a);
    spun
}

fn part_one( (dim_x, dim_y) : (usize, usize), input : &str) -> isize {
    let mut board = Vec::new();
    for _ in 0..dim_x {
        let mut v = Vec::new();
        for _ in 0..dim_y {
            v.push(false);
        }
        board.push(v);
    }
    let cmds = commands::parse(input).unwrap();
    for cmd in cmds {
        match cmd {
            Command::Rect(x, y) => {
                for i in 0..x {
                    for j in 0..y {
                        board[i][j] = true;
                    }
                }
            },
            Command::RotateRow(row, amt) => {
                let r = board.iter().map(|i| i[row]).collect::<Vec<_>>();
                let spun = rotate_vec(r, amt);
                for (i, col) in board.iter_mut().enumerate() {
                    col[row] = spun[i];
                }
            },
            Command::RotateCol(col, amt) => {
                let removed = board.remove(col);
                let spun = rotate_vec(removed, amt);
                board.insert(col, spun);
            },
        }
    }
    println!("=======");
    for i in 0..dim_y {
        for j in 0..dim_x {
            print!("{}", if board[j][i] { '1' } else { ' ' });
        }
        println!();
    }
    board.iter().fold(0, |acc, i| acc + i.iter().fold(0, |acc2, &j| acc2 + if j { 1 } else { 0 } ))
}

fn main() {
    let input = load_input();
    assert_eq!(part_one((7, 3), SAMPLE1), ANSWER1);
    println!("part one: {}", part_one((50, 6), &input));
    println!("part two is the ASCII art above");
}
