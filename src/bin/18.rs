use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;



/// the ExplosionReturn gets propogated upwards
struct ExplosionReturn {
    did_explode: bool, // was there an explosion somewhere down the recursion
    immediate_explode: bool, // was there an explosion from this immediate call
    left_val: u32, // the value from the left side of the explosion (if non-zero then still needs to be consumed at some point)
    right_val: u32, // the value from the right side of the explosion (if non-zero then still needs to be consumed at some point)
}

/*

impl ExplosionReturn {
    fn new(did_explode: bool, immediate_explode: bool, left_val: u32, right_val
}
 */

#[derive(Debug, PartialEq, Clone)]
enum SnailFish {
    Regular(u32),
    Pair{ a : Box<SnailFish>, b: Box<SnailFish> },
}

impl SnailFish {
    fn magnitude(&self) -> u32 {
	match self {
	    SnailFish::Regular(val) => *val,
	    SnailFish::Pair{a, b} =>  3 * a.magnitude() + 2 * b.magnitude(),
	}	
    }
    
    fn recursive_pretty(&self) {
	match self {
	    SnailFish::Regular(val) => print!("{}", *val),
	    SnailFish::Pair{a, b} => {
		print!("[");
		a.recursive_pretty();
		print!(",");
		b.recursive_pretty();		
		print!("]");		
	    }
	}	
    }
    
    fn pretty_print(&self) {
	self.recursive_pretty();
	println!();
    }

    
    /// if the fish is regular gets its value, else None
    fn get_numeric_val(&self) -> Option<u32> {
	match self {
	    SnailFish::Regular(val) => Some(*val),
	    SnailFish::Pair{a, b} => None,
	}	
    }

    fn add(x: SnailFish, y: SnailFish) -> SnailFish {
	SnailFish::Pair { a: Box::new(x), b: Box::new(y) }
    }

    fn add_num_from_right(&mut self, num: u32) {
	match self {
	    SnailFish::Regular(val) => *val += num,
	    SnailFish::Pair{a, b} => b.add_num_from_right(num),
	}
    }

    fn add_num_from_left(&mut self, num: u32) {
	match self {
	    SnailFish::Regular(val) => *val += num,
	    SnailFish::Pair{a, b} => a.add_num_from_left(num),
	}
    }

    /// check if anything explodes or splits,
    /// we return early if we find an explosion so that the process can start over again
    /// if no reduction occured, we return false
    fn reduce(&mut self) -> bool {
	let result = self.check_explode(1);
	if result.did_explode {
	    //println!("did explode");
	    return true;
	}
	let (did_split_any, _) = self.check_split();
	if did_split_any {
	    //println!("did split");	    
	    return true;
	}
	false
    }

    fn full_reduction(&mut self) {
	while self.reduce() {
	    //println!("after one reduction");
	    //self.pretty_print();
	}
    }
    
    /// check explode reutrns two bools, the first one indicates if any recursive calls exploded (and propogates all the way up),
    /// the second indicates if this immediate call exploded
    fn check_explode(&mut self, depth: u32) -> ExplosionReturn {
	//println!("check explode for {:?} at depth = {:?}", self, depth);
	
	match self {
	    SnailFish::Regular(_) => return ExplosionReturn {did_explode: false, immediate_explode: false, left_val: 0, right_val: 0},
	    SnailFish::Pair{a, b} => {
		if depth == 5 {
		    // time to explode!
		    //println!("time to explode! {:?}, {:?}", a, b);
		    
		    return ExplosionReturn {
			did_explode: true,
			immediate_explode: true,
			left_val: a.get_numeric_val().unwrap(), // we can unwrap, since only a pair of regular fish can explode
			right_val: b.get_numeric_val().unwrap()};
		} else {
		    let ExplosionReturn {did_explode, immediate_explode, left_val, mut right_val} =  a.check_explode(depth + 1);
		    //println!("did_explode from a = {:?}", did_explode);
		    if right_val != 0{
			b.add_num_from_left(right_val);
			right_val = 0; 	// we consumed the right val, so we dont propogate it upwards
		    }
		    if immediate_explode {
			// if a exploded, the new a is just a regular(0)
			*a = Box::new(SnailFish::Regular(0));
		    }
		    if did_explode {
			// short circuit if the left side exploded
			return ExplosionReturn {did_explode: did_explode, immediate_explode: false, left_val: left_val, right_val: right_val};
		    }
		    
		    let ExplosionReturn {did_explode, immediate_explode, mut left_val, right_val} =  b.check_explode(depth + 1);
		    //println!("did_explode from b = {:?}", did_explode);		    
		    if left_val != 0{
			a.add_num_from_right(left_val);
			left_val = 0; 	// we consumed the left val, so we dont propogate it upwards
		    }
		    if immediate_explode {
			// if a exploded, the new a is just a regular(0)
			*b = Box::new(SnailFish::Regular(0));
		    }
		    return ExplosionReturn {did_explode: did_explode, immediate_explode: false, left_val: left_val, right_val: right_val};
		}
	    }
	}
    }

