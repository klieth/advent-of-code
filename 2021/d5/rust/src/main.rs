use std::path::Path;
use std::fs::File;
use std::io::Read;

use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(',');

        let x = points.next()
            .ok_or(format!("couldn't find first point"))?
            .parse()
            .map_err(|e| format!("{}", e))?;
        let y = points.next()
            .ok_or(format!("couldn't find second point"))?
            .parse()
            .map_err(|e| format!("{}", e))?;

        Ok(Point { x, y })
    }
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    // TODO is there a way to do this while returning 'impl Iterator'?
    fn points(&self) -> Vec<(usize, usize)> {
        if self.vertical() {
            if self.start.x < self.end.x {
                (self.start.x ..= self.end.x).map(|x| (x, self.start.y)).collect()
            } else {
                (self.end.x ..= self.start.x).map(|x| (x, self.start.y)).collect()
            }
        } else if self.horizontal() {
            if self.start.y < self.end.y {
                (self.start.y ..= self.end.y).map(|y| (self.start.x, y)).collect()
            } else {
                (self.end.y ..= self.start.y).map(|y| (self.start.x, y)).collect()
            }
        } else {
            let x_values: Vec<usize> = if self.start.x < self.end.x {
                (self.start.x ..= self.end.x).collect()
            } else {
                (self.end.x ..= self.start.x).rev().collect()
            };

            let y_values: Vec<usize> = if self.start.y < self.end.y {
                (self.start.y ..= self.end.y).collect()
            } else {
                (self.end.y ..= self.start.y).rev().collect()
            };

            x_values.into_iter().zip(y_values.into_iter()).collect()
        }
    }
}

impl FromStr for LineSegment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = s.split(" -> ");

        let start = pairs.next()
            .ok_or(format!("couldn't find first coordinate"))?
            .parse()?;
        let end = pairs.next()
            .ok_or(format!("couldn't find second coordinate"))?
            .parse()?;

        Ok(LineSegment { start, end })
    }
}

struct Board {
    inner: Vec<Vec<usize>>,
}

impl Board {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn mark_line(&mut self, line: &LineSegment) {
        for (x, y) in line.points() {
            let mut col = self.inner.get_mut(x);

            if col.is_none() {
                self.inner.resize(x + 1, Vec::new());
                col = self.inner.get_mut(x);
            }

            // safe due to the None check above
            let col = col.unwrap();

            let mut loc = col.get_mut(y);

            if loc.is_none() {
                col.resize(y + 1, 0);
                loc = col.get_mut(y);
            }

            // safe due to the None check above
            let loc = loc.unwrap();

            *loc += 1;
        }
    }

    fn crossings(&self) -> usize {
        self.inner.iter().fold(0, |acc, col| {
            acc + col.iter().fold(0, |acc, loc_crossings| if *loc_crossings >= 2 { acc + 1 } else { acc })
        })
    }
}

use std::fmt;
impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for col in self.inner.iter() {
            for loc in col {
                if *loc > 0 {
                    write!(f, "{}", loc)?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn straight_lines(lines: &Vec<LineSegment>) -> usize {
    let mut board = Board::new();

    for line in lines {
        if line.horizontal() || line.vertical() {
            //println!("drawing line: {:?}", line);
            board.mark_line(line);
            //dbg!(&board);
        } else {
            //println!("skipping line because not horiz or vert: {:?}", line);
        }
    }

    board.crossings()
}

fn all_lines(lines: &Vec<LineSegment>) -> usize {
    let mut board = Board::new();

    for line in lines {
        board.mark_line(line);
    }

    board.crossings()
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

    let lines = input.lines()
        .map(|l| l.parse().expect("failed to parse line segment"))
        .collect();

    println!("part 1: {}", straight_lines(&lines));
    println!("part 2: {}", all_lines(&lines));
}
