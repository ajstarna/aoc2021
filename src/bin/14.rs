use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use std::collections::{HashMap, HashSet};

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

/// read the input file and return the template (as a map of (char, char) to u64), and return the
/// map of insertion rules (where CH -> B is represented as (C, H) -> B
/// also return the last char of the template, since we will need this for counting
fn read_file_v2() -> (HashMap<(char, char), u64>, HashMap<(char, char), char>, char) {
    let buffered = get_buffered_reader("14");
    let mut template = HashMap::new();
    let mut rules =  HashMap::new();
    let mut last_char = None;
    for line in buffered.lines().flatten() {
	if template.is_empty() {
	    // read in the template from the first line of the file
	    let chars = line.chars().collect::<Vec<char>>();
	    last_char = Some(chars[chars.len()-1]);
	    for i in 0..chars.len()-1 {
		let pair = (chars[i], chars[i+1]);
		let counter = template.entry(pair).or_insert(0);
		*counter += 1;	    
	    }
	    continue;
	}
	if line.is_empty() {
	    continue;
	}
	let parts = line.split(" -> ").collect::<Vec<&str>>();
	let chars = parts[0].chars().collect::<Vec<char>>();
	rules.insert( (chars[0], chars[1]),  parts[1].chars().next().unwrap());
    }
    (template, rules, last_char.unwrap())
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


fn part1(num_steps: u64) {
    let (mut template, rules) = read_file();
    println!("template = {:?}", template);
    println!("rules = {:?}", rules);
    for i in 0..num_steps {
	process_step(&mut template, &rules);
	println!("after step {}: len of template {:?}", i+1, template.len());
	println!("{:?}", template);
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

/// given a mutable template and some rules, we iterate over all the rules and apply them to the template
/// The rules take effect at the "same" time.
fn process_step_v2(template: &mut HashMap<(char, char), u64>, rules: &HashMap<(char, char), char>) {
    let mut insertions = HashMap::new();
    let mut deletions = HashSet::new();    
    for ((a,b), count) in template.iter() {
	// does this pair need to have a rule applied to it?
	//println!("looking at {:?}", (a,b));
	if let Some(c) = rules.get(&(*a,*b)) {
	    //println!("pushhing {:?} {} time", (*a,*b,*c), count);
	    let left_counter = insertions.entry((*a,*c)).or_insert(0);
	    *left_counter += count;
	    let right_counter = insertions.entry((*c, *b)).or_insert(0);
	    *right_counter += count;
	    deletions.insert((*a,*b));
	}
    }

    println!("deletions = {:?}", deletions);
    for deleted_pair in deletions.iter() {
	template.remove(deleted_pair);
    }    
    println!("insertions = {:?}", insertions);
    for (inserted_pair, count) in insertions.iter() {
	let counter = template.entry(*inserted_pair).or_insert(0);
	*counter += count;
    }
}


fn part2(num_steps: u64) {
    let (mut template, rules, last_char) = read_file_v2();
    println!("template = {:?}", template);
    println!("rules = {:?}", rules);

    for i in 0..num_steps {
	process_step_v2(&mut template, &rules);

	println!("after step {}: {:?}", i+1, template);

	let mut current_counts = HashMap::new();
	current_counts.insert(&last_char, 1); // we always want to make sure to count this
	for ((a,_), count) in &template {
	    // only ount a character in the left side of a pair to avoid overcounting
	    let counter_a = current_counts.entry(a).or_insert(0);
	    *counter_a += count;	    
	    //let counter_b = current_counts.entry(b).or_insert(0);
	    //*counter_b += count;	    
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
    let num_steps = &args[2].parse::<u64>().unwrap();    
    match part.as_str() {
	"1" => part1(*num_steps),
	"2" => part2(*num_steps),
	_ => panic!("invalid part number argument!"),
    }
}
