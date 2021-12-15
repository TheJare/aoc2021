use crate::utils::read_file;
use anyhow::Result;
use itertools::iproduct;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// https://adventofcode.com/2021/day/15

pub type Point = (usize, usize);
pub type Map = Vec<Vec<u8>>;

pub fn read_input(args: &crate::File) -> Result<Map> {
    let file = read_file(&args.file)?;
    let input = file
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect_vec())
        .collect_vec();
    // Expand it 5x
    let (w, h) = (input[0].len(), input.len());
    let mut xmap = vec![vec![0; w * 5]; h * 5];
    for (y, x) in iproduct!(0..h * 5, 0..w * 5) {
        let xtra = y / h + x / w;
        let v = ((input[y % h][x % w] as usize + xtra - 1) % 9) as u8 + 1;
        xmap[y][x] = v;
    }
    Ok(xmap)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Node {
    pub cost: u16,
    pub pos: Point,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

pub fn run(input: &[Vec<u8>]) -> (usize, usize) {
    let (w, h) = (input[0].len(), input.len());
    let (p1w, p1h) = (w / 5, h / 5);
    let mut open = BinaryHeap::<Node>::new();
    let mut costs = vec![vec![u16::MAX; w]; h];

    open.push(Node {
        cost: 0,
        pos: (0, 0),
    });
    costs[0][0] = 0;

    let mut insert = |x: usize, y: usize, cost: u16, open: &mut BinaryHeap<Node>| {
        let cost = cost + input[y][x] as u16;
        if costs[y][x] > cost {
            costs[y][x] = cost;
            open.push(Node { cost, pos: (x, y) });
        }
    };

    let mut prio_max = 0;
    let mut r1 = usize::MAX;
    while let Some(node) = open.pop() {
        prio_max = prio_max.max(open.len());
        if node.pos == (p1w - 1, p1h - 1) {
            r1 = r1.min(node.cost as usize);
        }
        if node.pos == (w - 1, h - 1) {
            return (r1, node.cost as usize);
        }
        if node.pos.0 < w - 1 {
            insert(node.pos.0 + 1, node.pos.1, node.cost, &mut open);
        }
        if node.pos.0 > 0 {
            insert(node.pos.0 - 1, node.pos.1, node.cost, &mut open);
        }
        if node.pos.1 < h - 1 {
            insert(node.pos.0, node.pos.1 + 1, node.cost, &mut open);
        }
        if node.pos.1 > 0 {
            insert(node.pos.0, node.pos.1 - 1, node.cost, &mut open);
        }
    }
    (r1, 0)
}

pub fn day15(args: &crate::File) -> Result<()> {
    let map = read_input(args)?;

    let result = run(&map);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
