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

#[derive(Debug)]
pub enum Give {
    Output(usize),
    Bot(usize),
}

#[derive(Debug)]
pub enum Command {
    Input(usize, usize),
    Bot(usize, Give, Give),
}

peg_file! commands("grammar2.rustpeg");

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
    Output(Option<usize>),
}

fn part_two(input: &str, o1: usize, o2: usize, o3: usize) -> usize {
    let c = commands::parse(input).unwrap();

    let mut fab = Graph::<NodeType, EdgeType>::new();
    let mut inputs = Vec::new();
    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();
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
                    Give::Output(to_out) => {
                        let f = bots.get(&from_bot).unwrap();
                        let o = outputs.entry(to_out).or_insert(fab.add_node(NodeType::Output(None)));
                        fab.update_edge(*f, *o, EdgeType::Low);
                    },
                    Give::Bot(to_bot) => {
                        bots.entry(to_bot).or_insert(fab.add_node(NodeType::Bot(Vec::new())));
                        let f = bots.get(&from_bot).unwrap();
                        let t = bots.get(&to_bot).unwrap();
                        fab.update_edge(*f, *t, EdgeType::Low);
                    },
                }
                match high {
                    Give::Output(to_out) => {
                        let f = bots.get(&from_bot).unwrap();
                        let o = outputs.entry(to_out).or_insert(fab.add_node(NodeType::Output(None)));
                        fab.update_edge(*f, *o, EdgeType::High);
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
            values.append(held);
        }
        values.sort();
        let (low, high) = get_low_high_targets(&fab, &node);
        match fab.node_weight_mut(low) {
            Some(&mut NodeType::Bot(ref mut held)) => {
                held.push(values[0]);
            },
            Some(&mut NodeType::Output(ref mut out)) => {
                *out = Some(values[0]);
                println!("{:?}", out);
            }
            _ => {}
        }
        match fab.node_weight_mut(high) {
            Some(&mut NodeType::Bot(ref mut held)) => {
                held.push(values[1]);
            },
            Some(&mut NodeType::Output(ref mut out)) => {
                *out = Some(values[1]);
            }
            _ => {}
        }
        if let Some(&NodeType::Output(Some(i))) = fab.node_weight(*outputs.get(&0).unwrap()) {
            if let Some(&NodeType::Output(Some(j))) = fab.node_weight(*outputs.get(&1).unwrap()) {
                if let Some(&NodeType::Output(Some(k))) = fab.node_weight(*outputs.get(&2).unwrap()) {
                    return i * j * k;
                }
            }
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
    println!("part two: {}", part_two(&input, 0, 1, 2));
}
