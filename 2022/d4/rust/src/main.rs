const TEST_DATA: &'static str = include_str!("../../test");
const INPUT_DATA: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day4Error {
    ParseError(String),
}

#[derive(Debug)]
struct SectionIDRange {
    start: usize,
    end: usize,
}

impl SectionIDRange {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        for v in [other.start, other.end] {
            if v >= self.start && v <= self.end {
                return true;
            }
        }

        return false;
    }
}

impl TryFrom<&str> for SectionIDRange {
    type Error = Day4Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut ends = value.split("-");
        // unwrap: safe becuase input manually verified to be numbers
        let start = ends.next().ok_or_else(|| Day4Error::ParseError(format!("failed to parse start")))
            .map(|v| v.parse().unwrap())?;
        let end = ends.next().ok_or_else(|| Day4Error::ParseError(format!("failed to parse end")))
            .map(|v| v.parse().unwrap())?;
        Ok(Self { start, end })
    }
}

fn parse(raw: &str) -> Result<Vec<(SectionIDRange, SectionIDRange)>, Day4Error> {
    let mut range_sets = vec![];

    for line in raw.lines() {
        let mut sides = line.split(",");
        let left_side: SectionIDRange = sides.next()
            .ok_or_else(|| Day4Error::ParseError(format!("no left side found")))?
            .try_into()?;
        let right_side: SectionIDRange = sides.next()
            .ok_or_else(|| Day4Error::ParseError(format!("no right side found")))?
            .try_into()?;
        range_sets.push( (left_side.into(), right_side.into()) );
    }

    Ok(range_sets)
}

fn part1(sections: &[(SectionIDRange, SectionIDRange)]) -> usize {
    sections.iter()
        .filter(|(r1, r2)| r1.contains(r2) || r2.contains(r1))
        .count()
}

fn part2(sections: &[(SectionIDRange, SectionIDRange)]) -> usize {
    sections.iter()
        .filter(|(r1, r2)| r1.overlaps(r2) || r2.overlaps(r1))
        .count()
}

fn main() -> Result<(), Day4Error> {
    let testData = parse(TEST_DATA)?;
    let inputData = parse(INPUT_DATA)?;
    println!("part1 (test): {:?}", part1(&testData));
    println!("part1 (actual): {:?}", part1(&inputData));
    println!("part2 (test): {:?}", part2(&testData));
    println!("part2 (actual): {:?}", part2(&inputData));
    Ok( () )
}
