use itertools::*;
use std::collections::*;

fn one(input: &str) -> u64 {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|x| *x == 'S').map(|x| (x, y)))
        .unwrap();
    let end = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|x| *x == 'E').map(|x| (x, y)))
        .unwrap();

    fewest_steps_to_end(&map, HashSet::new(), start, end)
}

fn fewest_steps_to_end(
    map: &Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>,
    (x, y): (usize, usize),
    end: (usize, usize),
) -> u64 {
    if (x, y) == end {
        0
    } else {
        let c = map[y][x];
        let next_c = char::from_u32(c as u32 + 1).unwrap();

        let next_steps = vec![(x + 1, y), (x - 1, y), (x, y -1), (x, y + 1)];

        let next_steps = next_steps.iter().filter()
        //.filter(|(x, y)| map[*y][*x] == c || map[*y][*x] == next)
    }
}

fn two(input: &str) -> u64 {
    0
}

fn main() {
    println!("{}", one(include_str!("input12.txt")));
    println!("{}", two(include_str!("input12.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test12.txt")), 10605);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test12.txt")), 10605);
    }
}
