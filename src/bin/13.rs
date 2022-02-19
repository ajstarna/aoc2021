use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::{HashSet};

/// we read the file and hold all the points as a tuple of (x: u32, y: u32)
/// and the commands as a Vector of tuples like ('y', 4)
fn read_file() -> (HashSet<(u32, u32)>, Vec<(char, u32)>) {
    let buffered = get_buffered_reader("13");
    let mut points =  HashSet::<(u32, u32)>::new();
    let mut commands =  Vec::<(char, u32)>::new();
    for line in buffered.lines().flatten() {
	if line.is_empty() {
	    continue;
	}
	
	if line.starts_with("fold") {
	    let parts = line.split(' ');
	    let final_parts: Vec<&str> = parts.last().unwrap().split('=').collect();
	    commands.push((final_parts[0].chars().next().unwrap(), final_parts[1].parse::<u32>().unwrap()));
	} else {
	    let parts: Vec<u32> = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
	    let point = (parts[0], parts[1]);
	    points.insert(point);
	}
    }
    (points, commands)
}

/// we fold all the ppints along x=num or y=num depending on axis
/// any point to the right of x=num gets folded to the left
/// e.g. for axis='x' and num=7, a point of (9,2) will get folded to (5,2)
/// (9,2) is removed from the points, and (5,2) is inserted
fn fold(points: &mut HashSet<(u32, u32)>, num: u32, axis: char) {
    let mut to_keep = HashSet::new(); // store the points that will not change
    let mut to_add = Vec::new();// store the points to add after the keeping
    for point in points.iter() {
	let (x, y) = point;
	match axis {
	    'x' => {
		if x > &num {
		    // we would want to fold this point over
		    let diff = x - num;
		    let new_x = num - diff;
		    to_add.push((new_x, *y));
		} else {
		    // this point does not get folded, so we just keep it in the set
		    to_keep.insert((*x, *y));
		}
	    },
	    'y' => {
		if y > &num {
		    // we would want to fold this point over
		    let diff = y - num;
		    let new_y = num - diff;
		    to_add.push((*x, new_y));
		} else {
		    // this point does not get folded, so we just keep it in the set
		    to_keep.insert((*x, *y));
		}
	    },
	    _ => println!("weird axis"),
	}
		
    }
    points.retain(|x| to_keep.contains(x));
    for new in to_add {
	points.insert(new);
    }
    
}

/// given a set of points, this prints them out like a grid
fn display_points(points: &HashSet<(u32, u32)>) {
    let max_x = points.iter().map(|x| x.0).max().unwrap() + 1; // we add 1 because we also have 0,0
    let max_y = points.iter().map(|x| x.1).max().unwrap() + 1;
    println!("max x = {:?}, max y = {:?}", max_x, max_y);
    let mut x;
    let mut y;
    for i in 0..(max_x * max_y) {
	x = i % max_x;
	y = i / max_x;
	// println!("{:?}", (x, y));
	if points.contains(&(x, y)) {
	    print!("#");
	} else {
	    print!(".");
	}
	if x == max_x - 1 {
	    println!();
	}
    }
}

fn run() {
    let (mut points, commands) = read_file();
    println!("points({:?}) = {:?}", points.len(), points);    
    for command in commands {
	println!("points len = {:?}", points.len());
	println!("command = {:?}", command);		
	let (axis, num) = command;
	fold(&mut points, num, axis);
    }
    println!("final points({:?}) = {:?}", points.len(), points);
    display_points(&points);
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
