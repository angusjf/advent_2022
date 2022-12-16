use itertools::*;
use std::collections::*;

fn parse(line: &str) -> (&str, (u32, Vec<&str>)) {
    let mut iter = line.split(" ");
    iter.next();
    let valve = iter.next().unwrap();
    iter.next();
    iter.next();
    let rate = iter.next().unwrap().strip_prefix("rate=").unwrap().strip_suffix(";").unwrap().parse().unwrap();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    let leads_to: Vec<_> = iter.map(|s| s.strip_suffix(",").unwrap_or(s)).collect();
    (valve, (rate, leads_to))
}

fn next_cave<'a>(cave: &'a str, valves: &'a HashMap<&'a str, (u32, Vec<&'a str>)>, visited: &'a HashSet<&'a str>) -> &'a str {
    let (_rate, links_to) = &valves[cave];
    links_to[0]
}

fn one(input: &str) -> u32 {
    let valves: HashMap<&str, (u32, Vec<&str>)> = input.lines().map(parse).collect();

    let mut cave = "AA";

    let mut visited: HashSet<&str> = HashSet::new();

    let mut pressure_released: u32 = 0;

    for _ in 0..30 {
        pressure_released += visited.iter().map(|cave| valves[cave].0).sum::<u32>();

        cave = next_cave(cave, &valves, &visited);
        
        visited.insert(cave);
    }

    pressure_released
}

fn two(input: &str) -> u32 {
    0
}

fn main() {
    println!("{}", one(include_str!("input16.txt")));
    println!("{}", two(include_str!("input16.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test16.txt")), 24000);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test16.txt")), 45000);
    }
}
