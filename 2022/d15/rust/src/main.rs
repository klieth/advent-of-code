use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

const TEST: &'static str = include_str!("../../test");
const INPUT: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day15Error {
    ParseError(String),
}

impl From<ParseIntError> for Day15Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseError(format!("{}", error))
    }
}

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
struct Position(isize, isize);

impl FromStr for Position {
    type Err = Day15Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fst, snd) = match s.split_once(", ") {
            Some(r) => r,
            None => return Err(Day15Error::ParseError(format!("failed to split position"))),
        };

        let x = match fst.strip_prefix("x=") {
            Some(r) => r.parse()?,
            None => return Err(Day15Error::ParseError(format!("failed to strip 'x='"))),
        };

        let y = match snd.strip_prefix("y=") {
            Some(r) => r.parse()?,
            None => return Err(Day15Error::ParseError(format!("failed to strip 'y='"))),
        };

        Ok(Position(x, y))
    }
}

#[derive(Debug,PartialEq,Eq)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn new(a: isize, b: isize) -> Self {
        if a < b {
            Range { start: a, end: b }
        } else {
            Range { start: b, end: a }
        }
    }

    fn includes(&self, subject: isize) -> bool {
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
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn size(&self) -> isize {
        self.end - self.start + 1
    }

    fn clamp(mut self, min: isize, max: isize) -> Option<Range> {
        self.start = self.start.max(min);
        self.end = self.end.min(max);

        if self.start < self.end {
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
struct Sensor {
    loc: Position,
    nearest_beacon: Position
}

impl Sensor {
    fn range_at_line(&self, line: isize) -> Option<Range> {
        let dx = (self.loc.0 - self.nearest_beacon.0).abs();
        let dy = (self.loc.1 - self.nearest_beacon.1).abs();

        let dl = (self.loc.1 - line).abs();

        if dl <= dx + dy {
            Some(Range::new(
                    self.loc.0 - (dx + dy) + dl,
                    self.loc.0 + (dx + dy) - dl,
            ))
        } else {
            None
        }
    }
}

impl FromStr for Sensor {
    type Err = Day15Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = match s.strip_prefix("Sensor at ") {
            Some(r) => r,
            None => return Err(Day15Error::ParseError(format!("Line doesn't start with 'Sensor at'"))),
        };

        let (fst, snd) = match s.split_once(": closest beacon is at ") {
            Some(r) => r,
            None => return Err(Day15Error::ParseError(format!("Failed to split sensor line"))),
        };

        Ok(Sensor {
            loc: fst.parse()?,
            nearest_beacon: snd.parse()?,
        })
    }
}

fn parse(raw: &str) -> Result<Vec<Sensor>, Day15Error> {
    raw.lines().map(str::parse).collect()
}

fn merge_overlaps(ranges: &mut Vec<Range>) -> bool {
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

fn part1(sensors: &[Sensor], target_line: isize) -> isize {
    let mut ranges = Vec::new();

    for sensor in sensors {
        if let Some(range) = sensor.range_at_line(target_line) {
            ranges.push(range);
        }
    }

    while merge_overlaps(&mut ranges) { }

    let candidate = ranges.iter().map(|r| r.size()).sum::<isize>();
    let beacons_on_line = sensors.iter()
        .filter_map(|s| if s.nearest_beacon.1 == target_line { Some(s.nearest_beacon.clone()) } else { None })
        .collect::<HashSet<_>>()
        .len();

    candidate - (beacons_on_line as isize)
}

fn tuning_frequency(x: isize, y: isize) -> isize {
    (x * 4000000) + y
}

fn part2(sensors: &[Sensor], search_space: isize) -> isize {
    for line in 0..=search_space {
        let mut ranges = sensors.iter().filter_map(|s| {
            s.range_at_line(line as isize)
                .and_then(|r| r.clamp(0, search_space))
        }).collect::<Vec<_>>();

        while merge_overlaps(&mut ranges) {}

        if ranges.len() > 2 {
            unreachable!("input guarantees that at most one single spot is empty within the search range")
        } else if ranges.len() == 2 && ranges.iter().map(|r| r.size()).sum::<isize>() < search_space + 1 {
            if ranges[0].start < ranges[1].start {
                return tuning_frequency(ranges[0].end + 1, line);
            } else {
                return tuning_frequency(ranges[1].end + 1, line);
            }
        } else if ranges.len() == 1 && ranges[0].size() < search_space + 1 {
            if ranges[0].start == 1 {
                return tuning_frequency(0, line);
            } else if ranges[0].end == search_space - 1 {
                return tuning_frequency(search_space, line);
            }
        }
    }

    unreachable!("input guarantees that there is an open spot for the beacon")
}

fn main() -> Result<(), Day15Error> {
    let test = parse(TEST)?;
    let input = parse(INPUT)?;
    println!("part1 (test): {}", part1(&test, 10));
    println!("part1 (actual): {}", part1(&input, 2000000));
    println!("part2 (test): {}", part2(&test, 20));
    println!("part2 (actual): {}", part2(&input, 4000000));
    Ok( () )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        let sensor: Sensor = "Sensor at x=8, y=7: closest beacon is at x=2, y=10".parse().unwrap();

        let range = sensor.range_at_line(10);

        assert_eq!(range, Range::new(2, 14));
    }
}
