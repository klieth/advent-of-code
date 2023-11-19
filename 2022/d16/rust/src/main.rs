use std::cell::RefCell;
use std::collections::{
    BTreeSet,
    HashMap,
    hash_map::Entry,
    HashSet,
    VecDeque,
};
use std::num::ParseIntError;
use std::rc::{Rc, Weak};

const TEST: &'static str = include_str!("../../test");
const INPUT: &'static str = include_str!("../../input");

#[derive(Debug)]
enum Day16Error {
    ParseError(String),
}

impl From<ParseIntError> for Day16Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseError(format!("{}", error))
    }
}

#[derive(Debug)]
struct Valve {
    name: String,
    rate: usize,
    connections: RefCell<Vec<Weak<Valve>>>,
}

impl Valve {
    fn new(name: String, rate: usize) -> Self {
        Self {
            name, rate,
            connections: RefCell::new(Vec::new()),
        }
    }
}

fn parse(raw: &str) -> Result<HashMap<String, Rc<Valve>>, Day16Error> {
    let mut valves = HashMap::new();
    let mut to_connect = Vec::new();

    for line in raw.lines() {
        let line = line.strip_prefix("Valve ")
            .ok_or_else(|| Day16Error::ParseError(format!("line did not start with 'Valve '")))?;

        let (valve_def, conns) = line.split_once("; ")
            .ok_or_else(|| Day16Error::ParseError(format!("line could not be split on '; '")))?;

        let (name, rate_raw) = valve_def.split_once(" has flow rate=")
            .ok_or_else(|| Day16Error::ParseError(format!("valve_def could not be split")))?;

        let conns = conns.strip_prefix("tunnels lead to valves ")
            .or_else(|| conns.strip_prefix("tunnel leads to valve "))
            .ok_or_else(|| Day16Error::ParseError(format!("conns had wrong prefix")))?;

        valves.insert(name.to_string(), Rc::new(Valve::new(name.to_string(), rate_raw.parse()?)));
        to_connect.push( (name.to_string(), conns) );
    }

    for (src, conns_raw) in to_connect {
        let src = &valves[&src];
        for conn in conns_raw.split(", ") {
            let conn = valves.get(conn)
                .ok_or_else(|| Day16Error::ParseError(format!("could not connect {}: no valve named {}", src.name, conn)))?;
            src.connections.borrow_mut().push(Rc::downgrade(conn));
        }
    }

    Ok(valves)
}

fn dijkstra(start: &Rc<Valve>, distances: &mut HashMap<String, usize>) {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(Rc::clone(start));

    if let Entry::Vacant(v) = distances.entry(start.name.clone()) {
        v.insert(0);
    } else {
        unreachable!("the distances map should always be empty");
    }

    while let Some(u) = to_visit.pop_front() {
        for v in u.connections.borrow().iter() {
            // unwrap: valve references are never changed such that they become invalid
            let v = v.upgrade().unwrap();

            let alt = distances[&u.name] + 1;
            distances.entry(v.name.clone())
                .and_modify(|d| {
                    if alt < *d {
                        *d = alt;
                        to_visit.push_back(Rc::clone(&v));
                    }
                }).or_insert_with(|| {
                    to_visit.push_back(Rc::clone(&v));
                    alt
                });
        }
    }
}

struct State {
    current_valve: Rc<Valve>,
    opened: BTreeSet<String>,
    elapsed: usize,
    released: usize,
}

