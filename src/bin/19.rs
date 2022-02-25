// Note: I based this solution on a solution outlined in https://www.reddit.com/r/adventofcode/comments/rjwhdv/2021_day19_i_need_help_understanding_how_to_solve/ by ignotos
// This is the first day I really struggled with solving and just wanted to move on from
use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Beacon(i32, i32, i32);

impl Beacon {
    fn distance(beacon1: Beacon, beacon2: Beacon) -> f64 {
	((i32::pow(beacon1.0 - beacon2.0, 2) + i32::pow(beacon1.1 - beacon2.1, 2) + i32::pow(beacon1.2 - beacon2.2, 2)) as f64).sqrt()
    }

    // beacon1 - beacon2, treated like vector subtraction
    fn subtract(beacon1: Beacon, beacon2: Beacon) -> Beacon {
	Beacon(beacon1.0 - beacon2.0, beacon1.1 - beacon2.1, beacon1.2 - beacon2.2)
    }
    
    // beacon1 + beacon2, treated like vector addition
    fn add(beacon1: Beacon, beacon2: Beacon) -> Beacon {
	Beacon(beacon1.0 + beacon2.0, beacon1.1 + beacon2.1, beacon1.2 + beacon2.2)
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    number: usize,
    beacons: Vec<Beacon>,
    distances: Vec<f64>,
    all_rotations: Vec<Vec<Beacon>>,
    rotation_and_offset_from_0: Option<(usize, Beacon)>, // the index of the proper rotation and offset to line up with scanner0
}

impl Scanner {
    fn new(number: usize) -> Self {	
	Self {
	    number: number,
	    beacons: Vec::<Beacon>::new(),
	    distances: Vec::<f64>::new(),
	    all_rotations: vec![Vec::new(); 24],
	    rotation_and_offset_from_0: None,
	}
    }

    // push the beacon as read from the file, but also push all 24 possible rotations of it into selfall_rotations
    fn push(&mut self, beacon: Beacon) {
	self.beacons.push(beacon);
	let Beacon(x, y, z) = beacon;
	self.all_rotations[0].push(Beacon(x, y, z));
	self.all_rotations[1].push(Beacon(x, z, -y ));
	self.all_rotations[2].push(Beacon(x, -y, -z));
	self.all_rotations[3].push(Beacon(x, -z, y));
	self.all_rotations[4].push(Beacon(-x, y, -z));
	self.all_rotations[5].push(Beacon(-x, z, y));
	self.all_rotations[6].push(Beacon(-x, -y, z));
	self.all_rotations[7].push(Beacon(-x, -z, -y));				
	self.all_rotations[8].push(Beacon(y, -x, z));
	self.all_rotations[9].push(Beacon(y, z, x));
	self.all_rotations[10].push(Beacon(y, -z, -x));
	self.all_rotations[11].push(Beacon(-y, x, z));
	self.all_rotations[12].push(Beacon(-y, -x, -z));
	self.all_rotations[13].push(Beacon(-y, z, -x));
	self.all_rotations[14].push(Beacon(-y, -z, x));
	self.all_rotations[15].push(Beacon(z, x, y));
	self.all_rotations[16].push(Beacon(z, -x, -y));
	self.all_rotations[17].push(Beacon(z, y, -x));
	self.all_rotations[18].push(Beacon(z, -y, x));
	self.all_rotations[19].push(Beacon(-z, x, -y));
	self.all_rotations[20].push(Beacon(-z, -x, y));
	self.all_rotations[21].push(Beacon(-z, y, x));
	self.all_rotations[22].push(Beacon(-z, -y, -x));
	
    }

    /// for each pair of beacons in the scanner, calculate the distance between them
    fn calculate_distances(&mut self) {
	for (i, b1) in self.beacons.iter().enumerate() {
	    for b2 in self.beacons[i+1..].iter() {
		let distance = Beacon::distance(*b1, *b2);
		self.distances.push(distance);
	    }
	}
    }


