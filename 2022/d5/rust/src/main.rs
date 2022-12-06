use std::str::FromStr;
use std::fmt;

const TEST_DATA: &'static str = include_str!("../../test");
const INPUT_DATA: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day5Error {
    ParseError(String),
}

impl From<std::num::ParseIntError> for Day5Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseError(value.to_string())
    }
}

#[derive(Clone)]
struct CrateState {
    stacks: Vec<Vec<char>>,
}

impl CrateState {
    fn perform_part1(&mut self, cmd: &Command) {
        let Command { num, from, to } = cmd;

        for _ in 0..*num {
            let ch = self.stacks[*from - 1].pop().expect("tried to grab a crate from an empty stack");
            self.stacks[*to - 1].push(ch);
        }
    }

    fn perform_part2(&mut self, cmd: &Command) {
        let Command { num, from, to } = cmd;

        let from = &mut self.stacks[*from - 1];
        let mut chs = from.split_off(from.len() - num);
        self.stacks[*to - 1].append(&mut chs);
    }

    fn read_top(&self) -> String {
        self.stacks.iter().map(|s| *s.last().expect("tried to read top of empty stack")).collect()
    }
}

impl fmt::Display for CrateState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // unwrap: safe because the state will fail to parse if it has no crates.
        let height = self.stacks.iter().map(|v| v.len() - 1).max().unwrap();

        for h in (0..=height).rev() {
            for stack in self.stacks.iter() {
                if let Some(ch) = stack.get(h) {
                    write!(f, "[{}] ", ch)?;
                } else {
                    write!(f, "    ")?;
                }
            }
            writeln!(f, "")?;
        }

        for (idx, _) in self.stacks.iter().enumerate() {
            write!(f, " {}  ", idx + 1)?;
        }

        Ok( () )
    }
}

impl FromStr for CrateState {
    type Err = Day5Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(Day5Error::ParseError(format!("No starting state found")));
        }

        let mut state = CrateState {
            stacks: vec![],
        };

        let lines = s.lines();

        for line in lines {
            let mut idx = 0;
            while let Some(ch) = line.chars().nth((idx * 4) + 1) {
                match ch {
                    'A'..='Z' => {
                        while state.stacks.len() < idx + 1 {
                            state.stacks.push(vec![]);
                        }

                        // unwrap: safe because we ensure proper length above
                        let stack = state.stacks.get_mut(idx).unwrap();
                        stack.insert(0, ch);
                    }
                    ' ' => {},
                    _ => return Err(Day5Error::ParseError(format!("unexpected character in stack: {} at idx: {}", ch, idx))),
                }

                idx += 1;
            }
        }

        Ok(state)
    }
}

struct Command {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Command {
    type Err = Day5Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn expect(actual: Option<&str>, expected: &str) -> Result<(), Day5Error> {
            if let Some(a) = actual {
                if a == expected {
                    Ok( () )
                } else {
                    Err(Day5Error::ParseError(format!("Expected '{}', but got: {}", expected, a)))
                }
            } else {
                Err(Day5Error::ParseError(format!("Expected '{}', but the line was terminated", expected)))
            }
        }

        let mut words = s.split_whitespace();

        expect(words.next(), "move")?;
        let num = words.next().ok_or(Day5Error::ParseError(format!("No number of crates to move found")))?
            .parse()?;
        expect(words.next(), "from")?;
        let from = words.next().ok_or(Day5Error::ParseError(format!("No stack to move from")))?
            .parse()?;
        expect(words.next(), "to")?;
        let to = words.next().ok_or(Day5Error::ParseError(format!("No stack to move to")))?
            .parse()?;

        Ok(Command { num, from, to })
    }
}

fn parse(raw: &str) -> Result<(CrateState, Vec<Command>), Day5Error> {
    let mut lines = raw.lines();

    let start_state = {
        let state_lines = lines.by_ref().take_while(|line| line.len() > 0).collect::<Vec<_>>();
        state_lines[0..state_lines.len() - 1].join("\n").parse()?
    };

    let mut commands = vec![];
    for raw_command in lines {
        commands.push(raw_command.parse()?);
    }

    Ok( (start_state, commands) )
}

fn part1(start_state: &CrateState, commands: &[Command]) -> String {
    let mut state = Clone::clone(start_state);

    for command in commands {
        state.perform_part1(command);
    }

    state.read_top()
}

fn part2(start_state: &CrateState, commands: &[Command]) -> String {
    let mut state = Clone::clone(start_state);

    for command in commands {
        state.perform_part2(command);
    }

    state.read_top()
}

fn main() -> Result<(), Day5Error> {
    let (test_start_state, test_commands) = parse(TEST_DATA)?;
    let (input_start_state, input_commands) = parse(INPUT_DATA)?;
    println!("part1 (test): {:?}", part1(&test_start_state, &test_commands));
    println!("part1 (actual): {:?}", part1(&input_start_state, &input_commands));
    println!("part2 (test): {:?}", part2(&test_start_state, &test_commands));
    println!("part2 (actual): {:?}", part2(&input_start_state, &input_commands));
    Ok( () )
}
