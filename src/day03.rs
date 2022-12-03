use std::{iter::Skip, iter::Take, str::Chars};

fn half(s: &str) -> (Take<Chars<'_>>, Skip<Chars<'_>>) {
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
            let (mut start, end) = half(line);
            let end: Vec<_> = end.collect();
            let c = start.find(|c| end.contains(c)).unwrap();
            c
        })
        .map(priority)
        .sum()
}

fn two(input: &str) -> u32 {
    input
        .lines()
        // .map(|line| {
        //     let (mut start, end) = half(line);
        //     let end: Vec<_> = end.collect();
        //     let c = start.find(|c| end.contains(c)).unwrap();
        //     c
        // })
        .array_chunks()
        .map(|[one, two, three]| {
            one.chars()
                .find(|x| two.contains(*x) && three.contains(*x))
                .unwrap()
        })
        .map(priority)
        .sum()
}

pub fn solve(input: &str) -> String {
    two(input).to_string()
}
