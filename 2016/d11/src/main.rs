extern crate itertools;
use itertools::Itertools;
use std::hash::{Hash,Hasher};

const LOG : bool = false;
const ELEMENTS : usize = 10;

#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord)]
enum Element {
    Hydrogen,
    Lithium,
    Strontium,
    Plutonium,
    Thulium,
    Ruthenium,
    Curium,
    Promethium,
    Elerium,
    Dilithium,
}

#[derive(Clone,Debug,Hash,PartialEq,Eq,PartialOrd,Ord)]
enum Item {
    Generator(Element),
    Microchip(Element),
    Elevator,
}

fn clone_building(b: &Vec<Vec<Item>>, e: usize, d: usize) -> Vec<Vec<Item>> {
    let mut b = b.clone();
    let ef = b.remove(e);
    b.insert(e, ef.clone());
    let df = b.remove(d);
    b.insert(d, df.clone());
    b
}

fn test_floor(f: &Vec<Item>) -> bool {
    use Item::*;
    if f.iter().any(|i| if let Generator(_) = *i { true } else { false }) {
        let mut chips = f.iter().filter(|&i| if let Microchip(_) = *i { true } else { false });
        chips.all(|c| {
            if let Microchip(ref t) = *c {
                if f.contains(&Generator(t.clone())) {
                    true
                } else {
                    false
                }
            } else {
                unreachable!()
            }
        })
    } else {
        true
    }
}

struct FloorCombinations {
    max: usize,
    one: usize,
    two: usize,
    first: bool,
}
impl FloorCombinations {
    fn new(max: usize) -> Self {
        FloorCombinations {
            max: max,
            one: 0,
            two: 0,
            first: true,
        }
    }
}
impl Iterator for FloorCombinations {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first { self.first = false; return Some( (self.one, self.two) ); }
        self.two += 1;
        if self.two >= self.max {
            self.one += 1;
            self.two = self.one;
            if self.one >= self.max {
                return None;
            }
        }
        Some( (self.one, self.two) )
    }
}

fn hash_building(building: &Vec<Vec<Item>>) -> u64 {
    use Item::*;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let mut to_hash : Vec<_> = (0..ELEMENTS + 1).map(|_| vec![0, 0]).collect();
    let mut elevator = 0;
    for (floor_no, floor_vec) in building.into_iter().enumerate() {
        for item in floor_vec {
            match *item {
                Generator(ref el) => to_hash[el.clone() as usize][0] = floor_no,
                Microchip(ref el) => to_hash[el.clone() as usize][1] = floor_no,
                Elevator => elevator = floor_no,
            }
        }
    }
    to_hash.sort();
    to_hash.push(vec![elevator]);
    if LOG { println!("to_hash {:?}", to_hash); }
    to_hash.hash(&mut hasher);
    hasher.finish()
}

fn space(amt: usize) -> String {
    std::iter::repeat(" ").take(amt).collect()
}

fn print_building(building: &Vec<Vec<Item>>, elevator: usize, steps: usize) {
    println!("{}STEP {}", space(steps * 2), steps + 1);
    for (number, floor) in building.iter().enumerate().rev() {
        print!("{}F{}{}: ", space(steps * 2), number+1, if number == elevator { "*" } else { " " } );
        for item in floor {
            print!("{:?} ", item);
        }
        println!("");
    }
}

fn test_finished(building: &Vec<Vec<Item>>) -> bool {
    (0..3).all(|i| building[i].len() == 0)
}

fn part_one(start_building: Vec<Vec<Item>>) -> usize {
    let mut queue = std::collections::VecDeque::new();
    let mut visited = Vec::new();
    let initial_hash = hash_building(&start_building);
    if LOG { println!("initial_hash {}", initial_hash); }
    visited.push(initial_hash);
    queue.push_back( (start_building, 0, 0) );
    let mut curr_steps = 0;
    loop {
        let (building, elevator, steps) = queue.pop_front().unwrap();
        if curr_steps <= steps { println!("steps: {}", steps); curr_steps = steps + 1; }
        if LOG { print_building(&building, elevator, steps); }
        let mut destinations = Vec::new();
        if elevator > 0 { destinations.push(elevator - 1); }
        if elevator < 3 { destinations.push(elevator + 1); }
        for destination in destinations {
            for (i, j) in FloorCombinations::new(building[elevator].len()) {
                let mut b = clone_building(&building, elevator, destination);
                if i == j {
                    let item = b[elevator].remove(i);
                    if LOG { println!("{}Moving one item: {:?}", space(steps * 2), item); }
                    b[destination].push(item);
                } else {
                    let item = b[elevator].remove(i);
                    if LOG { println!("{}Moving two items: {:?}", space(steps * 2), item); }
                    b[destination].push(item);
                    // all the items have shifted left, so we shift our index left
                    let item = b[elevator].remove(j - 1);
                    if LOG { println!("{}                : {:?}", space(steps * 2), item); }
                    b[destination].push(item);
                }
                if test_floor(&b[elevator]) && test_floor(&b[destination]) {
                    if LOG { print_building(&b, destination, steps + 1); }
                    if test_finished(&b) { return steps + 1 };
                    b[destination].push(Item::Elevator);
                    let h = hash_building(&b);
                    b[destination].pop();
                    if LOG { println!("Hashed to {}", h); }
                    if !visited.contains(&h) {
                        visited.push(h);
                        queue.push_back( (b, destination, steps + 1) );
                        if LOG { println!("adding"); }
                    } else {
                        if LOG { println!("NOT adding"); }
                    }
                }
            }
        }
    }
}

fn main() {
    use Item::*;
    use Element::*;
    let input = vec![
        vec![ Microchip(Hydrogen), Microchip(Lithium) ],
        vec![ Generator(Hydrogen) ],
        vec![ Generator(Lithium) ],
        vec![],
    ];
    println!("part one sample {}", part_one(input));
    let input = vec![
        vec![Generator(Thulium), Microchip(Thulium), Generator(Plutonium), Generator(Strontium)],
        vec![Microchip(Plutonium), Microchip(Strontium)],
        vec![Generator(Promethium), Microchip(Promethium), Generator(Ruthenium), Microchip(Ruthenium)],
        vec![],
    ];
    println!("part one other {}", part_one(input));
    let input = vec![
        vec![Generator(Strontium), Microchip(Strontium), Generator(Plutonium), Microchip(Plutonium)],
        vec![Generator(Thulium), Generator(Ruthenium), Microchip(Ruthenium), Generator(Curium), Microchip(Curium)],
        vec![Microchip(Thulium)],
        vec![],
    ];
    println!("part one {}", part_one(input));
    let input = vec![
        vec![Generator(Strontium), Microchip(Strontium), Generator(Plutonium), Microchip(Plutonium), Generator(Elerium), Microchip(Elerium), Generator(Dilithium), Microchip(Dilithium)],
        vec![Generator(Thulium), Generator(Ruthenium), Microchip(Ruthenium), Generator(Curium), Microchip(Curium)],
        vec![Microchip(Thulium)],
        vec![],
    ];
    println!("part two {}", part_one(input));
}
