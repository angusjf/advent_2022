use std::collections::*;

fn color(n: i64) -> char {
    '#'
    // match n % 4 {
    //     0 => '0',
    //     1 => '1',
    //     2 => '2',
    //     3 => '3',
    //     _ => '4',
    // }
}
const MAX: i64 = 10;
fn one(input: &str) -> i64 {
    let tet_flat = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let tet_plus = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
    let tet_j = vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
    let tet_l = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let tet_o = vec![(0, 0), (1, 0), (0, 1), (1, 1)];

    let tetronimos = vec![tet_flat, tet_plus, tet_j, tet_l, tet_o];

    let mut grid = HashMap::new();

    let mut tetronimos = tetronimos.iter().cycle();

    let mut x = 2;
    let mut y = 3;
    let mut tet = tetronimos.next().unwrap();

    let mut rocks = 0;

    for c in input.chars().cycle() {
        if MAX == 10 {
            print(x, y, tet, &grid);
        }
        {
            println!("{c}");
            if c == '<' {
                let possible = tet.iter().all(|(tet_x, tet_y)| {
                    let real_x = tet_x + x;
                    let real_y = tet_y + y;
                    if real_x == 0 {
                        return false;
                    }
                    !grid.contains_key(&(real_x - 1, real_y))
                });
                if possible {
                    x -= 1;
                } else {
                    println!("can't move {c}");
                }
            } else if c == '>' {
                let possible = tet.iter().all(|(tet_x, tet_y)| {
                    let real_x = tet_x + x;
                    let real_y = tet_y + y;
                    if real_x == 6 {
                        return false;
                    }
                    !grid.contains_key(&(real_x + 1, real_y))
                });
                if possible {
                    x += 1;
                } else {
                    println!("can't move {c}");
                }
            }
        }
        if MAX == 10 {
            print(x, y, tet, &grid);
        }
        {
            y -= 1;

            let hit = tet.iter().any(|(tet_x, tet_y)| {
                let real_x = tet_x + x;
                let real_y = tet_y + y;
                real_y == -1 || grid.contains_key(&(real_x, real_y))
            });

            if hit {
                tet.iter().for_each(|(tet_x, tet_y)| {
                    let real_x = tet_x + x;
                    let real_y = tet_y + y;
                    grid.insert((real_x, real_y + 1), color(rocks));
                });

                let highest_point = grid.keys().map(|(_, y)| y).max().unwrap();

                tet = tetronimos.next().unwrap();

                rocks += 1;
                println!("{rocks}");
                if rocks == MAX {
                    break;
                }

                x = 2;
                y = highest_point + 4;
            }
        }
    }

    y
}

fn print(x: i64, y: i64, tet: &Vec<(i64, i64)>, grid: &HashMap<(i64, i64), char>) {
    let tet: HashSet<_> = tet
        .iter()
        .map(|(tet_x, tet_y)| (x + tet_x, y + tet_y))
        .collect();

    println!("{x} {y} {:?}", tet);
    for y in (0..y + 5).rev() {
        print!("|");
        for x in 0..7 {
            if let Some(c) = grid.get(&(x, y)) {
                print!("{c}");
            } else if tet.contains(&(x, y)) {
                print!("@");
            } else {
                print!(" ");
            }
        }
        print!("|");
        println!();
    }
    println!("+-------+");
}

fn main() {
    println!("{}", one(include_str!("test17.txt")));
    // println!("{}", one(include_str!("input17.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test17.txt")), 1651);
    }
}
