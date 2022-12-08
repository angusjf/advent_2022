use std::collections::*;

fn one(input: &str) -> u64 {
    let mut input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut seen = HashSet::new();

    let mut count = 0;

    println!("LEFT");
    for y in 1..input.len() - 1 {
        let mut max = input[y][0];
        for x in 1..input[0].len() - 1 {
            let tree = input[y][x];
            if tree > max {
                if !seen.contains(&(y, x)) {
                    count += 1;
                    print!("{}", input[y][x]);
                }
                max = tree;
                seen.insert((y, x));
            }
            print!("_");
        }
        println!(".");
    }
    println!("{:?}", seen);

    println!("RIGHT");
    for y in 1..input.len() - 1 {
        let mut max = input[y][input[0].len() - 1];
        for x in (1..input[0].len() - 1).rev() {
            let tree = input[y][x];
            if tree > max {
                if !seen.contains(&(y, x)) {
                    print!("{}", input[y][x]);
                    count += 1;
                }
                max = tree;
                seen.insert((y, x));
            }
            print!("_");
        }
        println!(".");
    }
    println!("{:?}", seen);

    println!("TOP");
    for x in 1..input[0].len() - 1 {
        let mut max = input[input.len() - 1][x];
        for y in (1..input.len() - 1).rev() {
            let tree = input[y][x];
            // println!("   {:?}", (y, x));
            if tree > max {
                if !seen.contains(&(y, x)) {
                    print!("{}", input[y][x]);
                    // print!("{:?} > {:?} = {:?}", input[y][x], max, input[y][x] > max);
                    count += 1;
                }
                max = tree;
                seen.insert((y, x));
            }
            print!("_");
        }
        println!(".");
    }
    println!("{:?}", seen);

    println!("BOTTOM");
    for x in 1..input[0].len() - 1 {
        let mut max = input[0][x];
        for y in 1..input.len() - 1 {
            let tree = input[y][x];
            if tree > max {
                if !seen.contains(&(y, x)) {
                    print!("{} ({})", input[y][x], max);
                    count += 1;
                }
                max = tree;
                seen.insert((y, x));
            }
            print!("_");
        }
        println!(".");
    }
    println!("{:?}", seen);

    count + (input.len() as u64 * 2) + ((input[0].len() - 2) as u64 * 2)
}

fn two(input: &str) -> u64 {
    let mut input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut max_score = 0;
    for me_y in 1..input.len() - 1 {
        for me_x in 1..input[0].len() - 1 {
            println!("[{}, {}]", me_x, me_y);
            let me = input[me_y][me_x];
            let mut right = 0;
            for x in (me_x + 1)..=input[0].len() - 1 {
                right += 1;
                let tree = input[me_y][x];
                println!("r   [{}, {}] = {}", x, me_y, tree);
                if tree >= me {
                    break;
                }
            }
            let mut left = 0;
            for x in (0..me_x).rev() {
                left += 1;
                let tree = input[me_y][x];
                println!("l   [{}, {}] = {}", x, me_y, tree);
                if tree >= me {
                    break;
                }
            }
            let mut down = 0;
            for y in (me_y + 1)..=input.len() - 1 {
                down += 1;
                let tree = input[y][me_x];
                println!("d   [{}, {}] = {}", me_x, y, tree);
                if tree >= me {
                    break;
                }
            }
            let mut up = 0;
            for y in (0..me_y).rev() {
                up += 1;
                let tree = input[y][me_x];
                println!("u   [{}, {}] = {}", me_x, y, tree);
                if tree >= me {
                    break;
                }
            }
            let score = left * right * up * down;
            println!(" => {}*{}*{}*{} = {}", left, right, up, down, score);
            if score > max_score {
                max_score = score
            }
        }
    }
    max_score
}

fn main() {
    println!("{:?}", two(include_str!("input08.txt")));
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn one() {
    //     assert_eq!(crate::one(include_str!("test08.txt")), 95437);
    // }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test08.txt")), 24933642);
    }
}
