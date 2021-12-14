extern crate aoc2021;

use aoc2021::day14;
use aoc2021::File;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

fn day14_full(c: &mut Criterion) {
    c.bench_function("day14_full", |b| {
        b.iter(|| {
            black_box(day14::day14(&File {
                file: PathBuf::from("data/day14.txt"),
            }))
        })
    });
}

fn day14_read(c: &mut Criterion) {
    c.bench_function("day14_read", |b| {
        b.iter(|| {
            black_box(day14::read_input(&File {
                file: PathBuf::from("data/day14.txt"),
            }))
        })
    });
}

fn day14_run(c: &mut Criterion) {
    let r = day14::read_input(&File {
        file: PathBuf::from("data/day14.txt"),
    })
    .unwrap();
    c.bench_function("day14_run", |b| {
        b.iter(|| black_box(day14::run(&r.0, &r.1)))
    });
}

criterion_group!(day14, day14_full, day14_read, day14_run);
criterion_main!(day14);
