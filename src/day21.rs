use crate::utils::read_from_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/21

pub fn read_input(args: &crate::File) -> Result<(usize, usize)> {
    let lines = read_from_file(&args.file)?;
    let (a, b) = lines
        .flatten()
        .map(|v| {
            v.split(": ")
                .nth(1)
                .and_then(|v| v.trim().parse().ok())
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    Ok((a, b))
}

pub fn run_1(mut a: usize, mut b: usize) -> usize {
    let mut dice = 0;
    let (mut s1, mut s2) = (0, 0);
    while s1 < 1000 && s2 < 1000 {
        a = (a + dice % 100 + (dice + 1) % 100 + (dice + 2) % 100 + 3) % 10;
        dice += 3;
        s1 += a + 1;
        if s1 < 1000 {
            b = (b + dice % 100 + (dice + 1) % 100 + (dice + 2) % 100 + 3) % 10;
            dice += 3;
            s2 += b + 1;
        }
    }
    s1.min(s2) * dice
}

const COMBOS: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

pub fn run_2_rec(
    cache: &mut [(usize, usize)],
    pos1: usize,
    pos2: usize,
    score1: usize,
    score2: usize,
) -> (usize, usize) {
    let k = (pos1 + 10 * pos2 + 100 * score1 + 2100 * score2) as usize;
    let r = cache[k];
    if r.0 != usize::MAX {
        return r;
    }
    let (wins1, wins2) = COMBOS
        .iter()
        .enumerate()
        .fold((0, 0), |(wins1, wins2), (score, score_times)| {
            let pos1 = (pos1 + score + 3) % 10;
            let score1 = score1 + pos1 + 1;
            if score1 >= 21 {
                (wins1 + score_times, wins2)
            } else {
                let (sub_wins2, sub_wins1) = run_2_rec(cache, pos2, pos1, score2, score1);
                (wins1 + score_times * sub_wins1, wins2 + score_times * sub_wins2)
            }
        });
    cache[k] = (wins1, wins2);
    (wins1, wins2)
}

pub fn run_2(pos1: usize, pos2: usize) -> usize {
    let mut cache = [(usize::MAX, usize::MAX); 2100 * 21];
    let (wins1, wins2) = run_2_rec(&mut cache, pos1, pos2, 0, 0);
    wins1.max(wins2)
}

pub fn run((input_pos1, input_pos2): (usize, usize)) -> (usize, usize) {
    let r1 = run_1(input_pos1 - 1, input_pos2 - 1);
    let r2 = run_2(input_pos1 - 1, input_pos2 - 1);

    (r1, r2)
}

pub fn day21(args: &crate::File) -> Result<()> {
    let input = read_input(args)?;

    let result = run(input);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
