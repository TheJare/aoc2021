use crate::utils::read_from_file;
use anyhow::Result;
use std::path::PathBuf;

// https://adventofcode.com/2021/day/2

fn iterate<B, F>(file: &PathBuf, initial_value: B, f: F) -> Result<B>
where
    F: Fn(B, &str, i32) -> B,
{
    Ok(read_from_file(&file)?
        .flatten()
        .fold(initial_value, |acc, l| {
            let mut split = l.split(' ');
            let command = split.next();
            let amount = split.next().and_then(|a| a.parse::<i32>().ok());
            if let (Some(s), Some(v)) = (command, amount) {
                f(acc, s, v)
            } else {
                acc
            }
        }))
}

pub fn day2(args: &crate::File) -> Result<()> {
    let (horizontal_pos, depth) = iterate(&args.file, (0, 0), |acc, s, v| match s {
        "down" => (acc.0, acc.1 + v),
        "up" => (acc.0, acc.1 - v),
        "forward" => (acc.0 + v, acc.1),
        _ => acc,
    })?;
    println!("Result of Part 1 is {}", horizontal_pos * depth);

    let (horizontal_pos, depth, _) = iterate(&args.file, (0, 0, 0), |acc, s, v| match s {
        "down" => (acc.0, acc.1, acc.2 + v),
        "up" => return (acc.0, acc.1, acc.2 - v),
        "forward" => return (acc.0 + v, acc.1 + acc.2 * v, acc.2),
        _ => acc,
    })?;
    println!("Result of Part 2 is {}", horizontal_pos * depth);

    Ok(())
}
