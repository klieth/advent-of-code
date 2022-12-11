use std::num::ParseIntError;
use std::str::FromStr;

const TEST: &'static str = include_str!("../../test");
const INPUT: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day20Error {
    ParseError(String),
}

impl From<ParseIntError> for Day20Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseError(format!("{}", e))
    }
}

#[derive(Debug, Clone)]
struct IPRange {
    start: usize,
    end: usize,
}

impl IPRange {
    fn includes(&self, subject: usize) -> bool {
        subject >= self.start && subject <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        // can omit self.includes(other.end)
        other.includes(self.start) || other.includes(self.end) || self.includes(other.start)
    }

    fn adjacent(&self, other: &Self) -> bool {
        self.end + 1 == other.start || self.start == other.end + 1
    }

    fn combine(&self, other: &Self) -> Self {
        IPRange {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl FromStr for IPRange {
    type Err = Day20Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-")
            .ok_or_else(|| Day20Error::ParseError(format!("failed to find a range on line")))?;

        Ok(IPRange {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn parse(raw: &str) -> Result<Vec<IPRange>, Day20Error> {
    let mut ranges = vec![];

    for line in raw.lines() {
        ranges.push(line.parse()?);
    }

    while merge_overlaps(&mut ranges) {}
    ranges.sort_by_key(|r| r.start);

    Ok(ranges)
}

fn merge_overlaps(ranges: &mut Vec<IPRange>) -> bool {
    for idx in 0..ranges.len() {
        let subject = &ranges[idx];
        for j in idx + 1..ranges.len() {
            if subject.overlaps(&ranges[j]) || subject.adjacent(&ranges[j]) {
                let one = ranges.remove(idx);
                let two = ranges.remove(j - 1);
                ranges.push(one.combine(&two));
                return true;
            }
        }
    }

    false
}

fn part1(ranges: &[IPRange]) -> usize {
    if ranges[0].start == 0 {
        ranges[0].end + 1
    } else {
        0
    }
}

fn part2(ranges: &[IPRange], max_search_space: usize) -> usize {
    // don't need to check for the beginning of the ranges, because zero is always blacklisted.
    (max_search_space - ranges.last().unwrap().end) +
        ranges.windows(2).map(|s| s[1].start - s[0].end - 1).sum::<usize>()
}

fn main() -> Result<(), Day20Error> {
    let test = parse(TEST)?;
    let input = parse(INPUT)?;
    println!("part1 (test): {}", part1(&test));
    println!("part1 (actual): {}", part1(&input));
    println!("part2 (test): {}", part2(&test, 9));
    println!("part2 (actual): {}", part2(&input, u32::MAX as usize));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_overlap() {
        let one = IPRange {
            start: 1,
            end: 3,
        };
        let two = IPRange {
            start: 2,
            end: 4,
        };

        assert!(one.overlaps(&two));
        assert!(two.overlaps(&one));
    }

    #[test]
    fn one_contains_two() {
        let one = IPRange {
            start: 1,
            end: 4,
        };
        let two = IPRange {
            start: 2,
            end: 3,
        };

        assert!(one.overlaps(&two));
        assert!(two.overlaps(&one));
    }
}
