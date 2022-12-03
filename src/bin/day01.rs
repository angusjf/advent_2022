fn one(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}

fn two(input: &str) -> u32 {
    let mut sums: Vec<_> = input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect();
    sums.sort_unstable();
    sums.iter().rev().take(3).sum()
}

fn main() {
    println!("{}", one(include_str!("input01.txt")));
    println!("{}", two(include_str!("input01.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test01.txt")), 24000);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test01.txt")), 45000);
    }
}
