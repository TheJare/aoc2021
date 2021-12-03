use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/3

fn read_values(args: &crate::File) -> Result<(usize, Vec<u64>)> {
    let nums = read_file(&args.file)?;
    let nums = nums
        .lines()
        .flat_map(|line| u64::from_str_radix(line.trim(), 2))
        .collect_vec();
    let mask = nums.iter().fold(0, |acc, num| acc | num);
    Ok((find_bit_width(mask), nums))
}

fn find_bit_width(mut mask: u64) -> usize {
    // ((mask & (0 - mask)).log2()+1).try_into().unwrap() // log2 is unstable rust still
    let mut i = 0;
    while mask > 0 {
        mask = mask >> 1;
        i = i + 1;
    }
    i
}

fn find_common_bit(mask: u64, values: &Vec<u64>) -> u64 {
    let bit_balance: i32 = values
        .iter()
        .map(|num| if num & mask == 0 { -1 } else { 1 })
        .sum();
    if bit_balance >= 0 {
        mask
    } else {
        0
    }
}

fn find_common_bits(width: usize, values: &Vec<u64>) -> u64 {
    (0..width).fold(0, |acc, i| acc | find_common_bit(1u64 << i, values))
}

fn find_unique_value(width: usize, mut values: Vec<u64>, comparator: u64) -> u64 {
    for bit in (0..width).rev() {
        let collected_num = values.len();
        if collected_num == 1 {
            break;
        }
        let cur_mask = 1u64 << bit;
        let most_common_bit = find_common_bit(cur_mask, &values) ^ comparator;

        for i in (0..collected_num).rev() {
            if ((values[i] ^ most_common_bit) & cur_mask) != 0 {
                values.swap_remove(i);
            }
        }
    }
    *values.get(0).unwrap_or(&0)
}

pub fn day3(args: &crate::File) -> Result<()> {
    let (width, values) = read_values(&args)?;
    let gamma = find_common_bits(width, &values);
    let not_gamma = (1u64 << width) - 1 - gamma;

    let o2_generator_rating = find_unique_value(width, values.clone(), 0);
    let co2_scrubber_rating = find_unique_value(width, values, !0);

    println!("Result of Part 1 is {}", gamma * not_gamma);
    println!(
        "Result of Part 2 is {}",
        o2_generator_rating * co2_scrubber_rating
    );

    Ok(())
}
