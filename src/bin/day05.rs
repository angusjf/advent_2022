use itertools::*;

fn one(input: &str) -> String {
    let line_len = input.clone().lines().next().unwrap().len();
    let n = (line_len + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(n);
    for _ in 0..n {
        stacks.push(vec![]);
    }
    dbg!(&stacks);
    for line in input.lines() {
        dbg!(&line);
        let s = line.chars().nth(1).unwrap();
        if s.is_digit(10) {
            break;
        }

        for i in (1..line.len()).step_by(4) {
            let c = line.chars().nth(i).unwrap();
            if c != ' ' {
                stacks[(i - 1) / 4].push(c);
            }
        }
    }
    for i in 0..n {
        stacks[i].reverse();
    }
    dbg!(&stacks);
    input
        .lines()
        .filter(|line| line.starts_with("move"))
        .for_each(|line| {
            let (_move, n, _from, a, _to, b) = line.split(" ").collect_tuple().unwrap();
            let (n, a, b): (u32, u32, u32) =
                (n.parse().unwrap(), a.parse().unwrap(), b.parse().unwrap());
            for _ in 0..n {
                let c = stacks[a as usize - 1].pop().unwrap();
                stacks[b as usize - 1].push(c);
            }
        });
    dbg!(&stacks);

    stacks
        .iter()
        .map(|stack| stack.iter().last().unwrap())
        .collect()
}

fn two(input: &str) -> String {
    let line_len = input.clone().lines().next().unwrap().len();
    let n = (line_len + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(n);
    for _ in 0..n {
        stacks.push(vec![]);
    }
    dbg!(&stacks);
    for line in input.lines() {
        dbg!(&line);
        let s = line.chars().nth(1).unwrap();
        if s.is_digit(10) {
            break;
        }

        for i in (1..line.len()).step_by(4) {
            let c = line.chars().nth(i).unwrap();
            if c != ' ' {
                stacks[(i - 1) / 4].push(c);
            }
        }
    }
    for i in 0..n {
        stacks[i].reverse();
    }
    dbg!(&stacks);
    input
        .lines()
        .filter(|line| line.starts_with("move"))
        .for_each(|line| {
            let (_move, n, _from, a, _to, b) = line.split(" ").collect_tuple().unwrap();
            let (n, a, b): (u32, u32, u32) =
                (n.parse().unwrap(), a.parse().unwrap(), b.parse().unwrap());
            let mut xs = vec![];
            for _ in 0..n {
                let c = stacks[a as usize - 1].pop().unwrap();
                xs.push(c);
            }
            for _ in 0..n {
                let c = xs.pop().unwrap();
                stacks[b as usize - 1].push(c);
            }
        });
    dbg!(&stacks);

    stacks
        .iter()
        .map(|stack| stack.iter().last().unwrap())
        .collect()
}

fn main() {
    println!("{}", one(include_str!("input05.txt")));
    println!("{}", two(include_str!("input05.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test05.txt")), "CMZ");
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test05.txt")), "MCD");
    }
}
