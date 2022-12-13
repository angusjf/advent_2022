use core::fmt;
use itertools::*;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone)]
enum Nest {
    Cell(u8),
    Nil,
    Pair(Box<(Nest, Nest)>),
}

use Nest::*;

impl fmt::Debug for Nest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell(a) => write!(f, "{a}"),
            Nil => write!(f, "[]"),
            Pair(b) => {
                write!(f, "[")?;
                b.0.fmt(f)?;
                write!(f, " | ")?;
                b.1.fmt(f)?;
                write!(f, "]")
            }
        }
    }
}

impl PartialOrd for Nest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use std::cmp::Ordering::*;
        match (self, other) {
            (Cell(a), Cell(b)) => a.partial_cmp(b),
            (Nil, Nil) => Some(Equal),
            (_, Nil) => Some(Greater),
            (Nil, _) => Some(Less),
            (Cell(a), b) => (Pair(Box::new((Cell(*a), Nil)))).partial_cmp(b),
            (a, Cell(b)) => a.partial_cmp(&Pair(Box::new((Cell(*b), Nil)))),
            (Pair(box_a), Pair(box_b)) => match box_a.0.partial_cmp(&box_b.0) {
                Some(Less) => Some(Less),
                Some(Greater) => Some(Greater),
                _ => box_a.1.partial_cmp(&box_b.1),
            },
        }
    }
}

impl Ord for Nest {
    fn cmp(&self, other: &Self) -> Ordering {
        use std::cmp::Ordering::*;
        match (self, other) {
            (Cell(a), Cell(b)) => a.cmp(b),
            (Nil, Nil) => Equal,
            (_, Nil) => Greater,
            (Nil, _) => Less,
            (Cell(a), b) => (Pair(Box::new((Cell(*a), Nil)))).cmp(b),
            (a, Cell(b)) => a.cmp(&Pair(Box::new((Cell(*b), Nil)))),
            (Pair(box_a), Pair(box_b)) => match box_a.0.cmp(&box_b.0) {
                Less => Less,
                Greater => Greater,
                Equal => box_a.1.cmp(&box_b.1),
            },
        }
    }
}

fn parse(input: &str) -> (Nest, &str) {
    if &input[0..2] == "10" {
        (Cell(10), &input[2..])
    } else if let Some(n) = &input[0..1].parse().ok() {
        (Cell(*n), &input[1..])
    } else if &input[0..1] == "[" {
        let (x, more) = parse_list(&input[1..]);
        (x, more)
    } else {
        unimplemented!()
    }
}

fn parse_list(input: &str) -> (Nest, &str) {
    if &input[0..1] == "]" {
        (Nil, &input[1..])
    } else {
        let (x, more) = parse(input);
        if &more[0..1] == "," {
            let (xs, more_more) = parse_list(&more[1..]);
            (Pair(Box::new((x, xs))), more_more)
        } else {
            (Pair(Box::new((x, Nil))), &more[1..])
        }
    }
}

fn one(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .enumerate()
        .filter_map(|(i, chunk)| {
            let (a, b) = chunk.take(2).collect_tuple().unwrap();

            let a = parse(a).0;
            let b = parse(b).0;

            if a.partial_cmp(&b)?.is_lt() {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn two(input: &str) -> usize {
    let mut list = input
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|chunk| chunk.take(2))
        .map(|x| parse(x).0)
        .collect::<Vec<_>>();

    let two = Pair(Box::new((Pair(Box::new((Cell(2), Nil))), Nil)));
    let six = Pair(Box::new((Pair(Box::new((Cell(6), Nil))), Nil)));

    list.push(two.clone());
    list.push(six.clone());

    list.sort();

    (list.iter().position(|x| x == &two).unwrap() + 1)
        * (list.iter().position(|x| x == &six).unwrap() + 1)
}

fn main() {
    // println!("{:?}", one(include_str!("input13.txt")));
    println!("{:?}", one(include_str!("test13.txt")));
    println!("{:?}", two(include_str!("test13.txt")));
}
