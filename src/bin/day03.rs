#![feature(iter_array_chunks)]

use std::{iter::Skip, iter::Take, str::Chars};

fn halve(s: &str) -> (Take<Chars<'_>>, Skip<Chars<'_>>) {
    let half = s.len() / 2;
    let cs = s.chars();
    (cs.clone().take(half), cs.skip(half))
}

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    }
}

fn one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (mut start, end) = halve(line);
            let end: String = end.collect();
            start.find(|c| end.contains(*c)).unwrap()
        })
        .map(priority)
        .sum()
}

fn two(input: &str) -> u32 {
    input
        .lines()
        .array_chunks()
        .map(|[one, two, three]| {
            one.chars()
                .find(|x| two.contains(*x) && three.contains(*x))
                .unwrap()
        })
        .map(priority)
        .sum()
}

fn main() {
    println!("{}", one(include_str!("input03.txt")));
    println!("{}", two(include_str!("input03.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test03.txt")), 157);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test03.txt")), 70);
    }
}
