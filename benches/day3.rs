use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate aoc2021;

use aoc2021::day3;
use aoc2021::File;

fn day3_full(c: &mut Criterion) {
    c.bench_function("day3_full", |b| {
        b.iter(|| {
            black_box(day3::day3(&File {
                file: PathBuf::from("data/day4.txt"),
            }))
        })
    });
}

criterion_group!(day3, day3_full);
criterion_main!(day3);
