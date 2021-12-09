use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_09::{part_1, part_2, parse_map};

pub fn part_1_bench(c: &mut Criterion) {
    let map = parse_map::<_, 100, 100>(input::lines!());

    c.bench_function("part 1", |b| b.iter(|| part_1(map)));
}

pub fn part_2_bench(c: &mut Criterion) {
    let map = parse_map::<_, 100, 100>(input::lines!());

    c.bench_function("part 2", |b| b.iter(|| part_2(map)));
}

criterion_group!(benchmarks, part_1_bench, part_2_bench);
criterion_main!(benchmarks);