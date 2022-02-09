use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};


fn read_nums() -> Vec<i32> {
    let buffered = get_buffered_reader("7");
    let mut nums: Vec<i32> = Vec::new();
    for line in buffered.lines().flatten() {
	let split = line.split(',').map(|x| x.parse::<i32>().unwrap());
	for num in split {
	    nums.push(num);
	}
    }
    nums
}

fn part1() {
    let mut nums = read_nums();
    nums.sort_unstable();
    println!("nums = {:?}", nums);
    let mid_idx = nums.len() / 2;
    let mid = nums[mid_idx];
    println!("mid idx = {:?}, which is value = {:?}", mid_idx, mid);
    let fuel: i32 = nums.iter().map(|&x| (x - mid).abs()).sum();
    println!("fuel = {:?}", fuel);    
}


/// given a postion that we want to move all the numbers to, we determine how much fuel that will take
fn determine_fuel_part_2(position: i32, nums: &Vec<i32>) -> f64{
    let dist: Vec<i32> = nums.iter().map(|&x| (x - position).abs()).collect(); //map(|x| (x + 1) * (x / 2)).sum();
    // println!("dist = {:?}", dist);        
    let fuel: f64 = dist.iter().map(|&x| (x as f64 + 1.) * (x as f64 / 2.)).sum();
    println!("position = {:?}", position);        
    println!("fuel = {:?}", fuel);
    fuel
    
}

fn part2() {
    let mut nums = read_nums();
    nums.sort_unstable();
    println!("nums = {:?}", nums);
    let mean: f64 = (nums.iter().sum::<i32>() as f64 ) / nums.len() as f64;
    let mean = mean.round() as i32;
    println!("mean = {:?}", mean);
    let mut lowest_fuel = f64::INFINITY;
    let mut best_position = None;

    // this isn't very satisfying, but the mean isn't quite the right answer (at least not always?)
    // so I am searching in a neighbourhood around the mean.
    // My test input gave a solution of mean -1.
    // I don't have a proof that mean - 1 is the answer or an actual solution, but I am moving on for now
    for position in (mean-10)..(mean+10) {
	let fuel = determine_fuel_part_2(position, &nums);
	if fuel < lowest_fuel {
	    lowest_fuel = fuel;
	    best_position = Some(position);
	}
    }
    println!("the best position {:?} gives us fuel of {:?}", best_position, lowest_fuel);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);    
    let part = &args[1];
    println!("part = {}", part);
    match part.as_str() {
	"1" => part1(),
	"2" => part2(),
	_ => panic!("invalid part number argument!"),
    }
}
