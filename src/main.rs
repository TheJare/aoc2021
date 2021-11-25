use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

mod day1;

#[derive(Debug, StructOpt)]
#[structopt(about = "File argument")]
pub struct File {
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Advent of Code 2021")]
enum Aoc2022 {
    Day1(File),
    Day1_2(File),
}

fn main() -> Result<()> {
    let opt = Aoc2022::from_args();
    println!("{:?}", opt);
    match opt {
        Aoc2022::Day1(args) => day1::day1(&args),
        Aoc2022::Day1_2(args) => day1::day1_2(&args),
    }
}
