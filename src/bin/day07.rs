use itertools::*;

#[derive(Debug)]
enum Cmd {
    Ls,
    Cd(String),
    Dir(String),
    File(u64, String),
}

use Cmd::*;

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    children: Vec<u32>,
    parent: Option<u32>,
    data: Option<u64>,
    id: u32,
}

fn one(input: &str) -> u64 {
    let cmds: Vec<_> = input
        .lines()
        .map(|line| {
            if line.starts_with("$ ls") {
                Ls
            } else if line.starts_with("$ cd ") {
                Cd(line[5..].to_string())
            } else if line.starts_with("dir ") {
                Dir(line[4..].to_string())
            } else {
                let (size, name) = line.split(" ").collect_tuple().unwrap();
                File(size.parse().unwrap(), name.to_string())
            }
        })
        .collect();

    let mut nodes = vec![];
    let mut fs = Node {
        name: "/",
        children: vec![],
        parent: None,
        data: None,
        id: 0,
    };
    nodes.push(fs);
    let mut next_id = 1;
    let mut cursor = 0;

    let mut i = 0;

    loop {
        if i >= cmds.len() - 1 {
            break;
        }
        match &cmds[i] {
            Cd(dir) => {
                if dir == "/" {
                    cursor = 0;
                } else if dir == ".." {
                    cursor = nodes
                        .iter()
                        .find(|n| n.id == cursor)
                        .unwrap()
                        .parent
                        .unwrap();
                } else {
                    let sub = Node {
                        name: dir,
                        children: vec![],
                        parent: Some(cursor),
                        data: None,
                        id: next_id,
                    };

                    nodes
                        .iter_mut()
                        .find(|n| n.id == cursor)
                        .unwrap()
                        .children
                        .push(next_id);

                    cursor = next_id;

                    next_id = next_id + 1;

                    nodes.push(sub);
                }
            }
            Ls => {}
            Dir(_) => {}
            File(size, name) => {
                let file = Node {
                    name,
                    children: vec![],
                    parent: Some(cursor),
                    data: Some(*size),
                    id: next_id,
                };
                nodes.push(file);
                nodes
                    .iter_mut()
                    .find(|n| n.id == cursor)
                    .unwrap()
                    .children
                    .push(next_id);
                next_id = next_id + 1;
            }
        }
        i = i + 1;
    }

    println!("{:?}", nodes);

    let mut folder_sizes = vec![];

    get_folder_sizes(&nodes, 0, &mut folder_sizes);

    print_tree(&nodes, 0, 0);

    println!("{:?}", folder_sizes);

    let total = 70000000;

    let used = file_size(&nodes, 0);

    let free = total - used;

    let needed = 30000000 - free;

    *folder_sizes
        .iter()
        .filter(|size| **size >= needed)
        .min()
        .unwrap()
}

fn print_tree(nodes: &Vec<Node>, id: u32, indent: usize) {
    let node = nodes.iter().find(|x| x.id == id).unwrap();
    match node.data {
        Some(data) => {
            println!(
                "{}- {} ({}) [{}]",
                " ".repeat(indent * 4),
                node.name,
                data,
                id,
            )
        }
        _ => {
            println!("{}- {} (dir) [{}]", " ".repeat(indent * 4), node.name, id,);
            node.children
                .iter()
                .for_each(|child| print_tree(&nodes, *child, indent + 1));
        }
    }
}

fn get_folder_sizes(nodes: &Vec<Node>, id: u32, folder_sizes: &mut Vec<u64>) {
    nodes[id as usize]
        .children
        .iter()
        .for_each(|child| get_folder_sizes(nodes, *child, folder_sizes));

    if nodes[id as usize].data.is_none() {
        let size = file_size(&nodes, id);

        folder_sizes.push(size);
    }
}

fn part_one(nodes: &Vec<Node>, id: u32) -> u64 {
    let node = nodes.iter().find(|x| x.id == id).unwrap();
    node.children
        .iter()
        .map(|child| part_one(nodes, *child))
        .filter(|size| *size <= 100_000)
        .sum::<u64>()
        + node.data.unwrap_or_default()
}

fn file_size(nodes: &Vec<Node>, id: u32) -> u64 {
    let node = nodes.iter().find(|x| x.id == id).unwrap();
    match node.data {
        None => {
            let sum = node
                .children
                .iter()
                .map(|child| file_size(&nodes, *child))
                .sum();
            sum
        }
        Some(data) => data,
    }
}

fn two(input: &str) -> u64 {
    0
}

fn main() {
    // println!("{:?}", one(include_str!("test07.txt")));
    println!("{:?}", one(include_str!("input07.txt")));
    // println!("{:?}", two(include_str!("input07.txt")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        // assert_eq!(crate::one(include_str!("test07.txt")), vec![]);
    }
}
