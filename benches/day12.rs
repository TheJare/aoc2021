extern crate aoc2021;

use aoc2021::day12;
use aoc2021::File;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

fn day12_full(c: &mut Criterion) {
    c.bench_function("day12_full", |b| {
        b.iter(|| {
            black_box(day12::day12(&File {
                file: PathBuf::from("data/day12.txt"),
            }))
        })
    });
}

fn day12_read(c: &mut Criterion) {
    c.bench_function("day12_read", |b| {
        b.iter(|| {
            black_box(day12::read_input(&File {
                file: PathBuf::from("data/day12.txt"),
            }))
        })
    });
}

fn day12_run(c: &mut Criterion) {
    let r = day12::read_input(&File {
        file: PathBuf::from("data/day12.txt"),
    })
    .unwrap();
    c.bench_function("day12_run", |b| b.iter(|| black_box(day12::run(&r))));
}

criterion_group!(day12, day12_full, day12_read, day12_run);
criterion_main!(day12);
