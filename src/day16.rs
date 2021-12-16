use crate::utils::read_file;
use anyhow::Result;
use itertools::Itertools;

// https://adventofcode.com/2021/day/16

pub fn read_input(args: &crate::File) -> Result<Vec<u8>> {
    let file = read_file(&args.file)?;
    let input = (0..file.len() / 2)
        .flat_map(|i| u8::from_str_radix(&file[i * 2..i * 2 + 2], 16))
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

    pub fn get_bits(&mut self, n: usize) -> usize {
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
        let mut version = self.get_bits(3);
        let ptype = self.get_bits(3);
        let mut value = 0;
        if ptype == 4 {
            loop {
                let d = self.get_bits(5);
                value = (value << 4) + (d & 0xF);
                if (d & 0x10) == 0 {
                    break;
                }
            }
        } else {
            let num_is_bits = self.get_bits(1) == 0;
            let mut values = vec![];
            let mut num = self.get_bits(if num_is_bits { 15 } else { 11 });
            let len = if num_is_bits {
                self.pos + num
            } else {
                self.len
            };
            while self.pos < len && num > 0 {
                let (versions, value) = self.get_contents();
                version += versions;
                values.push(value);
                num -= !num_is_bits as usize; // can just sub 1 always but meh
            }
            value = match (ptype, &values[..]) {
                (0, _) => values.iter().sum::<usize>(),
                (1, _) => values.iter().product(),
                (2, _) if !values.is_empty() => *values.iter().min().unwrap(),
                (3, _) if !values.is_empty() => *values.iter().max().unwrap(),
                (5, &[a, b]) => (a > b) as usize,
                (6, &[a, b]) => (a < b) as usize,
                (7, &[a, b]) => (a == b) as usize,
                _ => panic!("unknown operator {} or broken operands {:?}", ptype, values),
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
