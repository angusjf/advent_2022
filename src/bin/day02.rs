use std::str::FromStr;
use Move::*;
use Outcome::*;

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn points(self: &Self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn points(self: &Self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn defeats(self: &Self) -> Self {
        match self {
            Rock => Self::Scissors,
            Paper => Self::Rock,
            Scissors => Self::Paper,
        }
    }

    fn against(self: &Self, opponent: &Self) -> Outcome {
        if self.defeats() == *opponent {
            Win
        } else if opponent.defeats() == *self {
            Lose
        } else {
            Draw
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(()),
        }
    }
}

fn one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let their_move = line[0..1].parse().unwrap();
            let our_move: Move = line[2..3].parse().unwrap();
            our_move.points() + our_move.against(&their_move).points()
        })
        .sum()
}

fn two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let their_move: Move = line[0..1].parse().unwrap();
            let result = line[2..3].parse().unwrap();
            let our_move = match result {
                Win => their_move.defeats().defeats(),
                Lose => their_move.defeats(),
                Draw => their_move.clone(),
            };
            our_move.points() + result.points()
        })
        .sum()
}

fn main() {
    println!("{}", one(include_str!("input02.txt")));
    println!("{}", two(include_str!("input02.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test02.txt")), 15);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test02.txt")), 12);
    }
}
