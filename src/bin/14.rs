use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::{HashSet, HashMap};

/// read the input file and return the template (as a Vec of chars), and return the
/// Vec of insertion rules (where CH -> B is represented as (C, H, B) tuple of chars
fn read_file() -> (Vec<char>, Vec<(char, char, char)>) {
    let buffered = get_buffered_reader("14");
    let mut template = Vec::new();
    let mut rules =  Vec::new();
    for line in buffered.lines().flatten() {
	if template.is_empty() {
	    // read in the template from the first line of the file
	    for c in line.chars() {
		template.push(c);
	    }
	    continue;
	}
	
	if line.is_empty() {
	    continue;
	}
	let parts = line.split(" -> ").collect::<Vec<&str>>();
	let chars = parts[0].chars().collect::<Vec<char>>();
	rules.push( (chars[0], chars[1], parts[1].chars().next().unwrap()) );
    }
    (template, rules)
}

/// given a mutable template and some rules, we iterate over all the rules and apply them to the template
/// The rules take effect at the "same" time.
fn process_step(template: &mut Vec<char>, rules: &Vec<(char, char, char)>) {
    let mut insertions = Vec::new();
    for i in 0..template.len() - 1 {
	for rule in rules {
	    let (a, b, c) = rule;
	    if template[i] == *a && template[i+1] == *b {
		// we found a spot where we want to insert a new letter
		insertions.push((*c, i+1)); // the inserted char will now live at indec i+1
	    }
	}
    }
    insertions.sort_by(|a, b| a.1.cmp(&b.1)); // sort by the index that we will insert
    //println!("insertions = {:?}", insertions);
    // now do the insertions in order. As we apply them, the indices of the following insertions goes up
    let mut count = 0;
    for insertion in insertions {
	let (letter, mut idx) = insertion;
	idx += count;
	template.insert(idx, letter);
	count += 1;
    }
}

fn run(num_steps: u32) {
    let (mut template, rules) = read_file();
    println!("template = {:?}", template);
    println!("rules = {:?}", rules);
    for i in 0..num_steps {
	process_step(&mut template, &rules);
	println!("after step {}: {:?}", i+1, template);
	let mut current_counts = HashMap::new();
	for ch in &template {
	    let counter = current_counts.entry(ch).or_insert(0);
	    *counter += 1;	    
	}
	println!("current_counts = {:?}", current_counts);
	let max = current_counts.iter().max_by(|a, b| a.1.cmp(b.1)).map(|x| x.1).unwrap();
	let min = current_counts.iter().min_by(|a, b| a.1.cmp(b.1)).map(|x| x.1).unwrap();
	dbg!(max - min);
	println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);    
    let part = &args[1];
    println!("part = {}", part);
    let num_steps = &args[2].parse::<u32>().unwrap();    
    match part.as_str() {
	"1" => run(*num_steps),
	"2" => run(*num_steps),
	_ => panic!("invalid part number argument!"),
    }
}
