use crate::utils::read_ints_from_file;
use anyhow::Result;

// https://adventofcode.com/2021/day/1

pub fn day1_1(args: &crate::File) -> Result<()> {
    let result = read_ints_from_file(&args.file)?
        .windows(2)
        .filter(|a| a[1] > a[0])
        .count();
    Ok(println!("Solution is {}", result))
}

pub fn day1_2(args: &crate::File) -> Result<()> {
    let result = read_ints_from_file(&args.file)?
        .windows(4)
        .filter(|a| a[3] > a[0])
        .count();
    Ok(println!("Solution is {}", result))
}
