/// this one finally worked!!!!
/// had to use hints to realize that my state tree was needlessly big. the Amphipods should either move into the hall or directly "home",
/// before, I was allowing them to move 1 step at a time up and down the rooms and along the hall, but this was bloated.
/// Note: i somehow dont get the solution for the example... (off by only a few moves of C; but I get the star. hmm)
/// also not as fast as ppl online say their's are, but so be it
use std::{env, io};
use std::collections::{HashSet, HashMap};
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

const AMBER_INDICES: [usize;4] = [11,15,19,23];
const BRONZE_INDICES: [usize;4] = [12,16,20,24];
const COPPER_INDICES: [usize;4] = [13,17,21,25];
const DESERT_INDICES: [usize;4] = [14,18,22,26];

lazy_static! {

    static ref DOOR_INDICES: HashSet<usize> = {
	// spots right outside a door; not valid to stop here
        let mut m = HashSet::new();
        m.insert(2);
        m.insert(4);
        m.insert(6);
        m.insert(8);
	m
    };

    static ref DOOR_MAPPING: HashMap<Colour, usize> = {
	// maps from a given colour to the index of the spot outside its door
        let mut m = HashMap::new();
        m.insert(Colour::Amber, 2);
        m.insert(Colour::Bronze, 4);
        m.insert(Colour::Copper, 6);
        m.insert(Colour::Desert, 8);	
        m
    };

    static ref COLOUR_TO_ROOM_INDICES: HashMap<Colour, [usize;4]> = {
        let mut m = HashMap::new();
        m.insert(Colour::Amber, AMBER_INDICES);
        m.insert(Colour::Bronze, BRONZE_INDICES);
        m.insert(Colour::Copper, COPPER_INDICES);
        m.insert(Colour::Desert, DESERT_INDICES);	
        m
	
    };    
    static ref ROOM_INDICES_TO_COLOUR: HashMap<usize, Colour> = {
        let mut m = HashMap::new();
	for room_idx in AMBER_INDICES {
            m.insert(room_idx, Colour::Amber);
	}
	for room_idx in BRONZE_INDICES {
            m.insert(room_idx, Colour::Bronze);
	}
	for room_idx in COPPER_INDICES {
            m.insert(room_idx, Colour::Copper);
	}
	for room_idx in DESERT_INDICES {
            m.insert(room_idx, Colour::Desert);
	}
        m
	
    };
    
    static ref MOVES_MAPPING: HashMap<usize, Vec<(usize, u64)>> = {
        let mut m = HashMap::new();
	m.insert(0, vec![(1,1)]);
	
	m.insert(1, vec![(0,1), (2,2), (7,2)]);
	m.insert(2, vec![(1,2), (3,2), (7,2), (8,2)]);
	m.insert(3, vec![(2,2), (4,2), (8,2), (9,2)]);
	m.insert(4,  vec![(3,2), (5,2), (9,2), (10,2)]);
	m.insert(5, vec![(6,1), (4, 2), (10, 2)]);
	m.insert(6, vec![(5,1)]);
	m.insert(7, vec![(1,2), (2,2), (11,1)]);
	m.insert(8, vec![(2,2), (3,2), (12,1)]);
	m.insert(9,  vec![(3,2), (4,2), (13,1)]);
	m.insert(10, vec![(4,2), (5,2), (14,1)] );
	m.insert(11, vec![(7,1), (15,1)] );
	m.insert(12, vec![(8,1), (16,1)] );
	m.insert(13, vec![(9,1), (17,1)]);
	m.insert(14, vec![(10, 1), (18,1)]);
	m.insert(15, vec![(11, 1), (19,1)]);
	m.insert(16, vec![(12, 1), (20,1)]);
	m.insert(17, vec![(13, 1), (21,1)]);
	m.insert(18, vec![(14, 1), (22,1)]);
	m.insert(19, vec![(15, 1)]);
	m.insert(20, vec![(16, 1)]);
	m.insert(21, vec![(17, 1)]);
	m.insert(22, vec![(18, 1)]);	
        m
    };

    // locked ampipods (will be in the hallway) can only move home. And it depends on colour
    static ref LOCKED_MOVES_MAPPING: HashMap<(usize, Colour), Vec<(usize, u64)>> = {
        let mut m = HashMap::new();
	m.insert((0, Colour::Amber), vec![(7,3)]);
	m.insert((1, Colour::Amber), vec![(7,2)]);
	m.insert((2, Colour::Amber), vec![(7,2)]);
	m.insert((3, Colour::Amber), vec![(7,4)]);
	m.insert((4, Colour::Amber), vec![(7,6)]);
	m.insert((5, Colour::Amber), vec![(7,8)]);
	m.insert((6, Colour::Amber), vec![(7,9)]);		
	
	m.insert((0, Colour::Bronze), vec![(8,5)]);
	m.insert((1, Colour::Bronze), vec![(8,4)]);
	m.insert((2, Colour::Bronze), vec![(8,2)]);
	m.insert((3, Colour::Bronze), vec![(8,2)]);
	m.insert((4, Colour::Bronze), vec![(8,4)]);
	m.insert((5, Colour::Bronze), vec![(8,6)]);
	m.insert((6, Colour::Bronze), vec![(8,7)]);		
	
	m.insert((0, Colour::Copper), vec![(9,7)]);
	m.insert((1, Colour::Copper), vec![(9,6)]);
	m.insert((2, Colour::Copper), vec![(9,4)]);
	m.insert((3, Colour::Copper), vec![(9,2)]);
	m.insert((4, Colour::Copper), vec![(9,2)]);
	m.insert((5, Colour::Copper), vec![(9,4)]);
	m.insert((6, Colour::Copper), vec![(9,5)]);		

	m.insert((0, Colour::Desert), vec![(10,9)]);
	m.insert((1, Colour::Desert), vec![(10,8)]);
	m.insert((2, Colour::Desert), vec![(10,6)]);
	m.insert((3, Colour::Desert), vec![(10,4)]);
	m.insert((4, Colour::Desert), vec![(10,2)]);
	m.insert((5, Colour::Desert), vec![(10,2)]);
	m.insert((6, Colour::Desert), vec![(10,3)]);		
	m
    };

    // tells s which inices must be clear for the locked_moves_mapping to apply
    // e.g to move a bronze ampipod for 5 into 8, then 3 and 4 must be empty
    static ref LOCKED_PATH: HashMap<(usize, Colour), Vec<usize>> = {
        let mut m = HashMap::new();
	m.insert((0, Colour::Amber), vec![1]);
	m.insert((1, Colour::Amber), vec![]);
	m.insert((2, Colour::Amber), vec![]);
	m.insert((3, Colour::Amber), vec![2]);
	m.insert((4, Colour::Amber), vec![2,3]);
	m.insert((5, Colour::Amber), vec![2,3,4]);
	m.insert((6, Colour::Amber), vec![2,3,4,5]);		
	
	m.insert((0, Colour::Bronze), vec![1,2]);
	m.insert((1, Colour::Bronze), vec![2]);
	m.insert((2, Colour::Bronze), vec![]);
	m.insert((3, Colour::Bronze), vec![]);
	m.insert((4, Colour::Bronze), vec![3]);
	m.insert((5, Colour::Bronze), vec![3,4]);
	m.insert((6, Colour::Bronze), vec![3,4,5]);		
	
	m.insert((0, Colour::Copper), vec![1,2,3]);
	m.insert((1, Colour::Copper), vec![2,3]);
	m.insert((2, Colour::Copper), vec![3]);
	m.insert((3, Colour::Copper), vec![]);
	m.insert((4, Colour::Copper), vec![]);
	m.insert((5, Colour::Copper), vec![4]);
	m.insert((6, Colour::Copper), vec![4,5]);		

	m.insert((0, Colour::Desert), vec![1,2,3,4]);
	m.insert((1, Colour::Desert), vec![2,3,4]);
	m.insert((2, Colour::Desert), vec![3,4]);
	m.insert((3, Colour::Desert), vec![4]);
	m.insert((4, Colour::Desert), vec![]);
	m.insert((5, Colour::Desert), vec![]);
	m.insert((6, Colour::Desert), vec![5]);		
	m
    };
    
}

