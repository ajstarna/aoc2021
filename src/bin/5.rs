use std::{env, cmp};
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};

#[derive(Debug)]
enum LineType {
    Horizontal,
    Vertical,
    DiagonalRight, // down to the right
    DiagonalLeft, // down to the left
}

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    line_type: LineType,
}

impl Line {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
	let line_type: LineType;
	if x1 == x2 {
	    // vertical line
	    line_type = LineType::Vertical;
	} else if y1 == y2 {
	    // horizontal line
	    line_type = LineType::Horizontal;
	} else if (x2 > x1 && y2 > y1) ||  (x2 < x1 && y2 < y1)  {
	    // if one of the points has a bigger y AND x value than the other other point,
	    // then we know the line is angled down and to the right
	    line_type = LineType::DiagonalRight;
	} else {
	    line_type = LineType::DiagonalLeft;    	    
	}
	
	Line { x1, y1, x2, y2, line_type }
    }
}

/// the grid vec represents a row x col 2d plane of values
struct Grid {
    grid: Vec<i32>,
    rows: usize,
    cols: usize,
}

impl Grid {

    fn new(rows: usize, cols: usize) -> Self {
	let grid = vec![0; rows * cols];   	
	Grid { grid, rows, cols }
    }
			
    /// given a line from point to point,
    /// we increment the values on the grid where the line intersects
    fn mark_grid(&mut self, line: &Line, allow_diagonal: bool) {
	// println!("about to mark line: {:?}", line);
	let Line{x1, y1, x2, y2, line_type} = line;
	let mut start_idx = y1 * self.cols + x1;
	let mut end_idx = y2 * self.cols + x2;
	if start_idx > end_idx {
	    // we always want to iterate from smaller to bigger
	    (start_idx, end_idx) = (end_idx, start_idx);
	}
	// println!("start_idx: {:?}, end_idx: {:?}", start_idx, end_idx);
	match line_type {
	    LineType::Vertical => {
		for i in (start_idx..=end_idx).step_by(self.cols) {
		    self.grid[i] += 1;
		}
		// self.print();
	    }
	    LineType::Horizontal => {	    
		for i in start_idx..=end_idx {
		    self.grid[i] += 1;
		}
		// self.print();
	    }
	    LineType::DiagonalRight => {	    	    
		// we know we have a 45 degree angle
		// step by a full row, then 1 more
		if allow_diagonal {
		    for i in (start_idx..=end_idx).step_by(self.cols + 1) {
			self.grid[i] += 1;
		    }
		    // self.print();
		}
	    }
	    LineType::DiagonalLeft => {
		// we know we have a 45 degree angle
		// step by a full row, then 1 less
		if allow_diagonal {
		    for i in (start_idx..=end_idx).step_by(self.cols - 1) {
			self.grid[i] += 1;
		    }
		    // self.print();
		}
	    }
	    
	}
	
    }

    /// returns how many individual squares on the grid have overlapping lines,
    /// i.e. how many have a count of at least 2
    fn count_overlap(&self) -> usize {
	self.grid.iter().filter(|&x| *x >= 2).count()
    }


    /// helper method for displaying the grid.
    /// infeasible to use when we have the full sized input file
    fn print(&self) {
	for (i, item) in self.grid.iter().enumerate() {
	    if i % self.cols == 0 {
		println!();
	    }
	    print!("{:?} ", item);
	}
	println!();
    }
    
}

			
/// allow_diagonal says if we mark diagonal lines on the grid along with horizontal and vertical
fn run(allow_diagonal: bool) {
    let buffered = get_buffered_reader("5");
    let mut lines = Vec::new(); // store all the lines as they are read
    let mut max_x = 0; // keep track of how big the grid will be
    let mut max_y = 0; // keep track of how big the grid will be
    for line in buffered.lines().flatten() {
	//println!("line = '{:?}'", line);
	let coords: Vec<&str> = line.split(" -> ").collect();
	let point1: Vec<usize> = coords[0].split(',').map(|x| x.parse::<usize>().unwrap()).collect();
	let point2: Vec<usize> = coords[1].split(',').map(|x| x.parse::<usize>().unwrap()).collect();
	if let [x1, y1] = point1[..] {
	    if let [x2, y2] = point2[..] {
		max_x = cmp::max(max_x, x1);
		max_x = cmp::max(max_x, x2);		    
		max_y = cmp::max(max_y, y1);
		max_y = cmp::max(max_y, y2);		    
		let line = Line::new(x1, y1, x2, y2);
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
    // we add one to the max x and y because we also use the 0,0 coord
    // then we go through each line, and increment the values that the line interects
    let mut grid = Grid::new((max_y + 1) as usize, (max_x +1) as usize);
    for line in &lines {
	grid.mark_grid(line, allow_diagonal);
    }

    println!("after marking all the lines on the grid, there are {} tiles with at least two lines overlapping", grid.count_overlap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);    
    let part = &args[1];
    println!("part = {}", part);
    match part.as_str() {
	"1" => run(false),
	"2" => run(true),
	_ => panic!("invalid part number argument!"),
    }
}
