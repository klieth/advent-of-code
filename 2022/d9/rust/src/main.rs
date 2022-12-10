use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

const TEST: &'static str = include_str!("../../test");
const TEST2: &'static str = include_str!("../../test2");
const INPUT: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day9Error {
    ParseError(String),
}

impl From<ParseIntError> for Day9Error {
    fn from(error: ParseIntError) -> Self {
        Day9Error::ParseError(format!("{}", error))
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Right => (1, 0),
            Direction::Left  => (-1, 0),
            Direction::Up    => (0, 1),
            Direction::Down  => (0, -1),
        }
    }
}

impl FromStr for Direction {
    type Err = Day9Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            x => Err(Day9Error::ParseError(format!("unknown direction: {}", x))),
        }
    }
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    dist: usize,
}

impl FromStr for Command {
    type Err = Day9Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_dir, raw_dist) = s.split_once(" ")
            .ok_or_else(|| Day9Error::ParseError(format!("failed to split line on space character")))?;

        let dir = raw_dir.parse()?;
        let dist = raw_dist.parse()?;

        Ok(Command {
            dir, dist,
        })
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new() -> Self {
        Position {
            x: 0,
            y: 0,
        }
    }

    fn step(&mut self, d: &Direction) {
        let (dx, dy) = d.delta();
        self.x += dx;
        self.y += dy;
    }

    fn chase(&mut self, followee: &Position) {
        let (dx, dy) = (followee.x - self.x, followee.y - self.y);

        if dx > 1 || dx < -1 {
            self.x += dx / dx.abs();
            if dy != 0 {
                self.y += dy / dy.abs();
            }
        } else if dy > 1 || dy < -1 {
            self.y += dy / dy.abs();
            if dx != 0 {
                self.x += dx / dx.abs();
            }
        }
    }
}

fn parse(raw: &str) -> Result<Vec<Command>, Day9Error> {
    // requires nightly
    // raw.lines().map(|m| m.parse()).try_collect()

    let mut commands = vec![];

    for line in raw.lines() {
        commands.push(line.parse()?);
    }

    Ok(commands)
}

/*
fn display_locations(rope: &[Position]) {
    let rx = rope.iter().map(|p| p.x).min().unwrap().min(0)..=rope.iter().map(|p| p.x).max().unwrap().max(0);
    let ry = rope.iter().map(|p| p.y).min().unwrap().min(0)..=rope.iter().map(|p| p.y).max().unwrap().max(0);

    for y in ry.rev() {
        for x in rx.clone() {
            if let Some(idx) = rope.iter().position(|p| p.x == x && p.y == y) {
                if idx == 0 {
                    print!("H");
                } else {
                    print!("{}", idx);
                }
            } else if x == 0 && y == 0 {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
*/

fn pull(commands: &[Command], rope_length: usize) -> usize {
    let mut positions = vec![Position::new(); rope_length];

    let mut tail_visited = HashSet::new();

    for command in commands {
        for _ in 0..command.dist {
            positions[0].step(&command.dir);
            for pos in 1..rope_length {
                let followee = &positions[pos - 1].clone();
                positions[pos].chase(&followee);
            }
            tail_visited.insert(positions[rope_length - 1].clone());
        }

        /*
        println!("{:?}", command);
        display_locations(&positions);
        println!("visited: {}", tail_visited.len());
        println!();
        */
    }

    tail_visited.len()
}

