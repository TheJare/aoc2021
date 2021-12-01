use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

#[allow(dead_code)]
pub fn read_file(file: &PathBuf) -> Result<String> {
    Ok(fs::read_to_string(&file).with_context(|| format!("Failed to read from {:?}", file))?)
}

pub fn read_from_file(file: &PathBuf) -> Result<Lines<BufReader<File>>> {
    Ok(
        BufReader::new(
            File::open(file).with_context(|| format!("Failed to read from {:?}", file))?,
        )
        .lines(),
    )
}

pub fn read_ints_from_file(file: &PathBuf) -> Result<Vec<i32>> {
    let nums = read_from_file(file)?
        .flatten()
        .flat_map(|l| l.parse()) // skip empty or non-numeric lines
        .collect();
    Ok(nums)
}
