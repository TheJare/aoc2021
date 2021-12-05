use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

// https://adventofcode.com/2021/day/5

pub fn read_input(args: &crate::File) -> Result<Vec<i32>> {
    let file = read_file(&args.file)?;
    let tokens = file
        .split(|c: char| !c.is_ascii_digit())
        .flat_map(|v| v.parse())
        .collect_vec();
    Ok(tokens)
}

fn run_step(floor: &mut HashMap<(i32, i32), usize>, vents: &Vec<i32>, diagonals: bool) -> usize {
    for (&x0, &y0, &x1, &y1) in vents.into_iter().tuples() {
        let (dx, dy) = (x1 - x0, y1 - y0);
        let r = dx.abs().max(dy.abs());
        let (dx, dy) = (dx.signum(), dy.signum()); // could divide by r to do any-slope DDA

        if (!diagonals && (dy == 0 || dx == 0)) || (diagonals && dx != 0 && dy != 0) {
            (0..=r).for_each(|i| {
                let pos = (x0 + dx * i, y0 + dy * i);
                match floor.get_mut(&pos) {
                    Some(count) => {
                        *count = *count + 1;
                    }
                    None => {
                        floor.insert(pos, 1);
                    }
                }
            });
        }
    }
    floor.values().filter(|&&v| v > 1).count()
}

pub fn run(vents: Vec<i32>) -> (usize, usize) {
    let mut floor = HashMap::<(i32, i32), usize>::new();
    (
        run_step(&mut floor, &vents, false),
        run_step(&mut floor, &vents, true),
    )
}

pub fn day5(args: &crate::File) -> Result<()> {
    let vents = read_input(&args)?;

    let result = run(vents);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