    fn check_split(&mut self) -> (bool, bool) {
	//println!("check split for {:?}", self);
	
	match self {
	    SnailFish::Regular(val) => {return (*val > 9, *val > 9);},
	    SnailFish::Pair{a, b} => {
		let (did_split_any, did_split_immediate) = a.check_split();
		//println!("did_split_immediate from a = {:?}", did_split_immediate);
		if did_split_immediate {
		    let val = a.get_numeric_val().unwrap();
		    let left = (val as f64 / 2.0).floor() as u32;
		    let right = (val as f64 / 2.0).ceil() as u32;		    
		    *a = Box::new(SnailFish::Pair{
			a: Box::new(SnailFish::Regular(left)),
			b: Box::new(SnailFish::Regular(right))});
		}
		if did_split_any {
		    return (true, false);		    
		}
		
		let (did_split_any, did_split_immediate) = b.check_split();		
		//println!("did_split_immediate from b = {:?}", did_split_immediate);
		if did_split_immediate {
		    let val = b.get_numeric_val().unwrap();
		    let left = (val as f64 / 2.0).floor() as u32;
		    let right = (val as f64 / 2.0).ceil() as u32;		    
		    *b = Box::new(
			SnailFish::Pair{
			    a: Box::new(SnailFish::Regular(left)),
			    b: Box::new(SnailFish::Regular(right)) }
		    );
		}
		return (did_split_any, false);
		
	    }
	}
    }
    
}

#[test]
fn test_add() {
    let x = SnailFish::Regular(4);
    let y = SnailFish::Regular(5);
    assert_eq!(SnailFish::add(x,y), construct_snailfish("[4,5]").unwrap());

    let x = construct_snailfish("[4,[7,[2,3]]]").unwrap();
    let y = construct_snailfish("[2,[5,5]]").unwrap();
    assert_eq!(SnailFish::add(x,y), construct_snailfish("[[4,[7,[2,3]]],[2,[5,5]]]").unwrap());
}
    
fn construct_snailfish(input: &str) -> Option<SnailFish> {
    // first find the comma that splits the pair
    // this will occur when there is a comma found when we have exaclty one open brace
    //println!("inside construct_sailfish: {:?}", input);
    let re = Regex::new(r"^([\d]+)$").unwrap();    
    if let Some(caps) = re.captures(&input) {
	let num = caps[1].parse::<u32>().unwrap();
	return Some(SnailFish::Regular(num));
    }
    
	
    let mut open_count = 0;
    for (i, c) in input.chars().enumerate() {
	if c == '[' {
	    open_count += 1;
	} else if c == ']'{
	    open_count -= 1;
	} else if c == ',' && open_count == 1 {
	    // this is the pair split
	    // check if either side of the split is a regular number, or if we need to recurse deeper
	    return Some(SnailFish::Pair {
		a: Box::new(construct_snailfish(&input[1..i]).unwrap()),
		b: Box::new(construct_snailfish(&input[i+1..input.len()-1]).unwrap()) }
	    );
	}
    }
    None
}

// read in the file and return a TargetArea
fn read_file() -> Vec<SnailFish> {
    let buffered = get_buffered_reader("18");
    let mut all_fish = Vec::new();
    for line in buffered.lines().flatten() {
	// each line represents a snailfish number
	all_fish.push(construct_snailfish(&line).unwrap());
    }
    all_fish
}


fn run1() {
    let all_fish = read_file();
    //println!("all fish = {:?}", all_fish);
    let mut acc = None;
    for fish in all_fish.into_iter() {
	let mut current = match acc.is_none() {
	    true => {
		acc = Some(fish);
		continue;
	    }, 
	    false => acc.unwrap(),
	};
	
	println!("\nacc before:");
	current.pretty_print();
	
	current = SnailFish::add(current, fish);
	
	println!("after addition:");
	current.pretty_print();
	
	current.full_reduction();
	println!("after reduction:");
	current.pretty_print();
	acc = Some(current);
    }

    println!("very end:");
    let mut t = acc.unwrap();
    t.full_reduction();
    t.pretty_print();
    println!("final magnitude = {:?}", t.magnitude());
    /*
    let final_result = all_fish
	.iter()
	.inspect(|acc| println!("acc = {:?}", acc))	
	.reduce(|acc, next|
		&SnailFish::add(*acc, *next)
		.full_reduction());
     */
}

fn run2() {
    let mut max_mag = 0;
    let all_fish1 = read_file();
    //println!("all fish = {:?}", all_fish1);
    let mut all_fish2 = read_file();
    //println!("all fish2 = {:?}", all_fish2);
    for (i, fish1_outer) in  all_fish1.iter().enumerate() {
	println!("i = {}", i);
	for j in i+1..all_fish2.len() {
	    let fish1 = fish1_outer.clone();		    
	    let fish2 = all_fish2[j].clone();	    
	    println!("comparing one way");
	    let mut current = SnailFish::add(fish1, fish2);	
	    current.full_reduction();
	    let mag = current.magnitude();
	    max_mag = u32::max(max_mag, mag);


	    let fish1 = fish1_outer.clone();		    
	    let fish2 = all_fish2[j].clone();	    
	    println!("comparing one way");
	    let mut current = SnailFish::add(fish2, fish1);	
	    current.full_reduction();
	    let mag = current.magnitude();
	    max_mag = u32::max(max_mag, mag);
	    
	}
    }
    println!("max mag = {}", max_mag);
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
