use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/5

pub fn read_input(args: &crate::File) -> Result<(Vec<i32>, i32)> {
    let file = read_file(&args.file)?;
    let tokens = file
        .split(|c: char| !c.is_ascii_digit())
        .flat_map(|v| v.parse())
        .collect_vec();
    let &max = tokens.iter().max().unwrap_or(&0);
    Ok((tokens, max + 1))
}

fn run_step(floor: &mut Vec<u8>, vents: &Vec<i32>, max: i32, diagonals: bool) -> usize {
    let mut acc = 0;
    for (&x0, &y0, &x1, &y1) in vents.into_iter().tuples() {
        let (dx, dy) = (x1 - x0, y1 - y0);
        let r = dx.abs().max(dy.abs());
        let (dx, dy) = (dx.signum(), dy.signum()); // could divide by r to do any-slope DDA

        if (!diagonals && (dy == 0 || dx == 0)) || (diagonals && dx != 0 && dy != 0) {
            (0..=r).for_each(|i| {
                let pos = ((x0 + dx * i) + max * (y0 + dy * i)) as usize;
                let count = floor[pos] + 1;
                floor[pos] = count;
                acc = acc + (count == 2) as usize;
            });
        }
    }
    acc
}

pub fn run(vents: Vec<i32>, max: i32) -> (usize, usize) {
    let mut floor = vec![0u8; (max * max) as usize];
    let step1 = run_step(&mut floor, &vents, max, false);
    (step1, step1 + run_step(&mut floor, &vents, max, true))
}

pub fn day5(args: &crate::File) -> Result<()> {
    let (vents, max) = read_input(&args)?;

    let result = run(vents, max);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
