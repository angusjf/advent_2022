#![feature(iter_intersperse)]
// use std::collections::*;

fn one(input: &str) -> i64 {
    let input: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let l = input.len() as i64;

    let mut output: Vec<_> = input.iter().enumerate().collect();

    println!("Initial arrangement:");

    for (i, n) in input.iter().enumerate() {
        println!(
            "{}",
            output
                .clone()
                .iter()
                .map(|(_, x)| format!("{x}"))
                .intersperse(",".to_string())
                .collect::<String>()
        );
        println!();

        let (found_at_index, x) = output
            .iter()
            .enumerate()
            .find(|(_, (j, _))| i == *j)
            .map(|(found_at_index, x)| (found_at_index, *x))
            .unwrap()
            .clone();

        output.retain(|(j, _)| i != *j);

        let new_i = (found_at_index as i64) + n;

        let new_i = if new_i < 0 {
            l + (new_i % l) - 1
        } else if new_i >= l {
            (new_i + 1) % l
        } else {
            new_i
        } as usize;

        println!(
            "{} moves between _ and _:",
            x.1,
            // output[new_i - 1].1,
            // output[new_i].1
        );
        if new_i == 0 {
            output.push(x);
        } else {
            output.insert(new_i, x);
        }
    }
    println!(
        "{}",
        output
            .clone()
            .iter()
            .map(|(_, x)| format!("{x}"))
            .intersperse(",".to_string())
            .collect::<String>()
    );

    let zero_index = output.iter().position(|(_, x)| **x == 0).unwrap();

    let one_thousand_after_zero = output[(zero_index + 1000) % output.len()].1;
    dbg!(one_thousand_after_zero);
    let two_thousand_after_zero = output[(zero_index + 2000) % output.len()].1;
    dbg!(two_thousand_after_zero);
    let three_thousand_after_zero = output[(zero_index + 3000) % output.len()].1;
    dbg!(three_thousand_after_zero);

    one_thousand_after_zero + two_thousand_after_zero + three_thousand_after_zero
}

fn main() {
    // println!("{}", one(include_str!("input20.txt")));

    println!("{}", one("0\n0\n-8\n0\n0"));
    // println!("{}", one("1\n2\n-2\n3\n-2\n0\n4"));
    // println!("{}", one("1\n2\n-3\n3\n-2\n0\n4"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test20.txt")), 3);
    }
}
