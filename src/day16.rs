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
    println!("{:?}", input);
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

    pub fn with_pos(source: &'a [u8], pos: usize, len: usize) -> Self {
        Self {
            source,
            pos,
            len: pos + len,
        }
    }

    pub fn bits_left(&self) -> usize {
        self.len - self.pos
    }

    pub fn get(&mut self, n: usize) -> usize {
        if self.pos + n <= self.len {
            let mut pending = n;
            let mut v = 0usize;
            while pending > 0 {
                let bpos = self.pos & 7;
                let bit = ((self.source[self.pos >> 3] >> (7 - bpos)) & 1) as usize;
                v = (v << 1) + bit;
                // println!(
                //     "getting bit {} -> {} of {} (byte is {})",
                //     self.pos,
                //     bit,
                //     pending,
                //     self.source[self.pos >> 3]
                // );
                self.pos += 1;
                pending -= 1;
            }
            // println!("value {}", v);
            v
        } else {
            panic!("ran out of bits");
        }
    }

    pub fn get_contents(&mut self, indent: usize) -> (usize, usize) {
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
            println!(
                "{:indent$}version {} op {} value {}",
                "",
                version,
                ptype,
                value,
                indent = indent
            );
        } else {
            println!(
                "{:indent$}version {} op {}",
                "",
                version,
                ptype,
                indent = indent
            );
            let len_type = self.get(1);
            let mut values = vec![0; 0];
            if len_type == 0 {
                let subp_len = self.get(15);
                let mut sub_decoder = Decoder::with_pos(self.source, self.pos, subp_len);
                while sub_decoder.bits_left() > 0 {
                    let (versions, value) = sub_decoder.get_contents(indent + 2);
                    version += versions;
                    values.push(value);
                }
                self.pos += subp_len;
            } else {
                let subp_num = self.get(11);
                for _ in 0..subp_num {
                    let (versions, value) = self.get_contents(indent + 2);
                    version += versions;
                    values.push(value);
                }
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
            println!("{:indent$}value {}", "", value, indent = indent);
        }
        (version, value)
    }
}

pub fn run(input: &[u8]) -> (usize, usize) {
    let mut decoder = Decoder::new(input);
    decoder.get_contents(0)
}

pub fn day16(args: &crate::File) -> Result<()> {
    let map = read_input(args)?;

    let result = run(&map);

    println!("Result of Part 1 is {}", result.0);
    println!("Result of Part 2 is {}", result.1);

    Ok(())
}
