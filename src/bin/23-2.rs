use std::{env, io};
use std::collections::{HashMap};
use priority_queue::PriorityQueue;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;
use std::hash::{Hash, Hasher};

#[macro_use]
extern crate lazy_static;

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


lazy_static! {
    static ref AMBER_INDICES: HashSet<usize> = {
        let mut m = HashSet::new();
        m.insert(7);
        m.insert(11);
        m.insert(15);
        m.insert(19);	
        m
    };
    static ref BRONZE_INDICES: HashSet<usize> = {
        let mut m = HashSet::new();
        m.insert(8);
        m.insert(12);
        m.insert(16);
        m.insert(20);	
        m
    };
    static ref COPPER_INDICES: HashSet<usize> = {
        let mut m = HashSet::new();
        m.insert(9);
        m.insert(13);
        m.insert(17);
        m.insert(21);	
        m
    };
    static ref DESERT_INDICES: HashSet<usize> = {
        let mut m = HashSet::new();
        m.insert(10);
        m.insert(14);
        m.insert(18);
        m.insert(22);	
        m
    };
    
}

const NUM_SPOTS: usize = 23;

/// the state keeps tracks of what is in each of the 15 rooms
#[derive(Debug, Copy, Clone, Eq)]
struct State {
    spots: [Option<Amphipod>; NUM_SPOTS],
    energy_spent: u64, // total amount of energy spent by all moves
    depth: u64,
    prev_end: usize, // the index of where the last move ended (useful for the rule that an Amphipod "locks" in the hallway until it can move back into a room
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
  #15#16#17#18#
  #19#20#21#22#
  #########
*/
impl State {
    fn new() -> Self {
		Self {spots: [None; NUM_SPOTS], energy_spent: 0, depth: 0, prev_end: 0}
    }

