use crate::utils::read_from_file;
use anyhow::Result;
use itertools::{iproduct, Itertools};
use regex::Regex;

// https://adventofcode.com/2021/day/22

type Point = (i64, i64, i64);
type Cube = (Point, Point);
type Command = (bool, Cube);

pub fn read_input(args: &crate::File) -> Result<Vec<Command>> {
    let line_parser =
        Regex::new(r"(\w+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
            .unwrap();
    let lines = read_from_file(&args.file)?;
    let r = lines
        .flatten()
        .map(|s| {
            let caps = line_parser.captures(&s)?;
            let cmd = caps.get(1)?.as_str() == "on";
            let x0 = caps.get(2)?.as_str().parse().ok()?;
            let x1 = caps.get(3)?.as_str().parse::<i64>().ok()?;
            let y0 = caps.get(4)?.as_str().parse().ok()?;
            let y1 = caps.get(5)?.as_str().parse::<i64>().ok()?;
            let z0 = caps.get(6)?.as_str().parse().ok()?;
            let z1 = caps.get(7)?.as_str().parse::<i64>().ok()?;
            Some((cmd, ((x0, y0, z0), (x1 + 1, y1 + 1, z1 + 1))))
        })
        .flatten()
        .collect_vec();
    Ok(r)
}

fn intersects(a: &Cube, b: &Cube) -> bool {
    a.0 .0 < b.1 .0
        && a.1 .0 > b.0 .0
        && a.0 .1 < b.1 .1
        && a.1 .1 > b.0 .1
        && a.0 .2 < b.1 .2
        && a.1 .2 > b.0 .2
}

fn is_empty_rect(a: &Cube) -> bool {
    a.0 .0 >= a.1 .0 && a.0 .1 >= a.1 .1 && a.0 .2 >= a.1 .2
}

fn split_rect(a: &[Cube], coord: i64, mask: (bool, bool, bool)) -> Vec<Cube> {
    let mut r = Vec::new();
    for src in a.iter() {
        if mask.0 && src.0 .0 < coord && src.1 .0 > coord {
            r.push((src.0, (coord, src.1 .1, src.1 .2)));
            r.push(((coord, src.0 .1, src.0 .2), src.1));
        } else if mask.1 && src.0 .1 < coord && src.1 .1 > coord {
            r.push((src.0, (src.1 .0, coord, src.1 .2)));
            r.push(((src.0 .0, coord, src.0 .2), src.1));
        } else if mask.2 && src.0 .2 < coord && src.1 .2 > coord {
            r.push((src.0, (src.1 .0, src.1 .1, coord)));
            r.push(((src.0 .0, src.0 .1, coord), src.1));
        } else {
            r.push(*src);
        }
    }
    r
}

fn exclude_rect(a: &[Cube], b: &Cube) -> Vec<Cube> {
    let mut r = Vec::new();
    for src in a.iter() {
        if !intersects(src, b) {
            r.push(*src);
        }
    }
    r
}

fn carve_rect(a: &Cube, b: &Cube) -> Vec<Cube> {
    let rects = vec![*a];
    if !intersects(a, b) {
        return rects;
    }
    let rects = split_rect(&rects, b.0 .0, (true, false, false));
    let rects = split_rect(&rects, b.1 .0, (true, false, false));
    let rects = split_rect(&rects, b.0 .1, (false, true, false));
    let rects = split_rect(&rects, b.1 .1, (false, true, false));
    let rects = split_rect(&rects, b.0 .2, (false, false, true));
    let rects = split_rect(&rects, b.1 .2, (false, false, true));

    exclude_rect(&rects, b)
}

pub fn run_2(input: &[Command]) -> usize {
    let mut state: Vec<Cube> = vec![];
    for cmd in input.iter() {
        if !is_empty_rect(&cmd.1) {
            let mut state2 = vec![];
            for c in state.iter() {
                let mut carved_cubes = carve_rect(c, &cmd.1);
                state2.append(&mut carved_cubes);
            }
            if cmd.0 {
                state2.push(cmd.1);
            }
            state = state2;
        }
    }
    println!("{:?}", state);
    state
        .iter()
        .map(|(p0, p1)| ((p1.0 - p0.0) * (p1.1 - p0.1) * (p1.2 - p0.2)) as usize)
        .sum()
}

pub fn run(input: &[Command]) -> (usize, usize) {
    let mut map = vec![vec![vec![false; 101]; 101]; 101];
    for &(state, (p0, p1)) in input.iter() {
        for (z, y, x) in iproduct!(
            p0.2.max(-50)..p1.2.min(51),
            p0.1.max(-50)..p1.1.min(51),
            p0.0.max(-50)..p1.0.min(51)
        ) {
            map[(z + 50) as usize][(y + 50) as usize][(x + 50) as usize] = state;
        }
    }
    let r1 = iproduct!(-50..51, -50..51, -50..51)
        .filter(|&(z, y, x)| map[(z + 50) as usize][(y + 50) as usize][(x + 50) as usize])
        .count();
    let r2 = run_2(input);

    (r1, r2)
}

pub fn day22(args: &crate::File) -> Result<()> {
    let input = read_input(args)?;
    println!("{:?}", input);

    let result = run(&input);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