const NUM_SPOTS: usize = 27;

/// the state keeps tracks of what is in each of the 15 rooms
#[derive(Debug, Copy, Clone, Eq)]
struct State {
    spots: [Option<Amphipod>; NUM_SPOTS],
    energy_spent: u64, // total amount of energy spent by all moves
    depth: u64,
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
we store the hallway as indices 0..=10, then the hall is 11..=26
*/
impl State {
    fn new() -> Self {
	Self {spots: [None; NUM_SPOTS], energy_spent: 0, depth: 0}
    }

    /// a heuristic of how well the postion is doing.
    /// the higher the score the "worse" the state
    /// we take the distance for each amphipod to get to the bottom of its room
    /// we also add a penalty for each amphipod in the hallway (since a state with more amphiods in their rooms is better)
    fn get_heuristic_energy(&self) -> u64 {
	let mut score = 0;
	let mut num_bad_rooms = 0;
	if let Some(Amphipod{colour, energy, ..}) = self.spots[0] {
	    //score += 1; // hallway is worse than room	    
	    score += match colour {
		Colour::Amber => 6 * energy,
		Colour::Bronze => 8 * energy,
		Colour::Copper => 10 * energy,
		Colour::Desert => 12 * energy,
	    }
	}
	if let Some(Amphipod{colour, energy, ..}) = self.spots[1] {
	    //score += 1; // hallway is worse than room	    	    
	    score += match colour {
			Colour::Amber => 5 * energy,
			Colour::Bronze => 7 * energy,
			Colour::Copper => 9 * energy,
			Colour::Desert => 11 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[2] {
	    //score += 1; // hallway is worse than room	    	    
	    score += match colour {
			Colour::Amber => 5 * energy,
			Colour::Bronze => 5 * energy,
			Colour::Copper => 7 * energy,
			Colour::Desert => 9 * energy,
		}
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[3] {
	    //score += 1; // hallway is worse than room	    	    
	    score += match colour {
			Colour::Amber => 7 * energy,
			Colour::Bronze => 5 * energy,
			Colour::Copper => 5 * energy,
			Colour::Desert => 7 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[4] {
	    //score += 1; // hallway is worse than room	    	    
	    score += match colour {
			Colour::Amber => 9 * energy,
			Colour::Bronze => 7 * energy,
			Colour::Copper => 5 * energy,
			Colour::Desert => 5 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[5] {
	    //score += 1;
	    score += match colour {
			Colour::Amber => 11 * energy,
			Colour::Bronze => 9 * energy,
			Colour::Copper => 7 * energy,
			Colour::Desert => 5 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[6] {
	    score += match colour {
		Colour::Amber => 12 * energy,
		Colour::Bronze => 10 * energy,
		Colour::Copper => 8 * energy,
		Colour::Desert => 6 * energy,
	    }
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[7] {
	    score += match colour {
		Colour::Amber => 3 * energy,
		Colour::Bronze => 7 * energy,
		Colour::Copper => 9 * energy,
		Colour::Desert => 11 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Amber => 0,
		_ => 1,
	    };
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[8] {
	    score += match colour {
		Colour::Amber => 7 * energy,
		Colour::Bronze => 3 * energy,
		Colour::Copper => 7 * energy,
		Colour::Desert => 9 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Bronze => 0,
		_ => 1,
	    };	    
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[9] {
	    score += match colour {
		Colour::Amber => 9 * energy,
		Colour::Bronze => 7 * energy,
		Colour::Copper => 3 * energy,
		Colour::Desert => 7 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Copper => 0,
		_ => 1,
	    };	    	    	    
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[10] {
	    score += match colour {
		Colour::Amber => 11 * energy,
		Colour::Bronze => 9 * energy,
		Colour::Copper => 7 * energy,
		Colour::Desert => 3 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Desert => 0,
		_ => 1,
	    };	    	    	    
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[11] {
	    score += match colour {
		Colour::Amber => 2 * energy,
		Colour::Bronze => 8 * energy,
		Colour::Copper => 10 * energy,
		Colour::Desert => 12 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Amber => 0,
		_ => 1,
	    };	    
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[12] {
	    score += match colour {
		Colour::Amber => 8 * energy,
		Colour::Bronze => 2 * energy,
		Colour::Copper => 8 * energy,
		Colour::Desert => 10 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Bronze => 0,
		_ => 1,
	    };	    	    
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[13] {
	    score += match colour {
		Colour::Amber => 10 * energy,
		Colour::Bronze => 8 * energy,
		Colour::Copper => 2 * energy,
		Colour::Desert => 8 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Copper => 0,
		_ => 1,
	    };	    	    	    
	}	    
	if let Some(Amphipod{colour, energy, ..}) = self.spots[14] {
	    score += match colour {
		Colour::Amber => 12 * energy,
		Colour::Bronze => 10 * energy,
		Colour::Copper => 8 * energy,
		Colour::Desert => 2 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Desert => 0,
		_ => 1,
	    };	    	    	    
	}
	if let Some(Amphipod{colour, energy, ..}) = self.spots[15] {
	    score += match colour {
		Colour::Amber => 1 * energy,
		Colour::Bronze => 9 * energy,
		Colour::Copper => 11 * energy,
		Colour::Desert => 13 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Amber => 0,
		_ => 1,
	    };	    
	}
	if let Some(Amphipod{colour, energy, ..}) = self.spots[16] {
	    score += match colour {
		Colour::Amber => 11 * energy,
		Colour::Bronze => 9 * energy,
		Colour::Copper => 1 * energy,
		Colour::Desert => 9 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Bronze => 0,
		_ => 1,
	    };	    	    
	}
	if let Some(Amphipod{colour, energy, ..}) = self.spots[17] {
	    score += match colour {
		Colour::Amber => 11 * energy,
		Colour::Bronze => 9 * energy,
		Colour::Copper => 1 * energy,
		Colour::Desert => 9 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Copper => 0,
		_ => 1,
	    };	    	    	    
	}
	if let Some(Amphipod{colour, energy, ..}) = self.spots[18] {
	    score += match colour {
		Colour::Amber => 13 * energy,
		Colour::Bronze => 11 * energy,
		Colour::Copper => 9 * energy,
		Colour::Desert => 1 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Desert => 0,
		_ => 1,
	    };	    	    	    
	}
	if let Some(Amphipod{colour, energy, ..}) = self.spots[19] {
	    score += match colour {
		Colour::Amber => 0 * energy,
		Colour::Bronze => 10 * energy,
		Colour::Copper => 12 * energy,
		Colour::Desert => 14 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Amber => 0,
		_ => 1,
	    };	    
	}
	if let Some(Amphipod{colour, energy, ..}) = self.spots[20] {
	    score += match colour {
		Colour::Amber => 10 * energy,
		Colour::Bronze => 0 * energy,
		Colour::Copper => 10 * energy,
		Colour::Desert => 12 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Bronze => 0,
		_ => 1,
	    };	    	    
	}
	if let Some(Amphipod{colour, energy, ..}) = self.spots[21] {
	    score += match colour {
		Colour::Amber => 12 * energy,
		Colour::Bronze => 10 * energy,
		Colour::Copper => 0 * energy,
		Colour::Desert => 10 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Copper => 0,
		_ => 1,
	    };	    	    	    
	}
	if let Some(Amphipod{colour, energy, ..}) = self.spots[22] {
	    score += match colour {
		Colour::Amber => 14 * energy,
		Colour::Bronze => 12 * energy,
		Colour::Copper => 10 * energy,
		Colour::Desert => 0 * energy,
	    };
	    num_bad_rooms += match colour {
		Colour::Desert => 0,
		_ => 1,
	    };	    	    	    
	}

	// we multiply score by 100 so that num_bad_rooms is basically just a tie-breaker in lower significant digits
	(score * 100) + num_bad_rooms
    }
    
    fn pretty_print(&self) {
	println!("#############");
	print!("#");
	for i in 0..11 {
	    if let Some(Amphipod{colour, ..}) = self.spots[i] {
		print!("{}", colour_to_char(colour));
	    } else {
		print!(".");
	    }
	    
	}
	println!("#");	
	print!("###");
	for i in 11..15 {
	    if let Some(Amphipod{colour, ..}) = self.spots[i] {
		print!("{}", colour_to_char(colour));
	    } else {
		print!(".");
	    }
	    print!("#");
	}
	println!("##");
	print!("  #");
	for i in 15..19 {
	    if let Some(Amphipod{colour, ..}) = self.spots[i] {
		print!("{}", colour_to_char(colour));
	    } else {
		print!(".");
	    }
	    print!("#");
	}
	println!("");	
	print!("  #");
	for i in 19..23 {
	    if let Some(Amphipod{colour, ..}) = self.spots[i] {
		print!("{}", colour_to_char(colour));
	    } else {
		print!(".");
	    }
	    print!("#");
	}
	println!("");
	print!("  #");
	for i in 23..27 {
	    if let Some(Amphipod{colour, ..}) = self.spots[i] {
		print!("{}", colour_to_char(colour));
	    } else {
		print!(".");
	    }
	    print!("#");
	}
	println!("");	
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
	index < 11
    }
    
    
    fn is_amber_done(&self) -> bool {
	for &idx in AMBER_INDICES.iter() {
	    if let Some(Amphipod{colour: Colour::Amber, ..}) = self.spots[idx] {
		
	    } else {
		return false
	    }
	}
	true
    }
    
    fn is_bronze_done(&self) -> bool {
	for &idx in BRONZE_INDICES.iter() {
	    if let Some(Amphipod{colour: Colour::Bronze, ..}) = self.spots[idx] {
		
	    } else {
		return false
	    }
	}
	true
    }
    
    fn is_copper_done(&self) -> bool {
	for &idx in COPPER_INDICES.iter() {
	    if let Some(Amphipod{colour: Colour::Copper, ..}) = self.spots[idx] {
		
	    } else {
		return false
	    }
	}
	true
    }
    
    fn is_desert_done(&self) -> bool {
	for &idx in DESERT_INDICES.iter() {
	    if let Some(Amphipod{colour: Colour::Desert, ..}) = self.spots[idx] {
		
	    } else {
		return false
	    }
	}
	true
    }


    /// check if there is an amphipod of the wrong colour in the room for the given colour
    fn is_room_dirty(&self, colour: Colour) -> bool {
	let room_indices = COLOUR_TO_ROOM_INDICES.get(&colour).unwrap();
	for &room_idx in room_indices.iter() {
	    if let Some(Amphipod{colour: colour2, ..}) = self.spots[room_idx] {
		if colour2 != colour {
		    return true; //found the wrong colour
		}
	    }
	}
	false
    }


    /// given a from_index of an ampipod to all the possible locations in the hallway
    /// returns a (possibly empty) vec of new states
    /// we assume we can clearly move out of the room
    fn move_into_hallway(&self, from_index: usize) -> Vec<Self> {
	let mut hall_states = Vec::new();		
	let room_colour = ROOM_INDICES_TO_COLOUR.get(&from_index).unwrap();
			
	let room_indices = COLOUR_TO_ROOM_INDICES.get(&room_colour).unwrap();
	let mut room_steps: u64 = 0;
	for &room_idx in room_indices.iter() {
	    room_steps += 1;
	    if room_idx == from_index {
		break;
	    }
	}
	//println!("{} steps to get out of the room", room_steps);
	let door_idx = DOOR_MAPPING.get(&room_colour).unwrap();
	let mut hall_steps_right = room_steps;
	for hall_idx in (*door_idx..11).skip(1) {
	    // to the right in the hall
	    //println!("looking at hall idx {}", hall_idx);

	    if !self.spots[hall_idx].is_none() {
		//println!("blocked");
		break;
	    }
	    hall_steps_right += 1;	    
	    if DOOR_INDICES.contains(&hall_idx) {
		// don't stop outside a door
		//println!("cannot stop at door");
		continue;
	    }
	    

	    //println!("new state from {} to {}, with {} steps", from_index, hall_idx, hall_steps_right);
	    let mut new_state = *self; // copy ourself
	    new_state.spots[hall_idx] = new_state.spots[from_index];	   
	    new_state.spots[from_index] = None;
	    new_state.energy_spent += new_state.spots[hall_idx].unwrap().energy * hall_steps_right;
	    new_state.depth += 1;
	    hall_states.push(new_state);
	}
	
	//println!("\n\ndoor id = {} step = {}", door_idx, steps);
	let mut hall_steps_left = room_steps;	
	for hall_idx in (0..*door_idx).rev() {
	    // to the left in the hall
	    //println!("hall id = {}", hall_idx);
	    if !self.spots[hall_idx].is_none() {
		//println!("blocked");
		break;
	    }
	    hall_steps_left += 1;	    
	    if DOOR_INDICES.contains(&hall_idx) {
		// don't stop outside a door
		//println!("cannot stop at door");
		continue;
	    }
	    //println!("new state from {} to {}, with {} steps", from_index, hall_idx, hall_steps_left);	    
	    let mut new_state = *self; // copy ourself
	    new_state.spots[hall_idx] = new_state.spots[from_index];	   
	    new_state.spots[from_index] = None;
	    new_state.energy_spent += new_state.spots[hall_idx].unwrap().energy * hall_steps_left;
	    new_state.depth += 1;
	    hall_states.push(new_state);
	}
	//println!("steps after = {}", steps);	    		    	
	hall_states
    }


	
    /// given a from index, we move the ampipod from the from index as deep into the room as we can
    /// we assume validation of a clear path to the room already occured
    fn move_into_room(&self, from_index: usize) -> Self {
	let colour = self.spots[from_index].unwrap().colour;
	//println!("trying to move {:?} from {}", colour, from_index, to_index);
	let door_idx = DOOR_MAPPING.get(&colour).unwrap();
	//println!("door idx = {}", door_idx);		    	
	let mut steps = std::cmp::max(from_index, *door_idx) - std::cmp::min(from_index, *door_idx) + 1; // add one cuz we are going at least one step into the room
	
	let room_indices = COLOUR_TO_ROOM_INDICES.get(&colour).unwrap();
	let mut to_index = room_indices[0]; // at least the first spot needs to be free to move into a room
	//println!("to idx = {}, steps to first spot in room = {}", to_index, steps);		    		
	assert!(self.spots[to_index].is_none());
	for &room_idx in room_indices.iter().skip(1) {
	    if self.spots[room_idx].is_none() {
		to_index = room_idx;
		steps += 1;
	    } else {
		break;
	    }
	}
	//println!("new state from {} to {}, with {} steps", from_index, to_index, steps);	    	
	let mut new_state = *self; // copy ourself
	new_state.spots[to_index] = new_state.spots[from_index];	   
	new_state.spots[from_index] = None;
	new_state.energy_spent += new_state.spots[to_index].unwrap().energy * steps as u64;
	new_state.depth += 1;
	new_state
    }

    /// the current state returns a vector of all other valid States based on moving on Amphipod
    fn get_valid_transitions(&self) -> Vec<Self> {
	let mut valid_states = Vec::new();	
	for from_index in 0..NUM_SPOTS {
	    //println!("from index = {}", from_index);
	    if let Some(Amphipod{colour, energy: _ }) = self.spots[from_index] {
		if State::is_hallway_index(from_index) {
		    // in hallway, can only move home if possible
		    // check for blockers
		    //println!("hallway!");
		    let door_idx = DOOR_MAPPING.get(&colour).unwrap();
		    //println!("door idx = {}", door_idx);
		    let higher = std::cmp::max(from_index, *door_idx);
		    let lower = std::cmp::min(from_index, *door_idx);
		    let clear_to_door = (lower..=higher)
			//.inspect(|x| println!("looking at {}", x))
			.all(|hall_idx| hall_idx == from_index || self.spots[hall_idx].is_none()); // we dont need the from_index to be none (obviously)
		    if !clear_to_door {
			// cannot move to the door, so no where to go really
			//println!("not clear to the door!");
			continue;
		    }
		    if self.is_room_dirty(colour) {
			// can't move into a room with the wrong colour in it
			//println!("room is dirty!");
			continue;
		    }
		    let new_state = self.move_into_room(from_index);
		    valid_states.push(new_state);

		} else {
		    // can move anywhere into the hallway that is not blocked and not right outside a room
		    // check for blockers
		    let room_colour = ROOM_INDICES_TO_COLOUR.get(&from_index).unwrap();
		    //println!("we are in room colour {:?}", room_colour);
			
		    if *room_colour == colour && !self.is_room_dirty(colour) {
			// we are in the proper room and there is nothing wrong with it, so dont move
			//println!("don't move out of a non-dirty room!");
			//self.pretty_print();
			continue;
		    }
			
		    let room_indices = COLOUR_TO_ROOM_INDICES.get(&room_colour).unwrap();
		    let mut clear_to_door = true;
		    for &room_idx in room_indices.iter() {
			if room_idx == from_index {
			    break;
			} else if !self.spots[room_idx].is_none() {
			    clear_to_door = false;
			    //println!("we cannot escape the room since {} is blocked", room_idx);
			    break;
			}
			
		    }
		    if !clear_to_door {
			// cannot move to the door, so no where to go really
			continue;
		    }
		    
		    let new_states = self.move_into_hallway(from_index);
		    valid_states.extend(new_states);
		}
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
    let buffered = get_buffered_reader("23-2");
    let mut lines =  buffered.lines().skip(2).flatten();

    let mut starting_state = State::new();
    // we read in the 8 amphipods, starting at index 7
    let re1 = Regex::new(r"###(\w)#(\w)#(\w)#(\w)###").unwrap();
    let line1 = lines.next().unwrap();
    println!("line 1 = {:?}", line1);
    if let Some(caps) = re1.captures(&line1) {
	println!("matched line 1");
	starting_state.set(11, &caps[1])?;
	starting_state.set(12, &caps[2])?;
	starting_state.set(13, &caps[3])?;
	starting_state.set(14, &caps[4])?;	
    } else {
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "unable to parse input file on the first Amphipod line"));
    }
    
    let line2 = lines.next().unwrap();
    let re2 = Regex::new(r"  #(\w)#(\w)#(\w)#(\w)#").unwrap();    
    println!("line 2 = {:?}",  line2);
    if let Some(caps) = re2.captures(&line2) {
	starting_state.set(15, &caps[1])?;
	starting_state.set(16, &caps[2])?;
	starting_state.set(17, &caps[3])?;
	starting_state.set(18, &caps[4])?;	
	
    } else {
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "unable to parse input file on the second Amphipod line"));
    }
    let line3 = lines.next().unwrap();
    let re3 = Regex::new(r"  #(\w)#(\w)#(\w)#(\w)#").unwrap();    
    println!("line 3 = {:?}",  line3);
    if let Some(caps) = re3.captures(&line3) {
	starting_state.set(19, &caps[1])?;
	starting_state.set(20, &caps[2])?;
	starting_state.set(21, &caps[3])?;
	starting_state.set(22, &caps[4])?;	
	
    } else {
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "unable to parse input file on the third Amphipod line"));
    }
    let line4 = lines.next().unwrap();
    let re4 = Regex::new(r"  #(\w)#(\w)#(\w)#(\w)#").unwrap();    
    println!("line 4 = {:?}",  line4);
    if let Some(caps) = re4.captures(&line4) {
	starting_state.set(23, &caps[1])?;
	starting_state.set(24, &caps[2])?;
	starting_state.set(25, &caps[3])?;
	starting_state.set(26, &caps[4])?;	
	
    } else {
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "unable to parse input file on the 4th Amphipod line"));
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
	if count > 100_000_000 {
	    println!("\n\ncount = {}", count);
	    println!("search queue len = {:?}", search_queue.len());
	    println!("size of state mapping = {}", state_mapping.len());
	    println!("current state = {:?}", current_state);
	    println!("state score = {}", state_score);	    	    
	    break;
	}

	if count % 75_000 == 0 {
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
	let next_states = current_state.get_valid_transitions();
	
	//println!("next states:");
	for state in next_states {
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
    //assert!(args.len() == 2);    
    let part = 2; //&args[1];
    println!("part = {}", part);
    run()
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
	state.set(11, "A");	
 	state.set(15, "A");
	state.set(19, "A");
 	state.set(23, "A");	
	
	state.set(12, "B");	
	state.set(16, "B");
	state.set(20, "B");	
	state.set(24, "B");
	
	state.set(13, "C");	
	state.set(17, "C");
	state.set(21, "C");
	state.set(25, "C");
	
	state.set(14, "D");	
	state.set(18, "D");
	state.set(22, "D");
	state.set(26, "D");	
	assert!(state.is_complete());
    }


    #[test]
    fn test_solve_easy() {
	let mut state = State::new();

	state.set(1, "A");	
 	state.set(15, "A");
	state.set(19, "A");
 	state.set(23, "A");	
	
	state.set(12, "B");	
	state.set(16, "B");
	state.set(20, "B");	
	state.set(24, "B");
	
	state.set(13, "C");	
	state.set(17, "C");
	state.set(21, "C");
	state.set(25, "C");
	
	state.set(14, "D");	
	state.set(18, "D");
	state.set(22, "D");
	state.set(26, "D");	

	let best = solve(state);
	assert_eq!(best, 2); // the Amber needs to move from 1 into 7, which is "two" steps X 1 energy each
    }

    #[test]
    fn test_solve_easy_2() {
	let mut state = State::new();
	state.set(0, "A");	
 	state.set(15, "A");
	state.set(19, "A");
 	state.set(23, "A");	
	
	state.set(12, "B");	
	state.set(16, "B");
	state.set(20, "B");	
	state.set(24, "B");
	
	state.set(13, "C");	
	state.set(17, "C");
	state.set(21, "C");
	state.set(25, "C");
	
	state.set(14, "D");	
	state.set(18, "D");
	state.set(22, "D");
	state.set(26, "D");	


	let best = solve(state);
	assert_eq!(best, 3); 
    }

    #[test]
    fn test_solve_easy_bronze() {
	let mut state = State::new();
	state.set(11, "A");	
 	state.set(15, "A");
	state.set(19, "A");
 	state.set(23, "A");	
	
	state.set(5, "B");	
	state.set(16, "B");
	state.set(20, "B");	
	state.set(24, "B");
	
	state.set(13, "C");	
	state.set(17, "C");
	state.set(21, "C");
	state.set(25, "C");
	
	state.set(14, "D");	
	state.set(18, "D");
	state.set(22, "D");
	state.set(26, "D");	

	
	let best = solve(state);
	assert_eq!(best, 20);
    }
    
    #[test]
    fn test_solve_easy_copper() {
	let mut state = State::new();
	state.set(11, "A");	
 	state.set(15, "A");
	state.set(19, "A");
 	state.set(23, "A");	
	
	state.set(12, "B");	
	state.set(16, "B");
	state.set(20, "B");	
	state.set(24, "B");
	
	state.set(7, "C");	
	state.set(17, "C");
	state.set(21, "C");
	state.set(25, "C");
	
	state.set(14, "D");	
	state.set(18, "D");
	state.set(22, "D");
	state.set(26, "D");	
	
	let best = solve(state);
	assert_eq!(best, 200);
    }
    
    #[test]
    fn test_solve_easy_desert() {
	let mut state = State::new();
	state.set(11, "A");	
 	state.set(15, "A");
	state.set(19, "A");
 	state.set(23, "A");	
	
	state.set(12, "B");	
	state.set(16, "B");
	state.set(20, "B");	
	state.set(24, "B");
	
	state.set(13, "C");	
	state.set(17, "C");
	state.set(21, "C");
	state.set(25, "C");
	
	state.set(10, "D");	
	state.set(18, "D");
	state.set(22, "D");
	state.set(26, "D");	

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
	assert_eq!(state.get_heuristic_energy(), 500);
	state.set(9, "C");	
	assert_eq!(state.get_heuristic_energy(), 30500);
	state.set(2, "C");	
	assert_eq!(state.get_heuristic_energy(), 100500);
	state.set(8, "C"); // this is a bad room, so we need to add 1
	assert_eq!(state.get_heuristic_energy(), 170501);
	    
    }

    
    #[test]
    fn test_solve_few_steps_2() {
	let mut state = State::new();
	state.set(10, "A");
 	state.set(15, "A");
	state.set(19, "A");
	state.set(23, "A");	
	
	state.set(8, "B");
	state.set(16, "B");
	state.set(20, "B");
	state.set(24, "B");		
	
	state.set(4, "C");
	state.set(17, "C");
	state.set(21, "C");
	state.set(25, "C");	
	
	state.set(0, "D");
	state.set(18, "D");
	state.set(22, "D");
	state.set(26, "D");		
	
	let best = solve(state);
	assert_eq!(best, 9359);
	
    }
    
    #[test]
    fn test_solve_few_steps_3() {
	let mut state = State::new();
	state.set(10, "A");
 	state.set(15, "A");
	state.set(19, "A");
	state.set(23, "A");	
	
	state.set(8, "B");
	state.set(16, "B");
	state.set(20, "B");
	state.set(24, "B");		
	
	state.set(11, "C");
	state.set(17, "C");
	state.set(21, "C");
	state.set(25, "C");	
	
	state.set(12, "D");
	state.set(18, "D");
	state.set(22, "D");
	state.set(26, "D");		
	
	let best = solve(state);
	assert_eq!(best, 8659);
	
    }

    #[test]
    fn test_solve_few_steps_4() {
	let mut state = State::new();
	state.set(11, "A");
 	state.set(15, "A");
	state.set(19, "A");
	state.set(23, "A");	
	
	state.set(12, "B");
	state.set(16, "B");
	state.set(20, "B");
	state.set(24, "B");		
	
	state.set(0, "C");
	state.set(17, "C");
	state.set(21, "C");
	state.set(25, "C");	
	
	state.set(1, "D");
	state.set(18, "D");
	state.set(22, "D");
	state.set(26, "D");		
	
	let best = solve(state);
	assert_eq!(best, 8700);
	
    }
    
}