    fn find_beacon_matches_for_offset(&self, rotation: &Vec<Beacon>, offset: Beacon) -> Vec<Beacon> {
	let mut matched_beacons = Vec::new(); // store the (hopefully 12) beacons from the given rotation that match self.beacons
	for beacon1 in self.beacons.iter() {
	    for beacon2 in rotation.iter() {
		if Beacon::add(*beacon1, offset) == *beacon2 {
		    //println!("found a match from {:?} to {:?} with offset {:?}", beacon1, beacon2, offset);
		    matched_beacons.push(*beacon2);
		}
	    }		
	}
	if matched_beacons.len() > 1 {
	    println!("matched_beacons = {:?}", matched_beacons);
	    println!("offset = {:?}", offset);	    
	}
	matched_beacons
    }
    
    /// given another scanner determine the orientation of that scanner that lines up with the first
    /// We return an option that stores the index of the matching rotation, and the associated offset
    fn find_rotation_match(&self, other: &Scanner) -> Option<(usize, Beacon)> {
	for (rotation_index, rotation) in other.all_rotations.iter().enumerate() {
	    // for this orientation, we check if we can line up 12 points using the same offset
	    for (self_index, beacon1) in self.beacons.iter().enumerate() {	
		for (lock_index, beacon2) in rotation.iter().enumerate() {
		    let offset = Beacon::subtract(*beacon2, *beacon1);
		    // if beacon2 is the same as beacon1, just at this offset, then we should be able to find 12
		    // beacons that have this offset
		    let matched_beacons = self.find_beacon_matches_for_offset(&rotation, offset);
		    if matched_beacons.len() == 12 {
			return Some((rotation_index, offset));
		    }
		    
		}
	    }
	}
	None
    }
    
}

// read in the file and return a vec of scanners, where the index is the scanner num from the file
fn read_file() -> Vec<Scanner> {
    let buffered = get_buffered_reader("19-small");
    let re = Regex::new(r"([-\d]+),([-\d]+),([-\d]+)").unwrap();
    
    let mut all_scanners = Vec::new();

    let mut current_scanner = Scanner::new(all_scanners.len());
    
    for line in buffered.lines().flatten() {
	if line.is_empty() {
	    all_scanners.push(current_scanner);
	    current_scanner = Scanner::new(all_scanners.len());
	    continue;
	}
	if let Some(caps) = re.captures(&line) {
	    let beacon = Beacon (
		caps[1].parse::<i32>().unwrap(),
		caps[2].parse::<i32>().unwrap(),
		caps[3].parse::<i32>().unwrap(),		
	    );
	    current_scanner.push(beacon);
	}
    }
    all_scanners.push(current_scanner);    
    all_scanners
}
/// given two scanners (with populated distances), we return how many distance values the two beacons have in common
fn compare_scanners(scanner1: &Scanner, scanner2: &Scanner) -> usize {
    assert!(!scanner1.distances.is_empty() && !scanner2.distances.is_empty());
    scanner1.distances.iter().filter(|x| scanner2.distances.contains(x)).count()
}



fn run1() {
    let mut all_scanners = read_file();
    for scanner in all_scanners.iter_mut() {
	scanner.calculate_distances();
    }
    let mut was_change: bool = true;
    let known_scanners = vec![all_scanners[0].clone()]; // we know scanner zero
    while was_change {
	was_change = false;
	for (i, scanner1) in known_scanners.iter().enumerate() {
	    println!("scanner1 = {:?}", scanner1);
	    for (j, scanner2) in all_scanners[1..].iter().enumerate() {
		//println!("scanner2 = {:?}", scanner2);
		let common = compare_scanners(scanner1, scanner2);
		println!("scanner {} and {} have {} distances in common", i, j+1, common);
		if common == 66 {
		    // 12 choose 2 = 66, so there are 12 pairs in common and therefore 66 distances in common
		    // we know from the question that 12 pairs in common is what we are looking for
		    if let Some((rotation_index, offset)) = scanner1.find_rotation_match(scanner2){
			let mut new_2 = scanner2.clone();
			new_2.rotation_and_offset_from_0 = Some((rotation_index, offset));
		    } else {
			eprintln!("we should have been able to find an appropriate rotation and offset!");
		    }
		}
	    }
	}
    }
}

fn run2() {
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);    
    let part = &args[1];
    println!("part = {}", part);
    match part.as_str() {
	"1" => run1(),
	"2" => run2(),
	_ => panic!("invalid part number argument!"),
    }
}
