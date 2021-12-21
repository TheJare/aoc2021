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
    a: usize,
    b: usize,
    s1: usize,
    s2: usize,
) -> (usize, usize) {
    let k = (a + 10 * b + 100 * s1 + 2100 * s2) as usize;
    let r = cache[k];
    if r.0 != usize::MAX {
        return r;
    }
    let (mut n1, mut n2) = (0, 0);
    for (score, score_times) in COMBOS.iter().enumerate() {
        let a = (a + score + 3) % 10;
        let s1 = s1 + a + 1;
        if s1 >= 21 {
            n1 += score_times;
        } else {
            let (na2, na1) = run_2_rec(cache, b, a, s2, s1);
            n1 += score_times * na1;
            n2 += score_times * na2;
        }
    }
    cache[k] = (n1, n2);
    (n1, n2)
}

pub fn run_2(a: usize, b: usize) -> usize {
    let mut cache = [(usize::MAX, usize::MAX); 2100 * 21];
    let (n1, n2) = run_2_rec(&mut cache, a, b, 0, 0);
    n1.max(n2)
}

pub fn run((a, b): (usize, usize)) -> (usize, usize) {
    let r1 = run_1(a - 1, b - 1);
    let r2 = run_2(a - 1, b - 1);

    (r1, r2)
}

pub fn day21(args: &crate::File) -> Result<()> {
    let input = read_input(args)?;

    let result = run(input);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
