use itertools::*;

fn two(input: &str) -> u32 {
    let mut sums: Vec<_> = input
        .lines()
        .group_by(|b| b.is_empty())
        .into_iter()
        .filter_map(|(empty, group)| {
            if empty {
                None
            } else {
                Some(group.map(|x| x.parse::<u32>().unwrap()))
            }
        })
        .map(|x| x.sum())
        .collect();
    sums.sort();
    sums.iter().rev().take(3).sum()
}

pub fn solve(input: &str) -> String {
    two(input).to_string()
}
