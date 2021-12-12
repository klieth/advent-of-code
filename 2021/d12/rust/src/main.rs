#![feature(drain_filter)]

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use std::collections::{HashSet};
use std::cell::RefCell;
use std::rc::Rc;

trait DepthLogger {
    fn new() -> Self;
    fn indent(&self) -> Self;
    fn log(&self, name: &str);
}

struct DepthDebug(usize);

impl DepthLogger for DepthDebug {
    fn new() -> Self {
        DepthDebug(0)
    }

    fn indent(&self) -> Self {
        DepthDebug(self.0 + 1)
    }

    fn log(&self, name: &str) {
        println!("{}{}", self, name);
    }
}

use std::fmt;
impl fmt::Display for DepthDebug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.0 {
            write!(f, "{}", ' ')?;
        }

        Ok(())
    }
}

struct DepthNull;

impl DepthLogger for DepthNull {
    fn new() -> Self { DepthNull }
    fn indent(&self) -> Self { DepthNull }
    fn log(&self, _name: &str) { }
}

trait VisitTracker: Clone {
    fn with_start(start: Rc<Cave>) -> Self where Self: Sized;
    fn visit(&mut self, small_cave: Rc<Cave>);
    fn done_visiting(&self, small_cave: &Rc<Cave>) -> bool;
}

#[derive(Clone)]
struct VisitOnce(Vec<Rc<Cave>>);

impl VisitTracker for VisitOnce {
    fn with_start(start: Rc<Cave>) -> Self {
        VisitOnce(vec![start])
    }

    fn visit(&mut self, small_cave: Rc<Cave>) {
        self.0.push(small_cave);
    }

    fn done_visiting(&self, small_cave: &Rc<Cave>) -> bool {
        self.0.contains(small_cave)
    }
}

#[derive(Clone)]
struct VisitTwice(Vec<Rc<Cave>>, bool);

impl VisitTracker for VisitTwice {
    fn with_start(start: Rc<Cave>) -> Self {
        VisitTwice(vec![start], false)
    }

    fn visit(&mut self, small_cave: Rc<Cave>) {
        if self.0.contains(&small_cave) {
            self.1 = true;
        } else {
            self.0.push(small_cave);
        }
    }

    fn done_visiting(&self, small_cave: &Rc<Cave>) -> bool {
        // special-case start, since that can't be visited twice no matter what.
        if small_cave.name == "start" {
            return true;
        }

        if self.1 {
            self.0.contains(small_cave)
        } else {
            false
        }
    }
}

// Safety: This will have unbreakable cycles (i.e., there is no use of rc::Weak, and the data it
// loads may have Caves that point at each other). Therefore, this would have a memory leak if any
// of those cycles were to be removed from the Map, or if the Map itself went out of scope.
// Practically, for this program, this Map cannot have any Caves removed from it, and is itself not
// dropped until the program terminates.
struct Map(Rc<Cave>);

impl Map {
    fn contains(&self, name: &str) -> bool {
        if self.0.name == name {
            true
        } else {
            self.0.contains(name, Vec::new())
        }
    }

    fn get(&self, name: &str) -> Option<Rc<Cave>> {
        if self.0.name == name {
            Some(self.0.clone())
        } else {
            self.0.get(name, Vec::new())
        }
    }

    fn connect(&mut self, (c1, c2): (&str, &str)) {
        let c1 = match self.get(c1) {
            Some(c) => c,
            None => Rc::new(Cave::new(c1)),
        };

        let c2 = match self.get(c2) {
            Some(c) => c,
            None => Rc::new(Cave::new(c2)),
        };

        c1.connected.borrow_mut().insert(c2.clone());
        c2.connected.borrow_mut().insert(c1);
    }

    fn dfs_count_paths<T: VisitTracker, D: DepthLogger>(&self, target: &str) -> usize {
        self.0.dfs_count_paths(target, T::with_start(self.0.clone()), D::new())
    }

    fn dfs_get_paths<T: VisitTracker>(&self, target: &str) -> Vec<Vec<String>> {
        self.0.dfs_get_paths(target, T::with_start(self.0.clone()))
    }
}

impl std::convert::From<Cave> for Map {
    fn from(n: Cave) -> Map {
        Map(Rc::new(n))
    }
}

