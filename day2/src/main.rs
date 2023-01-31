use std::char;
use std::cmp::Ordering;
use std::env;
use std::fs;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Victory = 6,
}

#[derive(Debug)]

struct Round {
    player: Shape,
    elf: Shape,
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Shape::Rock    , Shape::Paper)     => Some(Ordering::Less),
            (Shape::Paper   , Shape::Scissors)  => Some(Ordering::Less),
            (Shape::Scissors, Shape::Rock)      => Some(Ordering::Less),
            (Shape::Rock    , Shape::Rock)      => Some(Ordering::Equal),
            (Shape::Paper   , Shape::Paper)     => Some(Ordering::Equal),
            (Shape::Scissors, Shape::Scissors)  => Some(Ordering::Equal),
            (Shape::Rock    , Shape::Scissors)  => Some(Ordering::Greater),
            (Shape::Paper   , Shape::Rock)      => Some(Ordering::Greater),
            (Shape::Scissors, Shape::Paper)     => Some(Ordering::Greater),
        }
    }
}

impl Shape {
    fn from(character: char) -> Option<Shape> {
        match character {
            'A' | 'X' => Some(Shape::Rock),
            'B' | 'Y' => Some(Shape::Paper),
            'C' | 'Z' => Some(Shape::Scissors),
            _ => None,
        }
    }

    fn loses_to(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn wins_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

impl Outcome {
    fn from(character: char) -> Option<Outcome> {
        match character {
            'X' => Some(Outcome::Loss),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Victory),
            _ => None,
        }
    }
}

impl Round {
    fn get_outcome(&self) -> Outcome {
        let shape_order = self.player.partial_cmp(&self.elf).unwrap();

        match shape_order {
            Ordering::Less => Outcome::Loss,
            Ordering::Equal => Outcome::Draw,
            Ordering::Greater => Outcome::Victory,
        }
    }

    fn get_score(&self) -> u32 {
        (self.player as u32) + (self.get_outcome() as u32)
    }

    fn from(player_character: char, elf_character: char) -> Option<Round> {
        let player = Shape::from(player_character)?;
        let elf = Shape::from(elf_character)?;

        Some(Round { player, elf })
    }
}

fn shape_outcome(opponent: Shape, outcome: Outcome) -> Shape {
    match (outcome, opponent) {
        (Outcome::Draw, _)    => opponent,
        (Outcome::Victory, _) => opponent.loses_to(),
        (Outcome::Loss,    _) => opponent.wins_against(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    
    let r1: u32 = contents
        .lines()
        .map(|line| {
            let chars: Vec<_> = line.chars().collect();

            let elf_char = chars[0];
            let player_char = chars[2];

            
            let round = Round::from(player_char, elf_char).unwrap();
            let score = round.get_score();
            
            score

        })
        .sum();
        
    
    let r2: u32 = contents
        .lines()
        .map(|line| {
            let chars: Vec<_> = line.chars().collect();

            let elf_char = chars[0];
            let outcome_char = chars[2];

            let elf_shape = Shape::from(elf_char).unwrap();
            let outcome = Outcome::from(outcome_char).unwrap();

            let player_shape = shape_outcome(elf_shape, outcome);
            
            Round{elf: elf_shape, player: player_shape}.get_score()

        })
        .sum();

    println!("{r2}");
}