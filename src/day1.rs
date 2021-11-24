use anyhow::{anyhow, Context, Result};
use std::fs;

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
    // Read file
    let contents = fs::read_to_string(&args.file)
        .with_context(|| format!("Failed to read from {:?}", args.file))?;
    let nums = contents
        .lines()
        .flat_map(|l| l.parse::<i32>())
        .collect::<Vec<_>>();

    // nums.iter().for_each(|f| println!("{}", f));

    // Find the correct entry
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
