use itertools::*;

fn solve(input: &str, l: usize) -> usize {
    &input
        .as_bytes()
        .windows(l)
        .position(|window| window.iter().all_unique())
        .unwrap()
        + l
}

fn one(input: &str) -> usize {
    solve(input, 4)
}

fn two(input: &str) -> usize {
    solve(input, 14)
}

fn main() {
    println!("{}", one(include_str!("input06.txt")));
    println!("{}", two(include_str!("input06.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(crate::one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(crate::one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(crate::one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(crate::two("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(crate::two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(crate::two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
