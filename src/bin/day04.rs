use itertools::*;
use std::ops::Range;

fn ranges(line: &str) -> (Range<u32>, Range<u32>) {
    line.split(',')
        .map(|range| {
            let (min, max) = range
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            min..max + 1
        })
        .collect_tuple()
        .unwrap()
}

fn one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (a, b) = ranges(line);
            if a.clone().all(|a| b.contains(&a)) || b.clone().all(|b| a.contains(&b)) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (a, b) = ranges(line);
            if a.clone().any(|a| b.contains(&a)) || b.clone().any(|b| a.contains(&b)) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    println!("{}", one(include_str!("input04.txt")));
    println!("{}", two(include_str!("input04.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test04.txt")), 2);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test04.txt")), 4);
    }
}
