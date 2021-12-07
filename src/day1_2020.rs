use crate::utils::read_ints_from_file;
use anyhow::{anyhow, Result};
use itertools::Itertools;

// For custom arguments, do something like this
// use std::path::PathBuf;
// use structopt::StructOpt;
//
// #[derive(Debug, StructOpt)]
// #[structopt(about = "Day 1")]
// pub struct Day1 {
//     file: PathBuf,
// }

// Did this 2020 entry to get the ball rolling, keeping it for reference
// https://adventofcode.com/2020/day/1

pub fn day1(args: &crate::File) -> Result<()> {
    let nums = read_ints_from_file(&args.file)?;

    // Find the correct entry, functional style
    let entry = nums
        .into_iter()
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
        .map(|(a, b)| a * b);

    // Print outcome and return success code
    match entry {
        Some(result) => {
            println!("Solution is {}", result);
            Ok(())
        }
        None => Err(anyhow!("No values found")),
    }
}

pub fn day1_2(args: &crate::File) -> Result<()> {
    let nums = read_ints_from_file(&args.file)?;

    // Find the correct entry, procedural style
    for i in 0..nums.len() {
        for j in 0..i {
            for k in 0..j {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    println!("Solution is {}", nums[i] * nums[j] * nums[k]);
                    return Ok(());
                }
            }
        }
    }
    Err(anyhow!("No values found"))
}
