use itertools::*;
use std::collections::*;

fn parse(line: &str) -> (&str, (i64, Vec<&str>)) {
    let mut iter = line.split(" ");
    iter.next();
    let valve = iter.next().unwrap();
    iter.next();
    iter.next();
    let rate = iter
        .next()
        .unwrap()
        .strip_prefix("rate=")
        .unwrap()
        .strip_suffix(";")
        .unwrap()
        .parse()
        .unwrap();
    iter.next();
    iter.next();
    iter.next();
    iter.next();
    let leads_to: Vec<_> = iter.map(|s| s.strip_suffix(",").unwrap_or(s)).collect();
    (valve, (rate, leads_to))
}

fn calc_distances<'a>(
    valves: &'a HashMap<&'a str, (i64, Vec<&str>)>,
) -> HashMap<(&'a str, &'a str), i64> {
    let mut distances: HashMap<(&str, &str), i64> = valves
        .iter()
        .flat_map(|(from, (_, links_to))| links_to.iter().map(|to| ((*from, *to), 1)))
        .collect();

    let combinations: HashSet<(&str, &str)> = valves
        .keys()
        .flat_map(|from| valves.keys().map(|to| (*from, *to)))
        .collect();

    let keys = valves.keys();

    for _ in 1..keys.len() {
        combinations.iter().for_each(|(from, to)| {
            if from != to {
                match distances.get(&(*from, *to)) {
                    Some(_) => {}
                    None => {
                        let dist = keys
                            .clone()
                            .filter_map(|mid| match distances.get(&(*from, mid)) {
                                Some(first) => match distances.get(&(mid, to)) {
                                    Some(second) => Some(first + second),
                                    None => None,
                                },
                                None => None,
                            })
                            .min();
                        match dist {
                            Some(d) => {
                                distances.insert((*from, *to), d);
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    distances
}

fn selections<'a>(options: &'a Vec<&'a str>) -> impl Iterator<Item = (&str, Vec<&str>)> + 'a {
    options.iter().enumerate().map(move |(i, x)| {
        let start = &options[0..i];
        let end = &options[i + 1..];
        let mut others = Vec::from(start);
        others.extend_from_slice(end);
        (*x, others)
    })
}

fn value_one(
    cave: &str,
    mins: i64,
    valves: &HashMap<&str, (i64, Vec<&str>)>,
    options: Vec<&str>,
    dist: &HashMap<(&str, &str), i64>,
) -> i64 {
    selections(&options)
        .filter(|(choice, _)| dist[&(cave, *choice)] < mins)
        .map(|(chosen, rest)| {
            &valves[chosen].0 * (mins - dist[&(cave, chosen)] - 1)
                + value_one(chosen, mins - dist[&(cave, chosen)] - 1, valves, rest, dist)
        })
        .max()
        .unwrap_or(0)
}

fn value_two(
    cave: &str,
    mins: i64,
    valves: &HashMap<&str, (i64, Vec<&str>)>,
    options: Vec<&str>,
    dist: &HashMap<(&str, &str), i64>,
) -> i64 {
    selections(&options)
        .filter(|(choice, _)| dist[&(cave, *choice)] < mins)
        .map(|(chosen, rest)| {
            &valves[chosen].0 * (mins - dist[&(cave, chosen)] - 1)
                + value_two(
                    chosen,
                    mins - dist[&(cave, chosen)] - 1,
                    valves,
                    rest.clone(),
                    dist,
                )
        })
        .max()
        .unwrap_or(value_one("AA", mins, valves, options, dist))
}

fn two(input: &str) -> i64 {
    let valves: HashMap<&str, (i64, Vec<&str>)> = input.lines().map(parse).collect();

    let dist = calc_distances(&valves);

    dbg!(&valves);
    dbg!(&dist);

    dbg!(value_one(
        "AA",
        30,
        &valves,
        valves
            .iter()
            .filter(|(_, (r, _))| *r > 0)
            .map(|(x, _)| *x)
            .collect(),
        &dist,
    ));

    value_two(
        "AA",
        26,
        &valves,
        valves
            .iter()
            .filter(|(_, (r, _))| *r > 0)
            .map(|(x, _)| *x)
            .collect(),
        &dist,
    )
}

fn main() {
    println!("{}", two(include_str!("test16.txt")));
}
