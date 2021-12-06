use crate::utils::read_file;
use anyhow::Result;
use std::ops::Range;
use std::path::PathBuf;
use structopt::StructOpt;

// https://adventofcode.com/2021/day/5

type PopulationCount = u64; // good until day 442, then u128 up until day 951
type Hist = [PopulationCount; 9];

#[derive(Debug, StructOpt)]
#[structopt(about = "File argument")]
pub struct Day6Args {
    pub file: PathBuf,
    #[structopt(default_value = "18")]
    pub days: usize,
}

pub fn read_input(file: &PathBuf) -> Result<Hist> {
    let file = read_file(file)?;
    let mut hist: Hist = [0; 9];
    file.split(",")
        .flat_map(|v| v.trim().parse())
        .for_each(|v: usize| hist[v] += 1);
    Ok(hist)
}

pub fn run(hist: &mut Hist, days: Range<usize>) -> PopulationCount {
    for _day in days {
        let gen0 = hist[0];
        // println!("day {} {:?}", _day, hist);
        for i in 1..hist.len() {
            hist[i - 1] = hist[i];
        }
        hist[6] += gen0;
        hist[8] = gen0;
    }
    (*hist).into_iter().sum() // remove mutability for sum() to work
}

pub fn day6(args: &crate::Day6Args) -> Result<()> {
    let mut hist = read_input(&args.file)?;

    let test_result = run(&mut hist, 0..args.days);
    let result1 = run(&mut hist, args.days..80);
    let result2 = run(&mut hist, 80.max(args.days)..256);

    println!("Test result is {}", test_result);
    println!("Result of Part 1 is {}", result1);
    println!("Result of Part 2 is {}", result2);

    Ok(())
}
