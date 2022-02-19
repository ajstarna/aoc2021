use std::env;
use aoc2021::utils::{Grid};
use std::collections::HashSet;
use std::cmp::Ordering::Equal;    

fn run(times_five: bool ) {
    // 5x indicates if we want to multiply the board 5x (as explained in part2)
    let values = match times_five {
	false => Grid::<u32>::new_from_file("15"),
	true => Grid::<u32>::new_from_file_5x("15"),	
    };
    
    let mut distances = Grid::new(values.len(), values.width, u32::MAX);
    distances.set(0, 0); // the starting point
    let mut best_previous = Grid::<u32>::new(values.len(), values.width, 0);    
    println!("{:?}", values);
    println!();
    println!("{:?}", distances);
    println!();        
    println!("{:?}", best_previous);
    values.pretty_print();
    println!();        
    distances.pretty_print();
    println!();        
    best_previous.pretty_print();

    let mut visit_queue = HashSet::<usize>::new(); // the visit queue holds indices that are within range of our current exploration bounds
    let mut complete = HashSet::<usize>::new(); // holds the indices that have officially been visited and are now part of our exploration bounds
    visit_queue.insert(0);
    const INCLUDE_DIAGONAL: bool = false;        
    while !visit_queue.is_empty(){
	println!("len of complete = {:?}", complete.len());
	let shortest: usize = *visit_queue.iter()
	    .min_by(|&x, &y| (distances.get(*x)).partial_cmp(&(distances.get(*y))).unwrap_or(Equal)).unwrap();
	visit_queue.remove(&shortest);
	complete.insert(shortest);	
	let shortest_d = distances.get(shortest);
	let adjacents = values.get_adjacent_indices(shortest, INCLUDE_DIAGONAL);
	let unvisited: Vec<usize> = adjacents.iter().filter(|&x| !complete.contains(x)).copied().collect(); // still needs to be officially visited	

	for neighbour in unvisited {
	    visit_queue.insert(neighbour);	    
	    let new_distance = shortest_d + values.get(neighbour);
	    if new_distance < distances.get(neighbour) {
		//println!("new distance of {:?} for {:?} replaces {:?}", new_distance, neighbour, distances.get(neighbour));
		distances.set(neighbour, new_distance);
		best_previous.set(neighbour, shortest as u32); // the best previous to that neighbour is simply where we are at now
	    }
	}
    }
    println!();            
    distances.pretty_print();
    println!();        
    best_previous.pretty_print();

    println!("shortest distance from start to end = {:?}", distances.get(distances.len() - 1));
    
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
