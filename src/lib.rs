use day6::Day6Args;
use std::path::PathBuf;
use structopt::StructOpt;

pub mod day1;
pub mod day1_2020;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod utils;

#[derive(Debug, StructOpt)]
#[structopt(about = "File argument")]
pub struct File {
    pub file: PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Advent of Code 2021")]
pub enum AocEntries {
    Day1(File),
    Day2(File),
    Day3(File),
    Day4(File),
    Day5(File),
    Day6(Day6Args),
    Day1_1_2020(File),
    Day1_2_2020(File),
}
