use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::HashSet;

fn part1() {
    let buffered = get_buffered_reader("8");
    let mut num_1_4_7_8 = 0;
    for line in buffered.lines().flatten() {
	let split: Vec<&str> = line.split(" | ").collect();
	let outputs: Vec<&str> = split[1].split_whitespace().collect();	
	for signal in outputs.iter() {
	    match signal.len() {
		2 | 3 | 4 | 7 => num_1_4_7_8 += 1,
		_ => (),
	    }
	}
    }
    println!("the number of 1,4,7,8 = {:?}", num_1_4_7_8);
}


/// this solution is fairly brute force. maybe there is a better way?
fn part2() {
    let buffered = get_buffered_reader("8");
    let mut total_result = 0;
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    for line in buffered.lines().flatten() {
	// store the chars of each digit as we learn them so that we can easily see which chars are in common between signals
	let mut digit_chars = Vec::new();
	for i in 0..10 {
	    if i == 8 {
		digit_chars.push(HashSet::from_iter(letters));
	    } else {
		digit_chars.push(HashSet::new());
	    }
	}
	
	let split: Vec<&str> = line.split(" | ").collect();
	let inputs: Vec<&str> = split[0].split_whitespace().collect();
	let outputs: Vec<&str> = split[1].split_whitespace().collect();	
	//println!("inputs = {:?}", inputs);
	//println!("outputs = {:?}", outputs);
	for signal in inputs.iter().chain(outputs.iter()) {
	    match signal.len() {
		2 => {
		    // we found the digit "1" which covers top_right and bottom_right
		    // we now know that top right and bottom right could each be one of these two chars
		    for x in signal.chars() {
			digit_chars[1].insert(x);
		    }
		},
		3 => {
		    // found "7"
		    for x in signal.chars() {
			digit_chars[7].insert(x);
		    }
		},
		4 => {
		    // found "4"
		    for x in signal.chars() {
			digit_chars[4].insert(x);
		    }
		},
		_ => (),
	    }
	}
		
	    
	for signal in inputs.iter().chain(outputs.iter()) {
	    let chars: HashSet<char> = HashSet::from_iter(signal.chars());
	    match signal.len() {
		5 => {
		    // either 2, 3, 5
		    // 3 covers all of 1
		    if chars.intersection(&digit_chars[1]).count() == digit_chars[1].len() {
			for x in signal.chars() {
			    digit_chars[3].insert(x);
			}
		    }
		},
		6 => {
		    // either 0, 6, 9
		    // 9 covers all of 4
		    if chars.intersection(&digit_chars[4]).count() == digit_chars[4].len() {
			for x in signal.chars() {
			    digit_chars[9].insert(x);
			}
		    } else if chars.intersection(&digit_chars[1]).count() == digit_chars[1].len() {
			// zero covers 1
			for x in signal.chars() {
			    digit_chars[0].insert(x);
			}
		    } else {
			// otherwise must be the last 6 length: 6
			for x in signal.chars() {
			    digit_chars[6].insert(x);
			}
		    }
		},
		_ => (),
	    }
	}
	    
	for signal in inputs.iter().chain(outputs.iter()) {
	    let chars: HashSet<char> = HashSet::from_iter(signal.chars());
	    if signal.len() == 5 {
		// either 2, 3, 5
		// we already know 3
		if chars == digit_chars[3] {
		} else if digit_chars[9].intersection(&chars).count() == chars.len() {
		    // 9 covers 5 as well
		    for x in signal.chars() {
			digit_chars[5].insert(x);
		    }
		} else {
		    // 2 is the last remaining digit of length five (and it does not get covered by 9)
		    for x in signal.chars() {
			digit_chars[2].insert(x);
		    }
		}
	    }

	}
		
	//println!("all digit chars = {:?}", digit_chars);	
	let mut numbers: Vec<usize> = Vec::new();
	// we now have all the digits, so we can parse the output
	for signal in outputs.iter() {
	    let chars: HashSet<char> = HashSet::from_iter(signal.chars());	    
	    //println!("looking at {:?}", chars);
	    for (i, digit) in digit_chars.iter().enumerate() {
		if *digit == chars {
		    numbers.push(i)
		}
	    }
	}
	let result = numbers.iter().fold(0, |acc, elem| acc * 10 + elem);
	//println!("result = {:?}", result);
	total_result += result;
    }
    println!("total result = {:?}", total_result);
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
