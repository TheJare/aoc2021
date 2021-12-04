use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate aoc2021;

use aoc2021::day4;
use aoc2021::File;

fn day4_full(c: &mut Criterion) {
    c.bench_function("day4 full", |b| {
        b.iter(|| {
            black_box(day4::day4(&File {
                file: PathBuf::from("data/day4.txt"),
            }))
        })
    });
}

fn day4_read(c: &mut Criterion) {
    c.bench_function("day4 read", |b| {
        b.iter(|| {
            black_box(day4::read_input(&File {
                file: PathBuf::from("data/day4.txt"),
            }))
        })
    });
}

fn day4_run(c: &mut Criterion) {
    let r = day4::read_input(&File {
        file: PathBuf::from("data/day4.txt"),
    })
    .unwrap();
    c.bench_function("day4 run", |b| {
        b.iter_batched(
            || (r.0.clone(), r.1.clone()),
            |r| black_box(day4::run(r.0, r.1)),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, day4_full, day4_read, day4_run);
criterion_main!(benches);
