use crate::utils::read_ints_from_file;
use anyhow::Result;

// https://adventofcode.com/2021/day/1

fn count_increases(args: &crate::File, window_size: usize) -> Result<usize> {
    Ok(read_ints_from_file(&args.file)?
        .windows(window_size)
        .filter(|a| a[window_size - 1] > a[0])
        .count())
}

pub fn day1(args: &crate::File) -> Result<()> {
    println!("Part 1 solution is {}", count_increases(args, 2)?);
    println!("Part 2 solution is {}", count_increases(args, 4)?);
    Ok(())
}
