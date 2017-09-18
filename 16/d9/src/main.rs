
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

const TEST1 : [(&'static str, usize); 6] = [
    ("ADVENT", 6),
    ("A(1x5)BC", 7),
    ("(3x3)XYZ", 9),
    ("A(2x2)BCD(2x2)EFG", 11),
    ("(6x1)(1x3)A", 6),
    ("X(8x2)(3x3)ABCY", 18),
];

fn part_one(input: &str) -> usize {
    let mut chars = input.chars();
    let mut out = String::new();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let mut tmp = String::new();
                while let Some(c) = chars.next() {
                    if c == ')' {
                        break;
                    } else {
                        tmp.push(c);
                    }
                }
                println!("captured: {}", tmp);
                let mut n = tmp.split('x').map(|s| s.parse().unwrap());
                let run = n.next().unwrap();
                let times = n.next().unwrap();
                let mut tmp = String::new();
                for _ in 0..run {
                    tmp.push(chars.next().unwrap());
                }
                println!("writing {} times: {}", times, tmp);
                for _ in 0..times {
                    out.push_str(&tmp);
                }
            },
            'A'...'Z' => {
                out.push(c);
            },
            _ => {},
        }
        println!("out: {}", out);
    }
    out.len()
}

const TEST2 : [(&'static str, usize); 4] = [
    ("(3x3)XYZ", 9),
    ("X(8x2)(3x3)ABCY", 20),
    ("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920),
    ("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 445),
];

fn part_two(input: &str) -> usize {
    let mut chars = input.chars();
    let mut out = 0;
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let mut tmp = String::new();
                while let Some(c) = chars.next() {
                    if c == ')' {
                        break;
                    } else {
                        tmp.push(c);
                    }
                }
                let mut n = tmp.split('x').map(|s| s.parse().unwrap());
                let run = n.next().unwrap();
                let times = n.next().unwrap();
                let mut tmp = String::new();
                for _ in 0..run {
                    tmp.push(chars.next().unwrap());
                }
                let num = if tmp.contains('(') {
                    part_two(&tmp)
                } else {
                    tmp.len()
                };
                out += num * times;
            },
            'A'...'Z' => out += 1,
            _ => {},
        }
    }
    out
}

fn main() {
    let input = load_input();
    for &(i, o) in TEST1.iter() {
        assert_eq!(part_one(i), o);
    }
    println!("part one: {}", part_one(&input));
    for &(i, o) in TEST2.iter() {
        assert_eq!(part_two(i), o);
    }
    println!("part two: {}", part_two(&input));
}
