use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

const TEST: &'static str = include_str!("../../test");
const INPUT: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day11Error {
    ParseError(String),
}

impl From<ParseIntError> for Day11Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseError(format!("{}", error))
    }
}

#[derive(Debug, Clone)]
struct Item(usize);

impl Item {
    fn settle_worry(&mut self) {
        let Item(worry) = self;

        *worry /= 3;
    }

    fn settle_worry_by_factor(&mut self, factor: usize) {
        let Item(worry) = self;

        *worry %= factor;
    }
}

impl FromStr for Item {
    type Err = Day11Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Item(s.parse()?))
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Literal(usize),
    Old,
}

impl Expr {
    fn get(&self, old_val: usize) -> usize {
        match self {
            Expr::Literal(v) => *v,
            Expr::Old => old_val,
        }
    }
}

impl FromStr for Expr {
    type Err = Day11Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Expr::Old)
        } else {
            Ok(Expr::Literal(s.parse()?))
        }
    }
}


#[derive(Debug, Clone)]
enum Operation {
    Mul(Expr),
    Add(Expr),
}

impl Operation {
    fn apply(&self, item: &mut Item) {
        let Item(worry) = item;

        match self {
            Operation::Mul(snd) => *worry = *worry * snd.get(*worry),
            Operation::Add(snd) => *worry = *worry + snd.get(*worry),
        }
    }
}

impl FromStr for Operation {
    type Err = Day11Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(eq) = s.trim().strip_prefix("Operation: new = old ") {
            let (op, snd) = eq.split_once(" ").ok_or_else(|| Day11Error::ParseError(format!("failed to split op from snd")))?;
            match op {
                "*" => Ok(Operation::Mul(snd.parse()?)),
                "+" => Ok(Operation::Add(snd.parse()?)),
                x => Err(Day11Error::ParseError(format!("unrecognized operation: {}", x)))
            }
        } else {
            Err(Day11Error::ParseError(format!("operation line didn't start with proper prefix")))
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    div_by: usize,
    if_t: usize,
    if_f: usize,
}

impl Test {
    fn apply(&self, item: &Item) -> usize {
        if item.0 % self.div_by == 0 {
            self.if_t
        } else {
            self.if_f
        }
    }
}

impl TryFrom<&[&str]> for Test {
    type Error = Day11Error;

    fn try_from(s: &[&str]) -> Result<Self, Self::Error> {
        let div_by = if let Some(raw) = s[0].trim().strip_prefix("Test: divisible by ") {
            raw.parse()?
        } else {
            return Err(Day11Error::ParseError(format!("test line didn't start with proper prefix")));
        };

        let if_t = if let Some(raw) = s[1].trim().strip_prefix("If true: throw to monkey ") {
            raw.parse()?
        } else {
            return Err(Day11Error::ParseError(format!("if true line didn't start with proper prefix")));
        };

        let if_f = if let Some(raw) = s[2].trim().strip_prefix("If false: throw to monkey ") {
            raw.parse()?
        } else {
            return Err(Day11Error::ParseError(format!("if true line didn't start with proper prefix")));
        };

        Ok(Test { div_by, if_t, if_f })
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    held_items: VecDeque<Item>,
    times_inspected: usize,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn inspect(&mut self) -> Item {
        let mut item = self.held_items.pop_front().expect("monkey tried to inspect an item, but was holding no items");

        self.operation.apply(&mut item);
        self.times_inspected += 1;

        item
    }

    fn test(&self, item: &Item) -> usize {
        self.test.apply(item)
    }
}

fn parse(raw: &str) -> Result<Vec<Monkey>, Day11Error> {
    // investigate Iterator::array_chunks when it's stablized.
    let lines = raw.lines().collect::<Vec<_>>();
    let mut monkeys = vec![];

    for chunk in lines.chunks(7) {
        let mut held_items = VecDeque::new();
        if let Some(raw_item_list) = chunk[1].trim().strip_prefix("Starting items: ") {
            for raw_item in raw_item_list.split(", ") {
                held_items.push_back(raw_item.parse()?);
            }
        } else {
            return Err(Day11Error::ParseError(format!("items line didn't start with 'Starting items: '")));
        }

        let operation = chunk[2].parse()?;
        let test = chunk[3..=5].try_into()?;

        monkeys.push(Monkey {
            held_items,
            times_inspected: 0,
            operation,
            test,
        });
    }

    Ok(monkeys)
}

fn part1(monkeys: &[Monkey]) -> usize {
    let mut monkeys = monkeys.to_owned();

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            while monkeys[idx].held_items.len() > 0 {
                // monkey inspects item (worry level operation applied)
                let mut item = monkeys[idx].inspect();

                // worry level is divided by 3
                item.settle_worry();

                // test applied and item is thrown
                let target = monkeys[idx].test(&item);
                monkeys[target].held_items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.times_inspected.cmp(&m1.times_inspected));
    monkeys[0].times_inspected * monkeys[1].times_inspected
}

fn part2(monkeys: &[Monkey]) -> usize {
    let mut monkeys = monkeys.to_owned();
    // unwrap is safe because we always have monkeys
    let worry_settle_factor = monkeys.iter().map(|m| m.test.div_by).product::<usize>();

    for _ in 1..=10000 {
        for idx in 0..monkeys.len() {
            while monkeys[idx].held_items.len() > 0 {
                // monkey inspects item (worry level operation applied)
                let mut item = monkeys[idx].inspect();

                item.settle_worry_by_factor(worry_settle_factor);

                // test applied and item is thrown
                let target = monkeys[idx].test(&item);
                monkeys[target].held_items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.times_inspected.cmp(&m1.times_inspected));
    monkeys[0].times_inspected * monkeys[1].times_inspected
}

fn main() -> Result<(), Day11Error> {
    let test = parse(TEST)?;
    let input = parse(INPUT)?;
    println!("part1 (test): {}", part1(&test));
    println!("part1 (actual): {}", part1(&input));
    println!("part2 (test): {}", part2(&test));
    println!("part2 (actual): {}", part2(&input));
    Ok( () )
}
