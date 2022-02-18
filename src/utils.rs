use std::fs::File;
use std::io::{BufReader, BufRead};

/// gives a buffered reader to iterate over for the lines of a file
/// {day}.txt in the data folder.
pub fn get_buffered_reader(day: &str) -> BufReader<File>{
    let path = format!("data/{}.txt", day);
    let input = File::open(path).expect("could not open file!");
    BufReader::new(input)
}


/// Grid contains a row-major vec and the width of one row
#[derive(Debug)]
pub struct Grid<T> {
    nums: Vec<T>, // all the data
    pub width: usize, // keeps track of the width, i.e. when the next row begins in the data
}

impl<T: std::clone::Clone + From<u32>> Grid<T> {

    pub fn len(&self) -> usize {
	self.nums.len()
    }
    
    /// reads the given file and returns a grid. We assume each position is a single numeric haracter 
    pub fn new_from_file(file_prefix: &str) -> Self {
	let buffered = get_buffered_reader(file_prefix);
	let mut nums = Vec::<T>::new(); // to store the grid of number
	// first go through each
	let mut width = None;
	for line in buffered.lines().flatten() {
	    if let None = width {
		// since every line has the same length, we can just figure out the
		// width once on the very first line and set width
		width = Some(line.len());
	    }
	    for num in line.chars() {
		nums.push(T::from(num.to_digit(10).unwrap()));
	    }
	}
	let width = width.unwrap();
	Self { nums, width }
    }

    /// return a new grid with all the same given value values for a given length and width
    pub fn new(length: usize, width: usize, value: T) -> Self {
	let nums = vec![value; length];
	Self {nums, width}
    }
    
    /// given an indix in the grid, returns a Vector containing all adjacent indices,
    /// If "include_diagonal" == true, including diagonal above and below. At most 8 possible adjacents if we are not on any edges.
    fn get_adjacent_indices(&self, idx: usize, include_diagonal: bool) -> Vec<usize> {
	let mut adjacents = Vec::new();
	let is_top_row = idx < self.width;
	let is_bottom_row = idx >= self.nums.len() - self.width;	
	let is_left_edge = idx % self.width == 0;
	let is_right_edge = idx % self.width == self.width - 1;

	if !is_left_edge {
	    // adjacent to the left
	    adjacents.push(idx - 1);	    
	}
	if !is_right_edge {
	    // adjacent to the right	    
	    adjacents.push(idx + 1);	    
	}
	
	if !is_top_row {
	    // adjacent above
	    adjacents.push(idx - self.width);
	    if include_diagonal {
		// adjacent above left		
		if !is_left_edge {
		    adjacents.push(idx - self.width - 1)
		}
		// adjacent above right				
		if !is_right_edge {
		    adjacents.push(idx - self.width + 1)		
		}
	    }
	}

	if !is_bottom_row {
	    // adjacent below	    
	    adjacents.push(idx + self.width);
	    if include_diagonal {
		// adjacent below left			    
		if !is_left_edge {
		    adjacents.push(idx + self.width - 1)
		}
		// adjacent below right				
		if !is_right_edge {
		    adjacents.push(idx + self.width + 1)		
		}
	    }
	}
	adjacents
	
    }
}
