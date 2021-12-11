use crate::utils::read_file;
use anyhow::Result;
use itertools::FoldWhile::{Continue, Done};
use itertools::{iproduct, Itertools};

// https://adventofcode.com/2021/day/11

pub struct Map {
    pub cells: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut u8> {
        self.cells.get_mut((x + y * self.width) as usize)
    }

    pub fn increase_value(&mut self, x: i32, y: i32) {
        if let Some(v) = self.get_mut(x, y) {
            if *v < 10 {
                *v += 1;
                if *v == 10 {
                    iproduct!(
                        0.max(y - 1)..self.height.min(y + 2),
                        0.max(x - 1)..self.width.min(x + 2)
                    )
                    .for_each(|(y, x)| self.increase_value(x, y));
                }
            }
        }
    }

    pub fn sim(&mut self) -> usize {
        for (y, x) in iproduct!(0..self.height, 0..self.width) {
            self.increase_value(x, y);
        }
        self.cells
            .iter_mut()
            .filter_map(|c| (*c == 10).then(|| *c = 0))
            .count()
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

pub fn run(map: &mut Map) -> (usize, usize) {
    let (r1, r2) = (1..usize::MAX)
        .fold_while((Err(0), None), |(prev_flashes, prev_steps), step| {
            let num_flashes = map.sim();
            // Track result of part 1 in a Result, Ok once found
            let total_flashes = prev_flashes.or_else(|flashes| {
                if step > 100 {
                    Ok(flashes)
                } else {
                    Err(flashes + num_flashes)
                }
            });
            // Track result of part 2 in an Option, Some once found
            let steps_to_sync = prev_steps
                .or_else(|| (num_flashes == (map.width * map.height) as usize).then(|| step));
            // If both results are complete, stop folding and return them in a Done
            let cur = (total_flashes, steps_to_sync);
            if let (Ok(_), Some(_)) = cur {
                Done(cur)
            } else {
                Continue(cur)
            }
        })
        .into_inner();
    (r1.unwrap(), r2.unwrap())
}

pub fn day11(args: &crate::File) -> Result<()> {
    let mut map = read_input(args)?;

    let result = run(&mut map);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
