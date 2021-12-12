use std::collections::{HashMap, HashSet};

use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;
use self_cell::self_cell;

// https://adventofcode.com/2021/day/12

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

self_cell!(
    pub struct Map {
        owner: String,
        #[covariant]
        dependent: Graph,
    }
    impl {Debug}
);

pub fn read_input(args: &crate::File) -> Result<Map> {
    let file = read_file(&args.file)?;
    let map: Map = Map::new(file, |file| {
        let mut graph = HashMap::new();
        for line in file.split_ascii_whitespace() {
            if let Some((a, b)) = line.split('-').collect_tuple() {
                let (a, b) = (a.trim(), b.trim());
                // insert target edges except those pointing back to start
                if b != "start" && a != "end" {
                    graph.entry(a).or_insert_with(HashSet::new).insert(b);
                }
                if a != "start" && b != "end" {
                    graph.entry(b).or_insert_with(HashSet::new).insert(a);
                }
            }
        }
        graph
    });
    Ok(map)
}

fn is_repeatable(node: &str) -> bool {
    node.chars().any(|c| c.is_ascii_uppercase())
}

fn advance<'a>(
    map: &'a Graph,
    node: &str,
    visited: &mut HashSet<&'a str>,
    long_path: bool,
    _path: &str,
) -> (usize, usize) {
    let mut r1 = 0;
    let mut r2 = 0;

    if let Some(targets) = map.get(node) {
        for &target in targets.iter() {
            if target == "end" {
                // println!("{},end {}", _path, long_path);
                if long_path {
                    r2 += 1;
                } else {
                    r1 += 1;
                }
            } else if is_repeatable(target) {
                let r = advance(
                    map, target, visited, long_path,
                    "", //format!("{},{}", _path, target).as_str(),
                );
                r1 += r.0;
                r2 += r.1;
            } else {
                let is_visited = visited.contains(target);
                if !long_path || !is_visited {
                    if !is_visited {
                        visited.insert(target);
                    }
                    let r = advance(
                        map,
                        target,
                        visited,
                        long_path || is_visited,
                        "", //format!("{},{}", _path, target).as_str(),
                    );
                    if !is_visited {
                        visited.remove(target);
                    }
                    r1 += r.0;
                    r2 += r.1;
                }
            }
        }
    }
    (r1, r2)
}

pub fn run(map: &Graph) -> (usize, usize) {
    println!("map contains {} nodes: {:?}", map.len(), map);
    let (r1, r2) = advance(map, "start", &mut HashSet::new(), false, "start");
    (r1, r1 + r2)
}

pub fn day12(args: &crate::File) -> Result<()> {
    let map = read_input(args)?;

    let result = run(map.borrow_dependent());

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
