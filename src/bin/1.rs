use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() {
    let path = "data/1.txt";
    let input = File::open(path).expect("could not open file!");
    let buffered = BufReader::new(input);

    let mut previous: Option<i32> = None;
    let mut total = 0;
    for line in buffered.lines() {
	if let Ok(line) = line {
            println!("{}", line);
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
