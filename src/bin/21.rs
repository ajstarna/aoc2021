use std::env;
use std::collections::HashMap;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Player {
    position: u32, // the position is between 0 and 9 (to make the modulo easier), even though in the puzzle, they represent 1..=10
    score: u32,
}

impl Player {
    /// given an amount to move the player, we move them this many steps and update the position accordingly
    /// this also increments the score by the new position landed on
    fn advance(&mut self, movement: u32)  {
	self.position = (self.position + movement) % 10;
	self.score += self.position + 1;
    }
}

// read the players in from file with their starting positions and score set to 0, then return as a vec
fn read_file() -> Vec<Player> {
    let buffered = get_buffered_reader("21");
    let re = Regex::new(r"Player (\d+) starting position: (\d+)").unwrap();
    let mut players = Vec::new();
    
    for line in buffered.lines().flatten() {
	if let Some(caps) = re.captures(&line) {
	    let player = Player {
		position: caps[2].parse::<u32>().unwrap() - 1, // we sub 1 since our positions in the Player struct are 0..=9 instead of 1..=10
		score: 0,
	    };
	    players.push(player);
	} else {
	    panic!("cannot parse line: {}", line);
	}
    }    
	players
}

struct Die {
    num: u32,
    total_rolls: u32,
	
}

impl Die {
    fn new() -> Self {
	Die {num: 1, total_rolls: 0}	
    }
    
    fn roll(&mut self) -> u32 {
	self.total_rolls += 1;
	let result = self.num;
	self.num += 1;
	if self.num > 100 {
	    self.num = 1;
	}
	result
    }
}

fn run1( ) {
    let mut players = read_file();
    println!("players = {:?}", players);

    //let mut rng = rand::thread_rng();
    //rng.gen_range(0..10));
    let mut die = Die::new(); 
    //for player in players.iter_mut().cycle() {
    let mut index = 0;
    loop {
	let player = &mut players[index];
	let movement = (0..3) // three dice rolls
	    .map(|_| die.roll())
	    .sum();
	player.advance(movement);
	//println!("player = {:?}", player);
       if player.score >= 1000 {
           break;
       }	
	index = (index + 1) % players.len();
    }
    let min = players.iter()
	.map(|x| x.score)
	.min()
	.unwrap();
    println!("min score = {:?}, total_dice_rolls = {:?}", min, die.total_rolls);
    println!("min score * total dice rolls = {:?}", min *die.total_rolls);    
}

/// a game can simply be represented as 2 Player structs, each eith their position and score
/// As there become more an more games with the Dirac roll, more games will end up in smilar states, so we can count them together
#[derive(Debug, Eq, PartialEq, Hash)]
struct Game {
    p1: Player,
    p2: Player,
}

/// return all the possible total movements after 3 dirac dice rolls
fn roll_dirac_3_times() -> Vec<u32> {
    let mut all_results = Vec::new();
    for r1 in 1..=3 {
	for r2 in 1..=3 {
	    for r3 in 1..=3 {
		all_results.push(r1+r2+r3);
	    }
	}
    }
    all_results
}

fn run2( ) {
    const WINNING_SCORE: u32 = 21;
    let players = read_file();
    println!("players = {:?}", players);

    let mut all_games = HashMap::new(); // counts how many games with an identical state there are
    let starting_game = Game {p1: players[0], p2: players[1]};
    all_games.insert(starting_game, 1);

    let all_roll_results = roll_dirac_3_times(); // holds all the possible outcomes of the 3 rolls summed
    assert_eq!(all_roll_results.len(), 27);
    let mut loop_count = 0;
    let mut all_p1_wins = 0;
    let mut all_p2_wins = 0;    
    loop {
	loop_count += 1;
	println!("\n\n\nloop == {}", loop_count);
	let mut new_games = HashMap::<Game, u64>::new(); // the games that are spawn on this turn of dirac rolls
	for (game, count) in all_games {
	    // we are iterating over game states and their associated counts, i.e. how many universes have a game in this exact state right now
	    let Game{p1, p2} = game;

	    for roll_p1 in &all_roll_results {
		// each possible die roll on the 3 rolls does happen		
		let mut new_p1 = p1;
		new_p1.advance(*roll_p1);
		if new_p1.score >= WINNING_SCORE {
		    // p1 just won the game in this (count-many) universes, so add to the total and continue
		    all_p1_wins += count;
		    continue;
		}
		
		// for each outome for player 1, player 2 can also see every outcome
		for roll_p2 in &all_roll_results {
		    let mut new_p2 = p2;
		    new_p2.advance(*roll_p2);
		    if new_p2.score >= WINNING_SCORE {
			// p2 just won the game in this (count-many) universes, so add to the total and continue			
			all_p2_wins += count;
			continue;
		    }
		    
		    // there is a game in this new state based on p1 and p2's rolls
		    // add to the new_games mapping
		    let new_game = Game{p1: new_p1, p2: new_p2};
		    let state_count = new_games.entry(new_game).or_insert(0);
		    *state_count += count;
		}
	    }
	    
	}

	
	//println!("new games = {:?}", new_games);
	println!("new games len = {}", new_games.len());	
	println!("p1 wins = {}, p2 wins = {}", all_p1_wins, all_p2_wins);


	if new_games.len() == 0 {
	    // no new games were spawned, i.e. all the current games are done
	    println!("no more new games");
	    break;
	}
	
	all_games = new_games;
	if loop_count > 10000 {
	    println!("debug loop count break");
	    break; // for debug
	}
    }

    println!("the winning score is {}", std::cmp::max(all_p1_wins, all_p2_wins));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);    
    let part = &args[1];
    println!("part = {}", part);
    match part.as_str() {
	"1" => run1(),
	"2" => run2(),
	_ => panic!("invalid part number argument!"),
    }
}
