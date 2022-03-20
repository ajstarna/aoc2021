/*
Note: this first attempt did not work. I basically tried to brute force it (even using threading, which helped, but not enough).
See 24-2.rs for the next attempt.
*/

use std::{env};
use std::collections::HashMap;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;
use rayon::prelude::*;

fn contains_zero(input: u64, input_len: u32) -> bool {
    let mut sig_digit = input_len;
    while sig_digit > 0 {
	sig_digit -= 1;	
	let current_digit = (input / u64::pow(10, sig_digit)) % 10;
	//println!("current digit = {}", current_digit);
	if current_digit == 0 {
	    return true;
	}

    }
    false
}

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


// to keep track what is in a register at any given time
#[derive(Debug, Copy, Clone)]
enum RegType {
    Input,
    Value(i128),
}




//// given a vec of operations, determine which are redundant, and return a list of reduced operations that actually affect the final outcome
fn reduce_ops(operations: Vec<Operation>) -> Vec<Operation> {
    let mut reduced = Vec::new();
    let mut registers = HashMap::<char, RegType>::new(); // store the values of the variable
    registers.insert('w', RegType::Value(0));    
    registers.insert('x', RegType::Value(0));    
    registers.insert('y', RegType::Value(0));    
    registers.insert('z', RegType::Value(0));    
    
    for operation in operations {
	//println!("operation: {:?}", operation);
	if let RegValue::Register(lhs_register) = operation.lhs {
	    let lhs_value = registers.get(&lhs_register).unwrap();
	    match operation.op_type {
		OperationType::Inp => {
		    registers.insert(lhs_register, RegType::Input); // we know this register is storing input
		}
		OperationType::Add  => {
		    //println!("add");
		    let rhs_value = match operation.rhs.unwrap() {
			RegValue::Register(rhs_register) => *registers.get(&rhs_register).unwrap(),
			RegValue::Value(rhs_value) => RegType::Value(rhs_value),
		    };
		    match rhs_value {
			RegType::Value(0) => {
			    println!("no point adding 0");
			    continue;
			},
			RegType::Value(right_num) => {
			    if let RegType::Value(left_num) = lhs_value {
				registers.insert(lhs_register, RegType::Value(left_num + right_num));
			    } // else already input dependent
			},
			RegType::Input => {registers.insert(lhs_register, RegType::Input);},
		    }
		},
		OperationType::Mul  => {
		    if let RegType::Value(0) = lhs_value {
			println!("no point muling if already 0");
			continue;
		    }
		    let rhs_value = match operation.rhs.unwrap() {
			RegValue::Register(rhs_register) => *registers.get(&rhs_register).unwrap(),
			RegValue::Value(rhs_value) => RegType::Value(rhs_value),			
		    };
		    match rhs_value {
			RegType::Value(1) => {
			    println!("no point muling by 1");
			    continue;
			},
			RegType::Value(right_num) => {
			    if let RegType::Value(left_num) = lhs_value {
				registers.insert(lhs_register, RegType::Value(left_num * right_num));
			    } // else already input dependent
			},
			RegType::Input => {registers.insert(lhs_register, RegType::Input);},			
		    }
		},
		OperationType::Div  => {
		    if let RegType::Value(0) = lhs_value {
			println!("no point divin if already 0");
			continue;
		    }
		    let rhs_value = match operation.rhs.unwrap() {
			RegValue::Register(rhs_register) => *registers.get(&rhs_register).unwrap(),
			RegValue::Value(rhs_value) => RegType::Value(rhs_value),						
		    };
		    match rhs_value {
			RegType::Value(1) => {
			    println!("no point diving by 1");
			    continue;
			},
			RegType::Value(right_num) => {
			    if let RegType::Value(left_num) = lhs_value {
				registers.insert(lhs_register, RegType::Value(left_num / right_num));
			    } // else already input dependent
			},
			RegType::Input => {registers.insert(lhs_register, RegType::Input);},						
		    }
		},
		OperationType::Mod  => {			
		    //println!("mod");
		    let rhs_value = match operation.rhs.unwrap() {
			RegValue::Register(rhs_register) => *registers.get(&rhs_register).unwrap(),
			RegValue::Value(rhs_value) => RegType::Value(rhs_value),									
		    };
		    match rhs_value {
			RegType::Value(right_num) => {
			    if let RegType::Value(left_num) = lhs_value {
				if *left_num < right_num {
				    println!("no point modding if less than");
				    continue;
				}
				registers.insert(lhs_register, RegType::Value(left_num / right_num));
			    } // else already input dependent
			},
			RegType::Input => {registers.insert(lhs_register, RegType::Input);},									
		    }
		},
		OperationType::Eql  => {			    
		    //println!("eql");
		    let rhs_value = match operation.rhs.unwrap() {
			RegValue::Register(rhs_register) => *registers.get(&rhs_register).unwrap(),
			RegValue::Value(rhs_value) => RegType::Value(rhs_value),												
		    };

		    match rhs_value {
			RegType::Value(right_num) => {
			    if let RegType::Value(left_num) = lhs_value {
				let store = match *left_num == right_num {
				    true => 1,
				    false => 0,
				};
				registers.insert(lhs_register, RegType::Value(store));
			    } // else already input dependent
			},
			RegType::Input => {registers.insert(lhs_register, RegType::Input);},												
		    }
		},
	    }
	}
	reduced.push(operation);	
    }
    reduced
}

