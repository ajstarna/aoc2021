use std::env;
use std::collections::HashMap;
use aoc2021::utils::{Grid};
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};


// read the image "algorithm" which is contained on one line from the input file
fn read_algo() -> Vec<u8> {
    let buffered = get_buffered_reader("20-algorithm");    
    let line = buffered.lines().next().unwrap().unwrap();
    let algo: Vec<u8> = line.chars()
	.map(|c| {
	    match c {
		'#' => 1,
		'.' => 0,
		_ => panic!("unknown character in algorithm file!"),
	    }
	})
	.collect();
    algo
}

const INCLUDE_DIAGONAL: bool = true;
const INCLUDE_SELF: bool = true; // does our "adjeacent" indices include the given index. since we want the whole 3 by 3 grid, yes.

fn get_enhancement_value(idx: usize, grid: &Grid<u32>, algo: &Vec<u8>) -> u32 {
    let adjacent_indices = grid.get_adjacent_indices(idx, INCLUDE_DIAGONAL, INCLUDE_SELF);
    //println!("adjacent indices = {:?}", adjacent_indices);
    let adjacent_values: Vec<u32> = adjacent_indices.iter().map(|&x| grid.get(x)).collect();
    if adjacent_values.len() < 9 {
	// we are on the edge, and we know in this puzzle we are flipping the bits back and forth in the infinity zone
	return (grid.get(idx) + 1) % 2 as u32;
    }

    //println!("adjacent values = {:?}", adjacent_values);

    // convert the vec of 0s and 1s to an integer by folding from the least significant (right side of vec)
    // and considering it binary number
    let integer = adjacent_values.iter().rev().enumerate()
	.fold(0, |acc, (i, val)| {
	    //println!("i = {:?}, val = {:?}", i, val);
	    acc + u32::pow(2, i as u32) * val
	}) as usize; // convert to usize so we can index into algo
    //println!("integer = {:?}", integer);
    algo[integer] as u32
}

/// iterates over all indices in the grid and finds the corresponding enhancement vales for each
/// Returns a hashmap so that we can do the updates all at the same time afterwards
fn get_all_enhancement_values(grid: &Grid<u32>, algo: &Vec<u8>) -> HashMap<usize, u32> {
    let mut all_values = HashMap::new();
    for idx in 0..grid.len() {
	all_values.insert(idx, get_enhancement_value(idx, grid, algo));
    }
    all_values
}

fn run( ) {
    let buffer = 51; // how mant extra columns and rows on either end
    let mut grid = Grid::<u32>::new_from_file_day_20("20-image", buffer); // not the most optimum, but storing as <u32> for now since already built
    //grid.pretty_print();
    let algo = read_algo();
    println!("algo = {:?}", algo);

    let num_steps = 50;
    for i in 0..num_steps {
	println!("step num = {}", i);
	let all_values = get_all_enhancement_values(&grid, &algo);
	//println!("all values = {:?}", all_values);
	for (idx, val) in all_values {
	    grid.set(idx, val);
	}
	//grid.pretty_print();	
    }

    let num_lit: u32 = grid.nums.iter().sum();
    println!("num lit after {} steps and a buffer of size {} = {}", num_steps, buffer, num_lit);
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
