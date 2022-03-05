use std::env;
use std::collections::HashMap;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;


/// a square grid in
struct Grid3d {
    pub vals: Vec<bool>, // all the data
    pub size: usize, // when the next row begins in the data. size^2 would be when the next sheet in the third dimension starts
}

impl Grid3d {
    /// return a new grid with all the same given value values for a given length and width
    pub fn new(size: usize, value: bool) -> Self {
	let vals = vec![value; usize::pow(size, 3)];
	Self {vals, size}
    }

    /// for each step, turn all the corresponding values to the step.status
    fn run_steps(&mut self, steps: &Vec<Step>) {
	for step in steps {
	    println!("\nstep = {:?}", step);
	    for z in step.z.0..=step.z.1 {
		let z_step = z * usize::pow(self.size, 2);
		for y in step.y.0..=step.y.1 {
		    let y_step = y * self.size;
		    for x in step.x.0..=step.x.1 {
			let idx = z_step + y_step + x;
			//println!("for {}, {}, {}, idx = {}", x-50, y-50, z-50, idx);
			self.vals[idx] = step.status;
		    }
		}
	    }
	}
    }
    /// returns how many of our values are true/on at the moment
    fn count_on(&self) -> usize {
	self.vals.iter().filter(|&&x| x).count()
    }
}

/// holds the steps as read in from the file
/// each field is a tuple that represents the range of cubes in that dimension to set
/// and status is on or off as a bool
#[derive(Debug)]
struct Step {
    status: bool,
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),    
}

impl Step {

    /// given ranges for the 3 dimensions, first we check that they are all within bounds at least partially
    /// then we return a Step with these ranges, shifted into actual indices. i.e. we need the indices to be in 0..=100,
    /// so we shift everything up by 50
    fn try_new(status: bool, x_start: i32, x_end: i32, y_start: i32, y_end: i32, z_start: i32, z_end: i32) -> Option<Self> {
	let shift = 50;
	if x_start + shift > 100 || x_end + shift < 0 ||
	    y_start + shift > 100 || y_end + shift < 0 ||
	    z_start + shift > 100 || z_end + shift < 0 {
		return None;
	    }
	
	Some (
	    Step {
		status: status,
		x: ((x_start + shift) as usize, (x_end + shift) as usize),
		y: ((y_start + shift) as usize, (y_end + shift) as usize),
		z: ((z_start + shift) as usize, (z_end + shift) as usize),		
	    }
	)
    }
}


// read the steps in line by line
fn read_file() -> Vec<Step> {
    let buffered = get_buffered_reader("22-small");
    // e.g. on x=-20..26,y=-36..17,z=-47..7
    let re = Regex::new(r"(\w+) x=([-\d]+)..([-\d]+),y=([-\d]+)..([-\d]+),z=([-\d]+)..([-\d]+)").unwrap();
    let mut steps = Vec::new();
    
    for line in buffered.lines().flatten() {
	if let Some(caps) = re.captures(&line) {
	    let status = match &caps[1] {
		"on" => true,
		"off" => false,
		_ => panic!("invalid status on line: {}", line),
	    };
	    let x_start = caps[2].parse::<i32>().unwrap();
	    let x_end = caps[3].parse::<i32>().unwrap();
	    let y_start = caps[4].parse::<i32>().unwrap();
	    let y_end = caps[5].parse::<i32>().unwrap();
	    let z_start = caps[6].parse::<i32>().unwrap();
	    let z_end = caps[7].parse::<i32>().unwrap();

	    if let Some(step) = Step::try_new(status, x_start, x_end, y_start, y_end, z_start, z_end) {
		steps.push(step);		
	    } else {
		println!("range fully out of area in some dimension: {}", line);
	    }
	} else {
	    panic!("cannot parse line: {}", line);
	}
    }    
    steps
}

fn run1( ) {
    // the grid goes from -50 to 50, i.e. 101 possible indices in each dimension
    let mut grid = Grid3d::new(101, false);
    let steps = read_file();
    println!("steps =\n{:?}", steps);
    grid.run_steps(&steps);
    println!("after the steps, there are {} cubes turned on", grid.count_on());
}



//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////// Full refactor below for part 2 ///////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


/// an "off" cuboid as Zero volume in itself,
/// an "on" cuboid will have Positive volume,
/// and an "intersection" cuboid will either have Positive or Negative, depending if it came from a Negative or Positive, respectively
/// A negative-volume cube represents that we are "double counting" this volume, so we need to also subtract volume to even out at the end,
/// example, if two positive (on) cubes intersect, we can sum up their individual volumes, then subtract the double-counted intersection,
/// This process applies back and forth with more and more intersections, see: https://en.wikipedia.org/wiki/Inclusion%E2%80%93exclusion_principle
/// Note: I had to use reddit for help, where I got the hint about the inclusion/exclusion principle
#[derive(Debug)]
enum VolumeType {
    Positive,
    Negative,
    Zero,
}

#[derive(Debug)]
struct Cuboid {
    //status: bool,
    volume_type: VolumeType,    
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
    volume: i64,

}

