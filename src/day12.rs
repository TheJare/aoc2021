use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;
use std::cmp::Ordering;

// https://adventofcode.com/2021/day/12

#[derive(Debug)]
pub struct Map(Vec<u32>, usize, usize, usize);

pub fn read_input(args: &crate::File) -> Result<Map> {
    let file = read_file(&args.file)?;
    let mut nodes = file.split(|c: char| !c.is_ascii_alphabetic()).collect_vec();
    nodes.sort_unstable();
    nodes.dedup();
    let start = nodes.binary_search(&"start").unwrap();
    let end = nodes.binary_search(&"end").unwrap();
    let smallcave_min_index = nodes.partition_point(|&s| s.cmp("a") == Ordering::Less);
    let mut map = vec![0u32; nodes.len()];
    for line in file.split_ascii_whitespace() {
        if let Some((a, b)) = line.split('-').collect_tuple() {
            let (a, b) = (
                nodes.binary_search(&a.trim()).unwrap(),
                nodes.binary_search(&b.trim()).unwrap(),
            );
            map[a] |= 1 << b;
            map[b] |= 1 << a;
        }
    }
    Ok(Map(map, start, end, smallcave_min_index))
}

fn advance(map: &Map, node: usize, visited: u32, long_path: bool) -> (usize, usize) {
    let (mut r1, mut r2) = (0, 0);

    for target in 0..map.0.len() {
        let target_bit = 1 << target;
        if target != map.1 && map.0[node] & target_bit != 0 {
            if target == map.2 {
                if !long_path {
                    r1 += 1;
                }
                r2 += 1;
            } else {
                let is_visited = target >= map.3 && visited & target_bit != 0;
                if !long_path || !is_visited {
                    let r = advance(map, target, visited | target_bit, long_path || is_visited);
                    r1 += r.0;
                    r2 += r.1;
                }
            }
        }
    }
    (r1, r2)
}

pub fn run(map: &Map) -> (usize, usize) {
    // println!("map contains {} nodes: {:?}", map.0.len(), map);
    advance(map, map.1, 0, false)
}

pub fn day12(args: &crate::File) -> Result<()> {
    let map = read_input(args)?;

    let result = run(&map);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
