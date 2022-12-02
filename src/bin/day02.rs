use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let reader = BufReader::new(File::open("input/day02.txt").unwrap());

    let (score1, score2) = reader.lines().map(|line| {
        let line = line.unwrap();
        let opponent: Choice = line.chars().next().unwrap().into();
        let player: Choice = line.chars().last().unwrap().into();
        let desired: Outcome = line.chars().last().unwrap().into();

        let score1 = player.score() + player.wins(opponent).score();
        let score2 = opponent.play_for_outcome(desired).score() + desired.score();
        (score1, score2)
    }).reduce(|(s1,s2), (t1,t2)| (s1 + t1, s2 + t2)).unwrap();

    println!("The predicted total score is {score1}");
    println!("The predicted total score with the correct strategy is {score2}");
}

#[derive(Copy, Clone)]
enum Choice { Rock, Paper, Scissors }

#[derive(Copy, Clone)]
enum Outcome { Win, Lose, Draw }

impl From<char> for Choice {
    fn from(c: char) -> Choice {
        match c {
            'A'|'X' => Choice::Rock,
            'B'|'Y' => Choice::Paper,
            'C'|'Z' => Choice::Scissors,
            _ => panic!("Unexpected choice '{}'", c)
        }
    }
}

impl From<char> for Outcome {
    fn from(c: char) -> Outcome {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Unexpected outcome '{}'", c)
        }
    }
}

impl Choice {
    pub fn score(self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    pub fn wins(self, other: Choice) -> Outcome {
        match (self, other) {
            (Choice::Rock,Choice::Rock)|(Choice::Paper,Choice::Paper)|(Choice::Scissors,Choice::Scissors) => Outcome::Draw,
            (Choice::Rock,Choice::Paper)|(Choice::Paper,Choice::Scissors)|(Choice::Scissors,Choice::Rock) => Outcome::Lose,
            (Choice::Rock,Choice::Scissors)|(Choice::Paper,Choice::Rock)|(Choice::Scissors,Choice::Paper) => Outcome::Win,
        }
    }

    pub fn play_for_outcome(self, outcome: Outcome) -> Choice {
        match (self, outcome) {
            (Choice::Rock,Outcome::Lose)|(Choice::Paper,Outcome::Win)|(Choice::Scissors,Outcome::Draw) => Choice::Scissors,
            (Choice::Rock,Outcome::Win)|(Choice::Paper,Outcome::Draw)|(Choice::Scissors,Outcome::Lose) => Choice::Paper,
            (Choice::Rock,Outcome::Draw)|(Choice::Paper,Outcome::Lose)|(Choice::Scissors,Outcome::Win) => Choice::Rock,
        }
    }
}

impl Outcome {
    pub fn score(self) -> i32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}