impl<'a> std::iter::FromIterator<(&'a str, &'a str)> for Map {
    fn from_iter<I: IntoIterator<Item=(&'a str, &'a str)>>(i: I) -> Map {
        let mut map: Map = Cave::new("start").into();

        let mut remaining: Vec<_> = i.into_iter().collect();

        while remaining.len() > 0 {
            let to_connect: Vec<_> = remaining.drain_filter(|(c1, c2)| map.contains(c1) || map.contains(c2)).collect();

            for conn in to_connect {
                map.connect(conn)
            }
        }

        map
    }
}

#[derive(PartialEq,Eq)]
enum CaveSize {
    Small,
    Large,
}

impl CaveSize {
    fn from_name(name: &String) -> Self {
        if name.chars().all(|c| c.is_ascii_uppercase()) {
            CaveSize::Large
        } else {
            CaveSize::Small
        }
    }
}

#[derive(PartialEq,Eq)]
struct Cave {
    name: String,
    size: CaveSize,
    connected: RefCell<HashSet<Rc<Cave>>>,
}

impl Cave {
    fn new<T: Into<String>>(name: T) -> Self {
        let name = name.into();
        let size = CaveSize::from_name(&name);

        Cave {
            name,
            size,
            connected: RefCell::new(HashSet::new()),
        }
    }

    fn contains(&self, name: &str, visited: Vec<Rc<Cave>>) -> bool {
        for c in self.connected.borrow().iter().filter(|c| !visited.contains(c)) {
            if c.name == name {
                return true;
            } else {
                let mut visited = visited.clone();
                visited.push(c.clone());
                if c.contains(name, visited) {
                    return true;
                }
            }
        }

        false
    }

    fn get(&self, name: &str, visited: Vec<Rc<Cave>>) -> Option<Rc<Cave>> {
        for c in self.connected.borrow().iter().filter(|c| !visited.contains(c)) {
            if c.name == name {
                return Some(c.clone());
            } else {
                let mut visited = visited.clone();
                visited.push(c.clone());
                match c.get(name, visited) {
                    x @ Some(_) => return x,
                    _ => {},
                }
            }
        }

        None
    }

    fn dfs_count_paths<T: VisitTracker, D: DepthLogger>(&self, target: &str, small_visited: T, depth: D) -> usize {
        depth.log(&self.name);

        if self.name == target {
            return 1;
        }

        self.connected.borrow().iter()
            .filter(|c| !small_visited.done_visiting(c))
            .fold(0, |acc, c| {
                let mut small_visited = small_visited.clone();

                if c.size == CaveSize::Small {
                    small_visited.visit(c.clone());
                }

                acc + c.dfs_count_paths(target, small_visited, depth.indent())
            })
    }

    fn dfs_get_paths<T: VisitTracker>(&self, target: &str, small_visited: T) -> Vec<Vec<String>> {
        if self.name == target {
            return vec![vec![self.name.clone()]];
        }

        self.connected.borrow().iter()
            .filter(|c| !small_visited.done_visiting(c))
            .map(|c| {
                let mut small_visited = small_visited.clone();

                if c.size == CaveSize::Small {
                    small_visited.visit(c.clone());
                }

                let mut paths = c.dfs_get_paths(target, small_visited);

                for p in paths.iter_mut() {
                    p.push(self.name.clone());
                }

                paths
            })
            .collect::<Vec<_>>().concat()
    }
}

impl std::hash::Hash for Cave {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        self.name.hash(state);
    }
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

    let map: Map = input.lines()
        .map(|l| {
            let mut parts = l.split('-');
            (parts.next().expect("line was empty"), parts.next().expect("line didn't have a '-' character"))
        })
        .collect();

    /*
    let mut paths = map.dfs_get_paths::<VisitTwice>("end").into_iter()
        .map(|path| path.into_iter().rev().collect::<Vec<_>>().join(","))
        .collect::<Vec<_>>();

    paths.sort();

    for path in paths {
        println!("{}", path);
    }
    */

    let now = Instant::now();
    println!("part 1: {}", map.dfs_count_paths::<VisitOnce, DepthNull>("end"));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);

    let now = Instant::now();
    println!("part 2: {}", map.dfs_count_paths::<VisitTwice, DepthNull>("end"));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);
}
