use crate::utils::read_from_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/2

pub fn day2(args: &crate::File) -> Result<()> {
    let (horizontal_pos, depth, aim) =
        read_from_file(&args.file)?
            .flatten()
            .fold((0, 0, 0), |acc, l| {
                if let Some((command, amount)) = l.split(' ').next_tuple() {
                    if let Ok(v) = amount.parse::<i32>() {
                        return match command {
                            "down" => (acc.0, acc.1, acc.2 + v),
                            "up" => (acc.0, acc.1, acc.2 - v),
                            "forward" => (acc.0 + v, acc.1 + acc.2 * v, acc.2),
                            _ => acc,
                        };
                    }
                }
                acc
            });
    println!("Result of Part 1 is {}", horizontal_pos * aim);
    println!("Result of Part 2 is {}", horizontal_pos * depth);

    Ok(())
}
