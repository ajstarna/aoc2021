use std::env;
use aoc2021::utils::{get_buffered_reader};
use std::io::{BufRead};
use regex::Regex;
//use rand::Rng;

#[derive(Debug, Clone)]
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

fn run( ) {
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
