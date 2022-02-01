use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};


fn part1() {
    let buffered = get_buffered_reader();
    let mut previous: Option<i32> = None;
    let mut total = 0;
    for line in buffered.lines() {
	if let Ok(line) = line {
	    let current = line.parse::<i32>().expect("unable to parse line as an i32");
	    if let Some(num) = previous {
		if current > num {
		    total += 1;
		}
	    }
	    previous = Some(current);	    
	}
    }
    println!("total increases = {}", total);    
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
