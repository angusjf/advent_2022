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

const TIME: i64 = 6;

fn value_of_staying_at(
    min: i64,
    cave: &str,
    valves: &HashMap<&str, (i64, Vec<&str>)>,
    opened: HashSet<&str>,
) -> i64 {
    if min <= 0 {
        return 0;
    }

    let mut opened_with_this_cave = opened.clone();

    opened_with_this_cave.insert(cave);

    let (r, links_to) = &valves[cave];

    let value_after = links_to
        .iter()
        .map(|cave| value_of_moving_to(min - 1, cave, valves, opened_with_this_cave.clone()))
        .max()
        .unwrap();

    (r * min) + value_after
}

fn value_of_moving_to(
    min: i64,
    cave: &str,
    valves: &HashMap<&str, (i64, Vec<&str>)>,
    opened: HashSet<&str>,
) -> i64 {
    if min <= 0 {
        return 0;
    }

    // println!("{} {cave}: ", " ".repeat(((TIME - min) * 4) as usize));

    if opened.contains(cave) {
        let (_, links_to) = &valves[cave];

        let val = links_to
            .iter()
            .map(|cave| value_of_moving_to(min - 1, cave, valves, opened.clone()))
            .max()
            .unwrap();

        // println!("{} {val}", " ".repeat(((TIME - min) * 5) as usize));

        val
    } else {
        let value_of_staying = value_of_staying_at(min - 1, cave, valves, opened.clone());

        let value_of_leaving = {
            let (_, links_to) = &valves[cave];
            links_to
                .iter()
                .map(|cave| value_of_moving_to(min - 1, cave, valves, opened.clone()))
                .max()
                .unwrap()
        };

        // println!(
        //     "{} stay: {value_of_staying}, leave: {value_of_leaving}",
        //     " ".repeat(((TIME - min) * 5) as usize)
        // );

        std::cmp::max(value_of_staying, value_of_leaving)
    }
}

fn one(input: &str) -> i64 {
    let valves: HashMap<&str, (i64, Vec<&str>)> = input.lines().map(parse).collect();

    valves
        .iter()
        .for_each(|(k, (r, links))| println!("{k} {r} {}", links.join(" ")));

    let mut cave = "AA";

    let mut opened = HashSet::new();

    let mut pressure_released: i64 = 0;

    for min in (1..=TIME).rev() {
        dbg!(cave, &opened);

        let next_cave = {
            let value_of_cave = value_of_staying_at(min - 1, cave, &valves, opened.clone());

            let (_, links_to) = &valves[cave];

            let mut highest_value_other: Vec<_> = links_to
                .iter()
                .map(|cave| {
                    let val = value_of_moving_to(min - 1, cave, &valves, opened.clone());
                    (*cave, val)
                })
                .collect();

            highest_value_other.push((cave, value_of_cave));

            let (next_cave, next_cave_value) = highest_value_other
                .iter()
                .max_by_key(|(_, val)| val)
                .unwrap();

            dbg!(next_cave_value);

            next_cave.clone()
        };

        if next_cave == cave {
            // best to stay
            opened.insert(cave);
        }

        pressure_released += opened
            .iter()
            .map(|cave| {
                let (rate, _) = valves[cave];
                rate
            })
            .sum::<i64>();

        cave = next_cave;
    }

    pressure_released
}

fn two(input: &str) -> i64 {
    0
}

fn main() {
    // println!("{}", one(include_str!("test16-1.txt")));
    println!("{}", one(include_str!("test16.txt")));
    // println!("{}", one(include_str!("input16.txt")));
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
