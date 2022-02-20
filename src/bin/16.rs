use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};

use bitvec::prelude::*;

fn read_file() -> Option<Vec<u8>> {
    let buffered = get_buffered_reader("16");
    let bytes = match buffered.lines().flatten().next() { 
	Some(line) => {
	    println!("line = {:?}", line);
	    Some((0..line.len())
		 .step_by(2)
		 .map(|i| u8::from_str_radix(&line[i..i + 2], 16).expect("unable to parse as hex!"))
		 .collect())
	},
	None => None
    };
    bytes
}

/// given a vec of u8s, we calculate the decimal number if we were to concat the bytes together as binary numbers
/// ```
/// let chunks = ![7, 14, 5];
/// assert_eq!(concat_binary_chunks(chunks), 2220);
/// ```
fn concat_binary_chunks(chunks: Vec<u8>) -> u64 {

    let mut chunk_score = 0;
    let mut idx = 0;
    for (chunk_num, chunk) in chunks.iter().rev().enumerate() {
	println!("chunk_num = {}, chunk = {}", chunk_num, chunk);
	for i in 0..4 {
	    let bit = (chunk >> i) & 0b1;
	    println!("i = {}, idx = {}, bit = {}", i, idx,
		     bit as u64);
	    println!("adding {}", u64::pow(2, idx  as u32) * bit as u64);
	    chunk_score += u64::pow(2, idx as u32) * bit as u64;
	    idx += 1
	}




    }
    println!("final chunk score = {}", chunk_score);
    chunk_score
}

/// given a bit vector and a start index for a packet, we return the value of this packet (based on subpackets and type ID)
/// also returns an index for the next bit not included in this packet
fn process_packet(bits: &BitVec::<Msb0, u8>, start: usize) -> (u64, usize) {
    println!("\n\nprocess packet from start = {}", start);
    let mut value = 0;
    let version = bits[start..start+3].load_be::<u8>();
    println!("version = {}", &version);    
    let type_id = bits[start+3..start+6].load_be::<u8>();
    println!("type id {}", &type_id);
    let mut idx = start+6;    
    if type_id == 4 {
	println!("****** LITERAL *****");
	let mut chunks = Vec::new();
	loop {
	    let chunk = bits[idx+1..idx+5].load_be::<u8>();
	    chunks.push(chunk);
	    if !bits[idx] {
		// foud the last chunk since it started with a 0
		break;
	    }
	    idx += 5;
	}
	idx += 5; // increment once more to account for the last chunk
	println!("chunks = {:?}", chunks);
	// todo: if we need, concat the chunks into a binary number

	value += concat_binary_chunks(chunks);
	
    } else {
	// this is an operator type
	println!("operator");
	let mut sub_values = Vec::new();
	if bits[start+6] {
	    // number of sub-packets to follow

	    let num_sub = bits[start+7..start+18].load_be::<u16>();
	    println!("num sub packets = {}", num_sub);
	    idx = start + 18;
	    for i in 0..num_sub {
		let (sub_value, new_idx) = process_packet(&bits, idx);
		println!("numbered sub count {}, from idx {} = {}", i, idx, sub_value);
		sub_values.push(sub_value);
		idx = new_idx;
	    }

	} else {
	    // total length of bits
	    //println!("total length of sub bits from {} to {}", start+7, start+22);	    
	    let length = bits[start+7..start+22].load_be::<usize>();
	    idx = start + 22;
	    let end = start + 22 + length;
	    println!("length of sub packets {}, ending at {}", length, end);

	    let mut count = 0;
	    loop {
		let (sub_value, new_idx) = process_packet(&bits, idx);
		println!("length sub count {}, from idx {} = {}", count, idx, sub_value);
		count += 1;
		sub_values.push(sub_value);		
		idx = new_idx;
		if idx >= end {
		    println!("{} >= {} so we break", idx, end);
		    break;
		} else {
		    println!("{} < {} so we keep going", idx, end
		    );		    
		}
	    }
	}

	value += match type_id {
	    0 => sub_values.iter().sum::<u64>(),
	    1 => sub_values.iter().product::<u64>(),
	    2 => *sub_values.iter().min().unwrap(),
	    3 => *sub_values.iter().max().unwrap(),
	    5 => {
		if sub_values[0] > sub_values[1] {
		    1
		} else {
		    0
		}
	    }
	    6 => {
		if sub_values[0] < sub_values[1] {
		    1
		} else {
		    0
		}		
	    }
	    7 => {
		if sub_values[0] == sub_values[1] {
		    1
		} else {
		    0
		}
	    }
	    _ => {println!("UNKNOWN TYPE ID {}", type_id); 0},
	}
    }
    (value, idx)
}

fn run() {
    let bytes = read_file();
    println!("bytes = {:?}", bytes);
    if bytes.is_none() {
	println!("Unable to parse the file into bytes!");
	std::process::exit(-1);
    } else {
	let bits = BitVec::<Msb0, u8>::from_slice(&bytes.unwrap()).expect("could not convert bytes into bitvec");
	dbg!(&bits);
	let (value, idx) = process_packet(&bits, 0);
	println!("value of packet = {}, with the subsequent index = {}", value, idx);
    }
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
