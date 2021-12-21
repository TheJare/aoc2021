extern crate aoc2021;

use aoc2021::day21;
use aoc2021::File;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

fn day21_full(c: &mut Criterion) {
    c.bench_function("day21_full", |b| {
        b.iter(|| {
            black_box(day21::day21(&File {
                file: PathBuf::from("data/day21.txt"),
            }))
        })
    });
}

fn day21_read(c: &mut Criterion) {
    c.bench_function("day21_read", |b| {
        b.iter(|| {
            black_box(day21::read_input(&File {
                file: PathBuf::from("data/day21.txt"),
            }))
        })
    });
}

fn day21_run(c: &mut Criterion) {
    let r = day21::read_input(&File {
        file: PathBuf::from("data/day21.txt"),
    })
    .unwrap();
    c.bench_function("day21_run", |b| b.iter(|| black_box(day21::run(r))));
}

criterion_group!(day21, day21_full, day21_read, day21_run);
criterion_main!(day21);
