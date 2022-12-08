use std::fs::File;
use std::io::BufRead;
use std::{collections::BTreeSet, convert::TryFrom};

use anyhow::{bail, Error, Result};
use indextree::{Arena, NodeId};

struct ElfSystem {
    tree: Arena<usize>,
    size: usize,
    root: NodeId,
}

impl Default for ElfSystem {
    fn default() -> Self {
        let mut tree = Arena::default();
        let size = usize::default();
        let root = tree.new_node(0usize);

        Self { tree, size, root }
    }
}

enum Cmd {
    Cd(String),
    Ls,
}

impl TryFrom<String> for Cmd {
    type Error = Error;

    fn try_from(cmd_string: String) -> Result<Self, Self::Error> {
        let tokens = cmd_string.split(' ').collect::<Vec<_>>();

        let cmd = match tokens[0] {
            "cd" => Cmd::Cd(tokens[1].into()),
            "ls" => Cmd::Ls,
            _ => {
                bail!("Invalid command")
            }
        };

        Ok(cmd)
    }
}

pub fn day_7() -> Result<()> {
    day_7_1()?;
    day_7_2()
}

// 2061777
fn day_7_1() -> Result<()> {
    let es = build_fs_tree()?;

    let mut size_stack = Vec::new();
    let mut cur_size = 0usize;
    let mut total_size = 0usize;
    for edge in es.root.traverse(&es.tree) {
        match edge {
            indextree::NodeEdge::Start(node_id) => {
                size_stack.push(cur_size);

                let node = es.tree.get(node_id).unwrap();
                cur_size = *node.get();
            }
            indextree::NodeEdge::End(_) => {
                if cur_size <= 100000 {
                    total_size += cur_size;
                }

                cur_size += size_stack.pop().unwrap();
            }
        }
    }

    println!("Day 7-1: {}", total_size);

    Ok(())
}

fn day_7_2() -> Result<()> {
    const FS_SIZE: usize = 70_000_000;
    const MIN_SIZE: usize = 30_000_000;

    let es = build_fs_tree()?;

    let size_needed = MIN_SIZE - (FS_SIZE - es.size);
    let mut size_stack = Vec::new();
    let mut cur_size = 0usize;
    let mut size_set = BTreeSet::new();
    for edge in es.root.traverse(&es.tree) {
        match edge {
            indextree::NodeEdge::Start(node_id) => {
                size_stack.push(cur_size);

                let node = es.tree.get(node_id).unwrap();
                cur_size = *node.get();
            }
            indextree::NodeEdge::End(_) => {
                if cur_size >= size_needed {
                    size_set.insert(cur_size);
                }

                cur_size += size_stack.pop().unwrap();
            }
        }
    }

    let smallest_dir = size_set.iter().next().unwrap();
    println!("Day 7-1: {}", smallest_dir);

    Ok(())
}

fn build_fs_tree() -> Result<ElfSystem> {
    let file = File::open("input/day_7.txt")?;

    let mut es = ElfSystem::default();
    let mut lines = std::io::BufReader::new(file).lines();

    // Skip the first line and create root node
    let _ = lines.next().unwrap();
    let mut cur_dir = vec!["/".to_string()];

    let mut cur_node = es.root;
    for line in lines {
        let line = line?;
        let node = es.tree.get_mut(cur_node).unwrap();

        if line.starts_with('$') {
            let cmd = Cmd::try_from(line[2..].to_string())?;
            match cmd {
                Cmd::Cd(path) => {
                    if path.as_str() == ".." {
                        cur_node = node.parent().unwrap();
                        cur_dir.pop();
                    } else {
                        cur_dir.push(path);
                        let new_node = es.tree.new_node(0usize);
                        cur_node.append(new_node, &mut es.tree);
                        cur_node = new_node;
                    }
                }
                Cmd::Ls => {}
            }

            continue;
        }

        let tokens = line.split(' ').collect::<Vec<_>>();
        if let Ok(size) = tokens[0].parse::<usize>() {
            let node_size = node.get_mut();

            es.size += size;
            *node_size += size;
        }
    }

    Ok(es)
}
