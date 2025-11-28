use criterion::{Criterion, criterion_group, criterion_main};

const fn part1() {
    //
}

const fn part2() {
    //
}

fn bench_part1(c: &mut Criterion) {
    c.bench_function("day5 part1", |b| b.iter(part1));
}

fn bench_part2(c: &mut Criterion) {
    c.bench_function("day5 part2", |b| b.iter(part2));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
