use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

enum ParserState {
    Complete,
    Incomplete(String)
}

struct SnailfishParser<'a> {
    data: std::str::Chars<'a>,
    peek: Option<char>,
}

impl<'a> SnailfishParser<'a> {
    fn from_str(s: &'a str) -> Self {
        let mut data = s.chars();
        let peek = data.next();

        SnailfishParser {
            data,
            peek,
        }
    }

    fn finish(mut self) -> ParserState {
        match self.peek.take() {
            Some(c) => {
                let mut s = String::new();

                s.push(c);
                s.extend(self.data);

                ParserState::Incomplete(s)
            }
            None => ParserState::Complete
        }
    }

    fn peek(&self) -> Result<char, String> {
        // unwrap: we will know when a number is fully parsed, so we should never be peeking past
        // the end of the data.
        self.peek.ok_or_else(|| "peek with no more input".into())
    }

    fn next(&mut self) -> Result<char, String> {
        let old = self.peek;
        self.peek = self.data.next();
        old.ok_or_else(|| "no more input".into())
    }

    fn expect(&mut self, expected: char) -> Result<() ,String> {
        let c = self.next()?;

        if c == expected {
            Ok( () )
        } else {
            Err( format!("expected '{}', found: '{}'", expected, c) )
        }
    }

    fn parse_any(&mut self) -> Result<SnailfishNumber, String> {
        match self.peek()? {
            '[' => self.parse_pair(),
            '0'..='9' => self.parse_number(),
            other => panic!("unrecognized character: {}", other),
        }
    }

    fn parse_pair(&mut self) -> Result<SnailfishNumber, String> {
        self.expect('[')?;

        let left = self.parse_any()?;

        self.expect(',')?;

        let right = self.parse_any()?;

        self.expect(']')?;

        Ok( SnailfishNumber::Pair(Box::new(left), Box::new(right)) )
    }

    fn parse_number(&mut self) -> Result<SnailfishNumber, String> {
        let mut s = String::new();

        while self.peek.is_some() && self.peek()?.is_ascii_digit() {
            s.push(self.next()?);
        }

        s.parse::<usize>().map(SnailfishNumber::Value)
            .map_err(|e| format!("{}", e))
    }
}

enum ExplosionSide {
    Left,
    Right,
}

#[derive(Debug)]
enum ExplosionState {
    Done,
    Both(usize, usize),
    Left(usize),
    Right(usize),
}

impl ExplosionState {
    fn has_right(&self) -> bool {
        use ExplosionState::*;

        match self {
            Both(..) | Right(..) => true,
            _ => false
        }
    }

    fn has_left(&self) -> bool {
        use ExplosionState::*;

        match self {
            Both(..) | Left(..) => true,
            _ => false
        }
    }

    fn has_side(&self, side: ExplosionSide) -> bool {
        match side {
            ExplosionSide::Left => self.has_left(),
            ExplosionSide::Right => self.has_right(),
        }
    }

    fn split_left(self) -> Result<(usize, ExplosionState), ExplosionState> {
        match self {
            ExplosionState::Both(left, right) => Ok( (left, ExplosionState::Right(right)) ),
            ExplosionState::Left(v) => Ok( (v, ExplosionState::Done) ),
            _ => Err(self),
        }
    }

    fn split_right(self) -> Result<(usize, ExplosionState), ExplosionState> {
        match self {
            ExplosionState::Both(left, right) => Ok( (right, ExplosionState::Left(left)) ),
            ExplosionState::Right(v) => Ok( (v, ExplosionState::Done) ),
            _ => Err(self),
        }
    }

