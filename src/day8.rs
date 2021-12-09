use crate::utils::read_from_file;
use anyhow::Result;
use itertools::Itertools;
use std::path::Path;
use std::str::Split;

// https://adventofcode.com/2021/day/8

type Digit = (usize, u32);
type Entry = (Vec<Digit>, usize);

pub fn read_input(file: &Path) -> Result<Vec<Entry>> {
    fn grab_entries(sides: &mut Split<char>) -> Vec<Digit> {
        sides
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| {
                (
                    s.len(),
                    s.bytes().map(|c| c - b'a').fold(0, |acc, d| acc | (1 << d)),
                )
            })
            .collect_vec()
    }
    let entries = read_from_file(file)?
        .flatten()
        .map(|line| {
            let mut sides = line.split('|');
            let mut left = grab_entries(&mut sides);
            let mut right = grab_entries(&mut sides);
            let left_len = left.len();
            left.append(&mut right);
            (left, left_len)
        })
        .collect_vec();
    Ok(entries)
}

fn run_1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|e| {
            e.0.iter()
                .skip(e.1)
                .filter(move |digit| matches!(digit.0, 2 | 4 | 3 | 7))
                .count()
        })
        .sum()
}

pub fn solve(entry: &Entry) -> [u32; 10] {
    let mut digit_to_scrambled = [0u32; 10];
    let find = |f: &dyn Fn(&Digit) -> bool| entry.0.iter().find(|&e| f(e)).unwrap().1;

    digit_to_scrambled[1] = find(&|e| e.0 == 2);
    digit_to_scrambled[7] = find(&|e| e.0 == 3);
    digit_to_scrambled[4] = find(&|e| e.0 == 4);
    digit_to_scrambled[8] = find(&|e| e.0 == 7);
    let d7_or_4 = digit_to_scrambled[7] | digit_to_scrambled[4];
    let seg_eg = digit_to_scrambled[8] ^ d7_or_4;
    let seg_cf = digit_to_scrambled[1];
    digit_to_scrambled[6] =
        find(&|e| e.0 == 6 && (e.1 & seg_eg) == seg_eg && (e.1 & seg_cf) != seg_cf);

    let seg_bd = digit_to_scrambled[4] ^ digit_to_scrambled[1];
    digit_to_scrambled[5] = find(&|e| e.0 == 5 && (e.1 & seg_bd) == seg_bd);

    digit_to_scrambled[9] = digit_to_scrambled[5] | digit_to_scrambled[1];

    digit_to_scrambled[0] =
        find(&|e| e.0 == 6 && e.1 != digit_to_scrambled[6] && e.1 != digit_to_scrambled[9]);

    let seg_f = digit_to_scrambled[5] & digit_to_scrambled[1];
    digit_to_scrambled[3] =
        find(&|e| e.0 == 5 && e.1 != digit_to_scrambled[5] && (e.1 & seg_f) == seg_f);

    digit_to_scrambled[2] =
        find(&|e| e.0 == 5 && e.1 != digit_to_scrambled[3] && e.1 != digit_to_scrambled[5]);

    digit_to_scrambled
}

fn run_2(entries: &[Entry]) -> usize {
    let sum: u32 = entries
        .iter()
        .map(|e| {
            let s = solve(e);
            let num =
                e.0.iter()
                    .skip(e.1)
                    .map(|e| s.iter().position(|&i| i == e.1).unwrap())
                    .fold(0, |acc, d| acc * 10 + (d as u32));
            println!("num is {}", num);
            num
        })
        .sum();
    sum as usize
}

pub fn run(entries: &[Entry]) -> (usize, usize) {
    (run_1(entries), run_2(entries))
}

pub fn day8(args: &crate::File) -> Result<()> {
    let entries = read_input(&args.file)?;

    let (result1, result2) = run(&entries);

    println!("Result of Part 1 is {}", result1);
    println!("Result of Part 2 is {}", result2);

    Ok(())
}
