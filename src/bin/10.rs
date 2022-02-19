use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::HashMap;

fn run() {
    let buffered = get_buffered_reader("10");
    let mut total_score_invalid = 0;
    let score_map_invalid = HashMap::<char, u128>::from_iter([(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into_iter()); // each illegal character has its own score
    let score_map_completion = HashMap::<char, u128>::from_iter([(')', 1), (']', 2), ('}', 3), ('>', 4)].into_iter()); // each character has its own score to complete it
    let chunk_pairs = HashMap::<char, char>::from_iter([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')].into_iter()); // which open and closes go together
    let mut all_completion_scores = Vec::new();
    for line in buffered.lines().flatten() {
	//dbg!(line.chars());
	let mut valid = true;
	let mut stack: Vec<char> = Vec::new();	
	for c in line.chars() {
	    if chunk_pairs.contains_key(&c) {
		// if the pais mapping contains this char, then it must be an opening character, so we can just add it to the stack
		//println!("pushing {:?}", c);
		stack.push(c);
	    } else {
		// otherwise, we have a closing character, so we need to confirm that the top char on the stack is a match to be valid
		let top = stack.pop();
		//println!("popped {:?}", top);		
		if let Some(chunk_open) = top {
		    if chunk_pairs.get(&chunk_open).unwrap() != &c {
			// what we wanted to see to be valid
			valid = false;
		    }
		} else {
		    // invalid empty stack
		    //println!("invalid empty stack at {:?}", c);
		    valid = false;
		}
		if !valid {
		    let score = score_map_invalid.get(&c).unwrap();
		    total_score_invalid += score;
		    break; // only need to score the first bad character
		}
	    }
	}

	if valid {
	    // we have a valid but incomplete line
	    // determine the sequence of closing characters to finish off the line
	    println!("valid line: {:?}", line);
	    let mut completion_score = 0;
	    for c in stack.iter().rev() {
		//dbg!(completion_score);
		completion_score *= 5;
		let required = chunk_pairs.get(c).unwrap();
		completion_score += score_map_completion.get(required).unwrap();
	    }
	    dbg!(completion_score);
	    all_completion_scores.push(completion_score);
	}
    }
    println!("total score for invalid lines = {:?}", total_score_invalid); 
    all_completion_scores.sort_unstable();
    let mid_idx = (all_completion_scores.len() as f64 / 2.).floor() as usize;
    println!("mid completion score = {:?}", all_completion_scores[mid_idx]);
    
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