/// return a vector holding a chunk (vector of operations) for each input section
/// the input is broken into operation chunks that only depend on the current input and the current z-register value (x and y get wiped)
//TODO read this as chunks. then use the cache that saves the result for the chunk calculation on a given input and z value
fn read_lines() -> Vec<Operation> {
    let mut operations = Vec::new();
    let inp_re = Regex::new(r"^(\w+) (\w)$").unwrap();
    let op_re = Regex::new(r"^(\w+) (\w) ([-\w]+)$").unwrap();
    let buffered = get_buffered_reader("24");    
    for line in buffered.lines().flatten() {
	//println!("{}", line);
	if let Some(caps) = inp_re.captures(&line) {
	    let register = caps[2].chars().next().unwrap();
	    let operation = Operation {op_type: OperationType::Inp, lhs: RegValue::Register(register), rhs: None};
	    operations.push(operation);
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
	    operations.push(operation);	    
	} else {
	    panic!("weird line: {:?}", line);
	}
    }
    operations
}


// input is the number that we read starting from the most significant digit, any time we see and "inp X" instruction.
// input_len tells us where to start reading from
// e.g. input == 120, input_len== 3 tells us to first shift right twice, then once, etc, as we see inp
// returns if z == 1 at the end (is the model number valid)
fn run_with_input(input: u64, input_len: u32, operations: &Vec<Operation>) -> bool {
    if contains_zero(input, input_len) {
	// not valid
	//println!("contains zero!");
	return false;
    }
    let mut registers = HashMap::<char, i128>::new(); // store the values of the variable
    registers.insert('w', 0);    
    registers.insert('x', 0);
    registers.insert('y', 0);
    registers.insert('z', 0);

    let mut sig_digit = input_len;
    
    for operation in operations {
	//println!("operation: {:?}", operation);
	if let RegValue::Register(lhs_register) = operation.lhs {
	    let lhs_value = *registers.get(&lhs_register).unwrap();
	    match operation.op_type {
		OperationType::Inp => {
		    sig_digit -= 1;	    
		    //let RegValue::Register(register) = operation.lhs;
		    let current_digit = (input / u64::pow(10, sig_digit)) % 10;
		    //println!("inp({}) = {}, registers = {:?}", sig_digit, current_digit, registers);
		    registers.insert(lhs_register, current_digit.into());
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
    //println!("registers: {:?}", registers);
    *registers.get(&'z').unwrap() == 0
}


fn run() {
    let operations = read_lines();
    println!("there are {} raw operations to perform!", operations.len());
    //let operations = reduce_ops(operations);
    //println!("there are {} reduced operations to perform!", operations.len());    
    let input_len = 14;
    //let mut input = 13749986531295;


    // GOT THIS FAR *******************
    //input 99999987240000 is valid: false


    /*
    To run 100_000 as a single digit input
        real	0m21.015s
	user	0m18.850s
	sys	0m0.453s
     */
    
    let starting_input = 99999999999999;
    let ending_input = 99999999900000;
    let num_threads: u64 = 4;
    let remainders: Vec<u64> = (0..num_threads).collect();
    let maxes: Vec<Option<u64>> = remainders.par_iter()
        .map(|&x| run_for_remainder(starting_input, ending_input, 14, &operations, num_threads, x))
	.collect();
    println!("maxes = {:?}", maxes);
}


fn run_for_remainder(starting_input: u64, ending_input: u64, input_len: u32, operations: &Vec<Operation>, num_threads: u64, remainder: u64 ) -> Option<u64> {
    let mut input = starting_input;
    let mut num_run = 0;
    while input >= ending_input {
	if input % num_threads != remainder {
	    input -= 1;	    
	    continue;
	}
	num_run += 1;
	//let is_valid = run_with_input(input, input_len, &operations);
	let is_valid = true;
	if is_valid {
	    return Some(input);
	}
	input -= 1
    }
    println!("i ran {}", num_run);
    return None;
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
