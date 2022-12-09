use std::collections::HashSet;

fn keep_up((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    let dx = hx - tx;
    let dy = hy - ty;
    let touching = dx.abs() < 2 && dy.abs() < 2;

    // If the head is ever two steps directly
    // up, down, left, or right from the tail,
    // the tail must also move one step in that direction
    // so it remains close enough:
    if !touching {
        let same_row = hx == tx;
        let same_column = hy == ty;

        if same_row {
            if dy > 1 {
                (tx, ty + 1)
            } else if dy < -1 {
                (tx, ty - 1)
            } else {
                (tx, ty)
            }
        } else if same_column {
            if dx > 1 {
                (tx + 1, ty)
            } else if dx < -1 {
                (tx - 1, ty)
            } else {
                (tx, ty)
            }
        } else {
            // Otherwise, if the head and tail aren't touching
            // and aren't in the same row or column,
            // the tail always moves one step diagonally to keep up:
            if dy > 0 {
                if dx > 0 {
                    (tx + 1, ty + 1)
                } else if dx < 0 {
                    (tx - 1, ty + 1)
                } else {
                    unreachable!();
                }
            } else if dy < 0 {
                if dx > 0 {
                    (tx + 1, ty - 1)
                } else if dx < 0 {
                    (tx - 1, ty - 1)
                } else {
                    unreachable!();
                }
            } else {
                dbg!(dx, dy);
                unreachable!();
            }
        }
    } else {
        (tx, ty)
    }
}

fn solve(input: &str, n_knots: usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knots = vec![(0, 0); n_knots];
    visited.insert((0, 0));
    input
        .lines()
        .map(|line| {
            let c = line.chars().nth(0).unwrap();
            let n: i32 = line[2..].parse().unwrap();
            (c, n)
        })
        .for_each(|(c, n)| {
            for _ in 0..n {
                match c {
                    'U' => {
                        knots[0].1 -= 1;
                    }
                    'L' => {
                        knots[0].0 -= 1;
                    }
                    'D' => {
                        knots[0].1 += 1;
                    }
                    'R' => {
                        knots[0].0 += 1;
                    }
                    _ => unimplemented!(),
                }

                for i in 0..(n_knots - 1) {
                    let head = knots[i];
                    let tail = knots[i + 1];
                    let new = keep_up(head, tail);
                    knots[i + 1].0 = new.0;
                    knots[i + 1].1 = new.1;
                }
                visited.insert(knots[n_knots - 1]);
            }
        });
    visited.len()
}

fn one(input: &str) -> usize {
    solve(input, 2)
}

fn two(input: &str) -> usize {
    solve(input, 10)
}

fn main() {
    println!("{:?}", one(include_str!("input09.txt")));
    println!("{:?}", two(include_str!("input09.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("input09.txt")), 5695);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("input09.txt")), 2434);
    }
}
