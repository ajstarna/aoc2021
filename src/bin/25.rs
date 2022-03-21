use std::{thread, time};
use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};


#[derive(Debug, Copy, Clone)]
enum Cucumber {
    East,
    South,
}

struct Grid {
    pub spots: Vec<Option<Cucumber>>, // all the data
    pub width: usize, // keeps track of the width, i.e. when the next row begins in the data
    
}

impl Grid {
    pub fn new_from_file(file_prefix: &str) -> Self {
	let buffered = get_buffered_reader(file_prefix);
	let mut spots = vec![];
	// first go through each
	let mut width = None;
	for line in buffered.lines().flatten() {
	    if width.is_none() {
		// since every line has the same length, we can just figure out the
		// width once on the very first line and set width
		width = Some(line.len());
	    }
	    for val in line.chars() {
		match val {
		    '>' => spots.push(Some(Cucumber::East)),
		    'v' => spots.push(Some(Cucumber::South)),		    
		    '.' => spots.push(None),
		    _ => panic!("unknown char in file: {:?}", val)
		}
	    }
	}
	let width = width.unwrap();
	Self { spots, width }
    }

    pub fn pretty_print(&self) {
	for (i, val) in self.spots.iter().enumerate() {
	    let print_char = match val {
		Some(Cucumber::East) => '>',
		Some(Cucumber::South) => 'v',
		None => '.',		
	    };
	    print!("{}", print_char);
	    if i % self.width == self.width - 1 {
		println!();
	    }
	}
    }

    /// moves all south facing cucumbers (at once) and returns if anything was able to move
    fn run_east(&mut self) -> bool {
	let mut move_indices = vec![]; // which spots are clear to move to the right (and what is that spot - including wrap around)
	let mut any_moves = false;
	for (idx, val) in self.spots.iter().enumerate() {
	    if let Some(Cucumber::East) = val {
		let is_right_edge = idx % self.width == self.width - 1;
		let spot_to_move_into = match is_right_edge {
		    true => idx - (self.width - 1),
		    false => idx + 1,
		};
		if self.spots[spot_to_move_into].is_none() {
		    any_moves = true;
		    move_indices.push((idx, spot_to_move_into));
		}
	    }
	}

	// now actually do the moves   
	for (from, to) in move_indices {
	    self.spots[to] = self.spots[from];
	    self.spots[from] = None;
	}
	any_moves
    }

    /// moves all south facing cucumbers (at once) and returns if anything was able to move
    fn run_south(&mut self) -> bool {
	let mut move_indices = vec![]; // which spots are clear to move to the right (and what is that spot - including wrap around)
	let mut any_moves = false;
	for (idx, val) in self.spots.iter().enumerate() {
	    if let Some(Cucumber::South) = val {
		let is_bottom_row = idx >= self.spots.len() - self.width;			
		let spot_to_move_into = match is_bottom_row {
		    true => idx % self.width,
		    false => idx + self.width,
		};
		if self.spots[spot_to_move_into].is_none() {
		    any_moves = true;
		    move_indices.push((idx, spot_to_move_into));
		}
	    }
	}

	// now actually do the moves   
	for (from, to) in move_indices {
	    self.spots[to] = self.spots[from];
	    self.spots[from] = None;
	}
	any_moves
    }

    /// first move east, then south, and return if anything was able to move
    fn run_step(&mut self) -> bool {
	let any_east = self.run_east();
	let any_south = self.run_south();
	any_east || any_south
    }
}



fn run() {
    let mut grid = Grid::new_from_file("25");
    grid.pretty_print();

    let sleep_time = time::Duration::from_millis(300);
    for i in 1..1000 {
	let any_moves = grid.run_step();
	println!("after {} steps: moves = {}", i, any_moves);
	grid.pretty_print();
	//thread::sleep(sleep_time);
	if !any_moves {
	    break;
	}
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);    
    let part = &args[1];
    println!("part = {}", part);
    match part.as_str() {
	"1" => run(),
	"2" => run(),
	_ => panic!("invalid part number argument!"),
    }
}
