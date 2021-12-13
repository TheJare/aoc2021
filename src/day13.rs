use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

// https://adventofcode.com/2021/day/13

pub struct Input(HashSet<(i32, i32)>, Vec<i32>);

pub fn read_input(args: &crate::File) -> Result<Input> {
    let file = read_file(&args.file)?;
    let lines = file.lines();
    let mut dots = HashSet::new();
    let mut folds: Vec<i32> = Vec::new();
    for line in lines {
        let line = line.trim();
        if let Some((x, y)) = line
            .split(',')
            .flat_map(|v| v.trim().parse())
            .collect_tuple()
        {
            dots.insert((x, y));
        } else if let Some((cmd, n)) = line.trim().split('=').collect_tuple() {
            if let Ok(n) = n.trim().parse::<i32>() {
                folds.push((((cmd == "fold along x") as i32) * 2 - 1) * n);
            }
        }
    }
    Ok(Input(dots, folds))
}

pub fn run(dots: HashSet<(i32, i32)>, folds: &[i32]) -> (usize, String) {
    let mut dots = dots;
    let mut r1 = 0;
    for &fold in folds.iter() {
        let mut new_dots = HashSet::new();
        for &(x, y) in dots.iter() {
            if fold > 0 {
                new_dots.insert((if x > fold { 2 * fold - x } else { x }, y));
            } else {
                new_dots.insert((x, if y > (-fold) { -2 * fold - y } else { y }));
            }
        }
        r1 = r1.max(new_dots.len());
        dots = new_dots;
    }

    let (w, h) = dots.iter().fold((0, 0), |(w, h), (x, y)| {
        (w.max(1 + *x as usize), h.max(1 + *y as usize))
    });
    let mut map = vec![vec![' '; w]; h];
    dots.iter()
        .for_each(|&(x, y)| map[y as usize][x as usize] = '#');
    let r2 = map.iter().map(|l| l.iter().join("")).join("\n");
    (r1, r2)
}

pub fn day13(args: &crate::File) -> Result<()> {
    let Input(dots, folds) = read_input(args)?;

    let result = run(dots, &folds);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is\n{}", result.1);

    Ok(())
}
