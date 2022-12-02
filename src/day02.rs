enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    fn points(self: &Self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unimplemented!(),
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
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn defeats(self: &Self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn against(self: &Self, opponent: &Self) -> Result {
        if self.defeats() == *opponent {
            Result::Win
        } else if opponent.defeats() == *self {
            Result::Lose
        } else {
            Result::Draw
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'X' => Self::Rock,
            'B' => Self::Paper,
            'Y' => Self::Paper,
            'C' => Self::Scissors,
            'Z' => Self::Scissors,
            _ => unimplemented!(),
        }
    }
}

fn one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let their_move = Move::from_char(line.chars().nth(0).unwrap());
            let our_move = Move::from_char(line.chars().nth(2).unwrap());
            our_move.points() + our_move.against(&their_move).points()
        })
        .sum()
}

fn two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let their_move = Move::from_char(line.chars().nth(0).unwrap());
            let result = Result::from_char(line.chars().nth(2).unwrap());
            let our_move = match result {
                Result::Win => their_move.defeats().defeats(),
                Result::Lose => their_move.defeats(),
                Result::Draw => their_move.clone(),
            };
            our_move.points() + result.points()
        })
        .sum()
}

pub fn solve(input: &str) -> String {
    two(input).to_string()
}
