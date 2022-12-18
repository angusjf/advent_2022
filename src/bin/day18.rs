use itertools::*;
use std::collections::*;

fn parse(line: &str) -> (i64, i64, i64) {
    line.split(",")
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn print(cubes: &HashSet<(i64, i64, i64)>) {
    let mut cubes_2d: HashMap<_, _> = HashMap::new();
    for (x, y, _) in cubes {
        *cubes_2d.entry((*x, *y)).or_insert(0) += 1;
    }

    print_2d(&cubes_2d);

    let mut cubes_2d: HashMap<_, _> = HashMap::new();
    for (_, x, y) in cubes {
        *cubes_2d.entry((*x, *y)).or_insert(0) += 1;
    }

    print_2d(&cubes_2d);

    let mut cubes_2d: HashMap<_, _> = HashMap::new();
    for (x, _, y) in cubes {
        *cubes_2d.entry((*x, *y)).or_insert(0) += 1;
    }

    print_2d(&cubes_2d);
}

fn print_2d(cubes_2d: &HashMap<(i64, i64), i64>) {
    if cubes_2d.keys().len() == 0 {
        return;
    }
    let min_x = cubes_2d.keys().map(|(x, _)| x).min().unwrap();
    let max_x = cubes_2d.keys().map(|(x, _)| x).max().unwrap();
    let min_y = cubes_2d.keys().map(|(_, y)| y).min().unwrap();
    let max_y = cubes_2d.keys().map(|(_, y)| y).max().unwrap();

    let corner = cubes_2d.get(&(*min_x, *min_y)).unwrap();

    for y in *min_y..=*max_y {
        print!("{y:<3}|");
        for x in *min_x..=*max_x {
            match cubes_2d.get(&(x, y)) {
                Some(n) => {
                    if n == corner {
                        print!("  ");
                    } else {
                        print!("{0:>2}", corner - n)
                    }
                }
                None => print!("??"),
            }
        }
        println!("|");
    }

    println!();
}

fn flood_fill_with_limit(
    water_box: &mut HashSet<(i64, i64, i64)>,
    cubes: HashSet<(i64, i64, i64)>,
    (min_x, min_y, min_z): (i64, i64, i64),
    (max_x, max_y, max_z): (i64, i64, i64),
) {
    let mut queue: VecDeque<(i64, i64, i64)> = VecDeque::from([(min_x, min_y, min_z)]);
    while let Some(pos @ (x, y, z)) = queue.pop_back() {
        if (x >= min_x)
            && (y >= min_y)
            && (z >= min_z)
            && (x <= max_x)
            && (y <= max_y)
            && (z <= max_z)
        {
            if !water_box.contains(&pos) && !cubes.contains(&pos) {
                water_box.insert(pos);
                vec![
                    (0, 0, 1),
                    (0, 0, -1),
                    (0, 1, 0),
                    (0, -1, 0),
                    (1, 0, 0),
                    (1, 0, 0),
                ]
                .iter()
                .for_each(|(dx, dy, dz)| queue.push_front((x + dx, y + dy, z + dz)));
            }
        }
    }
    print(&water_box);
}

fn two(input: &str) -> i64 {
    let cubes: HashSet<_> = input.lines().map(parse).collect();

    let m = 1;
    let min_x = *cubes.iter().map(|(x, _, _)| x).min().unwrap() - m;
    let max_x = *cubes.iter().map(|(x, _, _)| x).max().unwrap() + m;
    let min_y = *cubes.iter().map(|(_, y, _)| y).min().unwrap() - m;
    let max_y = *cubes.iter().map(|(_, y, _)| y).max().unwrap() + m;
    let min_z = *cubes.iter().map(|(_, _, z)| z).min().unwrap() - m;
    let max_z = *cubes.iter().map(|(_, _, z)| z).max().unwrap() + m;

    let mut water_box = HashSet::new();

    flood_fill_with_limit(
        &mut water_box,
        cubes,
        (min_x, min_y, min_z),
        (max_x, max_y, max_z),
    );

    let water_total_faces: i64 = water_box
        .iter()
        .map(|(x, y, z)| {
            vec![
                (0, 0, 1),
                (0, 0, -1),
                (0, 1, 0),
                (0, -1, 0),
                (1, 0, 0),
                (-1, 0, 0),
            ]
            .iter()
            .map(|(dx, dy, dz)| {
                if !water_box.contains(&(x + dx, y + dy, z + dz)) {
                    1
                } else {
                    0
                }
            })
            .sum::<i64>()
        })
        .sum();

    let y_dim = 1 + max_y - min_y;
    let x_dim = 1 + max_y - min_y;
    let z_dim = 1 + max_z - min_z;

    let water_exterior_faces = 2 * (x_dim * y_dim) + 2 * (y_dim * z_dim) + 2 * (x_dim * z_dim);

    dbg!(water_exterior_faces, water_total_faces);

    water_total_faces - water_exterior_faces
}

fn main() {
    println!("{:?}", two(include_str!("test18.txt")));
    println!("{:?}", two(include_str!("input18.txt")));
}
