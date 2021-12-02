use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/2

pub fn day2(args: &crate::File) -> Result<()> {
    let (horizontal_pos, depth, aim) = read_file(&args.file)?
        .lines()
        .flat_map(|line| line.split_whitespace().next_tuple())
        .fold((0, 0, 0), |acc, (command, amount)| {
            match (command, amount.parse::<i32>()) {
                ("down", Ok(v)) => (acc.0, acc.1, acc.2 + v),
                ("up", Ok(v)) => (acc.0, acc.1, acc.2 - v),
                ("forward", Ok(v)) => (acc.0 + v, acc.1 + acc.2 * v, acc.2),
                _ => acc,
            }
        });
    println!("Result of Part 1 is {}", horizontal_pos * aim);
    println!("Result of Part 2 is {}", horizontal_pos * depth);

    Ok(())
}
