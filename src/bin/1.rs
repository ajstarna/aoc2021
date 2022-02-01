use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

fn get_buffered_reader() -> BufReader<File>{
    let path = "data/1.txt";
    let input = File::open(path).expect("could not open file!");
    BufReader::new(input)
}

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

fn part2 () {
    let buffered = get_buffered_reader();    
    let mut total = 0;

    let mut nums = VecDeque::new();
    
    for line in buffered.lines() {
	if let Ok(line) = line {
	    let current = line.parse::<i32>().expect("unable to parse line as an i32");
	    nums.push_back(current);

	    if nums.len() == 4 {
		// the len check just makes sure we don't do anything the first few lines
		let sum1: i32 = nums.range(..3).sum();
		let sum2: i32  = nums.range(1..).sum();
		if sum2 > sum1 {
		    total += 1;
		}
		nums.pop_front();
	    }
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
