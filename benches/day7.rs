extern crate aoc2021;

use aoc2021::day7;
use aoc2021::File;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

fn day7_full(c: &mut Criterion) {
    c.bench_function("day7_full", |b| {
        b.iter(|| {
            black_box(day7::day7(&File {
                file: PathBuf::from("data/day7.txt"),
            }))
        })
    });
}

fn day7_read(c: &mut Criterion) {
    c.bench_function("day7_read", |b| {
        b.iter(|| black_box(day7::read_input(&PathBuf::from("data/day7.txt"))))
    });
}

fn day7_run(c: &mut Criterion) {
    let r = day7::read_input(&PathBuf::from("data/day7.txt")).unwrap();
    c.bench_function("day7_run", |b| b.iter(|| black_box(day7::run(&r))));
}

criterion_group!(day7, day7_full, day7_read, day7_run);
criterion_main!(day7);
