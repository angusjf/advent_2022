use itertools::*;
use std::io::{self, Write};

use std::collections::*;

fn parse(line: &str) -> (&str, (i64, Vec<&str>)) {
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

fn calc_distances<'a>(valves: &'a HashMap<&'a str, (i64, Vec<&str>)>) -> HashMap<(&'a str, &'a str), i64> {
    let mut distances: HashMap<(&str, &str), i64> = valves.iter().flat_map(|(from, (_, links_to))| {
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

fn value(cache: &mut HashMap<(&str, i64, &str), i64>, cave: &str, open_valves: HashSet<&str>, valves: &HashMap<&str, (i64, Vec<&str>)>, mins: i64) -> i64 {
    println!("calculating value of {cave} at {mins}");
    let _ = std::io::stdout().flush();

    if mins <= 0 {
        return 0;
    }

    cache.insert((cave, mins, valves.iter().collect()), 0);

    let already_opened = open_valves.contains(cave);

    let (rate, links_to) = &valves[cave];

    let value_move = links_to.iter().map(|d| value(cache, d, open_valves.clone(), &valves, mins - 1)).max().unwrap();

    if already_opened  {

        value_move

    } else {
        let mut open_valves_open = open_valves.clone();

        open_valves_open.insert(cave);

        let value_open = rate * (mins - 1) + links_to.iter().map(|d| value(cache, d, open_valves_open.clone(), &valves, mins - 2)).max().unwrap();

        std::cmp::max(value_open, value_move)

    }
}

const TIME: i64 = 30;

fn one(input: &str) -> i64 {
    let valves: HashMap<&str, (i64, Vec<&str>)> = input.lines().map(parse).collect();

    let mut cave = "AA";

    let mut visited: HashSet<&str> = HashSet::new();

    let mut pressure_released: i64 = 0;

    let distances = calc_distances(&valves);

    distances.iter().for_each(|((from, to), dist)| println!("{} -> {}: {}", from, to, dist));

    for minuite in 1..=TIME {
        println!("starting {minuite} in cave {cave}");

        pressure_released += visited.iter().map(|cave| valves[cave].0).sum::<i64>();

        // NEXT
        let (_, links_to) = &valves[cave];

        let cache = HashMap::new();

        let (most_valuable_dest, its_value) =
                links_to.
                iter().
                map(|dest| (dest, value(cache, dest, visited.clone(), &valves, minuite - 1)))
                .max_by_key(|(_, value)| *value).unwrap();

        let stay_value = value(cache, cave, visited.clone(), &valves, minuite);

        if its_value > stay_value {
            println!("chosing to move to {most_valuable_dest}");
            cave = most_valuable_dest;
        } else {
            println!("chosing to stay and open {cave}");
            visited.insert(&cave);
        }
    }

    pressure_released
}

fn two(input: &str) -> i64 {
    0
}

fn main() {
    // println!("{}", one(include_str!("input16.txt")));
    println!("{}", one(include_str!("test16.txt")));
    // println!("{}", two(include_str!("input16.txt")));
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
