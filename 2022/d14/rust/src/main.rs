use std::collections::HashSet;
use std::num::ParseIntError;

const TEST: &'static str = include_str!("../../test");
const INPUT: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day14Error {
    ParseError(String),
}

impl From<ParseIntError> for Day14Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseError(format!("{}", error))
    }
}

struct SandUnit {
    x: usize,
    y: usize,
}

impl SandUnit {
    fn new() -> Self {
        Self { x: 500, y: 0 }
    }

    fn drop(&mut self, map: &HashSet<(usize,usize)>) -> bool {
        if !map.contains( &(self.x, self.y + 1) ) {
            self.y += 1;
            true
        } else if !map.contains( &(self.x - 1, self.y + 1) ) {
            self.x -= 1;
            self.y += 1;
            true
        } else if !map.contains( &(self.x + 1, self.y + 1) ) {
            self.x += 1;
            self.y += 1;
            true
        } else {
            false
        }
    }

    fn drop_floor(&mut self, map: &HashSet<(usize,usize)>, floor: usize) -> bool {
        if self.y + 1 == floor {
            false
        } else {
            self.drop(map)
        }
    }
}

impl From<SandUnit> for (usize,usize) {
    fn from(sand: SandUnit) -> (usize,usize) {
        (sand.x, sand.y)
    }
}

fn parse(raw: &str) -> Result<HashSet<(usize, usize)>, Day14Error> {
    let mut map = HashSet::new();

    for line in raw.lines() {
        let corners = line.split(" -> ")
            .map(|raw_corner| {
                let (fst, snd) = raw_corner.split_once(",").ok_or_else(|| Day14Error::ParseError(format!("failed to parse point")))?;
                Ok( (fst.parse()?, snd.parse()?) )
            }).collect::<Result<Vec<(usize, usize)>, Day14Error>>()?;

        if corners.len() < 2 {
            return Err(Day14Error::ParseError(format!("input line doesn't have at least two points; can't form any lines")));
        }

        for pair in corners.windows(2) {
            let (sx, sy) = pair[0];
            let (ex, ey) = pair[1];

            if sx == ex {
                let range = (sy.min(ey))..=(sy.max(ey));
                for y in range {
                    map.insert( (sx, y) );
                }
            } else if sy == ey {
                let range = (sx.min(ex))..=(sx.max(ex));
                for x in range {
                    map.insert( (x, sy) );
                }
            } else {
                return Err(Day14Error::ParseError(format!("line is not straight: ({},{}) ({},{})", sx, sy, ex, ey)));
            }
        }
    }

    Ok(map)
}

fn part1(map: &HashSet<(usize, usize)>) -> usize {
    let mut map = map.clone();
    // unwrap: safe because hash set will never be empty
    let bottom = map.iter().map(|(_,y)| *y).max().unwrap();

    let mut sand = SandUnit::new();
    let mut count = 0;

    while sand.y < bottom {
        if !sand.drop(&map) {
            count += 1;
            map.insert(sand.into());
            sand = SandUnit::new();
        }
    }

    count
}

fn part2(map: &HashSet<(usize, usize)>) -> usize {
    let mut map = map.clone();
    // unwrap: safe because hash set will never be empty
    let floor = map.iter().map(|(_,y)| *y).max().unwrap() + 2;

    let mut sand = SandUnit::new();
    let mut count = 0;

    loop {
        while sand.drop_floor(&map, floor) { }

        count += 1;

        if sand.y == 0 {
            return count;
        }

        map.insert(sand.into());
        sand = SandUnit::new();
    }
}

fn main() -> Result<(), Day14Error> {
    let test = parse(TEST)?;
    let input = parse(INPUT)?;
    println!("part1 (test): {}", part1(&test));
    println!("part1 (actual): {}", part1(&input));
    println!("part2 (test): {}", part2(&test));
    println!("part2 (actual): {}", part2(&input));
    Ok( () )
}
