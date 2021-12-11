use crate::utils::read_file;
use anyhow::Result;
use itertools::FoldWhile::{Continue, Done};
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;

// https://adventofcode.com/2021/day/11

pub struct Map {
    pub cells: Vec<i8>,
    pub width: i32,
    pub height: i32,
}

lazy_static! {
    static ref DIRS: Vec<(i32, i32)> = iproduct!(-1..=1, -1..=1)
        .filter(|&(x, y)| x != 0 || y != 0)
        .collect_vec();
}

impl Map {
    pub fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn value(&mut self, x: i32, y: i32) -> Option<&mut i8> {
        self.is_valid(x, y)
            .then(|| self.cells.get_mut((x + y * self.width) as usize))
            .flatten()
    }

    pub fn increase_value(&mut self, x: i32, y: i32) {
        if let Some(v) = self.value(x, y) {
            if *v >= 9 {
                *v = -1;
                DIRS.iter()
                    .for_each(|(dx, dy)| self.increase_value(x + dx, y + dy));
            } else if *v >= 0 {
                *v += 1;
            }
        }
    }

    pub fn sim(&mut self) -> usize {
        for (y, x) in iproduct!(0..self.height, 0..self.width) {
            self.increase_value(x, y);
        }
        self.cells
            .iter_mut()
            .filter(|c| **c < 0)
            .map(|c| *c = 0)
            .count()
    }
}

pub fn read_input(args: &crate::File) -> Result<Map> {
    let file = read_file(&args.file)?;
    let height = file.split_ascii_whitespace().count();
    let cells = file
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| (c as i8) - b'0' as i8)
        .collect_vec();
    let width = cells.len() / height;
    Ok(Map {
        cells,
        width: width as i32,
        height: height as i32,
    })
}

pub fn run(map: &mut Map) -> (usize, usize) {
    let l = (1..usize::MAX).fold_while((Err(0), None), |(prev_flashes, prev_steps), step| {
        let num_flashes = map.sim();
        let total_flashes = prev_flashes.or_else(|flashes| {
            if step > 100 {
                Ok(flashes)
            } else {
                Err(flashes + num_flashes)
            }
        });
        let steps_to_sync =
            prev_steps.or_else(|| (num_flashes == (map.width * map.height) as usize).then(|| step));
        let cur = (total_flashes, steps_to_sync);
        if let (Ok(_), Some(_)) = cur {
            Done(cur)
        } else {
            Continue(cur)
        }
    });
    if let (Ok(r1), Some(r2)) = l.into_inner() {
        (r1, r2)
    } else {
        (0, 0)
    }
}

pub fn day11(args: &crate::File) -> Result<()> {
    let mut map = read_input(args)?;

    let result = run(&mut map);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
