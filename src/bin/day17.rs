#![feature(iter_intersperse)]

/*
36: 11 G
..: 11 F
..: 11 E
..: 10 G
..: 10 F
..: 10 E
30: 9  G
..: 9  F
..: 9  E
..: 8  G
..: 8  F
..: 8  E
..: 7  G
..: 7  F
..: 7  E
..: 6  G
20: 6  F
19: 6  E
18: 5  G
17: 5  F
16: 5  E
15: 4  G
14: 4  F
13: 4  E
12: 3  G
11: 3  F
10: 3  E
 9: 2  G
 8: 2  F
 7: 2  E I already saw this at 4 and here I am at 7... what will the value at 36 be?
 6: 1  G          I know from here on it repeats every 3, so the value in 29 is what?
 5: 1  F                  I know the value in 27 turns will be (here: 2) + 9 * (E to E increase) (which is 1)
 4: 1  E                           which gives me 2 + 9 * 1 = 11
 3: 0  D
 2: 0  C
 1: 0  B
 0: 0  A
*/

use std::collections::*;

const MAX: i64 = 1000000000000;

fn tet_string(tet: &Vec<(i64, i64)>, highest: i64) -> String {
    tet.iter()
        .map(|(x, y)| format!("({},{})", x, y - highest))
        .collect()
}

fn one(input: &str) -> i64 {
    let tet_flat = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let tet_plus = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
    let tet_j = vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
    let tet_l = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let tet_o = vec![(0, 0), (1, 0), (0, 1), (1, 1)];

    let tetronimos = vec![tet_flat, tet_plus, tet_j, tet_l, tet_o];

    let mut grid: HashSet<(i64, i64)> = HashSet::new();

    let mut chars = input.lines().nth(0).unwrap().chars().cycle();
    let mut c: i64 = 0;
    let c_limit: i64 = input.lines().nth(0).unwrap().len() as i64;

    let mut seen = HashMap::new();

    let mut n_blocks: i64 = 0;

    let mut height_when_noticed_loop = None;
    let mut remaining_blocks = None;
    let mut height_differential = None;

    'outer: for tet in tetronimos.iter().cycle().take(MAX as usize) {
        let highest = highest_point(&grid);
        let mut tet: Vec<_> = tet
            .iter()
            .map(|(x, y)| (*x + 2, *y + highest + 4))
            .collect();
        let highest_points = landscape(&grid);

        'inner: loop {
            match chars.next().unwrap() {
                '<' => {
                    if !tet
                        .iter()
                        .any(|(x, y)| *x == 0 || grid.contains(&(*x - 1, *y)))
                    {
                        tet = tet.iter().map(|(x, y)| (*x - 1, *y)).collect();
                    }
                }

                '>' => {
                    if !tet
                        .iter()
                        .any(|(x, y)| *x == 6 || grid.contains(&(*x + 1, *y)))
                    {
                        tet = tet.iter().map(|(x, y)| (*x + 1, *y)).collect();
                    }
                }

                _ => unreachable!(),
            }

            c += 1;

            let val = format!(
                "{}-{}-{}",
                c % c_limit,
                highest_points.clone(),
                tet_string(&tet.clone(), highest),
            );

            if let Some((c_back_then, n_blocks_back_then, highest_point_back_then)) = seen.get(&val)
            {
                println!(
                    "I saw this configuration back at {c_back_then} [when there were {n_blocks_back_then} blocks & height was {highest_point_back_then}],"
                );

                println!(
                    " ... and here I am at {c} [where there are {n_blocks} blocks & height is {highest}]"
                );

                let repeats_every_c = c - c_back_then;

                let blocks_increase_every_repeat = n_blocks - n_blocks_back_then;

                let height_increase_every_repeat = highest - highest_point_back_then;

                println!("repeats_every_c: {repeats_every_c}");

                println!("blocks_increase_every_repeat: {blocks_increase_every_repeat}");

                println!("height_increase_every_repeat: {height_increase_every_repeat}");

                let repeats = (MAX - n_blocks) / blocks_increase_every_repeat;

                let height = highest + height_increase_every_repeat * repeats;
                let blocks = n_blocks + blocks_increase_every_repeat * repeats;

                println!(
                    "[repeats: {repeats}] after {} moves, blocks will be {blocks} & height will be {height}",
                    c + repeats_every_c * repeats,
                );

                // [repeats: 28571428569] after 5714285714149 moves, blocks will be 999999999978 & height will be 1514285714258

                // so we need to simulate MAX - blocks more blocks

                height_when_noticed_loop = Some(highest);
                remaining_blocks = Some(MAX - blocks);
                height_differential = Some(height);

                // break 'outer;
            }

            seen.insert(val.clone(), (c, n_blocks, highest));

            println!("{val}");

            if !tet
                .iter()
                .any(|(x, y)| *y == 0 || grid.contains(&(*x, *y - 1)))
            {
                tet = tet.iter().map(|(x, y)| (*x, *y - 1)).collect();
            } else {
                break 'inner;
            }
        }

        n_blocks += 1;

        remaining_blocks = match remaining_blocks {
            Some(x) => Some(x - 1),
            None => remaining_blocks,
        };

        if remaining_blocks == Some(-1) {
            println!(
                "finished simulation! height_when_noticed_loop: {:?}  highest: {:?}  height_differential: {:?}",
                height_when_noticed_loop, highest, height_differential
            );
            return 0;
        }

        tet.iter().for_each(|cell| {
            grid.insert(*cell);
        });
    }

    highest_point(&grid) + 1
}

fn highest_point(grid: &HashSet<(i64, i64)>) -> i64 {
    *grid.iter().map(|(_, y)| y).max().unwrap_or(&-1)
}

fn landscape(grid: &HashSet<(i64, i64)>) -> String {
    let highest = highest_point(grid);
    (0..7)
        .map(|x| {
            for y in (0..highest).rev() {
                if grid.contains(&(x, y)) {
                    return (highest - y).to_string();
                }
            }
            return "_".to_string();
        })
        .intersperse(",".to_string())
        .collect()
}

fn main() {
    // println!("{}", one(include_str!("test17.txt")));
    println!("{}", one(include_str!("input17.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test17.txt")), 3068);
    }
}
