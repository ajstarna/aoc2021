use std::env;
use std::collections::HashMap;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;


/// a square grid in
struct Grid3d {
    pub vals: Vec<bool>, // all the data
    pub size: usize, // when the next row begins in the data. size^2 would be when the next sheet in the third dimension starts
}

impl Grid3d {
    /// return a new grid with all the same given value values for a given length and width
    pub fn new(size: usize, value: bool) -> Self {
	let vals = vec![value; usize::pow(size, 3)];
	Self {vals, size}
    }

    /// for each step, turn all the corresponding values to the step.status
    fn run_steps(&mut self, steps: &Vec<Step>) {
	for step in steps {
	    println!("\nstep = {:?}", step);
	    for z in step.z.0..=step.z.1 {
		let z_step = z * usize::pow(self.size, 2);
		for y in step.y.0..=step.y.1 {
		    let y_step = y * self.size;
		    for x in step.x.0..=step.x.1 {
			let idx = z_step + y_step + x;
			//println!("for {}, {}, {}, idx = {}", x-50, y-50, z-50, idx);
			self.vals[idx] = step.status;
		    }
		}
	    }
	}
    }
    /// returns how many of our values are true/on at the moment
    fn count_on(&self) -> usize {
	self.vals.iter().filter(|&&x| x).count()
    }
}

/// holds the steps as read in from the file
/// each field is a tuple that represents the range of cubes in that dimension to set
/// and status is on or off as a bool
#[derive(Debug)]
struct Step {
    status: bool,
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),    
}

impl Step {

    /// given ranges for the 3 dimensions, first we check that they are all within bounds at least partially
    /// then we return a Step with these ranges, shifted into actual indices. i.e. we need the indices to be in 0..=100,
    /// so we shift everything up by 50
    fn try_new(status: bool, x_start: i32, x_end: i32, y_start: i32, y_end: i32, z_start: i32, z_end: i32) -> Option<Self> {
	let shift = 50;
	if x_start + shift > 100 || x_end + shift < 0 ||
	    y_start + shift > 100 || y_end + shift < 0 ||
	    z_start + shift > 100 || z_end + shift < 0 {
		return None;
	    }
	
	Some (
	    Step {
		status: status,
		x: ((x_start + shift) as usize, (x_end + shift) as usize),
		y: ((y_start + shift) as usize, (y_end + shift) as usize),
		z: ((z_start + shift) as usize, (z_end + shift) as usize),		
	    }
	)
    }
}


/// for part two. we use a "sparse" grid that is just a hashmap actually, so no need for usize
#[derive(Debug)]
struct Step2 {
    status: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),    
}

impl Step2 {
    fn new(status: bool, x_start: i32, x_end: i32, y_start: i32, y_end: i32, z_start: i32, z_end: i32) -> Self {
	Self {
	    status: status,
	    x: (x_start, x_end),
	    y: (y_start, y_end),
	    z: (z_start, z_end),		
	}
    }
}

// read the steps in line by line
fn read_file<T>(part2: bool) -> Vec<T> {
    let buffered = get_buffered_reader("22-small");
    // e.g. on x=-20..26,y=-36..17,z=-47..7
    let re = Regex::new(r"(\w+) x=([-\d]+)..([-\d]+),y=([-\d]+)..([-\d]+),z=([-\d]+)..([-\d]+)").unwrap();
    let mut steps = Vec::new();
    
    for line in buffered.lines().flatten() {
	if let Some(caps) = re.captures(&line) {
	    let status = match &caps[1] {
		"on" => true,
		"off" => false,
		_ => panic!("invalid status on line: {}", line),
	    };
	    let x_start = caps[2].parse::<i32>().unwrap();
	    let x_end = caps[3].parse::<i32>().unwrap();
	    let y_start = caps[4].parse::<i32>().unwrap();
	    let y_end = caps[5].parse::<i32>().unwrap();
	    let z_start = caps[6].parse::<i32>().unwrap();
	    let z_end = caps[7].parse::<i32>().unwrap();

	    if part2 {
		let step = Step::new(status, x_start, x_end, y_start, y_end, z_start, z_end);
		steps.push(step);				    
	    } else {
		if let Some(step) = T::try_new(status, x_start, x_end, y_start, y_end, z_start, z_end) {
		    steps.push(step);		
		} else {
		    println!("range fully out of area in some dimension: {}", line);
		}
	    }


	} else {
	    panic!("cannot parse line: {}", line);
	}
    }    
    steps
}

fn run1( ) {
    // the grid goes from -50 to 50, i.e. 101 possible indices in each dimension
    let mut grid = Grid3d::new(101, false);
    let steps = read_file();
    println!("steps =\n{:?}", steps);
    grid.run_steps(&steps);
    println!("after the steps, there are {} cubes turned on", grid.count_on());
}

fn run2( ) {
    let grid = Grid3d::new(50, false);
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);    
    let part = &args[1];
    println!("part = {}", part);
    match part.as_str() {
	"1" => run1(),
	"2" => run2(),
	_ => panic!("invalid part number argument!"),
    }
}
