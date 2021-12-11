use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/9

pub struct Map {
    pub cells: Vec<i8>,
    pub width: i32,
    pub height: i32,
}
const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
];

impl Map {
    pub fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn get_value(&self, x: i32, y: i32) -> i8 {
        if self.is_valid(x, y) {
            self.cells[(x + y * self.width) as usize]
        } else {
            0
        }
    }

    pub fn set_value(&mut self, x: i32, y: i32, v: i8) {
        if self.is_valid(x, y) {
            self.cells[(x + y * self.width) as usize] = v;
        }
    }

    pub fn increase_value(&mut self, x: i32, y: i32) {
        let v = self.get_value(x, y);
        if v >= 9 {
            self.set_value(x, y, -1);
            for (dx, dy) in DIRS.iter() {
                if self.is_valid(x, y) {
                    self.increase_value(x + dx, y + dy);
                }
            }
        } else if v >= 0 {
            self.set_value(x, y, v + 1);
        }
    }

    pub fn sim(&mut self) -> usize {
        for y in 0..self.height {
            for x in 0..self.width {
                self.increase_value(x, y);
            }
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

pub fn run(map: &mut Map) -> Result<(usize, usize)> {
    let mut r1 = 0;
    let mut r2 = usize::MAX;
    for step in 1..usize::MAX {
        let num_flashes = map.sim();
        if step <= 100 {
            r1 += num_flashes;
        }
        if num_flashes == (map.width * map.height) as usize {
            r2 = step
        }
        if step > 100 && r2 != usize::MAX {
            break;
        }
    }
    Ok((r1, r2))
}

pub fn day11(args: &crate::File) -> Result<()> {
    let mut map = read_input(args)?;

    let result = run(&mut map)?;

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
