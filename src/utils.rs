use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn read_lines_from_file(file: &PathBuf) -> Result<String> {
    Ok(fs::read_to_string(&file).with_context(|| format!("Failed to read from {:?}", file))?)
}

pub fn read_ints_from_file(file: &PathBuf) -> Result<Vec<i32>> {
    let nums = read_lines_from_file(file)?
        .lines()
        .flat_map(|l| l.parse::<i32>())
        .collect::<Vec<_>>();
    // nums.iter().for_each(|f| println!("{}", f));
    Ok(nums)
}
