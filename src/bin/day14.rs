use core::{fmt, time};
use itertools::*;
use std::{cmp::Ordering, collections::HashSet, thread};

fn parse(line: &str) -> Vec<(usize, usize)> {
    line.split(" -> ")
        .map(|pair| {
            pair.split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn collides_with(path: &Vec<(usize, usize)>, (x, y): &(usize, usize)) -> bool {
    path.iter()
        .zip(path.iter().skip(1))
        .any(|((x1, y1), (x2, y2))| {
            if x1 == x2 {
                x1 == x && ((y >= y1 && y <= y2) || (y >= y2 && y <= y1))
            } else if y1 == y2 {
                y1 == y && ((x >= x1 && x <= x2) || (x >= x2 && x <= x1))
            } else {
                unreachable!();
            }
        })
}

fn valid(
    next: &(usize, usize),
    paths: &Vec<Vec<(usize, usize)>>,
    grains: &HashSet<(usize, usize)>,
    max_y: usize,
) -> bool {
    if next.1 == max_y + 2 {
        false
    } else if paths.iter().any(|path| collides_with(&path, next)) {
        false
    } else if grains.contains(&next) {
        false
    } else {
        true
    }
}

fn print(
    next: &(usize, usize),
    paths: &Vec<Vec<(usize, usize)>>,
    grains: &HashSet<(usize, usize)>,
    min_x: usize,
    max_x: usize,
    max_y: usize,
) {
    for y in 0..=max_y + 2 {
        for x in min_x..=max_x {
            if next == &(x, y) {
                print!("*");
            } else if paths.iter().any(|path| collides_with(&path, &(x, y))) {
                print!("#");
            } else if grains.contains(&(x, y)) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn one(input: &str) -> usize {
    let paths: Vec<Vec<(usize, usize)>> = input.lines().map(parse).collect();

    let mut grains = HashSet::new();

    let max_y = *paths
        .iter()
        .map(|path| path.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap();
    let min_x = *paths
        .iter()
        .map(|path| path.iter().map(|(x, _)| x).min().unwrap())
        .min()
        .unwrap();
    let max_x = *paths
        .iter()
        .map(|path| path.iter().map(|(x, _)| x).max().unwrap())
        .max()
        .unwrap();

    'grains: loop {
        let mut grain = (500, 0);

        if !valid(&grain, &paths, &grains, max_y) {
            break 'grains;
        }

        'grain: loop {
            let down = (grain.0, grain.1 + 1);
            if valid(&down, &paths, &grains, max_y) {
                grain = down;
                continue;
            }
            let down_left = (grain.0 - 1, grain.1 + 1);
            if valid(&down_left, &paths, &grains, max_y) {
                grain = down_left;
                continue;
            }
            let down_right = (grain.0 + 1, grain.1 + 1);
            if valid(&down_right, &paths, &grains, max_y) {
                grain = down_right;
                continue;
            }
            break 'grain;
        }
        print!("\x1B[2J\x1B[1;1H"); // clear
        print(&grain, &paths, &grains, min_x, max_x, max_y);
        thread::sleep(time::Duration::from_millis(20));

        grains.insert(grain);
    }

    grains.len()
}

fn two(input: &str) -> usize {
    0
}

fn main() {
    // println!("{}", one(include_str!("test14.txt")));
    println!("{:?}", one(include_str!("input14.txt")));
}
