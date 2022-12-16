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

fn calc_distances<'a>(valves: &'a HashMap<&'a str, (u32, Vec<&str>)>) -> HashMap<(&'a str, &'a str), u32> {
    let mut distances: HashMap<(&str, &str), u32> = valves.iter().flat_map(|(from, (_, links_to))| {
        links_to.iter().map(|to| ((*from, *to), 1))
    }).collect();
    
    let combinations: HashSet<(&str, &str)> = valves.keys().flat_map(|from| valves.keys().map(|to| (*from, *to))).collect();

    let keys = valves.keys();

    for _ in 1..keys.len() {
        combinations.iter().for_each(|(from, to)| {
            match distances.get(&(*from, *to)) {
                Some(_) => {}
                None => {
                    let dist = keys.clone().filter_map(|mid| {
                            match distances.get(&(*from, mid)) {
                                Some(first) => {
                                    match distances.get(&(mid, to)) {
                                        Some(second) => Some(first + second),
                                        None => None
                                    }
                                },
                                None => None
                            }
                        }).min();
                    match dist {
                        Some(d) => {
                            distances.insert((*from, *to), d);
                        },
                        _ => {}
                    }
                }
            }
        });
    }

    distances
}

fn one(input: &str) -> u32 {
    let valves: HashMap<&str, (u32, Vec<&str>)> = input.lines().map(parse).collect();

    let mut cave = "AA";

    let mut visited: HashSet<&str> = HashSet::new();

    let mut pressure_released: u32 = 0;

    let distances = calc_distances(&valves);

    distances.iter().for_each(|((from, to), dist)| println!("{} -> {}: {}", from, to, dist));

    for minuite in 0..30 {
        println!("starting {minuite} in cave {cave}");

        pressure_released += visited.iter().map(|cave| valves[cave].0).sum::<u32>();

        // NEXT

        let (_rate, links_to) = &valves[cave];

        cave = links_to[0];

        visited.insert(&cave);
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
        assert_eq!(crate::one(include_str!("test16.txt")), 1651);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test16.txt")), 45000);
    }
}