impl Cuboid {
    fn new(volume_type: VolumeType, x_start: i64, x_end: i64, y_start: i64, y_end: i64, z_start: i64, z_end: i64) -> Self {
	let volume = match volume_type {
	    // we add 1 to the volume calculations, since there is always at least 1 volume (10..12 is 3 cubes, not exactly like the Rust range)
	    VolumeType::Positive => (1 + x_end - x_start) * (1+ y_end - y_start) * ( 1+ z_end - z_start),
	    VolumeType::Negative => -((1 + x_end - x_start) * (1 + y_end - y_start) * (1 + z_end - z_start)),
	    VolumeType::Zero => 0,
	};
	Self {
	    volume_type: volume_type,
	    x: (x_start, x_end),
	    y: (y_start, y_end),
	    z: (z_start, z_end),
	    volume: volume
	}
    }


    /// given another cuboid, this method finds the intersecting Cuboid and returns it (or None)
    fn find_intersection(&self, other: &Cuboid) -> Option<Self> {
	let Cuboid { volume_type, x,  y, z, .. } = other;
	let new_x_start = std::cmp::max(self.x.0, x.0);
	let new_x_end = std::cmp::min(self.x.1, x.1);
	if new_x_start > new_x_end {
	    // Note: we can allow the "equal to" case, since the cube itself has 1 volume (this was a bug I had at first with >= here)
	    //println!("no intersection in the x dimension");
	    return None;
	}
	let new_y_start = std::cmp::max(self.y.0, y.0);
	let new_y_end = std::cmp::min(self.y.1, y.1);
	if new_y_start > new_y_end {
	    //println!("no intersection in the y dimension");
	    return None;
	}
	let new_z_start = std::cmp::max(self.z.0, z.0);
	let new_z_end = std::cmp::min(self.z.1, z.1);
	if new_z_start > new_z_end {
	    //println!("no intersection in the z dimension");
	    return None;
	}

	// if we made it this far, we have some intersection in all dimensions
	// the volume type just swaps postive and negative, there should be no zero volume
	let new_volume_type = match volume_type {
	    VolumeType::Positive => VolumeType::Negative, 
	    VolumeType::Negative => VolumeType::Positive, 
	    VolumeType::Zero => {eprintln!("you should not be finding an intersection with a zero-volume cuboid!"); VolumeType::Zero},
	};
	Some(Cuboid::new(new_volume_type, new_x_start, new_x_end, new_y_start, new_y_end, new_z_start, new_z_end))
    }
}
// read the steps in line by line as cuboids
fn read_file2() -> Vec<Cuboid> { //(Vec<Cuboid>, Vec<Cuboid>) {
    let buffered = get_buffered_reader("22-small");
    // e.g. on x=-20..26,y=-36..17,z=-47..7
    let re = Regex::new(r"(\w+) x=([-\d]+)..([-\d]+),y=([-\d]+)..([-\d]+),z=([-\d]+)..([-\d]+)").unwrap();
    let mut cuboids = Vec::new();
    
    for line in buffered.lines().flatten() {
	if let Some(caps) = re.captures(&line) {
	    let volume_type = match &caps[1] {
		"on" => VolumeType::Positive,
		"off" => VolumeType::Zero,
		_ => panic!("invalid status on line: {}", line),
	    };
	    let x_start = caps[2].parse::<i64>().unwrap();
	    let x_end = caps[3].parse::<i64>().unwrap();
	    let y_start = caps[4].parse::<i64>().unwrap();
	    let y_end = caps[5].parse::<i64>().unwrap();
	    let z_start = caps[6].parse::<i64>().unwrap();
	    let z_end = caps[7].parse::<i64>().unwrap();

	    let cuboid = Cuboid::new(volume_type, x_start, x_end, y_start, y_end, z_start, z_end) ;
	    cuboids.push(cuboid);

	} else {
	    panic!("cannot parse line: {}", line);
	}
    }    
    //(on_cuboids, off_cuboids)
    cuboids
}

fn run2( ) {
    let cuboids = read_file2();
    let mut final_cuboids = Vec::new(); // final cuboids will hold all the read cuboids, and also all the "intersection" cuboids that get created
    println!("cuboids = {:?}", cuboids);
    for input_cuboid in cuboids {
	// for each input cuboid, compare with each final cuboid to add any needed intersection
	println!("\n\ninput cuboid = {:?}", input_cuboid);
	let mut new_intersections = Vec::new();
	for f_cuboid in &final_cuboids {
	    if let Some(intersection) = input_cuboid.find_intersection(f_cuboid) {
		new_intersections.push(intersection);
	    }
	}

	// append the new cuboid and any created intersections
	if let VolumeType::Zero = input_cuboid.volume_type {
	    // the "off" cuboids dont have a volume at the end, and any new cuboid we look at won't really interact, since off is sorta just the null state
	    println!("do not push a zero type (i.e. 'off' step) cuboid to final vec");
	} else {
	    final_cuboids.push(input_cuboid);
	}
	//println!("new_intersections = {:?}", new_intersections);
	final_cuboids.extend(new_intersections);
    }
    //println!("final cuboids = {:?}", final_cuboids);
    // we simply sum the volumes of the final_cuboids to see how many cubes are turned on
    let final_on: i64 = final_cuboids.iter().map(|x| x.volume).sum();
    println!("final on cubes = {:?}", final_on);
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