    fn split_side(self, side: ExplosionSide) -> Result<(usize, ExplosionState), ExplosionState> {
        match side {
            ExplosionSide::Left => self.split_left(),
            ExplosionSide::Right => self.split_right(),
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
enum SnailfishNumber {
    Value(usize),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl<'a> SnailfishNumber {
    fn parse(parser: &mut SnailfishParser<'a>) -> Result<Self, String> {
        parser.parse_any()
    }

    fn is_value(&self) -> bool {
        if let SnailfishNumber::Value(_) = self {
            true
        } else {
            false
        }
    }

    fn value(&self) -> usize {
        if let SnailfishNumber::Value(v) = self {
            *v
        } else {
            panic!("called value() on a pair")
        }
    }

    fn is_pair(&self) -> bool {
        !self.is_value()
    }

    fn take(self) -> usize {
        if let SnailfishNumber::Value(v) = self {
            v
        } else {
            panic!("called take on a Pair");
        }
    }

    fn left(&self) -> &SnailfishNumber {
        if let SnailfishNumber::Pair(left, _) = self {
            left
        } else {
            panic!("called left on a Pair");
        }
    }

    fn right(&self) -> &SnailfishNumber {
        if let SnailfishNumber::Pair(_, right) = self {
            right
        } else {
            panic!("called left on a Pair");
        }
    }

    fn reduce(&mut self) {
        loop {
            match self.explode(0) {
                None => match self.split() {
                    None => break,
                    _ => {}
                }
                _ => {}
            }
        }
    }

    fn explode(&mut self, depth: usize) -> Option<ExplosionState> {
        if let SnailfishNumber::Value(_) = self {
            panic!("explode() should never travel to a Value node");
        }

        // depth is 0-indexed, so 4 is actually the first level we want to start exploding
        if depth >= 4 && self.left().is_value() && self.right().is_value() {
            match std::mem::replace(self, SnailfishNumber::Value(0)) {
                SnailfishNumber::Pair(left, right) => return Some(ExplosionState::Both(left.take(), right.take())),
                _ => unreachable!()
            }
        }

        if let SnailfishNumber::Pair(ref mut left, ref mut right) = self {
            if left.is_pair() {
                let explosion = left.explode(depth + 1).map(|e| {
                    if e.has_right() {
                        right.apply_explosion(e, ExplosionSide::Right)
                    } else {
                        e
                    }
                });

                if explosion.is_some() {
                    return explosion;
                }
            }

            if right.is_pair() {
                let explosion = right.explode(depth + 1).map(|e| {
                    if e.has_left() {
                        left.apply_explosion(e, ExplosionSide::Left)
                    } else {
                        e
                    }
                });

                if explosion.is_some() {
                    return explosion;
                }
            }

            None
        } else {
            unreachable!()
        }
    }

    fn apply_explosion(&mut self, state: ExplosionState, side: ExplosionSide) -> ExplosionState {
        match self {
            SnailfishNumber::Value(v) => {
                match state.split_side(side) {
                    Ok( (number, new_state) ) => {
                        *v += number;
                        new_state
                    }
                    Err(old_state) => old_state,
                }
            }
            SnailfishNumber::Pair(left, right) => {
                match side {
                    ExplosionSide::Left => right.apply_explosion(state, side),
                    ExplosionSide::Right => left.apply_explosion(state, side),
                }
            }
        }
    }

    fn split(&mut self) -> Option<()> {
        if self.is_value() && self.value() >= 10 {
            let v = self.value();
            let n = v / 2;

            let _ = std::mem::replace(self, SnailfishNumber::Pair(
                Box::new(SnailfishNumber::Value(n)),
                Box::new(SnailfishNumber::Value(if v % 2 == 0 { n } else { n + 1 })),
            ));

            Some( () )
        } else if let SnailfishNumber::Pair(left, right) = self {
            left.split().or_else(|| right.split())
        } else {
            None
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            SnailfishNumber::Value(v) => *v,
            SnailfishNumber::Pair(left, right) => (3 * left.magnitude()) +  (2 * right.magnitude()),
        }
    }
}

impl std::str::FromStr for SnailfishNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = SnailfishParser::from_str(s);
        let res = Self::parse(&mut p);

        if let ParserState::Incomplete(leftover) = p.finish() {
            Err( format!("leftover input: {}", leftover) )
        } else {
            res
        }
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut res = SnailfishNumber::Pair(Box::new(self), Box::new(rhs));
        res.reduce();
        res
    }
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SnailfishNumber::Pair(left, right) => write!(f, "[{},{}]", left, right),
            SnailfishNumber::Value(v) => write!(f, "{}", v),
        }
    }
}

fn add_all_magnitude(input: &Vec<SnailfishNumber>) -> usize {
    let sum = input.iter().cloned().reduce(std::ops::Add::add).unwrap();

    sum.magnitude()
}

fn highest_magnitude_pair(input: &Vec<SnailfishNumber>) -> usize {
    let mut max = 0;

    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let m = (input[i].clone() + input[j].clone()).magnitude();
            if m > max {
                max = m;
            }

            let m = (input[j].clone() + input[i].clone()).magnitude();
            if m > max {
                max = m;
            }
        }
    }

    max
}

