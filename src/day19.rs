use crate::utils::read_from_file;
use anyhow::Result;
use itertools::{iproduct, Itertools};
use std::collections::HashSet;

// https://adventofcode.com/2021/day/19

type Point = (i32, i32, i32);

type Scanner = HashSet<Point>;

pub fn read_input(args: &crate::File) -> Result<Vec<Scanner>> {
    let lines = read_from_file(&args.file)?;
    let mut scanners = Vec::new();
    for line in lines.flatten() {
        if line.starts_with("---") {
            scanners.push(Scanner::new());
        } else if let Some(beacon) = line
            .split(',')
            .flat_map(|v| v.parse::<i32>())
            .collect_tuple()
        {
            scanners.last_mut().unwrap().insert(beacon);
        }
    }
    Ok(scanners)
}

fn rotate((x, y, z): Point, rotation_index: usize) -> Point {
    match rotation_index {
        0 => (x, y, z),
        1 => (y, z, x),
        2 => (z, x, y),

        3 => (-x, y, -z),
        4 => (y, -z, -x),
        5 => (-z, -x, y),

        6 => (-x, -y, z),
        7 => (-y, z, -x),
        8 => (z, -x, -y),

        9 => (x, -y, -z),
        10 => (-y, -z, x),
        11 => (-z, x, -y),

        12 => (-x, z, y),
        13 => (z, y, -x),
        14 => (y, -x, z),

        15 => (-x, -z, -y),
        16 => (-z, -y, -x),
        17 => (-y, -x, -z),

        18 => (x, -z, y),
        19 => (-z, y, x),
        20 => (y, x, -z),

        21 => (x, z, -y),
        22 => (z, -y, x),
        23 => (-y, x, z),

        _ => panic!("impossible rotation {}", rotation_index),
    }
}

fn sub_points(a: &Point, b: &Point) -> Point {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn find_matching_offset(scanner: &Scanner, candidate: &Scanner) -> Option<Point> {
    // Try every point in one set against every point in the other set
    let pairs = iproduct!(scanner.iter(), candidate.iter());
    for (point_from_scanner, point_from_candidate) in pairs {
        let offset = sub_points(point_from_candidate, point_from_scanner);
        // Check if there's at least 12 points in the incoming set that match when offset
        let num_identical = candidate
            .iter()
            .filter(|&v| scanner.contains(&sub_points(v, &offset)))
            .take(12) // Early out, only need 12 matching points
            .count();
        if num_identical == 12 {
            return Some(offset);
        }
    }
    None
}

fn try_match_scanner(main: &Scanner, candidate: &Scanner) -> Option<(Scanner, Point)> {
    // Brute force all 24 rotations
    for rot in 0..24 {
        let rotated_candidate = candidate.iter().map(|p| rotate(*p, rot)).collect();
        if let Some(offset) = find_matching_offset(main, &rotated_candidate) {
            return Some((rotated_candidate, offset));
        }
    }
    None
}

pub fn run(mut scanners: Vec<Scanner>) -> (usize, usize) {
    let mut main = scanners.swap_remove(0);
    let mut beacons = vec![(0, 0, 0)];
    while !scanners.is_empty() {
        let mut found = false;
        for i in (0..scanners.len()).rev() {
            if let Some((rotated, offset)) = try_match_scanner(&main, &scanners[i]) {
                println!("Removing scanner {}/{}", i, scanners.len());
                main.extend(rotated.iter().map(|v| sub_points(v, &offset)));
                scanners.swap_remove(i);
                found = true;
                beacons.push(offset);
            }
        }
        if !found {
            panic!(
                "ran a full iteration without finding a scanner to merge out of {}.\nmain had accumulated {} points",
                scanners.len(),
                main.len()
            );
        }
    }

    let r1 = main.len();
    let r2 = beacons
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let d = sub_points(a, b);
            d.0.abs() + d.1.abs() + d.2.abs()
        })
        .max()
        .unwrap() as usize;

    (r1, r2)
}

pub fn day19(args: &crate::File) -> Result<()> {
    let target = read_input(args)?;

    let result = run(target);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
