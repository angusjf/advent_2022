use itertools::Itertools;
use std::collections::*;

fn one(input: &str) -> u64 {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut seen = HashSet::new();

    let mut count = 0;

    for y in 1..input.len() - 1 {
        let mut max = input[y][0];
        for x in 1..input[0].len() - 1 {
            let tree = input[y][x];
            if tree > max {
                if !seen.contains(&(y, x)) {
                    count += 1;
                }
                max = tree;
                seen.insert((y, x));
            }
        }
    }

    for y in 1..input.len() - 1 {
        let mut max = input[y][input[0].len() - 1];
        for x in (1..input[0].len() - 1).rev() {
            let tree = input[y][x];
            if tree > max {
                if !seen.contains(&(y, x)) {
                    count += 1;
                }
                max = tree;
                seen.insert((y, x));
            }
        }
    }

    for x in 1..input[0].len() - 1 {
        let mut max = input[input.len() - 1][x];
        for y in (1..input.len() - 1).rev() {
            let tree = input[y][x];
            if tree > max {
                if !seen.contains(&(y, x)) {
                    count += 1;
                }
                max = tree;
                seen.insert((y, x));
            }
        }
    }

    for x in 1..input[0].len() - 1 {
        let mut max = input[0][x];
        for y in 1..input.len() - 1 {
            let tree = input[y][x];
            if tree > max {
                if !seen.contains(&(y, x)) {
                    count += 1;
                }
                max = tree;
                seen.insert((y, x));
            }
        }
    }

    count + (input.len() as u64 * 2) + ((input[0].len() - 2) as u64 * 2)
}

fn two(input: &str) -> u64 {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    (1..input.len() - 1)
        .cartesian_product(1..input[0].len() - 1)
        .map(|(me_y, me_x)| {
            let me = input[me_y][me_x];

            let left = (0..me_x).rev();
            let left = left
                .clone()
                .position(|x| input[me_y][x] >= me)
                .map(|x| x + 1)
                .unwrap_or(dbg!(left.len()));

            let right = (me_x + 1)..input[0].len();
            let right = right
                .clone()
                .position(|x| input[me_y][x] >= me)
                .map(|x| x + 1)
                .unwrap_or(dbg!(right.len()));

            let up = (0..me_y).rev();
            let up = up
                .clone()
                .position(|y| input[y][me_x] >= me)
                .map(|x| x + 1)
                .unwrap_or(dbg!(up.len()));

            let down = (me_y + 1)..input.len();
            let down = down
                .clone()
                .position(|y| input[y][me_x] >= me)
                .map(|x| x + 1)
                .unwrap_or(dbg!(down.len()));

            left * right * up * down
        })
        .max()
        .unwrap() as u64
}

fn main() {
    println!("{:?}", one(include_str!("input08.txt")));
    println!("{:?}", two(include_str!("input08.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("input08.txt")), 1695);
    }

    #[test]
    fn one_test() {
        assert_eq!(crate::one(include_str!("test08.txt")), 21);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("input08.txt")), 287040);
    }

    #[test]
    fn two_test() {
        assert_eq!(crate::two(include_str!("test08.txt")), 8);
    }
}