fn main() -> Result<(), Day9Error> {
    let test = parse(TEST)?;
    let test2 = parse(TEST2)?;
    let input = parse(INPUT)?;
    println!("part1 (test): {}", pull(&test, 2));
    println!("part1 (actual): {}", pull(&input, 2));
    println!("part2 (test): {}", pull(&test2, 10));
    println!("part2 (actual): {}", pull(&input, 10));
    Ok( () )
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! gen_test {
        ($name:ident, $hs:expr, $ts:expr, $dir:expr, $he:expr, $te:expr) => {
            #[test]
            fn $name() {
                let mut head = $hs;
                let mut tail = $ts;
                head.step(&$dir);
                tail.chase(&head);

                assert_eq!(head, $he, "head");
                assert_eq!(tail, $te, "tail");
            }
        }
    }

    gen_test!(tl_l, Position { x: 0, y: 2 }, Position { x: 1, y: 1 }, Direction::Left , Position { x: -1, y:  2 }, Position { x: 0, y: 2 });
    gen_test!(tl_r, Position { x: 0, y: 2 }, Position { x: 1, y: 1 }, Direction::Right, Position { x:  1, y:  2 }, Position { x: 1, y: 1 });
    gen_test!(tl_u, Position { x: 0, y: 2 }, Position { x: 1, y: 1 }, Direction::Up   , Position { x:  0, y:  3 }, Position { x: 0, y: 2 });
    gen_test!(tl_d, Position { x: 0, y: 2 }, Position { x: 1, y: 1 }, Direction::Down , Position { x:  0, y:  1 }, Position { x: 1, y: 1 });

    gen_test!(tc_l, Position { x: 1, y: 2 }, Position { x: 1, y: 1 }, Direction::Left , Position { x:  0, y:  2 }, Position { x: 1, y: 1 });
    gen_test!(tc_r, Position { x: 1, y: 2 }, Position { x: 1, y: 1 }, Direction::Right, Position { x:  2, y:  2 }, Position { x: 1, y: 1 });
    gen_test!(tc_u, Position { x: 1, y: 2 }, Position { x: 1, y: 1 }, Direction::Up   , Position { x:  1, y:  3 }, Position { x: 1, y: 2 });
    gen_test!(tc_d, Position { x: 1, y: 2 }, Position { x: 1, y: 1 }, Direction::Down , Position { x:  1, y:  1 }, Position { x: 1, y: 1 });

    gen_test!(tr_l, Position { x: 2, y: 2 }, Position { x: 1, y: 1 }, Direction::Left , Position { x:  1, y:  2 }, Position { x: 1, y: 1 });
    gen_test!(tr_r, Position { x: 2, y: 2 }, Position { x: 1, y: 1 }, Direction::Right, Position { x:  3, y:  2 }, Position { x: 2, y: 2 });
    gen_test!(tr_u, Position { x: 2, y: 2 }, Position { x: 1, y: 1 }, Direction::Up   , Position { x:  2, y:  3 }, Position { x: 2, y: 2 });
    gen_test!(tr_d, Position { x: 2, y: 2 }, Position { x: 1, y: 1 }, Direction::Down , Position { x:  2, y:  1 }, Position { x: 1, y: 1 });

    gen_test!(cl_l, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Direction::Left , Position { x: -1, y:  1 }, Position { x: 0, y: 1 });
    gen_test!(cl_r, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Direction::Right, Position { x:  1, y:  1 }, Position { x: 1, y: 1 });
    gen_test!(cl_u, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Direction::Up   , Position { x:  0, y:  2 }, Position { x: 1, y: 1 });
    gen_test!(cl_d, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Direction::Down , Position { x:  0, y:  0 }, Position { x: 1, y: 1 });

    gen_test!(cc_l, Position { x: 1, y: 1 }, Position { x: 1, y: 1 }, Direction::Left , Position { x:  0, y:  1 }, Position { x: 1, y: 1 });
    gen_test!(cc_r, Position { x: 1, y: 1 }, Position { x: 1, y: 1 }, Direction::Right, Position { x:  2, y:  1 }, Position { x: 1, y: 1 });
    gen_test!(cc_u, Position { x: 1, y: 1 }, Position { x: 1, y: 1 }, Direction::Up   , Position { x:  1, y:  2 }, Position { x: 1, y: 1 });
    gen_test!(cc_d, Position { x: 1, y: 1 }, Position { x: 1, y: 1 }, Direction::Down , Position { x:  1, y:  0 }, Position { x: 1, y: 1 });

    gen_test!(cr_l, Position { x: 2, y: 1 }, Position { x: 1, y: 1 }, Direction::Left , Position { x:  1, y:  1 }, Position { x: 1, y: 1 });
    gen_test!(cr_r, Position { x: 2, y: 1 }, Position { x: 1, y: 1 }, Direction::Right, Position { x:  3, y:  1 }, Position { x: 2, y: 1 });
    gen_test!(cr_u, Position { x: 2, y: 1 }, Position { x: 1, y: 1 }, Direction::Up   , Position { x:  2, y:  2 }, Position { x: 1, y: 1 });
    gen_test!(cr_d, Position { x: 2, y: 1 }, Position { x: 1, y: 1 }, Direction::Down , Position { x:  2, y:  0 }, Position { x: 1, y: 1 });

    gen_test!(bl_l, Position { x: 0, y: 0 }, Position { x: 1, y: 1 }, Direction::Left , Position { x: -1, y:  0 }, Position { x: 0, y: 0 });
    gen_test!(bl_r, Position { x: 0, y: 0 }, Position { x: 1, y: 1 }, Direction::Right, Position { x:  1, y:  0 }, Position { x: 1, y: 1 });
    gen_test!(bl_u, Position { x: 0, y: 0 }, Position { x: 1, y: 1 }, Direction::Up   , Position { x:  0, y:  1 }, Position { x: 1, y: 1 });
    gen_test!(bl_d, Position { x: 0, y: 0 }, Position { x: 1, y: 1 }, Direction::Down , Position { x:  0, y: -1 }, Position { x: 0, y: 0 });

    gen_test!(bc_l, Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Direction::Left , Position { x:  0, y:  0 }, Position { x: 1, y: 1 });
    gen_test!(bc_r, Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Direction::Right, Position { x:  2, y:  0 }, Position { x: 1, y: 1 });
    gen_test!(bc_u, Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Direction::Up   , Position { x:  1, y:  1 }, Position { x: 1, y: 1 });
    gen_test!(bc_d, Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Direction::Down , Position { x:  1, y: -1 }, Position { x: 1, y: 0 });

    gen_test!(br_l, Position { x: 2, y: 0 }, Position { x: 1, y: 1 }, Direction::Left , Position { x:  1, y:  0 }, Position { x: 1, y: 1 });
    gen_test!(br_r, Position { x: 2, y: 0 }, Position { x: 1, y: 1 }, Direction::Right, Position { x:  3, y:  0 }, Position { x: 2, y: 0 });
    gen_test!(br_u, Position { x: 2, y: 0 }, Position { x: 1, y: 1 }, Direction::Up   , Position { x:  2, y:  1 }, Position { x: 1, y: 1 });
    gen_test!(br_d, Position { x: 2, y: 0 }, Position { x: 1, y: 1 }, Direction::Down , Position { x:  2, y: -1 }, Position { x: 2, y: 0 });
}
