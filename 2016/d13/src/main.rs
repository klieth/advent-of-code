extern crate itertools;

use std::collections::{HashMap,VecDeque};
use itertools::Itertools;

struct Floor {
    cache: HashMap<(usize,usize), bool>,
    secret: usize,
}
impl Floor {
    fn new(secret: usize) -> Self {
        Floor {
            cache: HashMap::new(),
            secret: secret,
        }
    }
    fn is_space(&mut self, (x, y) : (usize, usize)) -> bool {
        *self.cache.entry( (x, y) ).or_insert({
            let val = x*x + 3*x + 2*x*y + y + y*y + self.secret;
            format!("{:b}", val).chars().fold(0, |acc, c| acc + if c == '1' { 1 } else { 0 }) % 2 == 0
        })
    }
}

fn motions(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut v = vec![ ((x + 1), y), (x, (y + 1)) ];
    if x > 0 { v.push( ((x - 1), y) ); }
    if y > 0 { v.push( (x, (y - 1)) ); }
    println!("{} {}, {:?}", x, y, v);
    v
}

fn part_one(input: usize, (dest_x, dest_y) : (usize, usize) ) -> isize {
    let mut floor = Floor::new(input);
    let mut queue = VecDeque::new();
    let mut visited = Vec::new();
    visited.push( (1, 1) );
    queue.push_back( (1, 1, 0) );
    loop {
        let (x, y, steps) = queue.pop_front().unwrap();
        visited.push( (x, y) );
        println!("stepped to {} {} in {} steps", x, y, steps);
        for (x, y) in motions(x, y) {
            if visited.contains( &(x, y) ) { continue; }
            if x == dest_x && y == dest_y { return steps + 1; }
            if floor.is_space( (x, y) ) {
                queue.push_back( (x, y, steps + 1) );
            }
        }
    }
}

fn part_two(input: usize, max_steps: usize) -> usize {
    let mut floor = Floor::new(input);
    let mut queue = VecDeque::new();
    let mut visited = Vec::new();
    let mut num = 0;
    visited.push( (1, 1) );
    queue.push_back( (1, 1, 0) );
    loop {
        let (x, y, steps) = queue.pop_front().unwrap();
        if steps > max_steps { return num; }
        num += 1;
        println!("stepped to {} {} in {} steps: num visited {}", x, y, steps, num);
        for (x, y) in motions(x, y) {
            if visited.contains( &(x, y) ) { continue; }
            if floor.is_space( (x, y) ) {
                visited.push( (x, y) );
                queue.push_back( (x, y, steps + 1) );
            }
        }
        println!("to_visit: {:?}", queue);
    }
}

fn main() {
    assert_eq!(part_one(10, (7, 4)), 11);
    println!("part one: {}", part_one(1352, (31, 39)));
    println!("10, 2");
    assert_eq!(part_two(10, 2), 5);
    println!("10, 3");
    assert_eq!(part_two(10, 3), 6);
    println!("10, 4");
    assert_eq!(part_two(10, 4), 9);
    println!("10, 5");
    assert_eq!(part_two(10, 5), 11);
    println!("part two: {}", part_two(1352, 50));
}
