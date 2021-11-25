use crate::utils::read_ints_from_file;
use anyhow::{anyhow, Result};

// For custom arguments, do something like this
// use std::path::PathBuf;
// use structopt::StructOpt;
//
// #[derive(Debug, StructOpt)]
// #[structopt(about = "Day 1")]
// pub struct Day1 {
//     file: PathBuf,
// }

// https://adventofcode.com/2020/day/1

pub fn day1(args: &crate::File) -> Result<()> {
    let nums = read_ints_from_file(&args.file)?;

    // Find the correct entry, functional style
    let entry = nums
        .iter()
        .enumerate()
        .flat_map(|(i, f)| {
            let invf = 2020 - f;
            nums[0..i].contains(&invf).then(|| f * invf)
        })
        .next();

    // Print outcome and return success code
    match entry {
        Some(result) => Ok(println!("Solution is {}", result)),
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
                    return Ok(println!("Solution is {}", nums[i] * nums[j] * nums[k]));
                }
            }
        }
    }
    Err(anyhow!("No values found"))
}
