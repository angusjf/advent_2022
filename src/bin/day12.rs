use itertools::*;
use std::collections::*;

fn one(input: &str) {
    let mut map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

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

    map[start.1][start.0] = 'a';
    map[end.1][end.0] = 'z';

    fewest_steps_to_end(&map, start, end);
}

fn fewest_steps_to_end(map: &Vec<Vec<char>>, start: (usize, usize), goal: (usize, usize)) {
    let mut q: BTreeSet<_> = (0..map.len())
        .flat_map(|y| (0..map[0].len()).map(move |x| (x, y)))
        .collect();

    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    dist.insert(goal, 0);

    while q.len() > 0 {
        let u = *q
            .iter()
            .min_by_key(|v| dist.get(*v).unwrap_or(&99999999))
            .unwrap();

        // println!("{:?}", map[u.1][u.0]);

        q.remove(&u);

        neighbours(map, u)
            .iter()
            .filter(|v| q.contains(v))
            .for_each(|v| {
                let alt = dist.get(&u).unwrap_or(&999999999) + 1;
                if alt < *dist.get(v).unwrap_or(&999999999) {
                    dist.insert(*v, alt);
                    prev.insert(*v, u);
                }
            });
    }

    let m = dist
        .iter()
        .filter(|(v, _dist)| map[v.1][v.0] == 'a')
        .map(|(v, dist)| dist)
        .min()
        .unwrap();
    // print_map(map, &dist);
    // if u == goal {
    //     let mut path = String::new();
    //     let mut cursor = u;
    //     while let Some(p) = prev.get(&cursor) {
    //         path.push(map[cursor.1][cursor.0]);
    //         cursor = *p;
    //     }
    //     let path: String = path.chars().rev().collect();
    //     println!("path: {path} ({})", path.len());
    // }

    // println!("ended");
    println!("{:?}", m);
    // println!("{:?}", dist);
}

fn print_map(map: &Vec<Vec<char>>, dist: &HashMap<(usize, usize), i32>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let u = (x, y);
            print!("{}[{}]", map[y][x], dist.get(&u).unwrap());
        }
        println!();
    }
    println!();
    print!("\x1B[2J\x1B[1;1H");
    println!();
}

// fn print_map(map: &Vec<Vec<char>>, dist: &HashMap<(usize, usize), i32>, curr: &(usize, usize)) {
//     for y in 0..map.len() {
//         for x in 0..map[0].len() {
//             let u = (x, y);
//             if *curr == u {
//                 print!("#");
//             } else if dist.get(&u).is_some() {
//                 print!(".");
//             } else {
//                 print!("{}", map[y][x]);
//             }
//         }
//         println!();
//     }
//     println!();
//     print!("\x1B[2J\x1B[1;1H");
//     println!();
// }

fn neighbours(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let xs: Vec<(i64, i64)> = vec![
        (x as i64 - 1, y as i64),
        (x as i64 + 1, y as i64),
        (x as i64, y as i64 - 1),
        (x as i64, y as i64 + 1),
    ];
    let c = map[y][x];
    let next_c = char::try_from(map[y][x] as u32 - 1).unwrap();
    xs.iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| {
            let x: usize = usize::try_from(*x).unwrap();
            let y: usize = usize::try_from(*y).unwrap();
            (x, y)
        })
        .filter(|(x, y)| *y < map.len() && *x < map[0].len())
        .filter(|(x, y)| next_c <= map[*y][*x])
        .collect()
}

fn two(input: &str) -> u64 {
    0
}

fn main() {
    one(include_str!("input12.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        crate::one(include_str!("test12.txt"));
        assert_eq!(0, 1);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test12.txt")), 10605);
    }
}
