use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;

#[derive(Debug)]
struct TargetArea {
    x1: i64,
    x2: i64,    
    y1: i64,
    y2: i64,    
}

#[derive(Debug)]
struct Probe {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
    max_y: i64,
}


impl Probe {
    fn new(x: i64, y: i64, vx: i64, vy: i64) -> Self {
	Self{x, y, vx, vy, max_y: y}
    }
    
    fn is_within_target(&self, target: &TargetArea) -> bool {
	self.x >= target.x1 && self.x <= target.x2 && self.y >= target.y1 && self.y <= target.y2 
    }

    // take a step in time based on velocity and physic rules
    fn take_step(&mut self) {
	self.x += self.vx;
	self.y += self.vy;
	self.max_y = std::cmp::max(self.y, self.max_y);
	if self.vx > 0 {
	    self.vx -= 1;
	} else if self.vx < 0 {
	    self.vx += 1;	    
	}
	self.vy -= 1;
    }
}

// read in the file and return a TargetArea
fn read_file() -> TargetArea {
    let buffered = get_buffered_reader("17");
    let line = buffered.lines().flatten().next().unwrap();
    let re = Regex::new(r"target area: x=([-\d]+)..([-\d]+), y=([-\d]+)..([-\d]+)").unwrap();
    if let Some(caps) = re.captures(&line) {	
	TargetArea {
	    x1: caps[1].parse::<i64>().unwrap(),
	    x2: caps[2].parse::<i64>().unwrap(),
	    y1: caps[3].parse::<i64>().unwrap(),
	    y2: caps[4].parse::<i64>().unwrap(),
	}
    } else {
	println!("invalid input!");
	std::process::exit(-1);
    }
}


/// a fairly brute for solution with some optimization breaks
/// there might be a nice way to solve how  many possible steps we need to allow, but just setting it high seems sufficient
fn run() {
    let target = read_file();
    dbg!(&target);

    let mut successes = Vec::new();
    // the x velocity cannot be more than the x2 boundary, or else we would necessarily overshoot the target after one step
    for vx in 1..target.x2+1 {
	for vy in target.y1..300 {
	    // the vy cannot be more negative than target.y1 or else we will be under right away
	    //println!("vy = {:?}", vy);
	    let mut probe = Probe::new(0, 0, vx, vy);
	    //println!("new probe: {:?}", probe);
	    for count in 0..500 {
		probe.take_step();
		if vx == 6 && vy == 0 {
		    println!("after {count} steps, probe = {:?}", probe);
		}
		if probe.is_within_target(&target) {
		    //println!("probed {:?} is within the target after {} steps", probe, count);
		    successes.push((vx, vy, probe.max_y));
		    break;
		}
		if probe.y < target.y1 {
		    // we are below the target so it is now hopeless
		    //println!("we are below the target so it is now hopeless");
		    break;
		}
		if probe.x > target.x2 {
		    // we are below the target so it is now hopeless
		    //println!("we are past the target so it is now hopeless");
		    break;
		}
		if (probe.x < target.x1 || probe.x > target.x2) && probe.vx == 0 {
		    //println!("no horizontal movement so we are doomed!");
		    break;
		    
		}
	    }
	}
    }
    println!("successes: {:?}", successes);
    println!("num successes: {:?}", successes.len());    
    let max_height = successes.iter().max_by(|a, b| a.2.cmp(&b.2));
    println!("settings for and max height = {:?}", max_height);
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
