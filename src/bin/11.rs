use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::HashSet;


struct Grid {
    nums: Vec<u32>, // all the data
    width: usize, // keeps track of the width, i.e. when the next row begins in the data
}

impl Grid {

    /// given an indix in the grid, returns a Vector containing all adjacent indices,
    /// including diagonal above and below. At most 8 possible adjacents if we are not on any edges.
    fn get_adjacent_indices(&self, idx: usize) -> Vec<usize> {
	let mut adjacents = Vec::new();
	let is_top_row = idx < self.width;
	let is_bottom_row = idx >= self.nums.len() - self.width;	
	let is_left_edge = idx % self.width == 0;
	let is_right_edge = idx % self.width == self.width - 1;

	if !is_left_edge {
	    adjacents.push(idx - 1);	    
	}
	if !is_right_edge {
	    adjacents.push(idx + 1);	    
	}
	
	if !is_top_row {
	    adjacents.push(idx - self.width);
	    if !is_left_edge {
		adjacents.push(idx - self.width - 1)
	    }
	    if !is_right_edge {
		adjacents.push(idx - self.width + 1)		
	    }
	}

	if !is_bottom_row {
	    adjacents.push(idx + self.width);
	    if !is_left_edge {
		adjacents.push(idx + self.width - 1)
	    }
	    if !is_right_edge {
		adjacents.push(idx + self.width + 1)		
	    }
	}
	adjacents
	
    }

    
    /// runs one full step of increased and flashes and returns the number of flashes that occured
    fn step(&mut self) -> usize {
	for num in &mut self.nums {
	    *num += 1
	}
	let flashed_indices = self.run_flashes();
	for idx in &flashed_indices {
	    self.nums[*idx] = 0;
	}
	flashed_indices.len()
    }
    
    fn run_flashes(&mut self) -> HashSet<usize> {

	let mut flashed_indices = HashSet::<usize>::new();
	let mut visit_queue = Vec::<usize>::new();

	// first determine which indices flashed just by the general +1 step increase
	for (i, num) in &mut self.nums.iter().enumerate() {
	    if num > &mut 9 {
		// we found a spot that should flash
		flashed_indices.insert(i);
		visit_queue.push(i);		    
	    }
	}

	while let Some(idx) = visit_queue.pop() {
	    let adjacents = self.get_adjacent_indices(idx);
	    for adj_idx in adjacents {
		self.nums[adj_idx] += 1;
		if self.nums[adj_idx] > 9 && !flashed_indices.contains(&adj_idx) {
		    // this is the first time that adjacent index flashes, so we add it to the set and the queue
		    flashed_indices.insert(adj_idx);
		    visit_queue.push(adj_idx);
		}
	    }
	}
	flashed_indices
    }
}

/// reads the file into a row-major vec and returns a Grid containing the numbers and the width
fn read_grid() -> Grid {
    let buffered = get_buffered_reader("11");
    let mut nums = Vec::new(); // to store the grid of number
    // first go through each
    let mut width = None;
    for line in buffered.lines().flatten() {
	if width.is_none() {
	    // since every line has the same length, we can just figure out the
	    // width once on the very first line and set width
	    width = Some(line.len());
	}
	for num in line.chars() {
	    nums.push(num.to_digit(10).unwrap());
	}
    }
    let width = width.unwrap();
    println!("nums = {:?}", nums);
    println!("width = {:?}", width);
    Grid { nums, width }
}

fn run(step_nums: u32) {
    let mut grid = read_grid();
    let mut total_flashes = 0;
    for i in 0..step_nums {
	let num_flashes = grid.step();
	total_flashes += num_flashes;
	if num_flashes == grid.nums.len() {
	    println!("all octopuses flashes on step = {:?}", i+1);
	}
    }
    dbg!(total_flashes);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);    
    let part = &args[1];
    let num_steps = &args[2].parse::<u32>().expect("unable to parse val as a u32");	            
    println!("part = {}", part);
    match part.as_str() {
	"1" => run(*num_steps),
	"2" => run(*num_steps),
	_ => panic!("invalid part number argument!"),
    }
}
