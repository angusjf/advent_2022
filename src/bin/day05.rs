use itertools::*;
use std::collections::LinkedList;

fn parse_stacks(input: &str) -> Vec<LinkedList<char>> {
    let mut stacks = vec![LinkedList::<char>::new(); (input.lines().next().unwrap().len() + 1) / 4];
    input
        .lines()
        .take_while(|line| !line.starts_with(" 1"))
        .for_each(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .for_each(|(i, c)| {
                    if c != ' ' {
                        stacks[i].push_front(c);
                    }
                });
        });
    stacks
}

fn parse_procedure<'a>(input: &'a str) -> impl Iterator<Item = (usize, usize, usize)> + 'a {
    input
        .lines()
        .filter(|line| line.starts_with("move"))
        .map(|line| {
            line.split(" ")
                .filter_map(|n| n.parse().ok())
                .collect_tuple()
                .unwrap()
        })
}

fn one(input: &str) -> String {
    let mut stacks = parse_stacks(input);
    parse_procedure(input).for_each(|(n, a, b)| {
        for _ in 0..n {
            let c = stacks[a - 1].pop_back().unwrap();
            stacks[b - 1].push_back(c);
        }
    });
    stacks.iter().filter_map(|stack| stack.back()).collect()
}

fn two(input: &str) -> String {
    let mut stacks = parse_stacks(input);
    parse_procedure(input).for_each(|(n, a, b)| {
        let at = stacks[a - 1].len() - n;
        let mut cs = stacks[a - 1].split_off(at);
        stacks[b - 1].append(&mut cs);
    });
    stacks.iter().filter_map(|stack| stack.back()).collect()
}

fn main() {
    println!("{}", one(include_str!("input05.txt")));
    println!("{}", two(include_str!("input05.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test05.txt")), "CMZ");
    }

    #[test]
    fn one_real() {
        assert_eq!(crate::one(include_str!("input05.txt")), "SPFMVDTZT");
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test05.txt")), "MCD");
    }

    #[test]
    fn two_real() {
        assert_eq!(crate::two(include_str!("input05.txt")), "ZFSJBPRFP");
    }
}
