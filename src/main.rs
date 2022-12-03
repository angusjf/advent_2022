#![feature(iter_array_chunks)]

mod day03;

use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    println!("{}", day03::solve(&buffer));
}
