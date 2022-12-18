use std::cmp::Ordering;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

const TEST: &'static str = include_str!("../../test");
const INPUT: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day13Error {
    ParseError(String),
    RuntimeError(String),
}

impl From<ParseIntError> for Day13Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseError(format!("{}", error))
    }
}

#[derive(Clone,Eq,PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Int(usize),
}

impl Packet {
    fn parse_inner<'a>(s: &'a str) -> Result<(Packet, &'a str), Day13Error> {
        match s.chars().nth(0) {
            Some('[') => Packet::parse_list(s),
            Some(_) => Packet::parse_int(s),
            _ => Err(Day13Error::ParseError(format!("attempted to parse empty string"))),
        }
    }

    fn parse_list<'a>(s: &'a str) -> Result<(Packet, &'a str), Day13Error> {
        let mut inner = s.strip_prefix("[")
            .ok_or_else(|| Day13Error::ParseError(format!("List did not start with '['")))?;

        let mut values = Vec::new();

        while !inner.starts_with("]") {
            let (value, rest) = Packet::parse_inner(inner)?;
            values.push(value);
            inner = rest;

            if let Some(r) = rest.strip_prefix(",") {
                inner = r;
            }
        }

        let rest = inner.strip_prefix("]")
            .ok_or_else(|| Day13Error::ParseError(format!("List did not end with ']'")))?;

        Ok( (Packet::List(values), rest) )
    }

    fn parse_int<'a>(s: &'a str) -> Result<(Packet, &'a str), Day13Error> {
        let (raw_int, rest) = if let Some(at) = s.find(|c: char| !c.is_digit(10)) {
            s.split_at(at)
        } else {
            s.split_at(s.len())
        };

        Ok( (Packet::Int(raw_int.parse()?), rest) )
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(i1), Packet::Int(i2)) => i1.cmp(i2),
            (Packet::List(l1), Packet::List(l2)) => {
                for (fst, snd) in l1.iter().zip(l2.iter()) {
                    match fst.cmp(snd) {
                        Ordering::Equal => {}
                        c => return c,
                    }
                }

                l1.len().cmp(&l2.len())
            }
            (Packet::List(l), i @ Packet::Int(_)) => match l.get(0).map(|p| p.cmp(i)).unwrap_or(Ordering::Less) {
                Ordering::Equal => if l.len() > 1 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
                o => o
            }
            (i @ Packet::Int(_), Packet::List(l)) => l.get(0).map(|p| i.cmp(p)).unwrap_or(Ordering::Greater),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = Day13Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (packet, rest) = Self::parse_inner(s)?;

        if rest.len() != 0 {
            return Err(Day13Error::ParseError(format!("found extra data after packet")));
        }

        Ok(packet)
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Packet::List(l) => f.debug_list().entries(l.iter()).finish(),
            Packet::Int(i) => fmt::Debug::fmt(i, f),
        }
    }
}

fn parse(raw: &str) -> Result<Vec<Packet>, Day13Error> {
    let mut packets = Vec::new();
    let lines = raw.lines().collect::<Vec<_>>();

    for raw_pair in lines.chunks(3) {
        packets.push(raw_pair[0].parse()?);
        packets.push(raw_pair[1].parse()?);
    }

    Ok(packets)
}

fn part1(packet_pairs: &[Packet]) -> usize {
    packet_pairs.chunks(2)
        .enumerate()
        .filter_map(|(idx, pair)| {
            if pair[0] < pair[1] {
                Some(idx + 1)
            } else {
                None
            }
        }).sum()
}

fn part2(packet_pairs: &[Packet]) -> Result<usize, Day13Error> {
    let two: Packet = "[[2]]".parse()?;
    let six: Packet = "[[6]]".parse()?;

    let mut packet_pairs = packet_pairs.to_owned();
    packet_pairs.push(two.clone());
    packet_pairs.push(six.clone());

    packet_pairs.sort();

    let two_idx = packet_pairs.binary_search(&two)
        .map_err(|i| Day13Error::RuntimeError(format!("failed to find 'two', expected at index: {}", i)))?;
    let six_idx = packet_pairs.binary_search(&six)
        .map_err(|i| Day13Error::RuntimeError(format!("failed to find 'six', expected at index: {}", i)))?;

    Ok((two_idx + 1) * (six_idx + 1))
}

fn main() -> Result<(), Day13Error> {
    let test = parse(TEST)?;
    let input = parse(INPUT)?;
    println!("part1 (test): {}", part1(&test));
    println!("part1 (actual): {}", part1(&input));
    println!("part2 (test): {}", part2(&test)?);
    println!("part2 (actual): {}", part2(&input)?);
    Ok( () )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn just_int() {
        let p: Packet = "5".parse().unwrap();

        assert_eq!(Packet::Int(5), p);
    }

    #[test]
    fn gt() {
        let fst: Packet = "[5,6]".parse().unwrap();
        let snd: Packet = "5".parse().unwrap();
        assert!(fst > snd);
    }
}
