use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/18

type Fish = Vec<i32>;

fn fmt_fish_r(fish: &[i32], pos: usize) -> (String, usize) {
    let c = fish[pos];
    if c < 0 {
        let (s1, pos) = fmt_fish_r(fish, pos + 1);
        let (s2, pos) = fmt_fish_r(fish, pos);
        (format!("[{},{}]", s1, s2), pos)
    } else {
        (format!("{}", c), pos + 1)
    }
}

fn fmt_fish(fish: &[i32]) -> String {
    let (s, _) = fmt_fish_r(fish, 0);
    s
}

pub fn parse_input(input: &str, mut pos: usize) -> (Fish, usize) {
    if let Some(c) = input.chars().nth(pos) {
        pos += 1;
        if c == '[' {
            let (mut a, pos) = parse_input(input, pos);
            let (mut b, pos) = parse_input(input, pos + 1);
            let mut r = vec![-1];
            r.append(&mut a);
            r.append(&mut b);
            return (r, pos + 1);
        } else if c.is_ascii_digit() {
            let mut v = c as usize - b'0' as usize;
            while let Some(c) = input.chars().nth(pos) {
                if !c.is_ascii_digit() {
                    break;
                }
                v = v * 10 + (c as usize - b'0' as usize);
                pos += 1;
            }
            return (vec![v as i32], pos);
        }
        panic!(
            "unexpected char {:?} at pos {} in input:\n{}",
            c, pos, input
        );
    }
    panic!("Ran out of input")
}

pub fn read_input(args: &crate::File) -> Result<Vec<Fish>> {
    let input = read_file(&args.file)?
        .lines()
        .map(|line| parse_input(line, 0).0)
        .collect_vec();

    Ok(input)
}

pub fn add(a: &mut Fish, b: &mut Fish) -> Fish {
    let mut r = vec![-1; 1];
    r.append(a);
    r.append(b);
    cleanup(&mut r);
    r
}

fn spread_num<'a>(it: impl Iterator<Item = &'a mut i32>, a: i32) {
    for j in it {
        if *j >= 0 {
            *j += a;
            break;
        }
    }
}

pub fn explode(num: &mut Fish, i: usize, depth: usize) -> Option<usize> {
    if num[i] < 0 {
        if depth >= 4 {
            let a = num[i + 1];
            let b = num[i + 2];
            // println!("explode [{},{}] in {}", a, b, fmt_fish(num));
            let mut r = num[0..i].to_vec();
            r.push(0);
            r.append(&mut num[(i + 3)..].to_vec());
            spread_num(r[..i].iter_mut().rev(), a);
            spread_num(r[i + 1..].iter_mut(), b);
            // println!(" -> {}", fmt_fish(&r));
            *num = r;
            None
        } else if let Some(i) = explode(num, i + 1, depth + 1) {
            explode(num, i, depth + 1)
        } else {
            None
        }
    } else {
        Some(i + 1)
    }
}

pub fn split(num: &mut Fish) -> bool {
    for i in 0..num.len() {
        if num[i] > 9 {
            // println!("split {} in {}", num[i], fmt_fish(num));
            let a = num[i] / 2;
            let b = num[i] - a;
            let mut r = num[..i].to_vec();
            r.append(&mut vec![-1, a, b]);
            r.append(&mut num[i + 1..].to_vec());
            // println!(" -> {}", fmt_fish(&r));
            *num = r;
            return true;
        }
    }
    false
}

pub fn cleanup(num: &mut Fish) {
    while explode(num, 0, 0).is_none() || split(num) {}
}

pub fn magnitude(num: &[i32], pos: usize) -> (i32, usize) {
    let c = num[pos];
    if c < 0 {
        let (s1, pos) = magnitude(num, pos + 1);
        let (s2, pos) = magnitude(num, pos);
        (3 * s1 + 2 * s2, pos)
    } else {
        (c, pos + 1)
    }
}

pub fn run(nums: Vec<Fish>) -> (i32, i32) {
    let mut added = nums
        .iter()
        .cloned()
        .reduce(|mut acc, mut num| add(&mut acc, &mut num))
        .unwrap();
    cleanup(&mut added);
    println!("All added: {}", fmt_fish(&added));
    let r1 = magnitude(&added, 0).0;

    let r2 = nums
        .iter()
        .tuple_combinations()
        .map(|(a, b)| magnitude(&add(&mut a.clone(), &mut b.clone()), 0).0)
        .max()
        .unwrap();
    (r1, r2)
}

pub fn day18(args: &crate::File) -> Result<()> {
    let target = read_input(args)?;

    let result = run(target);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
