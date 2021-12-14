use crate::utils::read_file;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::HashMap;

// https://adventofcode.com/2021/day/14

type Pair = (char, char);
pub struct Input(String, HashMap<Pair, char>);

pub fn read_input(args: &crate::File) -> Result<Input> {
    let file = read_file(&args.file)?;
    let mut lines = file.lines();
    let input = lines.next().context("empty file")?.trim().to_string();
    let mut rules = HashMap::new();
    for line in lines {
        if let Some((x, y)) = line.split("->").map(|v| v.trim()).collect_tuple() {
            if let (Some((a, b)), Some(c)) = (x.chars().collect_tuple(), y.chars().next()) {
                rules.insert((a, b), c);
            }
        }
    }
    Ok(Input(input, rules))
}

fn run_insertions(
    input: &str,
    rules: &HashMap<Pair, char>,
    hist: &mut HashMap<Pair, usize>,
    num: usize,
) -> (usize, usize) {
    for _ in 0..num {
        let mut new = HashMap::new();
        for (&(a, b), &n) in hist.iter() {
            if let Some(&c) = rules.get(&(a, b)) {
                *new.entry((a, c)).or_insert(0) += n;
                *new.entry((c, b)).or_insert(0) += n;
            } else {
                *new.entry((a, b)).or_insert(0) += n;
            }
        }
        *hist = new;
    }
    let mut letter_hist = HashMap::<char, usize>::new();
    if let Some(c) = input.chars().next() {
        letter_hist.insert(c, 1);
    }
    hist.iter()
        .for_each(|(&(_, b), &n)| *letter_hist.entry(b).or_insert(0) += n);
    let min = letter_hist.values().min().unwrap();
    let max = letter_hist.values().max().unwrap();

    (*min, *max)
}

pub fn run(input: &str, rules: &HashMap<Pair, char>) -> (usize, usize) {
    let mut hist: HashMap<Pair, usize> = HashMap::new();

    for (a, b) in input.chars().tuple_windows() {
        *hist.entry((a, b)).or_insert(0) += 1;
    }
    let (min1, max1) = run_insertions(input, rules, &mut hist, 10);
    let (min2, max2) = run_insertions(input, rules, &mut hist, 40 - 10);

    (max1 - min1, max2 - min2)
}

pub fn day14(args: &crate::File) -> Result<()> {
    let Input(input, rules) = read_input(args)?;

    let result = run(&input, &rules);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
