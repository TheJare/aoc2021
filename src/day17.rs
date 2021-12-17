use crate::utils::read_file;
use anyhow::{bail, Result};
use itertools::{iproduct, Itertools};

// https://adventofcode.com/2021/day/17

pub fn read_input(args: &crate::File) -> Result<(i32, i32, i32, i32)> {
    let file = read_file(&args.file)?;

    let input = file
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .flat_map(|s| s.parse())
        .collect_vec();
    if let [x0, x1, y0, y1] = input[..] {
        return Ok((x0, x1, y0, y1));
    }
    bail!("error in input {:?}", input);
}

pub fn run(tx0: i32, tx1: i32, ty0: i32, ty1: i32) -> (i32, i32) {
    let mut r1 = 0;
    let mut min_vx = 0;
    while min_vx * (min_vx + 1) / 2 < tx0 {
        min_vx += 1;
    }
    let mut total = 0;
    for (mut vx, mut vy) in iproduct!(min_vx..=tx1, ty0..=-ty0 - 1) {
        let mut hit_area = false;
        let (mut x, mut y) = (0, 0);
        while x <= tx1 && y >= ty0 {
            r1 = r1.max(y);
            hit_area = hit_area || (x >= tx0 && y <= ty1);
            if hit_area {
                break;
            }
            x += vx;
            y += vy;
            vx -= vx.signum();
            vy -= 1;
        }
        total += hit_area as i32;
    }
    (r1, total)
}

pub fn day17(args: &crate::File) -> Result<()> {
    let target = read_input(args)?;

    let result = run(target.0, target.1, target.2, target.3);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
