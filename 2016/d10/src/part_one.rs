#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate petgraph;

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

fn load_input() -> String {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("input.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}", why.description()),
        Ok(_) => s,
    }
}

const SAMPLE1 : &'static str = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
";
const ANSWER1 : usize = 2;

#[derive(Debug)]
pub enum Give {
    Output,
    Bot(usize),
}

#[derive(Debug)]
pub enum Command {
    Input(usize, usize),
    Bot(usize, Give, Give),
}

peg_file! commands("grammar.rustpeg");

#[derive(Debug)]
enum EdgeType {
    Low,
    High,
    Input
}

#[derive(Debug)]
enum NodeType {
    Input(usize),
    Bot(Vec<usize>),
    Output
}

fn part_one(input: &str, c1: usize, c2: usize) -> usize {
    let c = commands::parse(input).unwrap();

    let mut fab = Graph::<NodeType, EdgeType>::new();
    let output = fab.add_node(NodeType::Output);
    let mut inputs = Vec::new();
    let mut bots = HashMap::new();
    for command in c {
        match command {
            Command::Input(value, to_bot) => {
                let n = fab.add_node(NodeType::Input(value));
                inputs.push(n);
                let b = bots.entry(to_bot).or_insert(fab.add_node(NodeType::Bot(Vec::new())));
                fab.update_edge(n, *b, EdgeType::Input);
            },
            Command::Bot(from_bot, low, high) => {
                bots.entry(from_bot).or_insert(fab.add_node(NodeType::Bot(Vec::new())));
                match low {
                    Give::Output => {
                        let f = bots.get(&from_bot).unwrap();
                        fab.update_edge(*f, output, EdgeType::Low);
                    },
                    Give::Bot(to_bot) => {
                        bots.entry(to_bot).or_insert(fab.add_node(NodeType::Bot(Vec::new())));
                        let f = bots.get(&from_bot).unwrap();
                        let t = bots.get(&to_bot).unwrap();
                        fab.update_edge(*f, *t, EdgeType::Low);
                    },
                }
                match high {
                    Give::Output => {
                        let f = bots.get(&from_bot).unwrap();
                        fab.update_edge(*f, output, EdgeType::High);
                    },
                    Give::Bot(to_bot) => {
                        bots.entry(to_bot).or_insert(fab.add_node(NodeType::Bot(Vec::new())));
                        let f = bots.get(&from_bot).unwrap();
                        let t = bots.get(&to_bot).unwrap();
                        fab.update_edge(*f, *t, EdgeType::High);
                    },
                }
            },
        }
    }
    for i in inputs {
        let n = fab.neighbors(i).collect::<Vec<_>>();
        assert_eq!(1, n.len());
        if let Some(&NodeType::Input(value)) = fab.node_weight(i) {
            if let Some(&mut NodeType::Bot(ref mut hold)) = fab.node_weight_mut(n[0]) {
                assert!(hold.len() < 2);
                hold.push(value);
            }
        }
    }
    loop {
        let k = find_full_bot(&bots, &fab);
        let node = bots.remove(&k).unwrap();
        let mut values = Vec::new();
        if let Some(&mut NodeType::Bot(ref mut held)) = fab.node_weight_mut(node) {
            println!("bot {} held {:?}", k, held);
            if held.contains(&c1) && held.contains(&c2) { return k; }
            values.append(held);
        }
        values.sort();
        let (low, high) = get_low_high_targets(&fab, &node);
        if let Some(&mut NodeType::Bot(ref mut held)) = fab.node_weight_mut(low) {
            held.push(values[0]);
            println!("held {:?}", held);
        }
        if let Some(&mut NodeType::Bot(ref mut held)) = fab.node_weight_mut(high) {
            held.push(values[1]);
            println!("held {:?}", held);
        }
    }
    unreachable!();
}

fn get_low_high_targets(fab: &petgraph::Graph<NodeType, EdgeType>, node: &NodeIndex) -> (NodeIndex, NodeIndex) {
    let mut v = vec![None, None];
    for edge in fab.edges(*node) {
        match *edge.weight() {
            EdgeType::Low => v[0] = Some(edge.target()),
            EdgeType::High => v[1] = Some(edge.target()),
            _ => panic!("bot cannot have an input edge"),
        }
    }
    (v[0].unwrap(), v[1].unwrap())
}

fn find_full_bot(bots: &HashMap<usize, petgraph::graph::NodeIndex>, fab: &petgraph::Graph<NodeType, EdgeType>) -> usize {
    *bots.iter().find(|&(_,&v)| {
        if let Some(&NodeType::Bot(ref hold)) = fab.node_weight(v) {
            hold.len() == 2
        } else {
            false
        }
    }).unwrap().0
}

fn main() {
    let input = load_input();
    assert_eq!(part_one(SAMPLE1, 2, 5), ANSWER1);
    println!("part one: {}", part_one(&input, 17, 61));
}
