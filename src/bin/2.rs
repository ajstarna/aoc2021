use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};


fn part1() {
    let buffered = get_buffered_reader("2");
    let mut horizontal = 0;
    let mut depth = 0;
    for line in buffered.lines().flatten() {
	let split: Vec<&str> = line.split_whitespace().collect();
	let val = split[1].parse::<i32>().expect("unable to parse val as an i32");	    
	// println!("{:?}", split);
	match split[0] {
	    "forward" => horizontal += val,
	    "up" => depth -= val,
	    "down" => depth += val,
	    _ => println!("huh?"),
	}
    }
    println!("horizontal = {}", horizontal);
    println!("depth = {}", depth);
    println!("multiplied = {}", depth * horizontal);        
}

fn part2() {
    let buffered = get_buffered_reader("2");
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;    
    for line in buffered.lines().flatten() {
	let split: Vec<&str> = line.split_whitespace().collect();
	let val = split[1].parse::<i32>().expect("unable to parse val as an i32");	    
	// println!("{:?}", split);
	match split[0] {
	    "forward" => {
		horizontal += val;
		depth += aim * val;
	    },
	    "up" => aim -= val,
	    "down" => aim += val,
	    _ => println!("huh?"),
	}
    }
    println!("horizontal = {}", horizontal);
    println!("depth = {}", depth);
    println!("multiplied = {}", depth * horizontal);        
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
