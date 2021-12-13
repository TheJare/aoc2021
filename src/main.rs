use anyhow::Result;
use aoc2021::{
    day1, day10, day11, day12, day13, day1_2020, day2, day3, day4, day5, day6, day7, AocEntries,
};
use aoc2021::{day8, day9};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = AocEntries::from_args();
    println!("{:?}", opt);
    match opt {
        AocEntries::Day1(args) => day1::day1(&args),
        AocEntries::Day2(args) => day2::day2(&args),
        AocEntries::Day3(args) => day3::day3(&args),
        AocEntries::Day4(args) => day4::day4(&args),
        AocEntries::Day5(args) => day5::day5(&args),
        AocEntries::Day6(args) => day6::day6(&args),
        AocEntries::Day7(args) => day7::day7(&args),
        AocEntries::Day8(args) => day8::day8(&args),
        AocEntries::Day9(args) => day9::day9(&args),
        AocEntries::Day10(args) => day10::day10(&args),
        AocEntries::Day11(args) => day11::day11(&args),
        AocEntries::Day12(args) => day12::day12(&args),
        AocEntries::Day13(args) => day13::day13(&args),
        AocEntries::Day1_1_2020(args) => day1_2020::day1(&args),
        AocEntries::Day1_2_2020(args) => day1_2020::day1_2(&args),
    }
}
