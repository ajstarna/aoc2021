use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::{HashMap, HashSet};

// we represent the graph as an adjacency matrix
#[derive(Debug)]
struct Graph {
    matrix: Vec<bool>,
    num_nodes: usize,
    idx_to_name: HashMap<usize, String>,
    name_to_idx: HashMap<String, usize>,    
    capital_indices: HashSet<usize>,
    start_idx: usize,
    end_idx: usize,
    all_paths: HashSet<String>,
}

impl Graph {

    fn new(mut node_names: Vec<String>) -> Self {
	node_names.sort(); // sort just to make sure the graph representation is deterministic	
	let num_nodes = node_names.len();
	let matrix = vec![false; num_nodes * num_nodes];
	let mut idx_to_name = HashMap::<usize, String>::new();
	let mut name_to_idx = HashMap::<String, usize>::new();
	let mut capital_indices = HashSet::new();
	let mut start_idx_opt = None;
	let mut end_idx_opt = None;	
	for (idx, node) in node_names.iter().enumerate() {
	    idx_to_name.insert(idx, node.to_string());
	    name_to_idx.insert(node.clone(), idx);	    
	    if node == "start" {
		start_idx_opt = Some(idx);
	    } else if node == "end" {
		end_idx_opt = Some(idx);
	    }
	    if &node.to_uppercase() == node {
		capital_indices.insert(idx);
	    }
	    
	}
	let start_idx = start_idx_opt.unwrap();
	let end_idx = end_idx_opt.unwrap();
	let all_paths = HashSet::new();
	Self {matrix, num_nodes, idx_to_name, name_to_idx, capital_indices, start_idx, end_idx, all_paths}
    }
    
    /// add an edge for two given nodes.
    fn add_edge(&mut self, a: String, b: String) {
	let idx_a = self.name_to_idx.get(&a).unwrap();
	let idx_b = self.name_to_idx.get(&b).unwrap();
	// need to have a 2d matrix to represent the edges, and set the value for grid[idx_a, idx_b] and vice versa to 1
	self.matrix[idx_a * self.num_nodes + idx_b] = true;
	self.matrix[idx_b * self.num_nodes + idx_a] = true;	
    }

    /// given a path of indices, prints it out as node names
    fn pretty_path(&self, path: &Vec<usize>) {
	println!();
	//let path_string: Vec<String> = path.iter().map(|x| *self.idx_to_name.get(x).unwrap().clone()).collect();//   .join("-");	
	//println!("{:?}", path_string);
	for idx in path {
	    let name = self.idx_to_name.get(idx).unwrap();
	    print!("{}-", name);
	    //names.push(&"-".to_string());	    
	}
    }

    /*
    /// given a path of indices, add it as one string to self.all_paths
    fn add_to_all_paths(&self, path: &Vec<usize>) {
	let path_string = path.join("-");
    }*/
    
    fn print(&self) {
	for (i, val) in self.matrix.iter().enumerate() {
	    let num = match val {
		true => 1,
		false => 0,
	    };
	    print!("{} ", num);
	    if i % self.num_nodes == self.num_nodes - 1 {
		println!();
	    }
	}
    }

    fn traverse_from_start(&self) -> u32 {
	let mut starting_path = Vec::new();
	starting_path.push(self.start_idx);
	let mut starting_cannot_return = HashSet::new();	
	self.traverse(starting_path, starting_cannot_return, None)
    }

    // here we count paths where any single small letter cave occurs EXACTLY twice
    fn traverse_from_start_special_small(&self) -> u32 {
	let mut starting_path = Vec::new();
	starting_path.push(self.start_idx);
	let mut starting_cannot_return = HashSet::new();
	let mut all_special_paths = 0;
	for idx in 0..self.num_nodes {
	    if idx != self.start_idx && idx != self.end_idx && !self.capital_indices.contains(&idx) {
		println!("\n\nabout to run with special index = {:?}", idx);
		let special_path = starting_path.clone();
		let special_cannot_return = starting_cannot_return.clone();
		let current = self.traverse(special_path, special_cannot_return, Some(idx));
		dbg!(current);
		all_special_paths += current;
	    }
	}
	all_special_paths
    }
    
    /// returns how many paths we can find starting from the given path that we have already traversed,
    /// where we can visit capital caves as many times as we want, but a lower case node at most once
    /// cannot_return is a set of indices that are not valid to return to, i.e. lower case caves that have
    /// already been visited
    /// If a special idx is provided, then that index is allowed to be visited up to twice
    fn traverse(&self, path: Vec<usize>, mut cannot_return: HashSet<usize>, special_idx: Option<usize>) -> u32 {
	//println!("path = {:?}, cannot_return = {:?}", path, cannot_return);
	if path.is_empty() {
	    return 0;
	}
	
	let last_idx = path.last().unwrap();
	//dbg!(last_idx);

	if last_idx == &self.end_idx {
	    // we are done
	    self.pretty_path(&path);
	    if let Some(idx) = special_idx {
		// if we had a special index, then we only count this path if that idx occurs twice in it
		let occurence = path.iter().filter(|&x| x == &idx).count();
		if occurence == 2 {
		    // a valid path
		    return 1;
		} else {
		    return 0;
		}
	    } else {
		return 1;
	    }
	}
	
	if !self.capital_indices.contains(&last_idx) {
	    // if the current cave is not capital, then likely indicate that we can never return
	    if let Some(idx) = special_idx {

		if idx == *last_idx {
		    // if we are visiting the special index, 
		    if path[0..path.len() -1].iter().any(|&x| x == idx) {
			// and we are already in the path, then cannot return
			cannot_return.insert(*last_idx);
		    }
		} else {
		    // we are not the special index
		    cannot_return.insert(*last_idx);		    
		}
	    } else {
		// no special index to check
		cannot_return.insert(*last_idx);
	    }
	}
	
	let row_start = last_idx * self.num_nodes;
	let mut num_paths = 0;		
	for (idx, &adj) in self.matrix[row_start..row_start + self.num_nodes].iter().enumerate() {
	    //println!("{}, {}", idx, adj);
	    if adj && !cannot_return.contains(&idx) {
		// we have a cave that we would like to go visit
		let mut sub_path = path.clone();
		sub_path.push(idx);
		let sub_cannot_return = cannot_return.clone();
		let sub_num_paths = self.traverse(sub_path, sub_cannot_return, special_idx);
		//println!("sub path: {:?}, sub_cannot: {:?}, sub_num_paths = {:?}", sub_path, sub_cannot_return, sub_num_paths);
		num_paths += sub_num_paths;
	    }
	}
	num_paths
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
    graph.print();
    println!("graph = {:?}", graph);
    let num_paths = graph.traverse_from_start();
    dbg!(num_paths);
}

fn part2 () {
    let mut graph = read_graph();
    graph.print();
    println!("graph = {:?}", graph);
    let num_paths = graph.traverse_from_start_special_small() + graph.traverse_from_start();
    dbg!(num_paths);
    
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
