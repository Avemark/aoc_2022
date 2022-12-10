use std::io::{self, BufRead};

fn main() {
    let guide = strategy_guide();
    println!("Number of games: {}", guide.len());
    // for game in &guide {
    //     println!("score: {}", score_game(game));
    // }
    println!("Total: {}", score(&guide));
}

fn score(games: &Vec<Game>) -> i16 {
    games.iter().fold(0, |acc, game| {
        acc + score_game(game)
    })
}

fn score_game(game: &Game) -> i16 {
    let opener = new_move(game.opener);

    match game.response {
        'X' => opener.loss() as i16,
        'Y' => opener.draw() as i16,
        'Z' => opener.win() as i16,
        _ => 0,
    }
}

fn new_move(opener: char) -> Box<dyn RealMove> {
    match opener {
        'A' => Box::new(Rock {}),
        'B' => Box::new(Paper {}),
        'C' => Box::new(Scissors {}),
        _ => Box::new(Rock {}),
    }
}

fn strategy_guide() -> Vec<Game> {
    let mut games: Vec<Game> = vec![];
    for in_line in io::stdin().lock().lines() {
        match in_line {
            Ok(line) => {
                let mut chars = line.chars();
                let opener = chars.next().unwrap();
                chars.next();
                let response = chars.next().unwrap();
                games.push(
                    Game { opener, response, }
                )
            },
            Err(_) => {},
        }
    };
    return games
}

struct Game {
    opener: char,
    response: char,
}

struct Paper {}

impl Move for Paper {
    fn paper(&self) -> i8 { DRAW + PAPER }
    fn rock(&self) -> i8 { LOSS + ROCK }
    fn scissors(&self) -> i8 { WIN + SCISSORS }
}

struct Rock {}

impl Move for Rock {
    fn paper(&self) -> i8 { WIN + PAPER }
    fn rock(&self) -> i8 { DRAW + ROCK }
    fn scissors(&self) -> i8 { LOSS + SCISSORS }
}

struct Scissors {}

impl Move for Scissors {
    fn paper(&self) -> i8 { LOSS + PAPER }
    fn rock(&self) -> i8 { WIN + ROCK }
    fn scissors(&self) -> i8 { DRAW + SCISSORS }
}

const ROCK: i8 = 1;
const PAPER: i8 = 2;
const SCISSORS: i8 = 3;

const WIN: i8 = 6;
const DRAW: i8 = 3;
const LOSS: i8 = 0;

trait Move {
    fn paper(&self) -> i8;
    fn rock(&self) -> i8;
    fn scissors(&self) -> i8;

}

trait RealMove: Move {
    fn win(&self) -> i8;
    fn draw(&self) -> i8;
    fn loss(&self) -> i8;
}

impl RealMove for Rock{
    fn win(&self) -> i8 {
        self.paper()
    }
    fn loss(&self) -> i8 {
        self.scissors()
    }
    fn draw(&self) -> i8 {
        self.rock()
    }
}

impl RealMove for Paper {
  fn draw(&self) -> i8 {
      self.paper()
  }
  fn win(&self) -> i8 {
      self.scissors()
  }
  fn loss(&self) -> i8 {
      self.rock()
  }
}

impl RealMove for Scissors {
    fn draw(&self) -> i8 {
        self.scissors()
    }
    fn win(&self) -> i8 {
        self.rock()
    }
    fn loss(&self) -> i8 {
        self.paper()
    }
}
