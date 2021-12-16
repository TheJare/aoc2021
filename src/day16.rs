use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/16

fn hex_value(c: char) -> u8 {
    let c = c as u8;
    if c >= b'a' {
        c - b'a' + 10
    } else if c >= b'A' {
        c - b'A' + 10
    } else {
        c - b'0'
    }
}

pub fn read_input(args: &crate::File) -> Result<Vec<u8>> {
    let file = read_file(&args.file)?;
    let input = file
        .chars()
        .tuples()
        .map(|(h, l)| (hex_value(h) << 4) | hex_value(l))
        .collect_vec();
    Ok(input)
}

struct Decoder<'a> {
    pub source: &'a [u8],
    pub pos: usize,
    pub len: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            pos: 0,
            len: source.len() * 8,
        }
    }

    pub fn get(&mut self, n: usize) -> usize {
        if self.pos + n <= self.len {
            let mut v = 0usize;
            for nbit in self.pos..(self.pos + n) {
                let bpos = 7 - (nbit & 7);
                let bit = ((self.source[nbit / 8] >> bpos) & 1) as usize;
                v = v * 2 + bit;
            }
            self.pos += n;
            v
        } else {
            panic!("ran out of bits");
        }
    }

    pub fn get_contents(&mut self) -> (usize, usize) {
        let mut version = self.get(3);
        let ptype = self.get(3);
        let mut value = 0;
        if ptype == 4 {
            loop {
                let d = self.get(5);
                value = (value << 4) + (d & 0xF);
                if (d & 0x10) == 0 {
                    break;
                }
            }
        } else {
            let len_type = self.get(1);
            let mut values = vec![0; 0];
            let mut num = self.get(if len_type == 0 { 15 } else { 11 });
            let len = if len_type == 0 {
                self.pos + num
            } else {
                self.len
            };
            while self.pos < len && num > 0 {
                let (versions, value) = self.get_contents();
                version += versions;
                values.push(value);
                num -= (len_type != 0) as usize; // can just sub 1 always but meh
            }
            value = match ptype {
                0 => values.iter().sum::<usize>(),
                1 => values.iter().product(),
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => (values[0] > values[1]) as usize,
                6 => (values[0] < values[1]) as usize,
                7 => (values[0] == values[1]) as usize,
                _ => panic!("unknown operator {}", ptype),
            };
        }
        (version, value)
    }
}

pub fn run(input: &[u8]) -> (usize, usize) {
    let mut decoder = Decoder::new(input);
    decoder.get_contents()
}

pub fn day16(args: &crate::File) -> Result<()> {
    let map = read_input(args)?;

    let result = run(&map);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
