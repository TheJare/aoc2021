use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

mod day1;
mod day1_2020;
mod day2;
mod day3;
mod day4;
mod utils;

#[derive(Debug, StructOpt)]
#[structopt(about = "File argument")]
pub struct File {
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Advent of Code 2021")]
enum AocEntries {
    Day1(File),
    Day2(File),
    Day3(File),
    Day4(File),
    Day1_1_2020(File),
    Day1_2_2020(File),
}

fn main() -> Result<()> {
    let opt = AocEntries::from_args();
    println!("{:?}", opt);
    match opt {
        AocEntries::Day1(args) => day1::day1(&args),
        AocEntries::Day2(args) => day2::day2(&args),
        AocEntries::Day3(args) => day3::day3(&args),
        AocEntries::Day4(args) => day4::day4(&args),
        AocEntries::Day1_1_2020(args) => day1_2020::day1(&args),
        AocEntries::Day1_2_2020(args) => day1_2020::day1_2(&args),
    }
}
