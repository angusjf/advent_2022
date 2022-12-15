use itertools::*;

fn parse(line: &str) -> ((i64, i64), (i64, i64)) {
    // "Sensor at x"
    // "9, y"
    // "16: closest beacon is at x"
    // "10, y"
    // "16"
    let (sensor_x, sensor_y, beacon_x, beacon_y) = line.split('=').skip(1).collect_tuple().unwrap();
    let sensor_x = sensor_x.split(",").nth(0).unwrap().parse().unwrap();
    let sensor_y = sensor_y.split(":").nth(0).unwrap().parse().unwrap();
    let beacon_x = beacon_x.split(",").nth(0).unwrap().parse().unwrap();
    let beacon_y = beacon_y.parse().unwrap();
    ((sensor_x, sensor_y), (beacon_x, beacon_y))
}

fn one(input: &str) -> i64 {
    let sensors: Vec<_> = input
        .lines()
        .map(parse)
        .map(|((x, y), (beacon_x, beacon_y))| {
            ((x, y), (beacon_x, beacon_y), (beacon_x - x).abs() + (beacon_y - y).abs())
        }).collect();

    let min_x = sensors.iter().map(|((x, _), _, r)| x - r).min().unwrap();
    let max_x = sensors.iter().map(|((x, _), _, r)| x + r).max().unwrap();

    let mut num_positions_that_cannot_contain_a_beacon = 0;

    let y = 2000000;

    for x in min_x..max_x {
        if sensors.iter().any(|(_, beacon, _)| &(x, y) == beacon) {
            // position contains a beacon
        } else if sensors.iter().all(
            |sensor| distance_between_sensor_and_position_is_far_enough_away_that_it_could_contain_a_beacon(sensor, &(x, y))
        ) {
            // position could be a beacon
        } else {
            // position cannot possibly contain a beacon
            num_positions_that_cannot_contain_a_beacon += 1;
        }
    }

    num_positions_that_cannot_contain_a_beacon
}

fn distance_between_sensor_and_position_is_far_enough_away_that_it_could_contain_a_beacon(((sensor_x, sensor_y), _, r): &((i64, i64), (i64, i64), i64), (x, y): &(i64, i64)) -> bool {
    // dbg!((sensor_x - x).abs() , (sensor_y - y).abs(), *r);
    let dist = (sensor_x - x).abs() + (sensor_y - y).abs();
    dist > *r
}

fn two(input: &str) -> i64 {
    let sensors: Vec<_> = input
        .lines()
        .map(parse)
        .map(|((x, y), (beacon_x, beacon_y))| {
            ((x, y), (beacon_x, beacon_y), (beacon_x - x).abs() + (beacon_y - y).abs())
        }).collect();

    if max == 20 {
        print(&sensors);
    }

    for y in 0..=max {
        let ranges: Vec<_> = sensors.iter().map(
            |((sensor_x, sensor_y), _beacon, r)| {
                let y_diff = (sensor_y - y).abs();
                let x_diff = r - y_diff;
                // can't be in this range
                (sensor_x - x_diff, sensor_x + x_diff)
            }
        ).collect();

        let mut x = 0;

        while x <= max {
            if let Some((_min_x, max_x)) = ranges.iter().find(|(min_x, max_x)| x >= *min_x && x <= *max_x) {
                println!(" -> {max_x}");
                x = *max_x + 1;
            } else {
                println!("{x} {y}");
                return x * 4000000 + y;
            }
        }
        println!("{y}");
    }

    
    unreachable!();
}

fn print(sensors: &Vec<((i64, i64), (i64, i64), i64)>) {
    for full in sensors {
        let (sensor, beacon, _) = full;
        for y in 0..=max {
            for x in 0..=max {
                if &(x, y) == sensor {
                    // position contains a beacon
                    print!("S");
                } else if &(x, y) == beacon {
                    // position contains a beacon
                    print!("B");
                } else if distance_between_sensor_and_position_is_far_enough_away_that_it_could_contain_a_beacon(full, &(x, y)) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
        println!();
        println!("    o o o ");
    }
    println!();

    for y in 0..=max {
        for x in 0..=max {
            if sensors.iter().any(|(sensor, _, _)| &(x, y) == sensor) {
                // position contains a beacon
                print!("S");
            } else if sensors.iter().any(|(_, beacon, _)| &(x, y) == beacon) {
                // position contains a beacon
                print!("B");
            } else if let Some(_sensor) = sensors.iter().find(
                |sensor| !distance_between_sensor_and_position_is_far_enough_away_that_it_could_contain_a_beacon(sensor, &(x, y))
            ) {
                print!("#");
            } else {
                // all sensors are far enough away
                print!(".");
            }
        }
        println!();
    }
    println!();
}

// const max: i64 = 20;
const max: i64 = 4000000;

fn main() {
    println!("{}", two(include_str!("input15.txt")));
    // println!("{}", two(include_str!("test15.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert_eq!(crate::one(include_str!("test15.txt")), 26);
    }

    #[test]
    fn two() {
        assert_eq!(crate::two(include_str!("test15.txt")), 0);
    }
}
