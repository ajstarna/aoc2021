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

impl<T: std::clone::Clone + std::marker::Copy + From<u32> + std::fmt::Debug +
     std::ops::Add<Output = T> + std::cmp::PartialOrd + std::ops::Rem<Output = T>> Grid<T> {

    pub fn len(&self) -> usize {
	self.nums.len()
    }
    
    pub fn set(&mut self, index: usize, value: T) {
	self.nums[index] = value;
    }
    
    pub fn get(&self, index: usize) -> T {
	self.nums[index]
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
		// this is why we enforce From<u32>, since to_digit() returns that, but maybe there is
		// a better way to do this?
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
    pub fn get_adjacent_indices(&self, idx: usize, include_diagonal: bool) -> Vec<usize> {
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

    pub fn pretty_print(&self) {
	for (i, val) in self.nums.iter().enumerate() {
	    print!("{:?} ", val);
	    if i % self.width == self.width - 1 {
		println!();
	    }
	}
    }


    /// reads the given file and returns a grid. We assume each position is a single numeric character
    /// this multiples the grid to be 5x bigger in each dimension (as required for day 15 part 2)
    pub fn new_from_file_5x(file_prefix: &str) -> Self {
	let buffered = get_buffered_reader(file_prefix);
	let mut nums = Vec::<T>::new(); // to store the grid of number
	// first go through each
	let mut width = None;
	for line in buffered.lines().flatten() {
	    if let None = width {
		// since every line has the same length, we can just figure out the
		// width once on the very first line and set width
		width = Some(line.len() * 5);
	    }
	    for extra in 0..5 {
		// for each increment 0, 1,2,3,4 we add the current line + extra
		for num in line.chars() {
		    // this is why we enforce From<u32>, since to_digit() returns that, but maybe there is
		    // a better way to do this?
		    let mut extra_num = T::from(num.to_digit(10).unwrap()) + T::from(extra);
		    if extra_num > T::from(9) {
			// loop from 9 back to 1
			extra_num = (extra_num % T::from(10)) + T::from(1);
		    }
		    nums.push(extra_num);
		}
	    }
	}
	let width = width.unwrap();
	// now we need to times by five in the next dimension	
	let original_len = nums.len();
	let mut extra_nums = Vec::<T>::new();
	for extra in 1..5 {
	    // for each increment 1,2,3,4 we add the current full rows + extra
	    for num in nums.iter().take(original_len) {
		// this is why we enforce From<u32>, since to_digit() returns that, but maybe there is
		// a better way to do this?
		let mut extra_num = *num + T::from(extra);
		if extra_num > T::from(9) {
		    // loop from 9 back to 1
		    extra_num = (extra_num % T::from(10)) + T::from(1);
		}
		extra_nums.push(extra_num);
	    }
	}

	// finally, extend the original nums with the extra_nums
	nums.extend(extra_nums.iter());
	
	Self { nums, width }
    }
    
    
}
