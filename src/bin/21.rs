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

#[derive(Debug, Clone)]
struct PlayerDirac {
    position: u32, // the position is between 0 and 9 (to make the modulo easier), even though in the puzzle, they represent 1..=10
    scores: HashMap<u32, u32>, a mapping from score total, to number of universes with that score presently
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
    let buffered = get_buffered_reader("21-small");
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
#[derive(Hash)]
struct Game {
    p0: Player,
    p1: Player,
}

fn run2( ) {
    let mut players = read_file();
    println!("players = {:?}", players);

    let mut all_games = HashMap::new(); // counts how many games with an identical state there are
    let starting_game = Game {players[0], players[1]};
    all_games.insert(starting_game, 0);
    loop {
	let new_games = HashMap::new();
	if new_games.len() == 0 {
	    // no new games were spawned, i.e. all the current games are done
	    break;
	}
    }
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