fn parse_input(input: String) -> Vec<SnailfishNumber> {
    input.lines().map(|l| l.parse().expect("failed to parse SnailfishNumber")).collect()
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

    let input = parse_input(input);

    let now = Instant::now();
    println!("part 1: {}", add_all_magnitude(&input));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);

    let now = Instant::now();
    println!("part 2: {}", highest_magnitude_pair(&input));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_value_succeeds() {
        let n: Result<SnailfishNumber, _> = "1".parse();

        assert!(n.is_ok());
        assert_eq!(n.unwrap(), SnailfishNumber::Value(1));
    }

    #[test]
    fn parse_pair_succeeds() {
        let n: Result<SnailfishNumber, _> = "[1,2]".parse();

        assert!(n.is_ok());
        assert_eq!(n.unwrap(), SnailfishNumber::Pair(Box::new(SnailfishNumber::Value(1)), Box::new(SnailfishNumber::Value(2))));
    }

    #[test]
    fn parse_long_number_succeeds() {
        let n: Result<SnailfishNumber, _> = "[11,2]".parse();

        assert!(n.is_ok());
        assert_eq!(n.unwrap(), SnailfishNumber::Pair(Box::new(SnailfishNumber::Value(11)), Box::new(SnailfishNumber::Value(2))));
    }

    #[test]
    fn parse_pair_unclosed() {
        let n: Result<SnailfishNumber, _> = "[1,2".parse();

        assert!(n.is_err());
    }

    #[test]
    fn parse_pair_extra() {
        let n: Result<SnailfishNumber, _> = "[1,2],1".parse();

        assert!(n.is_err());
    }

    #[test]
    fn explode_one_right() {
        let mut n: SnailfishNumber = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        n.explode(0);
        let expected: SnailfishNumber = "[[[[0,9],2],3],4]".parse().unwrap();

        assert_eq!(n, expected);
    }

    #[test]
    fn explode_one_left() {
        let mut n: SnailfishNumber = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        n.explode(0);
        let expected: SnailfishNumber = "[7,[6,[5,[7,0]]]]".parse().unwrap();

        assert_eq!(n, expected);
    }

    #[test]
    fn explode_one_both() {
        let mut n: SnailfishNumber = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        n.explode(0);
        let expected: SnailfishNumber = "[[6,[5,[7,0]]],3]".parse().unwrap();

        assert_eq!(n, expected);
    }

    #[test]
    fn explode_one_leftmost() {
        let mut n: SnailfishNumber = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        n.explode(0);
        let expected: SnailfishNumber = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();

        assert_eq!(n, expected);
    }

    #[test]
    fn split_one() {
        let mut n: SnailfishNumber = "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse().unwrap();
        n.split();
        let expected: SnailfishNumber = "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".parse().unwrap();

        assert_eq!(n, expected);
    }

    #[test]
    fn reduce_full() {
        let mut n: SnailfishNumber = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap();
        n.reduce();
        let expected: SnailfishNumber = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();

        assert_eq!(n, expected);
    }

    #[test]
    fn add_simple() {
        let input = "[1,1]\n\
                     [2,2]\n\
                     [3,3]\n\
                     [4,4]";

        let input = parse_input(input.into());

        let expected: SnailfishNumber = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();

        assert_eq!(input.iter().cloned().reduce(std::ops::Add::add).unwrap(), expected);
    }

    #[test]
    fn add_explode() {
        let input = "[1,1]\n\
                     [2,2]\n\
                     [3,3]\n\
                     [4,4]\n\
                     [5,5]";

        let input = parse_input(input.into());

        let expected: SnailfishNumber = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();

        assert_eq!(input.iter().cloned().reduce(std::ops::Add::add).unwrap(), expected);
    }

    #[test]
    fn add_explode_split() {
        let input = "[1,1]\n\
                     [2,2]\n\
                     [3,3]\n\
                     [4,4]\n\
                     [5,5]\n\
                     [6,6]";

        let input = parse_input(input.into());

        let expected: SnailfishNumber = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();

        assert_eq!(input.iter().cloned().reduce(std::ops::Add::add).unwrap(), expected);
    }

    #[test]
    fn add_big() {
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n\
                     [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n\
                     [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n\
                     [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n\
                     [7,[5,[[3,8],[1,4]]]]\n\
                     [[2,[2,2]],[8,[8,1]]]\n\
                     [2,9]\n\
                     [1,[[[9,3],9],[[9,0],[0,7]]]]\n\
                     [[[5,[7,4]],7],1]\n\
                     [[[[4,2],2],6],[8,7]]";

        let input = parse_input(input.into());

        let expected: SnailfishNumber = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().unwrap();

        assert_eq!(input.iter().cloned().reduce(std::ops::Add::add).unwrap(), expected);
    }
}

