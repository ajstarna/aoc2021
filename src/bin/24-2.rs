/*
This is a refactor of day 24.rs. Used reddit to find the hint that we are actually searching through a space where the states can
be represented by the index into the number and the value of the z register. At each input, the other registers get wiped anyways.
Then we are searching on the possibly input values at each indices. 
e.g. if we know at index 12 and z value NUM that ANY future inputs for indices 13 and 14 lead to an invalid state, then next time
we end up at the state (12, NUM), we can early stop, since we know we have been here before and there is no solution.
*/
use std::{env};
use std::collections::HashMap;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;
//use rayon::prelude::*;


#[derive(Debug, Copy, Clone)]
enum OperationType {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Copy, Clone)]
enum RegValue {
    Register(char),
    Value(i128),
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    op_type: OperationType,
    lhs: RegValue,
    rhs: Option<RegValue>, // if type == Inp, rhs in None
    //is_read: bool, // keeps track if this operation is consumed later on
}

/// return a vector holding a chunk (vector of operations) for each input section
/// the input is broken into operation chunks that only depend on the current input and the current z-register value (x and y get wiped)
fn read_lines() -> Vec<Vec<Operation>> {
    let inp_re = Regex::new(r"^(\w+) (\w)$").unwrap();
    let op_re = Regex::new(r"^(\w+) (\w) ([-\w]+)$").unwrap();
    let buffered = get_buffered_reader("24");

    let mut all_chunks = vec![];    
    let mut current_chunk = vec![]; // holds the instructions for a single chunk, as delimited by an input instruction
    for line in buffered.lines().flatten() {
	//println!("{}", line);
	if let Some(caps) = inp_re.captures(&line) {
	    if current_chunk.len() > 0 {
		// we already had a chunk going (i.e. this isnt the very first one)
		all_chunks.push(current_chunk);
		current_chunk = vec![]; // reset the chunk
	    }
	    let register = caps[2].chars().next().unwrap();
	    let operation = Operation {op_type: OperationType::Inp, lhs: RegValue::Register(register), rhs: None};
	    current_chunk.push(operation);	    
	} else if let Some(caps) = op_re.captures(&line) {
	    let rhs_value = match caps[3].parse::<i128>() {
		Ok(value) => RegValue::Value(value),
		Err(_) => {
		    // if we can't parse thr right hand side as a number, it means it
		    // must be referring to a register
		    let rhs_register = caps[3].chars().next().unwrap();
		    RegValue::Register(rhs_register)
		},
	    };
	    let lhs_register = caps[2].chars().next().unwrap();
	    let lhs_value = RegValue::Register(lhs_register);
	    let operation = match &caps[1] {
		"add" => {
		    Operation {op_type: OperationType::Add, lhs: lhs_value, rhs: Some(rhs_value)}
		},
		"mul" => {
		    Operation {op_type: OperationType::Mul, lhs: lhs_value, rhs: Some(rhs_value)}		    
		},
		"div" => {
		    Operation {op_type: OperationType::Div, lhs: lhs_value, rhs: Some(rhs_value)}		    
		},
		"mod" => {
		    Operation {op_type: OperationType::Mod, lhs: lhs_value, rhs: Some(rhs_value)}		    
		},
		"eql" => {
		    Operation {op_type: OperationType::Eql, lhs: lhs_value, rhs: Some(rhs_value)}		    
		},
		_ => {eprintln!("unknown operation! {:?}", &caps[1]); panic!(); },		
	    };
	    current_chunk.push(operation);	    
	} else {
	    panic!("weird line: {:?}", line);
	}
    }
    // there is one last chunk to push
    assert!(current_chunk.len() > 0);
    all_chunks.push(current_chunk);    
    all_chunks
}


