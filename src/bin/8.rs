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

fn part2() {
    let buffered = get_buffered_reader("8");
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let eight: HashSet<char> = HashSet::from_iter(letters);
    for line in buffered.lines().flatten() {
	// each position starts with a set of all the possible letters, and as we get evidence, we remove possibilities
	let (mut zero, mut one, mut two, mut three, mut four, mut five, mut six, mut seven,  mut nine): 
	(HashSet<char>, HashSet<char>, HashSet<char>, HashSet<char>, HashSet<char>, HashSet<char>, HashSet<char>, HashSet<char>, HashSet<char>) = 
	    (HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new());


	
	let split: Vec<&str> = line.split(" | ").collect();
	let inputs: Vec<&str> = split[0].split_whitespace().collect();
	let outputs: Vec<&str> = split[1].split_whitespace().collect();	
	//println!("inputs = {:?}", inputs);
	//println!("outputs = {:?}", outputs);
	for signal in inputs.iter().chain(outputs.iter()) {
	    let chars: HashSet<char> = HashSet::from_iter(signal.chars());
	    match chars.len() {
		2 => {
		    // we found the digit "1" which covers top_right and bottom_right
		    // we now know that top right and bottom right could each be one of these two chars
		    for x in signal.chars() {
			one.insert(x);
		    }
		},
		3 => {
		    // found "7"
		    for x in signal.chars() {
			seven.insert(x);
		    }
		},
		4 => {
		    // found "4"
		    for x in signal.chars() {
			four.insert(x);
		    }
		},
		_ => (),
	    }
	}

	println!("eight = {:?}", eight);	
	println!("one = {:?}", one);
	println!("seven = {:?}", seven);
	println!("four = {:?}", four);
	
	for signal in inputs.iter().chain(outputs.iter()) {
	    let chars: HashSet<char> = HashSet::from_iter(signal.chars());
	    match chars.len() {
		5 => {
		    // either 2, 3, 5
		    // 3 covers all of 1
		    if chars.intersection(&one).count() == one.len() {
			for x in signal.chars() {
			    three.insert(x);
			}
		    }
		},
		6 => {
		    // either 0, 6, 9
		    // 9 covers all of 4
		    if chars.intersection(&four).count() == four.len() {
			for x in signal.chars() {
			    nine.insert(x);
			}
		    } else if chars.intersection(&one).count() == one.len() {
			// zero covers 1
			for x in signal.chars() {
			    zero.insert(x);
			}
		    } else {
			// otherwise must be the last 6 length: 6
			for x in signal.chars() {
			    six.insert(x);
			}
		    }
		},
		_ => (),
	    }
	}
	println!("three = {:?}", three);
	println!("nine = {:?}", nine);
	println!("zero = {:?}", zero);
	println!("six = {:?}", six);	


	for signal in inputs.iter().chain(outputs.iter()) {
	    let chars: HashSet<char> = HashSet::from_iter(signal.chars());
	    match chars.len() {
		5 => {
		    // either 2, 3, 5
		    // we already know 3
		    if chars == three {
			();
		    } else if nine.intersection(&chars).count() == chars.len() {
			// 9 covers 5 as well
			for x in signal.chars() {
			    five.insert(x);
			}
		    } else {
			// 2 is the last remaining digit of length five (and it does not get covered by 9)
			for x in signal.chars() {
			    two.insert(x);
			}
		    }
		},
		_ => (),
	    }
	}

	println!("five = {:?}", five);
	println!("two = {:?}", five);

	let mut numbers = Vec::new();
	// we now have all the digits, so we can parse the output
	for signal in outputs.iter() {
	    let chars: HashSet<char> = HashSet::from_iter(signal.chars());
	    println!("looking at {:?}", chars);
	    match chars {
		one => numbers.push(1),
		two => numbers.push(2),
		three => numbers.push(3),
		four => numbers.push(4),
		five => numbers.push(5),
		six => numbers.push(6),
		seven => numbers.push(7),
		eight => numbers.push(8),
		nine => numbers.push(9),
		zero => numbers.push(0),
		_ => println!("error! {:?} is not a known digit", signal),
	    }
	}
	println!("numbers = {:?}", numbers);
	break;	    	
    }
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
