use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};


fn check_win(board: & Vec<i32>) -> bool {
    let mut row_start = 0;
    while row_start < board.len() {
	if board[row_start..row_start+5].iter().all(|x| *x == -1) {
	    println!("row win starting at {:?}", row_start);
	    return true;
	}
	row_start += 5
    }
    for col_start in 0..5 {
	if board[col_start..board.len()].iter().step_by(5).all(|x| *x == -1) {
	    println!("col win starting at {:?}", col_start);
	    return true;
	}
	row_start += 5
    }
    
    false
}

fn part1() {
    let buffered = get_buffered_reader(4);
    let mut numbers_opt: Option<Vec<i32>> = None; // we only assing this once when i == 0. but have to make it mutable i guess compiler isn't smart enough?
    let mut board_opt: Option<Vec<i32>> = None; // holds the current board as we read it in
    let mut scores: Vec<Option<i32>> = Vec::new(); // keep track of the score for each board when it won
    let mut winning_indices: Vec<Option<usize>> = Vec::new(); // keep track on which turn each board won
    for (i, line) in buffered.lines().enumerate() {
	if let Ok(line) = line {
	    //println!("line = '{:?}'", line);
	    if i == 0 {
		let numbers = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
		println!("numbers = {:?}", numbers);
		numbers_opt = Some(numbers);
		continue;
	    }
	    if line == "" {
		// TODO: analyze the current board and fill its score and winning index
		if let Some(ref mut board) = board_opt {
		    println!("before {:?}", board);
		    let mut winner_flag = false;
		    for (count, num) in numbers_opt.as_ref().unwrap().iter().enumerate() {
			//println!("num = {}", num);
			for i in 0..board.len() {
			    //println!("board[i] = {}", board[i]);			    
			    if board[i] == *num {
				board[i] = -1; // indicate it was a winner
			    }
			}
			if check_win(board) {
			    winner_flag = true;
			    let remaining_sum: i32 = board.iter().filter(|x| **x != -1).sum();
			    let score = remaining_sum * num;
			    scores.push(Some(score));
			    winning_indices.push(Some(count));
			    break;
			}
		    }
		    if !winner_flag {
			scores.push(None);
			winning_indices.push(None);
		    }
		    println!("after {:?}", board);		    
		}
		// start a new board
		board_opt = Some(Vec::new());
		
	    } else {
		// read in part of the current board
		let mut split: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
		if let Some(ref mut board) = board_opt {
		    // should always be a board available in this case
		    board.append(&mut split);
		}
	    }
	}
    }
    println!("all scores = {:?}", scores);
    println!("all winning indicies = {:?}", winning_indices);
    let mut best_amount = f64::INFINITY;
    let mut best_board = None;
    for (i, val) in winning_indices.iter().enumerate() {
	if let Some(num) = val {
	    if (*num as f64) < best_amount {
		best_board = Some(i);
		best_amount = *num as f64;
		println!("won after {:?} numbers", *num);
	    }
	}
    }
    println!("the best board is {:?} with a score of {:?}, and needing {:?} numbers to be called", best_board, scores[best_board.unwrap()], best_amount);
}

fn part2() {
    let buffered = get_buffered_reader(2);
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;    
    for line in buffered.lines() {
	if let Ok(line) = line {
	    let split: Vec<&str> = line.split_whitespace().collect();
	    let val = split[1].parse::<i32>().expect("unable to parse val as an i32");	    
	    // println!("{:?}", split);
	    match split[0] {
		"forward" => {
		    horizontal += val;
		    depth += aim * val;
		},
		"up" => aim -= val,
		"down" => aim += val,
		_ => println!("huh?"),
	    }
	}
    }
    println!("horizontal = {}", horizontal);
    println!("depth = {}", depth);
    println!("multiplied = {}", depth * horizontal);        
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
