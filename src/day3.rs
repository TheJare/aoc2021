use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/3

fn find_most_common_bit(mask: u64, values: &[u64]) -> u64 {
    let bit_balance = values
        .iter()
        .fold(0, |acc, num| acc + (num & mask != 0) as i32 * 2 - 1);
    mask * (bit_balance >= 0) as u64
}

fn find_unique_value(mask: u64, mut values: Vec<u64>, comparator: u64) -> u64 {
    if mask > 0 && values.len() > 1 {
        let most_common_bit = find_most_common_bit(mask, &values) ^ comparator;
        values.retain(|v| ((v ^ most_common_bit) & mask) == 0);
        return find_unique_value(mask >> 1, values, comparator);
    }
    *values.get(0).unwrap_or(&0)
}

pub fn day3(args: &crate::File) -> Result<()> {
    let values = read_file(&args.file)?
        .lines()
        .flat_map(|line| u64::from_str_radix(line.trim(), 2))
        .collect_vec();
    let mask = values.iter().fold(0, |acc, num| acc | num);
    let width = (0..64).find(|i| 1 << i > mask).unwrap_or(0);

    let gamma_rate = (0..width).fold(0, |acc, i| acc | find_most_common_bit(1u64 << i, &values));
    let epsilon_rate = (1u64 << width) - 1 - gamma_rate;
    println!("Result of Part 1 is {}", gamma_rate * epsilon_rate);

    let o2_generator_rating = find_unique_value((1 << width) >> 1, values.clone(), 0);
    let co2_scrubber_rating = find_unique_value((1 << width) >> 1, values, !0);
    println!(
        "Result of Part 2 is {}",
        o2_generator_rating * co2_scrubber_rating
    );

    Ok(())
}
