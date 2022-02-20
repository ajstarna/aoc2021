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

/// given a bit vector and a start index for a packet, we return the sum of all version numbers from the packet and subpacket headers
/// also returns an index for the next bit not included in this packet
fn process_packet(bits: &BitVec::<Msb0, u8>, start: usize) -> (u32, usize) {
    println!("\n\nprocess packet from start = {}", start);
    let mut version_sum = 0;
    let version = bits[start..start+3].load_be::<u8>();
    println!("version = {}", &version);    
    version_sum += version as u32;
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
    } else {
	// this is an operator type
	println!("operator");
	if bits[start+6] {
	    // number of sub-packets to follow

	    let num_sub = bits[start+7..start+18].load_be::<u16>();
	    println!("num sub packets = {}", num_sub);
	    idx = start + 18;
	    for i in 0..num_sub {
		let (sub_count, new_idx) = process_packet(&bits, idx);
		println!("numbered sub count {}, from idx {} = {}", i, idx, sub_count);
		version_sum += sub_count;
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
		let (sub_count, new_idx) = process_packet(&bits, idx);
		println!("length sub count {}, from idx {} = {}", count, idx, sub_count);
		count += 1;
		version_sum += sub_count;
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
    }
    //println!("returning = {}, {}", version_sum, idx);    
    (version_sum, idx)
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
	let (version_sum, idx) = process_packet(&bits, 0);
	println!("sum of all header versions = {}, with the subsequent index = {}", version_sum, idx);
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
