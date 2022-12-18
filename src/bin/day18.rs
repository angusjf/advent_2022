use itertools::*;
use std::collections::*;

fn parse(line: &str) -> (i64, i64, i64) {
    line.split(",")
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn two(input: &str) -> i64 {
    let cubes: HashSet<_> = input.lines().map(parse).collect();

    let min_x = *cubes.iter().map(|(x, _, _)| x).min().unwrap() - 1;
    let min_y = *cubes.iter().map(|(_, y, _)| y).min().unwrap() - 1;
    let min_z = *cubes.iter().map(|(_, _, z)| z).min().unwrap() - 1;
    let max_x = *cubes.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let max_y = *cubes.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let max_z = *cubes.iter().map(|(_, _, z)| z).max().unwrap() + 1;

    let mut water_box = HashSet::new();

    let mut lava = 0;

    let mut queue: VecDeque<(i64, i64, i64)> = VecDeque::from([(min_x, min_y, min_z)]);

    while let Some(pos @ (x, y, z)) = queue.pop_back() {
        if (min_x..=max_x).contains(&x)
            && (min_y..=max_y).contains(&y)
            && (min_z..=max_z).contains(&z)
        {
            if cubes.contains(&pos) {
                lava += 1;
            }
            if !water_box.contains(&pos) && !cubes.contains(&pos) {
                water_box.insert(pos);
                vec![
                    (0, 0, 1),
                    (0, 0, -1),
                    (0, 1, 0),
                    (0, -1, 0),
                    (1, 0, 0),
                    (-1, 0, 0),
                ]
                .iter()
                .for_each(|(dx, dy, dz)| queue.push_front((x + dx, y + dy, z + dz)));
            }
        }
    }

    lava
}

fn main() {
    // println!("{:?}", two(include_str!("test18-1.txt")));
    println!("{:?}", two(include_str!("input18.txt")));
}
