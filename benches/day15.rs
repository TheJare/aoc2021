extern crate aoc2021;

use aoc2021::day15;
use aoc2021::File;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

fn day15_full(c: &mut Criterion) {
    c.bench_function("day15_full", |b| {
        b.iter(|| {
            black_box(day15::day15(&File {
                file: PathBuf::from("data/day15.txt"),
            }))
        })
    });
}

fn day15_read(c: &mut Criterion) {
    c.bench_function("day15_read", |b| {
        b.iter(|| {
            black_box(day15::read_input(&File {
                file: PathBuf::from("data/day15.txt"),
            }))
        })
    });
}

fn day15_run(c: &mut Criterion) {
    let r = day15::read_input(&File {
        file: PathBuf::from("data/day15.txt"),
    })
    .unwrap();
    c.bench_function("day15_run", |b| b.iter(|| black_box(day15::run(&r))));
}

criterion_group!(day15, day15_full, day15_read, day15_run);
criterion_main!(day15);
