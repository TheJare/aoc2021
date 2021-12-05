use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate aoc2021;

use aoc2021::day5;
use aoc2021::File;

fn day5_full(c: &mut Criterion) {
    c.bench_function("day5_full", |b| {
        b.iter(|| {
            black_box(day5::day5(&File {
                file: PathBuf::from("data/day5.txt"),
            }))
        })
    });
}

fn day5_read(c: &mut Criterion) {
    c.bench_function("day5_read", |b| {
        b.iter(|| {
            black_box(day5::read_input(&File {
                file: PathBuf::from("data/day5.txt"),
            }))
        })
    });
}

fn day5_run(c: &mut Criterion) {
    let r = day5::read_input(&File {
        file: PathBuf::from("data/day5.txt"),
    })
    .unwrap();
    c.bench_function("day5_run", |b| {
        b.iter_batched(
            || (r.0.clone(), r.1),
            |r| black_box(day5::run(r.0, r.1)),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(day5, day5_full, day5_read, day5_run);
criterion_main!(day5);
