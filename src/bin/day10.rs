use itertools::*;

fn one(input: &str) -> i64 {
    let mut cmds = input.lines();

    let mut signal_strengths = vec![1, 1];

    while let Some(cmd) = cmds.next() {
        let strength = *signal_strengths.last().unwrap();

        match cmd {
            "noop" => {
                signal_strengths.push(strength);
            }
            _ => {
                let n: i64 = cmd.split(" ").nth(1).unwrap().parse().unwrap();
                signal_strengths.push(strength);
                signal_strengths.push(strength + n);
            }
        }
    }
    dbg!(&signal_strengths);

    signal_strengths
        .iter()
        .enumerate()
        .filter(|(i, _)| (*i as i64 - 20) % 40 == 0)
        .map(|x| dbg!(x))
        .take(6)
        .map(|six| dbg!(six))
        .map(|(i, x)| x)
        .map(|x| dbg!(x))
        .sum()
}

fn two(input: &str) -> u32 {
    let mut cmds = input.lines();

    let mut signal_strengths = vec![1];

    while let Some(cmd) = cmds.next() {
        let strength = *signal_strengths.last().unwrap();

        match cmd {
            "noop" => {
                signal_strengths.push(strength);
            }
            _ => {
                let n: i64 = cmd.split(" ").nth(1).unwrap().parse().unwrap();
                signal_strengths.push(strength);
                signal_strengths.push(strength + n);
            }
        }
    }
    dbg!(&signal_strengths);

    signal_strengths
        .iter()
        .chunks(40)
        .into_iter()
        .map(|chunk| {
            chunk
                .into_iter()
                .enumerate()
                .map(|(i, x)| i as i64 == *x || i as i64 == *x - 1 || i as i64 == *x + 1)
        })
        .for_each(|row| {
            println!(
                "{:?}",
                row.map(|x| if x { '#' } else { '.' }).collect::<String>()
            );
        });
    0
}

fn main() {
    println!("{}", one(include_str!("input10.txt")));
    println!("{}", two(include_str!("input10.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one_tiny() {
        assert_eq!(crate::one(include_str!("test10-1.txt")), 24000);
    }

    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test10.txt")), 24000);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test10.txt")), 45000);
    }
}
