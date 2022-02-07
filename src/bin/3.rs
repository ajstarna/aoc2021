use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::HashSet;

fn part1() {
    let buffered = get_buffered_reader("3");
    let num_bits = 12;
    let mut sums = vec![0; num_bits]; // each value represents the sum for a given bit across all rows
    let mut num_lines = 0;
    for line in buffered.lines().flatten() {
	num_lines += 1;
	for (i, c) in line.chars().enumerate() {
	    match c {
		'0' => (),
		'1' => sums[i] += 1,
		_ => println!("invalid bit"),
	    }
	}
    }
    println!("sums = {:?}", sums);
    println!("num_lines = {:?}", num_lines);    
    
    let mut gamma = 0;
    let mut epsilon = 0;
    // the sum can tell us which bit was more common,
    // and the index in sums can be used as the binary value represented by that bit
    for (i, sum) in sums.iter().enumerate() {
	if *sum > num_lines / 2 {
	    // 1 was the most common bit
	    gamma += u64::pow(2, (num_bits - 1 - i) as u32);
	    // epsilon will bw zero in this bit, so nothing to add to total value
	} else {
	    // 0 was the most common bit
	    epsilon += u64::pow(2, (num_bits - 1 -i) as u32);
	    // gamma will bw zero in this bit, so nothing to add to total value	    
	}
    }

    println!("gamma = {}", gamma);
    println!("epsilon = {}", epsilon);
    println!("power = {}", epsilon * gamma);        
}

fn part2() {
    let buffered = get_buffered_reader("3");
    let num_bits = 12;
    // let mut sums = vec![0; num_bits]; // each value represents the sum for a given bit across all rows
    let mut num_lines = 0;

    // for each bit 0 to 12, we have a hashset, which stores the line indices
    // for lines with a 1 in their bit at that bit
    let mut ones_for_bit: Vec<HashSet<usize>> = Vec::new();
    for _ in 0..num_bits {
	ones_for_bit.push(HashSet::new());
    }
    let mut zeros_for_bit: Vec<HashSet<usize>> = Vec::new();
    for _ in 0..num_bits {
	zeros_for_bit.push(HashSet::new());
    }
    let mut numbers = Vec::new();
    for (line_idx, line) in buffered.lines().flatten().enumerate() {
	num_lines += 1;
	let num = i32::from_str_radix(&line, 2).expect("Not a binary number!");
	numbers.push(num);
	for (i, c) in line.chars().enumerate() {
	    match c {
		'0' => {
		    zeros_for_bit[i].insert(line_idx);
		},
		'1' => {
		    //sums[i] += 1;
		    ones_for_bit[i].insert(line_idx);
		},
		_ => println!("invalid bit"),
	    }
	}
    }
    
    // println!("sums = {:?}", sums);
    println!("num_lines = {}", num_lines);
    // now we know the most common based on sums, lets iterate
    // once more
    let mut oxygen_lines = HashSet::new();
    let mut c02_lines = HashSet::new();
    let mut oxygen_done = false;
    let mut c02_done = false;    
    //for (i, sum) in sums.iter().enumerate() {
    for i in 0..num_bits {
	println!("bit = {}", i);
	println!("oxygen_lines = {:?}", oxygen_lines);
	println!("c02_lines = {:?}", c02_lines);

	let one_lines = &ones_for_bit[i];
	let zero_lines = &zeros_for_bit[i];		
	
	if oxygen_lines.is_empty() {
	    // this is the first assignment
	    if one_lines.len() >= zero_lines.len() {
		oxygen_lines = one_lines.clone();		
		c02_lines = zero_lines.clone();		
	    } else {
		oxygen_lines = zero_lines.clone();		
		c02_lines = one_lines.clone();		
	    }
	    continue;
	}

	if ! oxygen_done {

	    let oxygen_ones: HashSet<usize> = oxygen_lines.iter().filter(|e| one_lines.contains(e)).copied().collect();
	    let oxygen_zeros: HashSet<usize> = oxygen_lines.iter().filter(|e| zero_lines.contains(e)).copied().collect();
	    if oxygen_ones.len() >= oxygen_zeros.len() {
		oxygen_lines = oxygen_ones;
	    } else {
		oxygen_lines = oxygen_zeros;
	    }
	    if oxygen_lines.len() == 1 {
		oxygen_done = true;
	    }
	}

	if !c02_done {
	    //let c02_ones: HashSet<usize> = c02_lines.iter().filter(|e| one_lines.contains(e)).map(|e| *e).collect();
	    //let c02_zeros: HashSet<usize> = c02_lines.iter().filter(|e| zero_lines.contains(e)).map(|e| *e).collect();
	    let c02_ones: HashSet<usize> = c02_lines.iter().filter(|e| one_lines.contains(e)).copied().collect();
	    let c02_zeros: HashSet<usize> = c02_lines.iter().filter(|e| zero_lines.contains(e)).copied().collect();
	    if c02_ones.len() >= c02_zeros.len() {
		c02_lines = c02_zeros;
	    } else {
		c02_lines = c02_ones;
	    }
	    if c02_lines.len() == 1 {
		c02_done = true;
	    }
	}
	
	if oxygen_done && c02_done {
	    break;
	}
    }

    println!("oxygen_lines = {:?}", oxygen_lines);
    println!("c02_lines = {:?}", c02_lines);
    
    assert_eq!(oxygen_lines.len(), 1);
    assert_eq!(c02_lines.len(), 1);    
    let oxygen_rating = numbers[oxygen_lines.into_iter().next().unwrap()];
    let c02_rating = numbers[c02_lines.into_iter().next().unwrap()];
    println!("oxygen_rating = {:?}", oxygen_rating);
    println!("c02_rating = {:?}", c02_rating);
    println!("life support = {:?}", oxygen_rating * c02_rating);    
     
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
