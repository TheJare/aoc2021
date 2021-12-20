use crate::utils::read_from_file;
use anyhow::Result;
use itertools::{iproduct, Itertools};

// https://adventofcode.com/2021/day/20

type Algorithm = Vec<u8>;

pub fn read_input(args: &crate::File) -> Result<(Algorithm, Vec<Vec<u8>>)> {
    let lines = read_from_file(&args.file)?;
    let lines = lines.flatten().collect_vec();

    let algorithm: Algorithm = lines[0].chars().map(|c| (c == '#') as u8).collect();
    let w = lines[2].len();
    let h = lines.len() - 2;
    let mut map = vec![vec![0u8; w + 220]; h + 220];
    for (i, j) in iproduct!(0..h, 0..w) {
        map[i + 110][j + 110] = (lines[i + 2].chars().nth(j).unwrap() == '#') as u8;
        // hm
    }
    Ok((algorithm, map))
}

pub fn count(map: &Vec<Vec<u8>>, d: usize) -> usize {
    let mut r = 0;
    for (y, x) in iproduct!(d..map.len() - d, d..map[0].len() - d) {
        r += map[y][x] as usize;
    }
    r
}

pub fn run_step(algorithm: &Algorithm, map: &mut Vec<Vec<u8>>, d: usize) {
    let mut new_map = vec![vec![0u8; map[0].len()]; map.len()];
    for (y, x) in iproduct!(d..map.len() - d, d..map[0].len() - d) {
        let mut v = 0;
        for (i, j) in iproduct!(y - 1..y + 2, x - 1..x + 2) {
            v = v * 2 + map[i][j] as usize;
        }
        new_map[y][x] = algorithm[v];
    }
    *map = new_map;
}

pub fn run(algorithm: Algorithm, mut map: Vec<Vec<u8>>) -> (usize, usize) {
    run_step(&algorithm, &mut map, 1);
    run_step(&algorithm, &mut map, 1);
    let r1 = count(&map, 3);
    for _ in 2..50 {
        run_step(&algorithm, &mut map, 1);
    }
    let r2 = count(&map, 55);
    (r1, r2)
}

pub fn day20(args: &crate::File) -> Result<()> {
    let input = read_input(args)?;

    let result = run(input.0, input.1);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
