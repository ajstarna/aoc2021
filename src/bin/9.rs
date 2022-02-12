use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::HashSet;

/// reads the file into a row-major vec and returns the width of one row with it
fn read_grid() -> (Vec<u32>, usize) {
    let buffered = get_buffered_reader("9");
    let mut grid = Vec::new(); // to store the grid of number
    // first go through each
    let mut width = None;
    for line in buffered.lines().flatten() {
	if let None = width {
	    // since every line has the same length, we can just figure out the
	    // width once on the very first line and set width
	    width = Some(line.len());
	}
	for num in line.chars() {
	    grid.push(num.to_digit(10).unwrap());
	}
	//let val = split[1].parse::<i32>().expect("unable to parse val as an i32");	    
    }
    let width = width.unwrap();
    println!("grid = {:?}", grid);
    println!("width = {:?}", width);
    (grid, width)
}
fn find_low_indices(grid: &Vec<u32>, width: usize) -> Vec<usize> {
    // go over the grid and look for low spots.
    // the the numbers to the left, right, above, and below are higher than
    // a number, it is a low spot. A number being on the "edge" of the grid only has
    // three other numbers to check for this condition.
    let mut low_indices = Vec::new(); // keep track of the low positions
    for (i, &num) in grid.iter().enumerate() {
	let above_idx = i as i32 - width as i32;
	if above_idx >= 0 && grid[above_idx as usize] <= num {
	    // number above is not lower than current
	    continue;
	}
	let below_idx = i + width;
	if below_idx < grid.len() && grid[below_idx] <= num {
	    // number below is not lower than current
	    continue;
	}
	if i % width != 0 && grid[i-1] <= num {
	    // number to the left is not lower than current
	    continue;
	}
	if i % width != width - 1 && grid[i+1] <= num {
	    // number to the right is not lower than current
	    continue;
	}
	//println!("i = {}", i);	
	//println!("we are low!");
	// we did not fail any conditional check, so we are at a low spot
	low_indices.push(i);
    }
    low_indices
}

fn part1() {
    let (grid, width) = read_grid();
    let low_indices = find_low_indices(&grid, width);
    println!("low indices = {:?}", low_indices);
    let risk: u32 = low_indices.iter().map(|&x| grid[x] + 1).sum();
    println!("risk = {:?}", risk);    
}



/// given a low idx as a starting point, we search for all the other indices that belong to the basin
/// formed by this low index. It will at least include the low index itself
/// "9" indicates the borders between basins
/// We visit the basin in a depth first manor, where we pop from the end of the visit queue, thus
/// diving deeper into the last candidate we see every time
fn scope_out_basin(grid: &Vec<u32>, width: usize, low_idx: usize) -> HashSet<usize> {
    let mut basin = HashSet::new();
    let mut visit_queue = Vec::new();
    visit_queue.push(low_idx);
    while let Some(idx) = visit_queue.pop() {
	basin.insert(idx);
	let num = grid[idx];
	let above_idx = idx as i32 - width as i32;
	if !basin.contains(&(above_idx as usize)) &&  above_idx >= 0 && grid[above_idx as usize] > num && grid[above_idx as usize] != 9 {
	    // the above number is part of the basin, so add it to the queue to explore
	    //println!("pushing above");
	    visit_queue.push(above_idx as usize);
	}
	let below_idx = idx + width;
	if !basin.contains(&below_idx) && below_idx < grid.len() && grid[below_idx] > num && grid[below_idx] != 9 {
	    // the below number is part of the basin, so add it to the queue to explore
	    //println!("pushing below");	    
	    visit_queue.push(below_idx);
	}
	if  idx % width != 0 && !basin.contains(&(idx -1)) && grid[idx-1] > num && grid[idx-1] != 9 {
	    // number to the left is part of basin, so add it to the queue to explore
	    //println!("pushing left");	    	    
	    visit_queue.push(idx-1);
	}
	if !basin.contains(&(idx + 1)) && idx % width != width - 1 && grid[idx+1] > num && grid[idx+1] != 9 {
	    // number to the right is part of the basin
	    //println!("pushing right");	    	    	    
	    visit_queue.push(idx+1);	    
	}
    }
    //println!("basin = {:?}", basin);
    basin
}

fn part2() {
    let (grid, width) = read_grid();
    let low_indices = find_low_indices(&grid, width);
    println!("low indices = {:?}", low_indices);
    let mut top_lens = vec![0, 0, 0];
    for low_idx in low_indices {
	// each low spot leads to exactly one basin.	
	let current_len = scope_out_basin(&grid, width, low_idx).len();
	println!("current len = {}", current_len);
	let mut to_replace_num = None;
	let mut to_replace_idx = None;
	// check if the current basin has a len that belongs in the top three
	for (i, &top_len) in top_lens.iter().enumerate() {
	    // for each of the top three, check if the current len is better than it.
	    // if so, we need to replace it.
	    // but if we find multiple that we are better than, replace the lowest of them
	    //println!("top len = {}", top_len);
	    //println!("to_replace: {:?}", to_replace_num);
	    //println!("to_replace idx: {:?}", to_replace_idx);	    	    
	    if current_len > top_len {
		if let Some(to_replace_num_inner) = to_replace_num{
		    if top_len < to_replace_num_inner {
			// this top length is actually lower than another one that we wanted to replace already,
			// so we should replace it instead
			to_replace_idx = Some(i);
			to_replace_num = Some(top_len);
		    } // else: we are already replacing a lower numbers
		}
		else{
		    to_replace_idx = Some(i);
		    to_replace_num = Some(top_len);
		} 
	    }
	}
	println!("after: {:?}", to_replace_num);
	if let Some(to_replace_num_inner) = to_replace_num {
	    top_lens[to_replace_idx.unwrap()] = current_len;
	    println!("top lens after replace = {:?}", top_lens);	    
	}
    }
    println!("top lengths = {:?}", top_lens);
    println!("multiplied = {:?}", top_lens.iter().map(|&x| x).product::<usize>());
    
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
