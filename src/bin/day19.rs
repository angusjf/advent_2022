use itertools::*;
use std::collections::*;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Price {
    Ore(u32),
    Clay(u32),
    Obsidian(u32),
}

use Price::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Target {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot,
}

use Target::*;

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    number: u32,
    ore_robot_cost: HashSet<Price>,
    clay_robot_cost: HashSet<Price>,
    obsidian_robot_cost: HashSet<Price>,
    geode_robot_cost: HashSet<Price>,
    max_ore_spend: u32,
    max_clay_spend: u32,
    max_obsidian_spend: u32,
}

fn parse_cost(input: &str) -> HashSet<Price> {
    let input = input.strip_prefix(" Each ").unwrap();
    let input = input.split(" ").collect::<Vec<_>>();
    input[3..]
        .iter()
        .filter(|word| **word != "and")
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let n = chunk.nth(0).unwrap();
            let n = n.parse().unwrap();
            match *chunk.nth(0).unwrap() {
                "ore" => Ore(n),
                "clay" => Clay(n),
                "obsidian" => Obsidian(n),
                _ => unimplemented!(),
            }
        })
        .collect()
}

fn parse(input: &str) -> Blueprint {
    let (blueprint_number, rest) = input.split(":").collect_tuple().unwrap();
    let (ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost) = rest
        .strip_suffix(".")
        .unwrap()
        .split(".")
        .map(|cost| parse_cost(cost))
        .collect_tuple()
        .unwrap();

    let robots = vec![
        &ore_robot_cost,
        &clay_robot_cost,
        &obsidian_robot_cost,
        &geode_robot_cost,
    ];

    let max_ore_spend = robots
        .iter()
        .map(|cost| {
            cost.iter()
                .map(|price| match price {
                    Ore(n) => *n,
                    _ => 0,
                })
                .sum()
        })
        .max()
        .unwrap();
    let max_clay_spend = robots
        .iter()
        .map(|cost| {
            cost.iter()
                .map(|price| match price {
                    Clay(n) => *n,
                    _ => 0,
                })
                .sum()
        })
        .max()
        .unwrap();
    let max_obsidian_spend = robots
        .iter()
        .map(|cost| {
            cost.iter()
                .map(|price| match price {
                    Obsidian(n) => *n,
                    _ => 0,
                })
                .sum()
        })
        .max()
        .unwrap();

    let blueprint_number = blueprint_number.strip_prefix("Blueprint ").unwrap();

    Blueprint {
        number: blueprint_number.parse().unwrap(),
        ore_robot_cost,
        clay_robot_cost,
        obsidian_robot_cost,
        geode_robot_cost,
        max_ore_spend,
        max_clay_spend,
        max_obsidian_spend,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    target: Target,
}

fn can_afford(state: State, price: &Price) -> bool {
    match price {
        Ore(n) => state.ore >= *n,
        Clay(n) => state.clay >= *n,
        Obsidian(n) => state.obsidian >= *n,
    }
}

fn pay(state: &mut State, price: &Price) {
    match price {
        Ore(n) => state.ore -= n,
        Clay(n) => state.clay -= *n,
        Obsidian(n) => state.obsidian -= *n,
    }
}

fn affordable(state: &State, price: &HashSet<Price>) -> bool {
    price.iter().all(|price| can_afford(state.clone(), price))
}

fn purchase_ore_robot(state: State, blueprint: &Blueprint) -> Option<State> {
    if state.obsidian_robots <= blueprint.max_ore_spend
        && affordable(&state, &blueprint.ore_robot_cost)
    {
        let mut new_state = state.clone();
        blueprint.ore_robot_cost.iter().for_each(|price| {
            pay(&mut new_state, &price);
        });
        new_state.ore_robots += 1;
        Some(new_state)
    } else {
        None
    }
}

fn purchase_clay_robot(state: State, blueprint: &Blueprint) -> Option<State> {
    if state.obsidian_robots <= blueprint.max_clay_spend
        && affordable(&state, &blueprint.clay_robot_cost)
    {
        let mut new_state = state.clone();
        blueprint.clay_robot_cost.iter().for_each(|price| {
            pay(&mut new_state, &price);
        });
        new_state.clay_robots += 1;
        Some(new_state)
    } else {
        None
    }
}

fn purchase_obsidian_robot(state: State, blueprint: &Blueprint) -> Option<State> {
    if state.obsidian_robots <= blueprint.max_obsidian_spend
        && affordable(&state, &blueprint.obsidian_robot_cost)
    {
        let mut new_state = state.clone();
        blueprint.obsidian_robot_cost.iter().for_each(|price| {
            pay(&mut new_state, &price);
        });
        new_state.obsidian_robots += 1;
        Some(new_state)
    } else {
        None
    }
}

fn purchase_geode_robot(state: State, blueprint: &Blueprint) -> Option<State> {
    if affordable(&state, &blueprint.geode_robot_cost) {
        let mut new_state = state.clone();
        blueprint.geode_robot_cost.iter().for_each(|price| {
            pay(&mut new_state, &price);
        });
        new_state.geode_robots += 1;
        Some(new_state)
    } else {
        None
    }
}

fn buy(state: State, blueprint: &Blueprint) -> Option<State> {
    match state.target {
        OreRobot => purchase_ore_robot(state, blueprint),
        ClayRobot => purchase_clay_robot(state, blueprint),
        ObsidianRobot => purchase_obsidian_robot(state, blueprint),
        GeodeRobot => purchase_geode_robot(state, blueprint),
    }
}

fn next_states(state: State, blueprint: &Blueprint) -> Vec<State> {
    let state = tick(state.clone());
    if let Some(state) = buy(state.clone(), blueprint) {
        vec![OreRobot, ClayRobot, ObsidianRobot, GeodeRobot]
            .iter()
            .map(|t| State {
                target: t.clone(),
                ..state
            })
            .collect()
    } else {
        vec![state]
    }
}

fn tick(state: State) -> State {
    State {
        ore: state.ore + state.ore_robots,
        clay: state.clay + state.clay_robots,
        obsidian: state.obsidian + state.obsidian_robots,
        geode: state.geode + state.geode_robots,
        ..state
    }
}

fn quality(blueprint: &Blueprint) -> u32 {
    let initial_state = State {
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        target: OreRobot,
    };

    let mut queue = VecDeque::from([(initial_state, 1)]);

    let mut max = 0;

    while let Some((state, mins)) = queue.pop_front() {
        dbg!(mins);
        if mins == 7 {
            dbg!(&state);
            if blueprint.number * state.geode > max {
                max = blueprint.number * state.geode;
            }
        } else {
            next_states(state, blueprint).iter().for_each(|state| {
                queue.push_back((state.clone(), mins + 1));
            });
        }
    }

    max
}

fn one(input: &str) -> u32 {
    input
        .lines()
        .map(parse)
        .take(1) // TODO
        .map(|blueprint| quality(&blueprint))
        .sum()
}

fn main() {
    println!("{:?}", one(include_str!("test19-1.txt")));
}
