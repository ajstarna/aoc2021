use std::{env, cmp};
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,    
}

/// the grid vec represents a row x col 2d plane of values
struct Grid {
    grid: Vec<i32>,
    rows: usize,
    cols: usize,
}

impl Grid {

    fn new(rows: usize, cols: usize) -> Self {
	let mut grid = vec![0; rows * cols];   	
	Grid { grid, rows, cols }
    }
    
    /// given a line from point to point,
    /// we increment the values on the grid where the line intersects
    fn mark_grid(&mut self, line: &Line) {
	let Line{x1, y1, x2, y2} = line;
	if x1 == x2 || y1 == y2 {
	    // only consider horizontal or vertical lines
	}
    }

}

fn part1() {
    let buffered = get_buffered_reader(5);
    let mut lines = Vec::new(); // store all the lines as they are read
    let mut max_x = 0; // keep track of how big the grid will be
    let mut max_y = 0; // keep track of how big the grid will be
    for line in buffered.lines().flatten() {
	//println!("line = '{:?}'", line);
	let coords: Vec<&str> = line.split(" -> ").collect();
	let point1: Vec<i32> = coords[0].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
	let point2: Vec<i32> = coords[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
	if let [x1, y1] = point1[..] {
	    if let [x2, y2] = point2[..] {
		max_x = cmp::max(max_x, x1);
		max_x = cmp::max(max_x, x2);		    
		max_y = cmp::max(max_y, y1);
		max_y = cmp::max(max_y, y2);		    
		let line = Line {x1, y1, x2, y2};
		lines.push(line);
	    } else {
		println!("problem with line {:?}", line);
	    }
	} else {
		println!("problem with line {:?}", line);		    
	    }

    }
    println!("max x = {:?}, max y = {:?}", max_x, max_y);

    // we represent the grid as a vector of 0s in row-major
    // then we go through each line, and increment the values that the line interects
    let mut grid = Grid::new(max_y as usize, max_x as usize);
    for line in &lines {
	grid.mark_grid(line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);    
    let part = &args[1];
    println!("part = {}", part);
    match part.as_str() {
	"1" => part1(),
	"2" => part1(),
	_ => panic!("invalid part number argument!"),
    }
}
