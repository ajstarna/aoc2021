use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::{HashMap, HashSet};

/// we read the file and hold all the points as a tuple of (u32, u32)
/// and the commands as a Vector of tuples like ('y', 4)
fn read_file() -> (HashSet<(u32, u32)>, Vec<(char, u32)>) {
    let buffered = get_buffered_reader("13-small");
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

fn part1() {
    let (points, commands) = read_file();
    dbg!(points);
    dbg!(commands);
    for command in commands {
	let b: u8 = command;
    }
}

fn part2 () {    
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
