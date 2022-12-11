use std::num::ParseIntError;
use std::str::FromStr;

const TEST: &'static str = include_str!("../../test");
const INPUT: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day10Error {
    ParseError(String),
}

impl From<ParseIntError> for Day10Error {
    fn from(error: ParseIntError) -> Self {
        Day10Error::ParseError(format!("{}", error))
    }
}

#[derive(Debug)]
enum Command {
    Addx(isize),
    Noop,
}

impl Command {
    fn cycles(&self) -> usize {
        match self {
            Command::Addx(_) => 2,
            Command::Noop => 1,
        }
    }

    fn apply(&self, state: &mut isize) {
        match self {
            Command::Addx(v) => *state += v,
            Command::Noop => {}
        }
    }
}

impl FromStr for Command {
    type Err = Day10Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("addx") {
            let (_, raw_num) = s.split_once(" ").expect("malformed addx");
            Ok(Command::Addx(raw_num.parse()?))
        } else if s.starts_with("noop") {
            Ok(Command::Noop)
        } else {
            unreachable!()
        }
    }
}

fn parse(raw: &str) -> Result<Vec<Command>, Day10Error> {
    let mut commands = vec![];

    for line in raw.lines() {
        commands.push(line.parse()?);
    }

    Ok(commands)
}

fn part1(commands: &[Command]) -> isize {
    let mut signal_sum = 0;

    let mut cycle = 1;
    let mut reg_x = 1;

    for command in commands {
        for _ in 0..command.cycles() {
            if (cycle - 20) % 40 == 0 {
                signal_sum += reg_x * cycle;
            }
            cycle += 1;
        }

        command.apply(&mut reg_x);
    }

    signal_sum
}

fn part2(commands: &[Command]) {
    let mut cycle = 1;
    let mut reg_x = 1;

    for command in commands {
        for _ in 0..command.cycles() {
            if reg_x >= (cycle % 40) - 2 && reg_x <= (cycle % 40) {
                print!("#");
            } else {
                print!(" ");
            }

            if cycle % 40 == 0 {
                println!();
            }

            cycle += 1;
        }

        command.apply(&mut reg_x);
    }
    println!()
}

fn main() -> Result<(), Day10Error> {
    let test = parse(TEST)?;
    let input = parse(INPUT)?;
    println!("part1 (test): {}", part1(&test));
    println!("part1 (actual): {}", part1(&input));
    part2(&test);
    part2(&input);
    Ok( () )
}
