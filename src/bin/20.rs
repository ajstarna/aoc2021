use std::env;
use aoc2021::utils::{Grid};
use std::collections::HashSet;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};


// read the image "algorithm" which is contained on one line from the input file
fn read_algo() -> Vec<u8> {
    let buffered = get_buffered_reader("20-small-algorithm");    
    let line = buffered.lines().next().unwrap().unwrap();
    let algo: Vec<u8> = line.chars()
	.map(|c| {
	    match c {
		'#' => 1,
		'.' => 0,
		_ => panic!("unknown character in algorithm file!"),
	    }
	})
	.collect();
    algo
}

fn run( ) {
    let grid = Grid::<u32>::new_from_file_day_20("20-small-image"); // not the most optimum, but storing as <u32> for now since already built
    grid.pretty_print();
    let algo = read_algo();
    println!("algo = {:?}", algo);
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
