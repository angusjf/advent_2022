#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win, Lose, Draw
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

fn score(a: &Move, b: &Move) -> u32 {
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

fn parse_move(c: char) -> Move {
    match c {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => unimplemented!(),
    }
}

fn parse_result(c: char) -> Result {
    match c {
        'X' => Result::Lose,
        'Y' => Result::Draw,
        'Z' => Result::Win,
        _ => unimplemented!(),
    }
}

fn parse_line(line: &str) -> (Move, Result) {
    (
        parse_move(line.chars().nth(0).unwrap()),
        parse_result(line.chars().nth(2).unwrap()),
    )
}

fn points((them, us): (&Move, &Move)) -> u32 {
    us.points() + score(us, them)
}

fn winning_move_against(x: &Move) -> Move {
    match x {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock
    }
}

fn losing_move_against(x: &Move) -> Move {
    match x {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

fn move_needed(them: &Move, result: Result) -> Move {
    match result {
        Result::Win => winning_move_against(them),
        Result::Lose => losing_move_against(them),
        Result::Draw => them.clone(),
    }
}

fn one(input: &str) -> u32 {
    let lines = input.lines().map(parse_line);
        lines.map(|(them, result)| {
            let (them, us) = (&them, move_needed(&them, result));
            dbg!(points((&them, &us)))
        })
        .sum()
}

pub fn solve(input: &str) -> String {
    one(input).to_string()
}
