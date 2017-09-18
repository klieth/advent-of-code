const SAMPLE1 : &'static str = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";
const ANSWER1 : isize = 42;

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
enum Token {
    Register(char),
    Integer(isize),
}

#[derive(Debug)]
enum Command {
    Cpy(Token, Token),
    Inc(Token),
    Dec(Token),
    Jnz(Token, Token),
}

fn parse_token(token: &str) -> Token {
    use Token::*;
    let c = token.chars().nth(0).unwrap();
    if c.is_numeric() || c == '-' {
        Integer(token.parse().unwrap())
    } else if c.is_alphabetic() {
        Register(c)
    } else {
        panic!("I don't recognize this token: {}", token)
    }
}

fn parse_command(command: &str) -> Command {
    use Command::*;
    let command = command.split_whitespace().collect::<Vec<_>>();
    match command[0] {
        "cpy" => Cpy(parse_token(command[1]), parse_token(command[2])),
        "inc" => Inc(parse_token(command[1])),
        "dec" => Dec(parse_token(command[1])),
        "jnz" => Jnz(parse_token(command[1]), parse_token(command[2])),
        c @ _ => panic!("Bad command: {}", c),
    }
}

fn part_one(input: &str) -> isize {
    use Command::*;
    use Token::*;
    use std::collections::HashMap;
    let text = input.lines().map(|l| parse_command(l)).collect::<Vec<_>>();
    let mut registers = "abcd".chars().map(|c| (c, if c == 'c' { 1 } else { 0 })).collect::<HashMap<_, _>>();
    let mut pc = 0;
    loop {
        if let Some(command) = text.get(pc as usize) {
            println!("executing: {:?}", command);
            match text[pc as usize] {
                Cpy(ref from, ref to) => {
                    let from = match *from {
                        Register(c) => *registers.get(&c).unwrap(),
                        Integer(i) => i,
                    };
                    match *to {
                        Register(c) => { registers.insert(c, from); },
                        Integer(_) => panic!("Illegal integer in cpy"),
                    }
                },
                Inc(ref reg) => {
                    match *reg {
                        Register(reg) => *registers.get_mut(&reg).unwrap() += 1,
                        Integer(_) => panic!("Illegal integer in inc"),
                    }
                },
                Dec(ref reg) => {
                    match *reg {
                        Register(reg) => *registers.get_mut(&reg).unwrap() -= 1,
                        Integer(_) => panic!("Illegal integer in inc"),
                    }
                },
                Jnz(ref test, ref amt) => {
                    let amt = match *amt {
                        Register(_) => panic!("Illegal register in jnz"),
                        Integer(i) => i,
                    };
                    match *test {
                        Register(c) => if *registers.get(&c).unwrap() != 0 { pc += amt - 1 },
                        Integer(i) => if i != 0 { pc += amt - 1 },
                    }
                },
            }
        } else {
            break;
        }
        pc += 1;
        println!("{:?}", registers);
    }
    *registers.get(&'a').unwrap()
}

fn main() {
    let input = load_input();
    assert_eq!(part_one(SAMPLE1), ANSWER1);
    println!("part one: {}", part_one(&input));
}
