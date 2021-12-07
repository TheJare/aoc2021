use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

#[allow(dead_code)]
pub fn read_file(file: &Path) -> Result<String> {
    fs::read_to_string(&file).with_context(|| format!("Failed to read from {:?}", file))
}

pub fn read_from_file(file: &Path) -> Result<Lines<BufReader<File>>> {
    Ok(
        BufReader::new(
            File::open(file).with_context(|| format!("Failed to read from {:?}", file))?,
        )
        .lines(),
    )
}

pub fn read_ints_from_file(file: &Path) -> Result<Vec<i32>> {
    let nums = read_from_file(file)?
        .flatten()
        .flat_map(|l| l.parse()) // skip empty or non-numeric lines
        .collect();
    Ok(nums)
}
