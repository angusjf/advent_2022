use itertools::*;

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: (char, String),
    test: (u128, usize, usize),
}

fn parse(lines: &str) -> Monkey {
    let (_, starting_items, operation, test, tru, fal) = lines.lines().collect_tuple().unwrap();

    let starting_items = starting_items.replace(",", "");
    let starting_items = starting_items
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();

    let (_, _, _operation, _new, _eq, _old_, opcode, operand) =
        operation.split(' ').collect_tuple().unwrap();

    let test = test
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .last()
        .unwrap();
    let tru = tru
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .last()
        .unwrap();
    let fal = fal
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .last()
        .unwrap();

    Monkey {
        items: starting_items,
        operation: (opcode.chars().nth(0).unwrap(), operand.to_string()),
        test: (test, tru, fal),
    }
}

fn one(input: &str) -> i64 {
    let mut monkeys: Vec<_> = input.split("\n\n").map(parse).collect();
    monkeys.iter().enumerate().for_each(|(i, monkey)| {
        println!(
            "Monkey {i}: {}",
            monkey.items.iter().map(|x| x.to_string()).join(", ")
        );
    });
    let mut inspections = vec![0; monkeys.len()];
    for round in 1..=20 {
        println!("--------------");
        for i in 0..monkeys.len() {
            let op = monkeys[i].operation.0;
            let val = monkeys[i].operation.1.clone();

            let test = monkeys[i].test.0;

            monkeys[i].items.iter_mut().for_each(|item| {
                inspections[i] += 1;
                *item = (match (op, &val) {
                    ('*', n) => *item * n.parse().unwrap_or(*item),
                    ('+', n) => *item + n.parse().unwrap_or(*item),
                    _ => unreachable!(),
                }) / 3;
            });

            let tru = monkeys[i].test.1;
            let fal = monkeys[i].test.2;

            let next: Vec<_> = monkeys[i]
                .items
                .iter()
                .map(|item| {
                    if *item % test == 0 {
                        (item.clone(), tru)
                    } else {
                        (item.clone(), fal)
                    }
                })
                .collect();
            monkeys[i].items.clear();
            next.iter().for_each(|(item, dest)| {
                monkeys[*dest as usize].items.push(*item);
            });
        }
        println!("Round {round} complete:");
        monkeys.iter().enumerate().for_each(|(i, monkey)| {
            println!(
                "Monkey {i}: {}",
                monkey.items.iter().map(|x| x.to_string()).join(", ")
            );
        });
    }
    inspections.sort();
    let monkey_business = inspections.iter().rev().take(2).product();
    monkey_business
}

fn two(input: &str) -> u128 {
    let mut monkeys: Vec<_> = input.split("\n\n").map(parse).collect();
    let mut inspections = vec![0; monkeys.len()];
    let total_test: u128 = monkeys.iter().map(|m| m.test.0).product();
    for _ in 1..=10000 {
        for i in 0..monkeys.len() {
            let op = monkeys[i].operation.0;
            let val = monkeys[i].operation.1.clone();

            let test = monkeys[i].test.0;

            monkeys[i].items.iter_mut().for_each(|item| {
                inspections[i] += 1;
                *item = match (op, &val) {
                    ('*', n) => *item * n.parse().unwrap_or(*item),
                    ('+', n) => *item + n.parse().unwrap_or(*item),
                    _ => unreachable!(),
                };
            });

            let tru = monkeys[i].test.1;
            let fal = monkeys[i].test.2;

            let next: Vec<_> = monkeys[i]
                .items
                .iter()
                .map(|item| {
                    if *item % test == 0 {
                        (item.clone(), tru)
                    } else {
                        (item.clone(), fal)
                    }
                })
                .collect();
            monkeys[i].items.clear();
            next.iter().for_each(|(item, dest)| {
                monkeys[*dest as usize].items.push(*item % total_test);
            });
        }
    }
    inspections.sort();
    let monkey_business = inspections.iter().rev().take(2).product();
    monkey_business
}

fn main() {
    println!("{}", one(include_str!("input11.txt")));
    println!("{}", two(include_str!("input11.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test11.txt")), 10605);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test11.txt")), 10605);
    }

    #[test]
    fn one_real() {
        assert_eq!(crate::one(include_str!("input11.txt")), 112815);
    }

    #[test]
    fn two_real() {
        assert_eq!(crate::two(include_str!("input11.txt")), 112815);
    }
}
