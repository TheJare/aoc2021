use crate::utils::read_file;
use anyhow::{Context, Result};
use itertools::Itertools;

// https://adventofcode.com/2021/day/9

pub struct Map {
    pub cells: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn get_value(&self, x: i32, y: i32) -> u8 {
        if self.is_valid(x, y) {
            self.cells[(x + y * self.width) as usize]
        } else {
            u8::MAX
        }
    }

    pub fn get_risk_level(&self, x: i32, y: i32) -> usize {
        let v = self.get_value(x, y);
        if self.get_value(x - 1, y) > v
            && self.get_value(x + 1, y) > v
            && self.get_value(x, y - 1) > v
            && self.get_value(x, y + 1) > v
        {
            v as usize + 1
        } else {
            0
        }
    }

    pub fn floodfill(&mut self, x: i32, y: i32) -> usize {
        let mut size = 0;
        if self.get_value(x, y) < 9 {
            let mut stack = vec![(x, y)];
            while let Some((x, y)) = stack.pop() {
                if self.get_value(x, y) < 9 {
                    size += 1;
                    self.cells[(x + y * self.width) as usize] = 9;
                    if self.get_value(x - 1, y) < 9 {
                        stack.push((x - 1, y));
                    }
                    if self.get_value(x + 1, y) < 9 {
                        stack.push((x + 1, y));
                    }
                    if self.get_value(x, y - 1) < 9 {
                        stack.push((x, y - 1));
                    }
                    if self.get_value(x, y + 1) < 9 {
                        stack.push((x, y + 1));
                    }
                }
            }
        }
        size
    }
}

pub fn read_input(args: &crate::File) -> Result<Map> {
    let file = read_file(&args.file)?;
    let height = file.split_ascii_whitespace().count();
    let cells = file
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| (c as u8) - b'0')
        .collect_vec();
    let width = cells.len() / height;
    Ok(Map {
        cells,
        width: width as i32,
        height: height as i32,
    })
}

pub fn run(map: &mut Map) -> Result<(usize, usize)> {
    let mut sum = 0;
    let mut basins: Vec<usize> = Vec::new();
    for y in 0..map.height {
        for x in 0..map.width {
            sum += map.get_risk_level(x, y);
        }
    }
    for y in 0..map.height {
        for x in 0..map.width {
            let basin_size = map.floodfill(x, y);
            if basin_size > 0 {
                basins.push(basin_size);
            }
        }
    }
    basins.sort_unstable_by(|a, b| b.cmp(a));
    let basin_value = basins.into_iter().take(3).product::<usize>();
    Ok((sum, basin_value))
}

pub fn day9(args: &crate::File) -> Result<()> {
    let mut map = read_input(args)?;

    let result = run(&mut map)?;

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
