use std::num::ParseIntError;
use std::ops::Range;

const TEST: &'static str = include_str!("../../test");
const INPUT: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day12Error {
    ParseError(String),
}

impl From<ParseIntError> for Day12Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseError(format!("{}", error))
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn neighbors(&self, x_bound: &Range<usize>, y_bound: &Range<usize>) -> Vec<Position> {
        [
            (self.x.checked_add(1), Some(self.y)),
            (Some(self.x), self.y.checked_add(1)),
            (self.x.checked_sub(1), Some(self.y)),
            (Some(self.x), self.y.checked_sub(1)),
        ].into_iter().filter_map(|coords| {
            if let (Some(x), Some(y)) = coords {
                if x_bound.contains(&x) && y_bound.contains(&y) {
                    Some(Position { x, y })
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }
}

#[derive(Debug, Clone)]
struct Loc {
    height: usize,
    distance: Option<usize>,
}

impl From<usize> for Loc {
    fn from(n: usize) -> Self {
        Self {
            height: n,
            distance: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    data: Vec<Vec<Loc>>,
    start: Position,
    end: Position,
}

fn parse(raw: &str) -> Result<Map, Day12Error> {
    let mut data = Vec::new();
    let mut start = None;
    let mut end = None;

    for (x, line) in raw.lines().enumerate() {
        let mut row = Vec::new();

        for (y, ch) in line.chars().enumerate() {
            row.push(match ch {
                'S' => {
                    if let None = start {
                        start = Some( Position { x, y } );
                    } else {
                        return Err(Day12Error::ParseError(format!("multiple starting locations found")));
                    }

                    0.into()
                }
                'E' => {
                    if let None = end {
                        end = Some( Position { x, y } );
                    } else {
                        return Err(Day12Error::ParseError(format!("multiple ending locations found")));
                    }

                    25.into()
                }
                c @ 'a'..='z' => ((c as usize) - ('a' as usize)).into(),
                n => return Err(Day12Error::ParseError(format!("unrecognized character: {}", n))),
            });
        }

        data.push(row);
    }

    match (start, end) {
        (Some(start), Some(end)) => Ok( Map { data, start, end } ),
        _ => Err(Day12Error::ParseError(format!("one of start or end was not set")))
    }
}

fn dijkstra<F: Fn(usize, usize) -> bool>(map: &mut Map, start: Position, can_travel: F) {
    let x_bounds = 0..map.data.len();
    let y_bounds = 0..map.data[0].len();

    let mut to_visit = std::collections::VecDeque::new();
    map.data[start.x][start.y].distance = Some(0);
    to_visit.push_back(start);

    while to_visit.len() > 0 {
        // unwrap: safe because we bounds check in the loop condition.
        let visiting = to_visit.pop_front().unwrap();

        let current_height = map.data[visiting.x][visiting.y].height;
        // unwrap: safe because a distance is always set before the location is visited.
        let current_distance = map.data[visiting.x][visiting.y].distance.unwrap();

        for Position { x: nx, y: ny } in visiting.neighbors(&x_bounds, &y_bounds) {
            if can_travel(map.data[nx][ny].height, current_height) {
                if map.data[nx][ny].distance.map(|r| current_distance + 1 < r).unwrap_or(true) {
                    map.data[nx][ny].distance = Some(current_distance + 1);
                    to_visit.push_back(Position { x: nx, y: ny });
                }
            }
        }
    }
}

fn part1(map: &Map) -> usize {
    let mut map = map.clone();
    let start = map.start.clone();

    dijkstra(&mut map, start, |neighbor_height, current_height| neighbor_height <= current_height + 1);

    // unwrap: safe because after this algorithm, all locations have a distance assigned.
    map.data[map.end.x][map.end.y].distance.unwrap()
}

fn part2(map: &Map) -> usize {
    let mut map = map.clone();
    let start = map.end.clone();

    dijkstra(&mut map, start, |neighbor_height, current_height| current_height <= neighbor_height + 1);

    // unwrap: safe because after this algorithm, all locations have a distance assigned.
    map.data.iter()
        .flatten()
        .filter_map(|loc| if loc.height == 0 { loc.distance } else { None })
        .min()
        .unwrap() // unwrap: safe because the input is guaranteed to have at least one location
                  // with a height of 0
}

fn main() -> Result<(), Day12Error> {
    let test = parse(TEST)?;
    let input = parse(INPUT)?;
    println!("part1 (test): {}", part1(&test));
    println!("part1 (actual): {}", part1(&input));
    println!("part2 (test): {}", part2(&test));
    println!("part2 (actual): {}", part2(&input));
    Ok( () )
}