fn part1(valves: &HashMap<String, Rc<Valve>>) -> usize {
    let flowing: Vec<_> = valves.iter()
        .filter_map(|(_, v)| if v.rate > 0 { Some(Rc::clone(v)) } else { None })
        .collect();

    let mut distances: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for (name, valve) in valves {
        let destinations = distances.entry(name.clone()).or_insert_with(|| HashMap::new());
        dijkstra(valve, destinations);
    }

    let mut search_states: VecDeque<_> = vec![State {
        current_valve: valves["AA"].clone(),
        opened: BTreeSet::new(),
        elapsed: 0,
        released: 0,
    }].into();

    let mut seen = HashSet::new();
    seen.insert((BTreeSet::new(), 0, 0));

    let mut max_released = 0;

    while let Some(State { current_valve, opened, elapsed, released }) = search_states.pop_front() {
        if opened.len() == flowing.len() {
            let release_rate: usize = flowing.iter().map(|v| v.rate).sum();
            let total_released = released + (release_rate * (30 - elapsed));
            max_released = max_released.max(total_released);
            continue;
        }

        let unopened = flowing.iter().filter(|v| !opened.contains(&v.name));
        for dest in unopened {
            // extra +1 to account for opening the valve
            let move_cost = distances[&current_valve.name][&dest.name] + 1;
            let new_elapsed = elapsed + move_cost;

            if new_elapsed >= 30 {
                let release_rate: usize = opened.iter().map(|v| valves[v].rate).sum();
                let total_released = released + (release_rate * (30 - elapsed));
                max_released = max_released.max(total_released);
                continue;
            }

            let release_rate: usize = opened.iter().map(|v| valves[v].rate).sum();
            let new_released = released + (release_rate * move_cost);

            let mut new_opened = opened.clone();
            new_opened.insert(dest.name.clone());

            if seen.insert((new_opened.clone(), new_elapsed, new_released)) {
                search_states.push_back(State {
                    current_valve: Rc::clone(dest),
                    opened: new_opened,
                    elapsed: new_elapsed,
                    released: new_released,
                });
            }
        }
    }

    max_released
}

fn part2(valves: &HashMap<String, Rc<Valve>>) -> usize {
    let flowing: Vec<_> = valves.iter()
        .filter_map(|(_, v)| if v.rate > 0 { Some(Rc::clone(v)) } else { None })
        .collect();

    let mut distances: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for (name, valve) in valves {
        let destinations = distances.entry(name.clone()).or_insert_with(|| HashMap::new());
        dijkstra(valve, destinations);
    }

    let mut search_states: VecDeque<_> = vec![State {
        current_valve: valves["AA"].clone(),
        opened: BTreeSet::new(),
        elapsed: 0,
        released: 0,
    }].into();

    let mut seen = HashSet::new();
    seen.insert((BTreeSet::new(), 0, 0));

    let mut max_released = 0;

    while let Some(State { current_valve, opened, elapsed, released }) = search_states.pop_front() {
        if opened.len() == flowing.len() {
            let release_rate: usize = flowing.iter().map(|v| v.rate).sum();
            let total_released = released + (release_rate * (30 - elapsed));
            max_released = max_released.max(total_released);
            continue;
        }

        let unopened = flowing.iter().filter(|v| !opened.contains(&v.name));
        for dest in unopened {
            // extra +1 to account for opening the valve
            let move_cost = distances[&current_valve.name][&dest.name] + 1;
            let new_elapsed = elapsed + move_cost;

            if new_elapsed >= 30 {
                let release_rate: usize = opened.iter().map(|v| valves[v].rate).sum();
                let total_released = released + (release_rate * (30 - elapsed));
                max_released = max_released.max(total_released);
                continue;
            }

            let release_rate: usize = opened.iter().map(|v| valves[v].rate).sum();
            let new_released = released + (release_rate * move_cost);

            let mut new_opened = opened.clone();
            new_opened.insert(dest.name.clone());

            if seen.insert((new_opened.clone(), new_elapsed, new_released)) {
                search_states.push_back(State {
                    current_valve: Rc::clone(dest),
                    opened: new_opened,
                    elapsed: new_elapsed,
                    released: new_released,
                });
            }
        }
    }

    max_released
}

fn main() -> Result<(), Day16Error> {
    let test = parse(TEST)?;
    let input = parse(INPUT)?;
    /*
    println!("part1 (test): {}", part1(&test));
    println!("part1 (actual): {}", part1(&input));
    */
    println!("part2 (test): {}", part2(&test));
    println!("part2 (actual): {}", part2(&input));
    Ok( () )
}
