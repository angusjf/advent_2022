enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn points(self: &Self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

fn score(a: Move, b: Move) -> u32 {
    match (a, b) {
        (Move::Rock, Move::Scissors) => WIN,
        (Move::Rock, Move::Paper) => LOSE,
        (Move::Paper, Move::Rock) => WIN,
        (Move::Paper, Move::Scissors) => LOSE,
        (Move::Scissors, Move::Paper) => WIN,
        (Move::Scissors, Move::Rock) => LOSE,
        _ => DRAW,
    }
}

fn parse_char(c: char) -> Move {
    match c {
        'X' => Move::Rock,
        'A' => Move::Rock,
        'Y' => Move::Paper,
        'B' => Move::Paper,
        'Z' => Move::Scissors,
        'C' => Move::Scissors,
        _ => unimplemented!(),
    }
}

fn parse_line(line: &str) -> (Move, Move) {
    (
        parse_char(line.chars().nth(0).unwrap()),
        parse_char(line.chars().nth(2).unwrap()),
    )
}

fn points((them, us): (Move, Move)) -> u32 {
    us.points() + score(us, them)
}

fn one(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .map(points)
        .sum()
}

pub fn solve(input: &str) -> String {
    one(input).to_string()
}
