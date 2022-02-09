use std::{env};
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};

const NEW_FISH_TIMER_START: u32 = 8;
const FISH_TIMER_RESET: u32 = 6;

#[derive(Debug)]
struct Fish {
    timer: u32
}

impl Fish {
    fn new() -> Self {
	Fish{ timer: NEW_FISH_TIMER_START }
    }

    /// updates the internal timer by decreasing it by 1
    /// if it was already at 0, then the timer resets and we return a bool indicating that
    /// we reset, and as such, a new fish should be spawned.
    /// If we do not currently reset, then return false to indicate no fish should be spawned.
    fn run_day(&mut self) -> bool {
	match self.timer {
	    0 => {
		self.timer = FISH_TIMER_RESET;
		true
	    },
	    _ => {
		self.timer -= 1;
		false
	    }
	}
    }
}

/// this solution will not work for part 2 because of the exponential growth, and this is brute force
/// was taking a long time to even get to 200 days, let alone 256, so i dont think tractible
fn run_part_1(total_days: u32) {
    let buffered = get_buffered_reader("6");
    let mut all_fish = Vec::new(); // store all the lattern fish
    for line in buffered.lines().flatten() {
	let start_times: Vec<u32> = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
	for time in start_times {
	    all_fish.push(Fish {timer: time});
	}
    }
    // println!("all fish = {:?}", all_fish);

    for _day in 0..total_days {
	// we call run_day() on each fish and cout how many return true to see how many we should spawn
	let spawn_count = all_fish.iter_mut().map(|fish| fish.run_day()).filter(|&should_spawn| should_spawn).count();
	// note reserve_exact didn't seem to help with run time
	// all_fish.reserve_exact(spawn_count); // reserve the space up front to save redundant moves of memory
	for _ in 0..spawn_count {
	    all_fish.push(Fish::new());
	}
    }
    println!("after {} days, the number of fish = {}", total_days, all_fish.len());
}

/// given a current timer of a single fish and the amount of days to run for, this function
/// returns the total number of fish that this one spawns plus one (to include itself)
/// e.g. count_family(3, 1) == 1
/// e.g. count_family(3, 4) == 2
fn count_family(timer: u32, days: u32) -> u32 {

    if timer >= days {
	// not enough  days left to trigger an offspring, so
	// we only return the count our ourself
	1 
    } else {
	// days > timer
	// after (time + 1) days, there will now be two fish: ourself and a new offspring.
	// they will each have remaining days left to produce even more offspring
	let remaining_days = days - (timer + 1); // we add plus one because it takes a day to go from 0 back to FISH_TIMER_RESET
	//if remaining_days % 10 == 0 {
	//    println!("remaining days = {remaining_days}");
	//}
	count_family(FISH_TIMER_RESET, remaining_days) + count_family(NEW_FISH_TIMER_START, remaining_days)
    }
}

fn run_part_2(total_days: u32) {
    let buffered = get_buffered_reader("6");
    for line in buffered.lines().flatten() {
	let final_count: u32 = line.split(',').map(|x| x.parse::<u32>().unwrap()).map(|timer| count_family(timer, total_days)).sum();
	println!("after {} days, the number of fish = {}", total_days, final_count);
	break; //should only be one line
    }
}

fn run_part_3(total_days: u32) {
    let buffered = get_buffered_reader("6");
    let mut fish_counts: Vec<u64> = vec![0; 9]; // each index represents the count of fish with their respective timer at that amount
    for line in buffered.lines().flatten() {
	let start_times: Vec<usize> = line.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
	for time in start_times {
	    fish_counts[time] += 1;
	}
	break; //should only be one line
    }

    for _day in 0..total_days {
	//println!("fish counts day {} = {:?}", day, fish_counts);	
	// on each day, the number of fish with their timer at index i is what used to be the number of fish with their timer at index i+1
	// exceptions: index 8 are the new spawns on this day, i.e. the same as the amount that was at index 0 the day before
	//             AND index 6 gets the same that were at index 0 the day before due to resetting of timer (in addition to previous index 7)
	let tmp = fish_counts[0];
	for i in 0..8 {
	    fish_counts[i] = fish_counts[i+1];
	}
	fish_counts[8] = tmp;
	fish_counts[6] += tmp;
    }

    println!("after {} days, the number of fish = {}", total_days, fish_counts.iter().sum::<u64>());
}


fn main() {
    // NOTE: for this one, there are still only 2 actual parts, but im naming each iteration of my solution
    //
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);    
    let part = &args[1];
    println!("part = {}", part);
    let total_days = args[2].parse::<u32>().unwrap();
    println!("total_days = {}", total_days);
    match part.as_str() {
	"1" => run_part_1(total_days),
	"2" => run_part_2(total_days),
	"3" => run_part_3(total_days),	
	_ => panic!("invalid part number argument!"),
    }
}
