use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,    
}

impl Beacon {
    fn distance(beacon1: Beacon, beacon2: Beacon) -> f64 {
	((i32::pow(beacon1.x - beacon2.x, 2) + i32::pow(beacon1.y - beacon2.y, 2) + i32::pow(beacon1.z - beacon2.z, 2)) as f64).sqrt()
    }
}

#[derive(Debug, Clone)]
struct Reading {
    beacons: Vec<Beacon>,
    distances: Vec<f64>,
}

impl Reading {
    fn new() -> Self {
	Self {
	    beacons: Vec::<Beacon>::new(),
	    distances: Vec::<f64>::new(),
	}
    }
    
    fn push(&mut self, beacon: Beacon) {
	self.beacons.push(beacon);
    }

    /// for each pair of beacons in the reading, calculate the distance between them
    fn calculate_distances(&mut self) {
	for (i, b1) in self.beacons.iter().enumerate() {
	    for b2 in self.beacons[i+1..].iter() {
		let distance = Beacon::distance(*b1, *b2);
		self.distances.push(distance);
	    }
	}
    }
}

// read in the file and return a vec of readings, where the index is the scanner num from the file
fn read_file() -> Vec<Reading> {
    let buffered = get_buffered_reader("19-small");
    let re = Regex::new(r"([-\d]+),([-\d]+),([-\d]+)").unwrap();
    
    let mut all_readings = Vec::new();
    let mut current_reading = Reading::new();
    
    for line in buffered.lines().flatten() {
	if line.is_empty() {
	    all_readings.push(current_reading);
	    current_reading = Reading::new();
	    continue;
	}
	if let Some(caps) = re.captures(&line) {
	    let beacon = Beacon {
		x: caps[1].parse::<i32>().unwrap(),
		y: caps[2].parse::<i32>().unwrap(),
		z: caps[3].parse::<i32>().unwrap(),		
	    };
	    current_reading.push(beacon);
	}
    }
    all_readings.push(current_reading);    
    all_readings
}

fn compare_readings(reading1: &Reading, reading2: &Reading) -> usize {
    reading1.distances.iter().filter(|x| reading2.distances.contains(x)).count()
}
	

fn run1() {
    let mut all_readings = read_file();
    for reading in all_readings.iter_mut() {
	reading.calculate_distances();
    }
    for (i, reading1) in all_readings.iter().enumerate() {
	println!("reading1 = {:?}", reading1);
	for reading2 in all_readings[i+1..].iter() {
	    println!("reading2 = {:?}", reading2);
	    compare_readings(reading1, reading2);
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
