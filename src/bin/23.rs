use std::{env, io};
use std::collections::{HashMap,VecDeque};
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;
use std::hash::{Hash, Hasher};


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Colour {
    Amber,
    Bronze,
    Copper,
    Desert,
}

fn colour_to_char(colour: Colour) -> char {
    match colour {
	Colour::Amber => 'A',
	Colour::Bronze => 'B',
	Colour::Copper => 'C',
	Colour::Desert => 'D',	
    }
}
    
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Amphipod {
    colour: Colour,
    energy: u64,
}

const NUM_SPOTS: usize = 15;

/// the state keeps tracks of what is in each of the 15 rooms
#[derive(Debug, Copy, Clone, Eq)]
struct State {
    spots: [Option<Amphipod>; NUM_SPOTS],
    energy_spent: u64, // total amount of energy spent by all moves
}

/// to hash a state, we only care about the spots, not the energy spent.
/// That way we can compare two states and see if the one with more energy spent is just redundant
impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.spots.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.spots == other.spots
    }
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

    fn pretty_print(&self) {
	println!("#############");
	print!("#");
	if let Some(Amphipod{colour, ..}) = self.spots[0] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	if let Some(Amphipod{colour, ..}) = self.spots[1] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!(".");
	if let Some(Amphipod{colour, ..}) = self.spots[2] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!(".");	
	if let Some(Amphipod{colour, ..}) = self.spots[3] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!(".");	
	if let Some(Amphipod{colour, ..}) = self.spots[4] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!(".");	
	if let Some(Amphipod{colour, ..}) = self.spots[5] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	if let Some(Amphipod{colour, ..}) = self.spots[6] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	println!("#");	
	print!("###");
	if let Some(Amphipod{colour, ..}) = self.spots[7] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");
	if let Some(Amphipod{colour, ..}) = self.spots[8] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");
	if let Some(Amphipod{colour, ..}) = self.spots[9] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");	
	if let Some(Amphipod{colour, ..}) = self.spots[10] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	println!("###");
	print!("  #");	
	if let Some(Amphipod{colour, ..}) = self.spots[11] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");
	if let Some(Amphipod{colour, ..}) = self.spots[12] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");
	if let Some(Amphipod{colour, ..}) = self.spots[13] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");	
	if let Some(Amphipod{colour, ..}) = self.spots[14] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	println!("#");
	println!("  #########");	
	


    }
    
    fn set(&mut self, index: usize, letter: &str) -> Result<(), io::Error> {
	assert!(index < NUM_SPOTS);
	match letter {
	    "A" => self.spots[index] = Some(Amphipod{colour: Colour::Amber, energy: 1}),
	    "B" => self.spots[index] = Some(Amphipod{colour: Colour::Bronze, energy: 10}),
	    "C" => self.spots[index] = Some(Amphipod{colour: Colour::Copper, energy: 100}),
	    "D" => self.spots[index] = Some(Amphipod{colour: Colour::Desert, energy: 1000}),
	    _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("unable to parse {:?} as an Amphipod for index {}", letter, index))),
	}
	Ok(())
    }

    // return a copied state from self, when we move whatever is in from_index to to_index
    // multiplier is usually 1, but we might set it to 2 to represent that we are moving to the "imaginary" spot right outside a room and then into a room
    fn try_new_from_move(&self, from_index: usize, to_index: usize, multiplier: u64) -> Option<Self> {
	let bad_room_indices = match self.spots[from_index].unwrap().colour {
	    Colour::Amber => [8, 9, 10, 12, 13, 14],
	    Colour::Bronze => [7, 11, 10, 12, 13, 14],
	    Colour::Copper => [7, 11, 8, 9, 13, 14],
	    Colour::Desert => [7, 11, 8, 9, 10, 12],
	};
	if bad_room_indices.contains(&to_index) {
	    // don't move into the wrong room!
	    //println!("don't move into the wrong room!");
	    return None;
	}
	let mut new_state = *self; // copy ourself
	new_state.spots[to_index] = new_state.spots[from_index];
	new_state.spots[from_index] = None;
	new_state.energy_spent += new_state.spots[to_index].unwrap().energy * multiplier;
	Some(new_state)
    }



    
    fn is_amber_done(&self) -> bool {
	if let (Some(Amphipod{colour: Colour::Amber, ..}), Some(Amphipod{colour: Colour::Amber, ..})) = (self.spots[7], self.spots[11]) {
	    //println!("we have amber in spot 7 and 11 like we wanted!");
	    return true;
	} else {
	    return false;	    
	}
    }
    
    fn is_bronze_done(&self) -> bool {    
	if let (Some(Amphipod{colour: Colour::Bronze, ..}), Some(Amphipod{colour: Colour::Bronze, ..})) = (self.spots[8], self.spots[12]) {	
	    println!("we have bronze in spot 8 and 12 like we wanted!");
	    return true;
	} else {
	    return false;
	}
    }
    
    fn is_copper_done(&self) -> bool {    
	if let (Some(Amphipod{colour: Colour::Copper, ..}), Some(Amphipod{colour: Colour::Copper, ..})) = (self.spots[9], self.spots[13]) {		
	    println!("we have copper where we wanted!");
	    return true;
	} else {
	    return false;
	}
    }
    
    fn is_desert_done(&self) -> bool {        
	if let (Some(Amphipod{colour: Colour::Desert, ..}), Some(Amphipod{colour: Colour::Desert, ..})) = (self.spots[10], self.spots[14]) {		    
	    println!("we have desert where we wanted!");
	    return true;
	} else {
	    return false;	    
	}
    }
    
    /// the current state returns a vector of all other valid States based on moving on Amphipod
    fn get_valid_transitions(&self) -> Vec<Self> {
	let mut valid_states = Vec::new();
	
	////////////////// 0 ////////////////////
	if ! self.spots[0].is_none() && self.spots[1].is_none() { 
	    // we can move from 0 to 1 if 0 is full and 1 is empty
	    if let Some(new_state) = self.try_new_from_move(0, 1, 1) {
		valid_states.push(new_state)
	    }
	}
	////////////////// 1 ////////////////////	
	if ! self.spots[1].is_none() {
	    for (idx, multiplier) in [(0,1), (2,2), (7,2)] {
		if self.spots[idx].is_none() {
		    if let Some(new_state) = self.try_new_from_move(1, idx, multiplier) {
			valid_states.push(new_state)
		    }
		}
	    }	    
	}
	////////////////// 2 ////////////////////	
	if !self.spots[2].is_none() {
	    for (idx, multiplier) in [(1,2), (3,2), (7,2), (8,2)] {
		if self.spots[idx].is_none() {
		    if let Some(new_state) = self.try_new_from_move(2, idx, multiplier) {
			valid_states.push(new_state);
		    }
		}
	    }	    
	}	
	////////////////// 3 ////////////////////	
	if !self.spots[3].is_none() {
	    for (idx, multiplier) in [(1,2), (3,2), (7,2), (8,2)] {
		if self.spots[idx].is_none() {
		    if let Some(new_state) = self.try_new_from_move(3, idx, multiplier) {
			valid_states.push(new_state)
		    }
		}
	    }	    
	}	
	////////////////// 4 ////////////////////	
	if !self.spots[4].is_none() {
	    for (idx, multiplier) in [(3,2), (5,2), (9,2), (10,2)] {
		if self.spots[idx].is_none() {
		    if let Some(new_state) = self.try_new_from_move(4, idx, multiplier) {
			valid_states.push(new_state)	    ;
		    }
		}
	    }	    
	}
	////////////////// 5 ////////////////////	
	if !self.spots[5].is_none() {
	    for (idx, multiplier) in [(6,1), (4, 2), (10, 2)] {
		if self.spots[idx].is_none() {
		    if let Some(new_state) = self.try_new_from_move(5, idx, multiplier) {
			valid_states.push(new_state)	    ;
		    }
		}
	    }
	}
	////////////////// 6 ////////////////////
	if !self.spots[6].is_none() && self.spots[5].is_none() { 
	    if let Some(new_state) = self.try_new_from_move(6, 5, 1) {
		valid_states.push(new_state);
	    }
	}
	////////////////// 7 ////////////////////
	if !self.spots[7].is_none() && !self.is_amber_done(){
	    // don't bother moving out of 7 if amber is done
	    for (idx, multiplier) in [(1,2), (2,2), (11,1)] {
		if self.spots[idx].is_none() {
		    if let Some(new_state) = self.try_new_from_move(7, idx, multiplier) {
			valid_states.push(new_state);
		    }
		}
	    }
	}
	////////////////// 8 ////////////////////
	if !self.spots[8].is_none() && !self.is_bronze_done(){
	    for (idx, multiplier) in [(2,2), (3,2), (12,1)] {
		if self.spots[idx].is_none() {
		    if let Some(new_state) = self.try_new_from_move(8, idx, multiplier) {
			valid_states.push(new_state);
		    }
		}
	    }
	}
	////////////////// 9 ////////////////////
	if ! self.spots[9].is_none() && !self.is_copper_done(){
	    for (idx, multiplier) in [(3,2), (4,2), (13,1)] {
		if self.spots[idx].is_none() {
		    if let Some(new_state) = self.try_new_from_move(9, idx, multiplier) {
			valid_states.push(new_state)
		    }
		}
	    }
	}
	////////////////// 10 ////////////////////
	if !self.spots[10].is_none() && !self.is_desert_done(){
	    for (idx, multiplier) in [(4,2), (5,2), (14,1)] {
		if self.spots[idx].is_none() {
		    if let Some(new_state) = self.try_new_from_move(10, idx, multiplier) {
			valid_states.push(new_state);
		    }
		}
	    }
	}
	////////////////// 11 ////////////////////
	if !self.spots[11].is_none() && self.spots[7].is_none() { 
	    if let Some(new_state) = self.try_new_from_move(11, 7, 1) {
		valid_states.push(new_state);
	    }
	}
	////////////////// 12 ////////////////////
	if !self.spots[12].is_none() && self.spots[8].is_none() { 
	    if let Some(new_state) = self.try_new_from_move(12, 8, 1) {
		valid_states.push(new_state);
	    }
	}
	////////////////// 13 ////////////////////
	if !self.spots[13].is_none() && self.spots[9].is_none() { 
	    if let Some(new_state) = self.try_new_from_move(13, 9, 1) {
		valid_states.push(new_state);
	    }
	}
	////////////////// 14 ////////////////////
	if !self.spots[14].is_none() && self.spots[10].is_none() { 
	    if let Some(new_state) = self.try_new_from_move(14, 10, 1) {
		valid_states.push(new_state);
	    }
	}

	valid_states
    }

    /// is the game over, i.e. are the colours all in the proper rooms?
    /// there is likely a better way to write this method?
    fn is_complete(&self) -> bool {
	self.is_amber_done() && self.is_bronze_done() && self.is_copper_done() && self.is_desert_done()
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
    let mut best_energy_per_state = HashMap::<State, u64>::new(); // keeps track of the best energy seen for a given setup of Amphipods, so that we can prune redundant states
    let mut count = 0;
    while !search_queue.is_empty() {
	count += 1;
	//println!("\n\ncount = {}", count);
	if count > 1000000 {
	    break;
	}

	//println!("search queue len = {:?}", search_queue.len());
	//println!("size of state mapping = {}", best_energy_per_state.len());
	let current_state = search_queue.pop_front().unwrap();

	if count % 100 == 0 {
	    println!("current state = {:?}", current_state);
	    current_state.pretty_print();
	}

	
	let best_energy_for_current = best_energy_per_state.entry(current_state).or_insert(current_state.energy_spent);
	if *best_energy_for_current < current_state.energy_spent {
	    // we have already seen this state with a better energy spent, so just move on
	    //println!("we have already seen this state with a better energy spent {}, so just move on", best_energy_for_current);
	    continue;
	} else {
	    //println!("new state or new best energy {}", current_state.energy_spent);
	}

	if current_state.energy_spent > best_energy {
	    // this state is not worth pursuing
	    //println!("this state already used too much energy");
	    continue
	}
	
	let next_states = current_state.get_valid_transitions();
	//println!("next states = \n");
	for state in &next_states {
	    //println!("{:?}", state);
	}
	    
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
    best_energy
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
    let best = solve(starting_state);
    println!("best energey needed to solve = {}", best);
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


    #[test]
    fn test_solve_easy() {
	let mut state = State::new();
 	state.set(1, "A"); // this is the only Amphipod that needs to move, so we should win soon
	state.set(11, "A");	
	state.set(8, "B");
	state.set(12, "B");	
	state.set(9, "C");
	state.set(13, "C");	
	state.set(10, "D");
	state.set(14, "D");	
	let best = solve(state);
	assert_eq!(best, 2); // the Amber needs to move from 1 into 7, which is "two" steps X 1 energy each
    }

    #[test]
    fn test_solve_easy_2() {
	let mut state = State::new();
 	state.set(0, "A"); // this is the only Amphipod that needs to move, so we should win soon
	state.set(11, "A");	
	state.set(8, "B");
	state.set(12, "B");	
	state.set(9, "C");
	state.set(13, "C");	
	state.set(10, "D");
	state.set(14, "D");	
	let best = solve(state);
	assert_eq!(best, 3); // the Amber needs to move from 0 to 1 (1 step), then 1 into 7, which is "two" steps
    }

    #[test]
    fn test_solve_easy_bronze() {
	let mut state = State::new();
 	state.set(7, "A"); 
	state.set(11, "A");	
	state.set(2, "B"); // this is the only Amphipod that needs to move, so we should win soon
	state.set(12, "B");	
	state.set(9, "C");
	state.set(13, "C");	
	state.set(10, "D");
	state.set(14, "D");	
	let best = solve(state);
	assert_eq!(best, 20);
    }
    
    #[test]
    fn test_solve_easy_copper() {
	let mut state = State::new();
 	state.set(7, "A"); 
	state.set(11, "A");	
	state.set(8, "B"); 
	state.set(12, "B");	
	state.set(4, "C"); // this is the only Amphipod that needs to move, so we should win soon
	state.set(13, "C");	
	state.set(10, "D");
	state.set(14, "D");	
	let best = solve(state);
	assert_eq!(best, 200);
    }
    
    #[test]
    fn test_solve_easy_desert() {
	let mut state = State::new();
 	state.set(7, "A"); 
	state.set(11, "A");	
	state.set(8, "B"); 
	state.set(12, "B");	
	state.set(9, "C"); 
	state.set(13, "C");	
	state.set(6, "D"); // this is the only Amphipod that needs to move, so we should win soon
	state.set(14, "D");	
	let best = solve(state);
	assert_eq!(best, 3000);
    }
    

}