// model_num is the number that we read starting from the most significant digit, any time we see an "inp X" instruction.
// all_chunks is the vec of all the individual chunks. index tells us which chunk we are operating, and which number in model_num to treat as input
// z_value is the current value of the z register (the only one that comes from the previous digig)
// returns the max model num that leads to z==1 at the end
// if part2, then we simply search from  1 to 9 instead of 9 to 1
fn find_max(best_model_num: &mut[u8; 14], model_num: [u8; 14], all_chunks: &Vec<Vec<Operation>>,
	    index: usize, input_z_value: i128, cache: &mut HashMap<(usize, i128), bool>, part2: bool) -> bool {
    //println!("model num: {:?} entering at index {}, for z-value = {}", model_num, index, input_z_value);    
    if let Some(valid_flag) = cache.get(&(index, input_z_value)) {
	//println!("found in cache at index {}, for z-value = {}: {}", index, input_z_value, valid_flag);
	return *valid_flag;
    }
    
    let mut registers = HashMap::<char, i128>::new(); // store the values of the variable
    // since we know the input always wipes the other registers, we can just set them to 0 to start
    registers.insert('w', 0);    
    registers.insert('x', 0);
    registers.insert('y', 0);
    registers.insert('z', input_z_value);

    let current_value = model_num[index];
    
    let current_chunk = &all_chunks[index];
    for operation in current_chunk {
	//println!("operation: {:?}", operation);
	if let RegValue::Register(lhs_register) = operation.lhs {
	    let lhs_value = *registers.get(&lhs_register).unwrap();
	    match operation.op_type {
		OperationType::Inp => {
		    registers.insert(lhs_register, current_value.into());
		},
		OperationType::Add  => {
		    //println!("add");
		    let rhs_value = match operation.rhs.unwrap() {
			RegValue::Register(rhs_register) => *registers.get(&rhs_register).unwrap(),
			RegValue::Value(rhs_value) => rhs_value,
		    };
		    registers.insert(lhs_register, lhs_value + rhs_value);		
		},
		OperationType::Mul  => {		
		    //println!("mul");
		    let rhs_value = match operation.rhs.unwrap() {
			RegValue::Register(rhs_register) => *registers.get(&rhs_register).unwrap(),
			RegValue::Value(rhs_value) => rhs_value,
		    };
		    registers.insert(lhs_register, lhs_value * rhs_value);				    
		},
		OperationType::Div  => {		    
		    //println!("div");
		    let rhs_value = match operation.rhs.unwrap() {
			RegValue::Register(rhs_register) => *registers.get(&rhs_register).unwrap(),
			RegValue::Value(rhs_value) => rhs_value,
		    };
		    registers.insert(lhs_register, lhs_value / rhs_value);				    
		},
		OperationType::Mod  => {			
		    //println!("mod");
		    let rhs_value = match operation.rhs.unwrap() {
			RegValue::Register(rhs_register) => *registers.get(&rhs_register).unwrap(),
			RegValue::Value(rhs_value) => rhs_value,
		    };
		    registers.insert(lhs_register, lhs_value % rhs_value);				    
		},
		OperationType::Eql  => {			    
		    //println!("eql");
		    let rhs_value = match operation.rhs.unwrap() {
			RegValue::Register(rhs_register) => *registers.get(&rhs_register).unwrap(),
			RegValue::Value(rhs_value) => rhs_value,
		    };

		    let store = match lhs_value == rhs_value {
			true => 1,
			false => 0,
		    };
		    registers.insert(lhs_register, store);
		},
	    }
	}
	//println!("registers: {:?}", registers);	
    }

    // we are done processing the current chunk
    //println!("registers: {:?}", registers);
    let next_z_value = *registers.get(&'z').unwrap();
    //println!("next z value = {:?}", next_z_value);
    if index == 13 {
	// we are at the far right part of the input, so simply return the verfict	    
	let is_valid = next_z_value == 0;// model_num[1] ==  && model_num[4] == 2;
	if is_valid {
	    *best_model_num = model_num;
	    println!("found best_model_num = {:?}", best_model_num);
	}
	return is_valid
	//return 
    } else {
	let mut is_valid = false;
	let mut next_model_num = model_num; // copy
	let next_index = index + 1;
	let num_iter: Vec<u8> = match part2 {
	    true => (1..=9).rev().collect(),
	    false => (1..=9).collect(),
	};
	for next_val in num_iter {
	    //println!("next val = {}", next_val);
	    next_model_num[next_index] = next_val;
	    //println!("next model num  = {:?}", next_model_num);
	    //println!("next index  = {:?}", next_index);	    
	    //best_model_num: &mut[u8; 14], model_num: [u8; 14], all_chunks: &Vec<Vec<Operation>>, index: usize, input_z_value: i128, cache: &mut HashMap<(usize, i128), bool>) -> bool {	    
	    if find_max(best_model_num, next_model_num, &all_chunks, next_index, next_z_value, cache, part2) {
		is_valid = true;
		break;
		//std::exit();
	    }
	}
	//println!("we now know that at index {}, for z-value = {}: {}", index+1, next_z_value, is_valid);	
	cache.insert((index+1, next_z_value), is_valid);
	is_valid	
    }
}


fn run(part2: bool) {
    let all_chunks = read_lines();
    //println!("all chunks = \n{:?}", all_chunks);
    println!("there are {:?} chunks", all_chunks.len());
    let mut model_num = [1; 14]; // this is actually arbitrary, since we end up setting the next number explicitley ech time
    let mut best_model_num = [0; 14];    
    let mut cache = HashMap::new();
    let index = 0;
    let z_value = 0;
    let num_iter: Vec<u8> = match part2 {
	true => (1..=9).rev().collect(),
	false => (1..=9).collect(),
    };
    
    for next_val in num_iter {
	model_num[index] = next_val;
	println!("model num = {:?}", model_num);	    	    	
	if find_max(&mut best_model_num, model_num, &all_chunks, index, z_value, &mut cache, part2) {
	    println!("found best_model_num = {:?}", best_model_num);
	    break;
	}
    }
    //println!("valid = {:?}", valid);
    println!("best model num = {:?}", best_model_num);
    for val in best_model_num {
	print!("{}", val);
    }
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);    
    let part = &args[1];
    println!("part = {}", part);
    match part.as_str() {
	"1" => run(false),
	"2" => run(true),
	_ => panic!("invalid part number argument!"),
    }
}
