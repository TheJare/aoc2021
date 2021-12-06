extern crate aoc2021;

use aoc2021::day6;
use aoc2021::day6::Day6Args;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

fn day6_full(c: &mut Criterion) {
    c.bench_function("day6_full", |b| {
        b.iter(|| {
            black_box(day6::day6(&Day6Args {
                file: PathBuf::from("data/day6.txt"),
                days: 256,
            }))
        })
    });
}

fn day6_read(c: &mut Criterion) {
    c.bench_function("day6_read", |b| {
        b.iter(|| black_box(day6::read_input(&PathBuf::from("data/day6.txt"))))
    });
}

fn day6_run(c: &mut Criterion) {
    let r = day6::read_input(&PathBuf::from("data/day6.txt")).unwrap();
    c.bench_function("day6_run", |b| {
        b.iter_batched(
            || r.clone(),
            |mut r| black_box(day6::run(&mut r, 0..256)),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(day6, day6_full, day6_read, day6_run);
criterion_main!(day6);