    /// a heuristic of how well the postion is doing.
    /// the higher the score the "worse" the state
    /// we take the distance for each amphipod to get to the bottom of its room
    /// we also add a penalty for each amphipod in the hallway (since a state with more amphiods in their rooms is better)
    fn get_heuristic_energy(&self) -> u64 {
	let mut score = 0;
	if let Some(Amphipod{colour, energy}) = self.spots[0] {
	    //score += 1; // hallway is worse than room	    
	    score += match colour {
		Colour::Amber => 6 * energy,
		Colour::Bronze => 8 * energy,
		Colour::Copper => 10 * energy,
		Colour::Desert => 12 * energy,
	    }
	}
	if let Some(Amphipod{colour, energy}) = self.spots[1] {
	    //score += 1; // hallway is worse than room	    	    
	    score += match colour {
			Colour::Amber => 5 * energy,
			Colour::Bronze => 7 * energy,
			Colour::Copper => 9 * energy,
			Colour::Desert => 11 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[2] {
	    //score += 1; // hallway is worse than room	    	    
	    score += match colour {
			Colour::Amber => 5 * energy,
			Colour::Bronze => 5 * energy,
			Colour::Copper => 7 * energy,
			Colour::Desert => 9 * energy,
		}
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[3] {
	    //score += 1; // hallway is worse than room	    	    
	    score += match colour {
			Colour::Amber => 7 * energy,
			Colour::Bronze => 5 * energy,
			Colour::Copper => 5 * energy,
			Colour::Desert => 7 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[4] {
	    //score += 1; // hallway is worse than room	    	    
	    score += match colour {
			Colour::Amber => 9 * energy,
			Colour::Bronze => 7 * energy,
			Colour::Copper => 5 * energy,
			Colour::Desert => 5 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[5] {
	    //score += 1;
	    score += match colour {
			Colour::Amber => 11 * energy,
			Colour::Bronze => 9 * energy,
			Colour::Copper => 7 * energy,
			Colour::Desert => 5 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[6] {
	    score += match colour {
		Colour::Amber => 12 * energy,
		Colour::Bronze => 10 * energy,
		Colour::Copper => 8 * energy,
		Colour::Desert => 6 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[7] {
	    score += match colour {
		Colour::Amber => 3 * energy,
		Colour::Bronze => 7 * energy,
		Colour::Copper => 9 * energy,
		Colour::Desert => 11 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[8] {
	    score += match colour {
		Colour::Amber => 7 * energy,
		Colour::Bronze => 3 * energy,
		Colour::Copper => 7 * energy,
		Colour::Desert => 9 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[9] {
	    score += match colour {
		Colour::Amber => 9 * energy,
		Colour::Bronze => 7 * energy,
		Colour::Copper => 3 * energy,
		Colour::Desert => 7 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[10] {
	    score += match colour {
		Colour::Amber => 11 * energy,
		Colour::Bronze => 9 * energy,
		Colour::Copper => 7 * energy,
		Colour::Desert => 3 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[11] {
	    score += match colour {
		Colour::Amber => 2 * energy,
		Colour::Bronze => 8 * energy,
		Colour::Copper => 10 * energy,
		Colour::Desert => 12 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[12] {
	    score += match colour {
		Colour::Amber => 8 * energy,
		Colour::Bronze => 2 * energy,
		Colour::Copper => 8 * energy,
		Colour::Desert => 10 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[13] {
	    score += match colour {
		Colour::Amber => 10 * energy,
		Colour::Bronze => 8 * energy,
		Colour::Copper => 2 * energy,
		Colour::Desert => 8 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy}) = self.spots[14] {
	    score += match colour {
		Colour::Amber => 12 * energy,
		Colour::Bronze => 10 * energy,
		Colour::Copper => 8 * energy,
		Colour::Desert => 2 * energy,
	    }
	}
	if let Some(Amphipod{colour, energy}) = self.spots[15] {
	    score += match colour {
		Colour::Amber => 1 * energy,
		Colour::Bronze => 9 * energy,
		Colour::Copper => 11 * energy,
		Colour::Desert => 13 * energy,
	    }
	}
	if let Some(Amphipod{colour, energy}) = self.spots[16] {
	    score += match colour {
		Colour::Amber => 11 * energy,
		Colour::Bronze => 9 * energy,
		Colour::Copper => 1 * energy,
		Colour::Desert => 9 * energy,
	    }
	}
	if let Some(Amphipod{colour, energy}) = self.spots[17] {
	    score += match colour {
		Colour::Amber => 11 * energy,
		Colour::Bronze => 9 * energy,
		Colour::Copper => 1 * energy,
		Colour::Desert => 9 * energy,
	    }
	}
	if let Some(Amphipod{colour, energy}) = self.spots[18] {
	    score += match colour {
		Colour::Amber => 13 * energy,
		Colour::Bronze => 11 * energy,
		Colour::Copper => 9 * energy,
		Colour::Desert => 1 * energy,
	    }
	}
	if let Some(Amphipod{colour, energy}) = self.spots[19] {
	    score += match colour {
		Colour::Amber => 0 * energy,
		Colour::Bronze => 10 * energy,
		Colour::Copper => 12 * energy,
		Colour::Desert => 14 * energy,
	    }
	}
	if let Some(Amphipod{colour, energy}) = self.spots[20] {
	    score += match colour {
		Colour::Amber => 10 * energy,
		Colour::Bronze => 0 * energy,
		Colour::Copper => 10 * energy,
		Colour::Desert => 12 * energy,
	    }
	}
	if let Some(Amphipod{colour, energy}) = self.spots[21] {
	    score += match colour {
		Colour::Amber => 12 * energy,
		Colour::Bronze => 10 * energy,
		Colour::Copper => 0 * energy,
		Colour::Desert => 10 * energy,
	    }
	}
	if let Some(Amphipod{colour, energy}) = self.spots[22] {
	    score += match colour {
		Colour::Amber => 14 * energy,
		Colour::Bronze => 12 * energy,
		Colour::Copper => 10 * energy,
		Colour::Desert => 0 * energy,
	    }
	}

	score
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

	print!("  #");	
	if let Some(Amphipod{colour, ..}) = self.spots[15] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");
	if let Some(Amphipod{colour, ..}) = self.spots[16] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");
	if let Some(Amphipod{colour, ..}) = self.spots[17] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");	
	if let Some(Amphipod{colour, ..}) = self.spots[18] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	println!("#");

	print!("  #");	
	if let Some(Amphipod{colour, ..}) = self.spots[19] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");
	if let Some(Amphipod{colour, ..}) = self.spots[20] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");
	if let Some(Amphipod{colour, ..}) = self.spots[21] {
	    print!("{}", colour_to_char(colour));
	} else {
	    print!(".");
	}
	print!("#");	
	if let Some(Amphipod{colour, ..}) = self.spots[22] {
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


    fn is_hallway_index(index: usize) -> bool {
	index < 7
    }
    
    // return a copied state from self, when we move whatever is in from_index to to_index
    // multiplier is usually 1, but we might set it to 2 to represent that we are moving to the "imaginary" spot right outside a room and then into a room
    fn try_new_from_move(&self, from_index: usize, to_index: usize, multiplier: u64) -> Option<Self> {
	let colour = self.spots[from_index].unwrap().colour;
	//println!("trying to move {:?} from {} to {}", colour, from_index, to_index);	
	match colour {
	    Colour::Amber => {
		if self.is_amber_done() || (AMBER_INDICES.contains(from_index) && to_index < from_index){
		    // if we are fully done or if we are currently attempting to move up the amber room
		    return None;
		}
		if (BRONZE_INDICES.contains(to_index)
		    || COPPER_INDICES.contains(to_index)
		    || DESERT_INDICES.contains(to_index))
		    && to_index > from_index {
			// don't go deeper into the wrong room
			return None;
		    }
		if to_index == 7  {
		    // we do not want to move into the Amber room if it means blocking the wrong colour in the room
		    for room_idx in AMBER_INDICES {
			if let Some(Amphipod{colour: colour2, ..}) = self.spots[room_idx] {
			    match colour2 {
				Colour::Amber => (),
				_ => return None,
			    }
			}
		    }
		}
	    },
	    Colour::Bronze => {
		if self.is_bronze_done() || (BRONZE_INDICES.contains(from_index) && to_index < from_index){
		    return None;
		}
		if (AMBER_INDICES.contains(to_index)
		    || COPPER_INDICES.contains(to_index)
		    || DESERT_INDICES.contains(to_index))
		    && to_index > from_index {
			// don't go deeper into the wrong room
			return None;
		    }
		
		if to_index == 8  {
		    // we do not want to move into the Amber room if it means blocking the wrong colour in the room
		    for room_idx in BRONZE_INDICES {		    
			if let Some(Amphipod{colour: colour2, ..}) = self.spots[room_idx] {
			    match colour2 {
				Colour::Bronze => (),
				_ => return None,
			    }
			}
		    }
		}		
	    },
	    Colour::Copper => {
		if self.is_copper_done() ||  (COPPER_INDICES.contains(from_index) && to_index < from_index){
		    return None;
		}
		if (BRONZE_INDICES.contains(to_index)
		    || AMBER_INDICES.contains(to_index)
		    || DESERT_INDICES.contains(to_index))
		    && to_index > from_index {
			// don't go deeper into the wrong room
			return None;
		    }
		
		if to_index == 9  {
		    // we do not want to move into the Copper room if it means blocking the wrong colour in the room
		    for room_idx in COPPER_INDICES {		    		    
			if let Some(Amphipod{colour: colour2, ..}) = self.spots[13] {
			    match colour2 {
				Colour::Copper => (),
				_ => return None,
			    }
			}
		    }
		}
	    },
	    Colour::Desert => {
		if self.is_desert_done() || (DESERT_INDICES.contains(from_index) && to_index < from_index){
		    return None;
		}
		if (BRONZE_INDICES.contains(to_index)
		    || AMBER_INDICES.contains(to_index)
		    || COPPER_INDICES.contains(to_index))
		    && to_index > from_index {
			// don't go deeper into the wrong room
			return None;
		    }
		
		if to_index == 10  {
		    // we do not want to move into the Desert room if it means blocking the wrong colour in the room
		    for room_idx in DESERT_INDICES {		    		    
			if let Some(Amphipod{colour: colour2, ..}) = self.spots[14] {
			    match colour2 {
				Colour::Desert => (),
				_ => return None,
			    }
			}
		    }
		}
		
	    },
	}
	let mut new_state = *self; // copy ourself
	new_state.spots[to_index] = new_state.spots[from_index];
	new_state.spots[from_index] = None;
	new_state.energy_spent += new_state.spots[to_index].unwrap().energy * multiplier;
	new_state.depth += multiplier;
	new_state.prev_end = to_index;
	Some(new_state)
    }



    
    fn is_amber_done(&self) -> bool {
	for idx in AMBER_INDICES {
	    if let Some(Amphipod{colour: Colour::Amber, ..}) = self.spots[idx] {
		
	    } else {
		return False
	    }
	}
	true
    }
    
    fn is_bronze_done(&self) -> bool {
	for idx in BRONZE_INDICES {
	    if let Some(Amphipod{colour: Colour::Bronze, ..}) = self.spots[idx] {
		
	    } else {
		return False
	    }
	}
	true
    }
    
    fn is_copper_done(&self) -> bool {
	for idx in COPPER_INDICES {
	    if let Some(Amphipod{colour: Colour::Copper, ..}) = self.spots[idx] {
		
	    } else {
		return False
	    }
	}
	true
    }
    
    fn is_desert_done(&self) -> bool {
	for idx in DESERT_INDICES {
	    if let Some(Amphipod{colour: Colour::Desert, ..}) = self.spots[idx] {
		
	    } else {
		return False
	    }
	}
	true
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
	    for (idx, multiplier) in [(2,2), (4,2), (8,2), (9,2)] {
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
    let buffered = get_buffered_reader("23-small-2");
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
    starting_state.pretty_print();
    //let mut search_queue = VecDeque::new();
    let mut search_queue = PriorityQueue::new();

    let starting_heuristic = starting_state.get_heuristic_energy();
    dbg!(starting_heuristic);
    search_queue.push(starting_state, -1 *(starting_state.energy_spent + starting_heuristic) as i64 );
    let mut best_energy = 9999999999999999999_u64;
    let mut best_depth = 0;
    let mut state_mapping = HashMap::<State, u64>::new(); // keeps track of the best energy seen for a given setup of Amphipods, so that we can prune redundant states
    let mut count = 0_u64;
    while !search_queue.is_empty() {
	count += 1;
	let (current_state, state_score) = search_queue.pop().unwrap();	
	if count > 50_000 {
	    println!("\n\ncount = {}", count);
	    println!("search queue len = {:?}", search_queue.len());
	    println!("size of state mapping = {}", state_mapping.len());
	    println!("current state = {:?}", current_state);
	    println!("state score = {}", state_score);	    	    
	    break;
	}

	if count % 10000 == 0 {
	//if count > 30050 {	    
	    println!("\n\ncount = {}", count);
	    println!("search queue len = {:?}", search_queue.len());
	    println!("size of state mapping = {}", state_mapping.len());
	    println!("best energy = {}, best depth = {}", best_energy, best_depth);
	    println!("current state = {:?}", current_state);
	    println!("state score = {}", state_score);
	    println!("heuristic = {}", current_state.get_heuristic_energy());	    	    	    
	    current_state.pretty_print();
	}
	if current_state.is_complete() {
	    // see if this state is done and with a new best energy
	    //println!("\n\ncount = {}", count);	    
	    //println!("A COMPLETE STATE");
	    //current_state.pretty_print();
	    if current_state.energy_spent < best_energy {
		best_energy = current_state.energy_spent;
		best_depth = current_state.depth;
	    }
	    continue;
	}
	if current_state.energy_spent > best_energy {
	    // this state is not worth pursuing
	    //println!("this state already used too much energy");
	    continue
	}

	
	let mut next_states = current_state.get_valid_transitions();
	    if count == 30051 {
		println!("next states: {:?}", next_states);		
	    }
	
	//println!("next states:");
	for state in next_states {
	    if count == 30051 {
		println!("{:?}", state);
	    }
	    if let Some(best_state_score) = state_mapping.get(&state) {
		if *best_state_score < current_state.energy_spent {
		    // we have already seen this state with a better energy spent, so just move on
		    //println!("we have already seen this state with a better energy spent {}, so just move on", best_state_score);
		    continue;
		} else {
		    //println!("new best energy {}", current_state.energy_spent);
		    state_mapping.insert(state, state.energy_spent);
		}	    
	    } else {
		//println!("completely new state");
		state_mapping.insert(state, state.energy_spent);	    
	    }

	    
	    let next_score = -1 * (state.energy_spent + state.get_heuristic_energy()) as i64;
	    //println!("next score = {}", next_score);
	    search_queue.push(state, next_score);
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

    #[test]
    fn test_state_hash() {
	let mut state_mapping = HashMap::<State, u64>::new();
	let mut state = State::new();
 	state.set(7, "A"); 
	state.set(11, "A");	
	state.set(8, "B"); 
	state.set(12, "B");	
	state.set(9, "C"); 
	state.set(13, "C");	
	state.set(6, "D");
	state.set(14, "D");

	let mut state2 = state; // copy it
	state2.energy_spent += 1; // change state 2 to have different energy
	state_mapping.insert(state, 100);
	assert!(!state_mapping.get(&state2).is_none());
	
    }

    #[test]
    fn test_heuristic() {
	let mut state = State::new();
 	state.set(7, "A"); 
	state.set(11, "A");	
	assert_eq!(state.get_heuristic_energy(), 1);
	state.set(9, "C");	
	assert_eq!(state.get_heuristic_energy(), 101);
	state.set(2, "C");	
	assert_eq!(state.get_heuristic_energy(), 601);
	state.set(8, "C");	
	assert_eq!(state.get_heuristic_energy(), 1101);
	    
    }

    /*
    #############
    #.....D.D...#
    ###.#B#C#.###
      #A#B#C#A#
      #########
     */
    #[test]
    fn test_solve_few_steps() {
	let mut state = State::new();
	state.set(11, "A");
 	state.set(8, "B");
 	state.set(12, "B"); 		
 	state.set(9, "C");
 	state.set(13, "C");
 	state.set(3, "D");
 	state.set(4, "D");
 	state.set(14, "A");
	let best = solve(state);
	assert_eq!(best, 7011);
	
    }
    /*
    #############
    #.....D.....#
    ###.#B#C#D###
      #A#B#C#A#
      #########
     */
    #[test]
    fn test_solve_few_more_steps() {
	let mut state = State::new();
	state.set(11, "A");
 	state.set(8, "B");
 	state.set(12, "B"); 		
 	state.set(9, "C");
 	state.set(13, "C");
 	state.set(3, "D");
 	state.set(10, "D");
 	state.set(14, "A");
	println!("\n\n\n\nblah!");
	let best = solve(state);
	println!("blah2!");	
	assert_eq!(best, 9011);
	
    }
    /*
    #############
    #...A.......#
    ###.#B#C#D###
      #A#B#C#D#
      #########
     */
    #[test]
    fn test_solve_easy_real() {
	// I seem to get to this state on the test input
	let mut state = State::new();
	state.energy_spent = 15158;
	state.prev_end = 2;	
	state.set(11, "A");
	state.set(2, "A");	
 	state.set(8, "B");
 	state.set(12, "B"); 		
 	state.set(9, "C");
 	state.set(13, "C");
 	state.set(10, "D");
 	state.set(14, "D");
	println!("\n\n\n\nblah!");
	let best = solve(state);
	println!("blah3!");	
	assert_eq!(best, 15160);
	
    }
    
}
