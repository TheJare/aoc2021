use crate::utils::read_ints_from_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/1

pub fn day1_1(args: &crate::File) -> Result<()> {
    let result = read_ints_from_file(&args.file)?
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();
    Ok(println!("Solution is {}", result))
}

pub fn day1_2(args: &crate::File) -> Result<()> {
    let result = read_ints_from_file(&args.file)?
        .into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();
    Ok(println!("Solution is {}", result))
}
