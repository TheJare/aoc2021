use crate::utils::read_file;
use anyhow::Result;
use std::path::Path;

// https://adventofcode.com/2021/day/7

pub fn read_input(file: &Path) -> Result<Vec<i64>> {
    let file = read_file(file)?;
    Ok(file.split(',').flat_map(|v| v.trim().parse()).collect())
}

pub fn run(crabs: &[i64]) -> (i64, i64) {
    match (crabs.iter().min(), crabs.iter().max()) {
        (Some(&min), Some(&max)) => (min..=max).fold((i64::MAX, i64::MAX), |(cur1, cur2), v| {
            crabs
                .iter()
                .map(move |&crab| {
                    let cost1 = (crab - v).abs();
                    let cost2 = cost1 * (cost1 + 1) / 2;
                    (cost1, cost2)
                })
                .fold((0, 0), |(acc1, acc2), (cost1, cost2)| {
                    (cur1.min(acc1 + cost1), cur2.min(acc2 + cost2))
                })
        }),
        _ => (0, 0),
    }
}

pub fn day7(args: &crate::File) -> Result<()> {
    let crabs = read_input(&args.file)?;

    let (result1, result2) = run(&crabs);

    println!("Result of Part 1 is {}", result1);
    println!("Result of Part 2 is {}", result2);

    Ok(())
}
