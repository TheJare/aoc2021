use crate::utils::read_from_file;
use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

// https://adventofcode.com/2021/day/10

pub fn read_input(args: &crate::File) -> Result<Vec<String>> {
    let file = read_from_file(&args.file)?.flatten().collect_vec();
    Ok(file)
}

lazy_static! {
    static ref CHARS: HashMap<char, (char, usize)> = {
        let mut m = HashMap::new();
        m.insert(')', ('(', 3));
        m.insert(']', ('[', 57));
        m.insert('}', ('{', 1197));
        m.insert('>', ('<', 25137));
        m
    };
    static ref REV_CHARS: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('(', 1);
        m.insert('[', 2);
        m.insert('{', 3);
        m.insert('<', 4);
        m
    };
}

fn score_line(line: &str) -> (usize, usize) {
    let mut stack = Vec::<char>::new();
    let (mut r1, mut r2) = (0, 0);
    for c in line.chars() {
        if let '(' | '[' | '{' | '<' = c {
            stack.push(c)
        } else {
            let r = stack
                .pop()
                .and_then(|s| CHARS.get(&c).map(|m| (s, m)))
                .and_then(|(s, m)| if s != m.0 { Some(m.1) } else { None });
            if let Some(r) = r {
                r1 = r;
                break;
            }
        }
    }
    if r1 == 0 {
        r2 = stack
            .iter()
            .rev()
            .flat_map(|c| REV_CHARS.get(c))
            .fold(0, |a, v| a * 5 + v);
    }
    (r1, r2)
}

pub fn run(lines: &[String]) -> Result<(usize, usize)> {
    let r = lines.iter().map(|s| score_line(s)).collect_vec();
    // unzipping was confusing type inference, so doing it manually
    let r0 = r.iter().fold(0, |a, v| a + v.0);
    let mut r1 = r.iter().map(|v| v.1).filter(|&v| v > 0).collect_vec();
    r1.sort_unstable();
    let r1 = r1[r1.len() / 2];
    Ok((r0, r1))
}

pub fn day10(args: &crate::File) -> Result<()> {
    let lines = read_input(args)?;

    let result = run(&lines)?;

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
