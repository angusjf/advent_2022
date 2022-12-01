use std::io;
use std::io::prelude::*;
use itertools::*;

fn two(input: &str) -> u32 {
    let mut sums = input
        .lines()
        .group_by(|b| b.is_empty())
        .into_iter()
        .filter_map(|(empty, group)|
            if empty {
                None
            } else {
                Some (group.into_iter().map(|x| x.parse::<u32>().unwrap()))
            }
        ).map(|x| x.sum::<u32>())
        .collect::<Vec<_>>();
    sums.sort_by(|a, b| b.cmp(a));
    sums.iter().take(3).sum()
}

pub fn solve() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    println!("{}", two(&buffer));
}
