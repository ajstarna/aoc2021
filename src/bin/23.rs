use std::{env, io};
use std::collections::VecDeque;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;


#[derive(Debug, Copy, Clone)]
enum Amphipod {
    Amber(u32),
    Bronze(u32),
    Copper(u32),
    Desert(u32),
}

const NUM_SPOTS: usize = 15;

/// the state keeps tracks of what is in each of the 15 rooms
#[derive(Debug, Copy, Clone)]
struct State {
    spots: [Option<Amphipod>; NUM_SPOTS],
    energy_spent: u64, // total amount of energy spent by all moves
}


/*
The indices of self.spots look sorta like this ha. Across the rows of the input file
The spots right outside the rooms don't need to be stored, since Amphipods are not allowed to stop there
#############
#01.2.3.4.56#
### 7# 8# 9#10###
  #11#12#13#14#
  #########
*/
impl State {
    fn new() -> Self {
	Self {spots: [None; NUM_SPOTS], energy_spent: 0}
    }

    fn set(&mut self, index: usize, letter: &str) -> Result<(), io::Error> {
	assert!(index < NUM_SPOTS);
	match letter {
	    "A" => self.spots[index] = Some(Amphipod::Amber(1)),
	    "B" => self.spots[index] = Some(Amphipod::Bronze(10)),
	    "C" => self.spots[index] = Some(Amphipod::Copper(100)),
	    "D" => self.spots[index] = Some(Amphipod::Desert(1000)),
	    _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("unable to parse {:?} as an Amphipod for index {}", letter, index))),
	}
	Ok(())
    }

    /// the current state returns a vector of all other valid States based on moving on Amphipod
    fn get_valid_transitions(&self) -> Vec<Self> {
	Vec::new()
    }

    /// is the game over, i.e. are the colours all in the proper rooms?
    /// there is likely a better way to write this method?
    fn is_complete(&self) -> bool {
	if let (Some(Amphipod::Amber(_)), Some(Amphipod::Amber(_))) = (self.spots[7], self.spots[11]) {
	    println!("we have amber in spot 7 and 11 like we wanted!");
	} else {
	    return false;	    
	}
	if let (Some(Amphipod::Bronze(_)), Some(Amphipod::Bronze(_))) = (self.spots[8], self.spots[12]) {
	    println!("we have bronze in spot 8 and 12 like we wanted!");
	} else {
	    return false;
	}
	if let (Some(Amphipod::Copper(_)), Some(Amphipod::Copper(_))) = (self.spots[9], self.spots[13]) {
	    println!("we have copper where we wanted!");
	} else {
	    return false;
	}
	if let (Some(Amphipod::Desert(_)), Some(Amphipod::Desert(_))) = (self.spots[10], self.spots[14]) {
	    println!("we have desert where we wanted!");
	} else {
	    return false;	    
	}
	true
    }
    
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_is_not_complete1() {
	let mut state = State::new();
	assert!(!state.is_complete());
    }
    
    #[test]
    fn test_is_not_complete2() {
	let mut state = State::new();
	state.set(7, "A");
	state.set(11, "A");	
	assert!(!state.is_complete());
    }

    #[test]
    fn test_is_not_complete3() {
	let mut state = State::new();
	state.set(7, "A");
	state.set(11, "A");	
	state.set(8, "B");
	state.set(12, "B");	
	assert!(!state.is_complete());
    }
    
    #[test]
    fn test_is_not_complete4() {
	let mut state = State::new();
	state.set(7, "A");
	state.set(11, "A");	
	state.set(8, "B");
	state.set(12, "B");	
	state.set(9, "C");
	state.set(13, "C");	
	assert!(!state.is_complete());
    }

    #[test]
    fn test_is_complete() {
	let mut state = State::new();
	state.set(7, "A");
	state.set(11, "A");	
	state.set(8, "B");
	state.set(12, "B");	
	state.set(9, "C");
	state.set(13, "C");	
	state.set(10, "D");
	state.set(14, "D");	
	assert!(state.is_complete());
    }

}


/// we read the file in and populate a State struct, which holds the 15 spots.
/// Since Amphipods cannot stop outside a room, those empty spaces don't actually need to be stored as a spot
fn read_file() -> Result<State, io::Error> {
    let buffered = get_buffered_reader("23-small");
    let mut lines =  buffered.lines().skip(2).flatten();

    let mut starting_state = State::new();
    // we read in the 8 amphipods, starting at index 7
    let re1 = Regex::new(r"###(\w)#(\w)#(\w)#(\w)###").unwrap();
    let line1 = lines.next().unwrap();
    println!("line 1 = {:?}", line1);
    if let Some(caps) = re1.captures(&line1) {
	println!("matched line 1");
	starting_state.set(7, &caps[1])?;
	starting_state.set(8, &caps[2])?;
	starting_state.set(9, &caps[3])?;
	starting_state.set(10, &caps[4])?;	
    } else {
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "unable to parse input file on the first Amphipod line"));
    }
    
    let line2 = lines.next().unwrap();
    let re2 = Regex::new(r"  #(\w)#(\w)#(\w)#(\w)#").unwrap();    
    println!("line 2 = {:?}",  line2);
    if let Some(caps) = re2.captures(&line2) {
	println!("matched line 1");	
	starting_state.set(11, &caps[1])?;
	starting_state.set(12, &caps[2])?;
	starting_state.set(13, &caps[3])?;
	starting_state.set(14, &caps[4])?;	
	
    } else {
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "unable to parse input file on the second Amphipod line"));
    }
    Ok(starting_state)
}

/// given a starting input state, returns the lowest amount of energy needed to solve it
fn solve(starting_state: State) -> u64  {
    let mut search_queue = VecDeque::new();
    search_queue.push_front(starting_state);
    let mut best_energy = 9999999999999999999_u64;
    while !search_queue.is_empty() {
	let current_state = search_queue.pop_front().unwrap();
	let next_states = current_state.get_valid_transitions();
	for state in next_states {
	    if state.is_complete() {
		// see if this state is done and with a new best energy
		best_energy = std::cmp::min(best_energy, state.energy_spent);
	    } else if state.energy_spent < best_energy {
		// this state isn't done, but the energy spent so far is still less than the best energy
		// TODO: if need be we could keep track of states that we have seen at the given state of Amphipods based on energy,
		// So if we see the same overall position but with a higher energy, then we know there is no point going further on this path
		// Otherwise we could keep moving a piece back and forth
		search_queue.push_back(state);
	    }
	}
    }
}

fn run() {
    let starting_state = read_file()
	.unwrap_or_else(|_|
			{
			    eprintln!("bad input! exitinging program");
			    std::process::exit(-1)
			}
	);
    println!("{:?}", starting_state);
    solve(state);
}
    
fn main() {
    // in this case, part1 happened to be so similar, I just made it do both
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
