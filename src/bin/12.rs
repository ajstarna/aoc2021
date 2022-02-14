use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::{HashMap, HashSet};

// we represent the graph as an adjacency matrix
#[derive(Debug)]
struct Graph {
    name_to_idx: HashMap<String, usize>,
}

impl Graph {

    fn new(mut node_names: Vec<String>) -> Self {
	let mut name_to_idx = HashMap::<String, usize>::new();
	node_names.sort(); // sort just to make sure the graph representation is deterministic
	for node in node_names {
	    name_to_idx.insert(node, name_to_idx.len());
	}
	Self {name_to_idx}
    }
    
    /// add an edge for two given nodes.
    fn add_edge(&mut self, a: String, b: String) {
	let idx_a = self.name_to_idx.get(&a).unwrap();
	let idx_b = self.name_to_idx.get(&b).unwrap();
	// need to have a 2d matrix to represent the edges, and set the value for grid[idx_a, idx_b] and vice versa to 1
	todo!();
    }
}

fn read_graph() -> Graph {
    let buffered = get_buffered_reader("12");
    let mut node_names =  HashSet::<String>::new();
    let mut read_lines = Vec::new();
    for line in buffered.lines().flatten() {
	for node in line.split('-') {
	    node_names.insert(String::from(node));
	}
	read_lines.push(line);
    }
    let node_names_vec = Vec::from_iter(node_names);
    let mut graph = Graph::new(node_names_vec);
    
    for line in read_lines {
	let pair: Vec<&str> = line.split('-').collect();
	graph.add_edge(String::from(pair[0]), String::from(pair[1]));
    }
    graph
}

fn part1() {
    let mut graph = read_graph();
    dbg!(&graph);
}

fn part2 () {
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);    
    let part = &args[1];
    println!("part = {}", part);
    match part.as_str() {
	"1" => part1(),
	"2" => part2(),
	_ => panic!("invalid part number argument!"),
    }
}
