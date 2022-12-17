use std::collections::*;

const MAX: i64 = 2022;

fn one(input: &str) -> i64 {
    let tet_flat = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let tet_plus = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
    let tet_j = vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
    let tet_l = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let tet_o = vec![(0, 0), (1, 0), (0, 1), (1, 1)];

    let tetronimos = vec![tet_flat, tet_plus, tet_j, tet_l, tet_o];

    let mut grid: HashSet<(i64, i64)> = HashSet::new();

    let mut chars = input.lines().nth(0).unwrap().chars().cycle();

    for tet in tetronimos.iter().cycle().take(MAX as usize) {
        let highest = highest_point(&grid);
        let mut tet: Vec<_> = tet
            .iter()
            .map(|(x, y)| (*x + 2, *y + highest + 4))
            .collect();

        'a: loop {
            // print(&tet, &grid);

            match chars.next().unwrap() {
                '<' => {
                    if !tet
                        .iter()
                        .any(|(x, y)| *x == 0 || grid.contains(&(*x - 1, *y)))
                    {
                        tet = tet.iter().map(|(x, y)| (*x - 1, *y)).collect();
                        // println!(" <-");
                    } else {
                        // println!(" |-");
                    }
                }

                '>' => {
                    if !tet
                        .iter()
                        .any(|(x, y)| *x == 6 || grid.contains(&(*x + 1, *y)))
                    {
                        // println!(" ->");
                        tet = tet.iter().map(|(x, y)| (*x + 1, *y)).collect();
                    } else {
                        // println!(" -|");
                    }
                }

                _ => unreachable!(),
            }

            // print(&tet, &grid);

            if !tet
                .iter()
                .any(|(x, y)| *y == 0 || grid.contains(&(*x, *y - 1)))
            {
                tet = tet.iter().map(|(x, y)| (*x, *y - 1)).collect();

                // println!(" |");
                // println!(" v");
            } else {
                // println!(" |");
                // println!(" –");

                break 'a;
            }
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

fn print(tet: &Vec<(i64, i64)>, grid: &HashSet<(i64, i64)>) {
    let highest_point = highest_point(grid);
    for y in (0..highest_point + 8).rev() {
        print!("{:<4}|", y);
        for x in 0..7 {
            if grid.contains(&(x, y)) {
                print!("#");
            } else if tet.contains(&(x, y)) {
                print!("@");
            } else {
                print!(" ");
            }
        }
        print!("|");
        println!();
    }
    println!("    +-------+");
    println!();
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
