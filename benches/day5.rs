use aoc_2025::fetch_aoc_input;
use criterion::{Criterion, criterion_group, criterion_main};
use std::ops::RangeInclusive;

struct Database {
    ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

fn parse_input(input: &str) -> Database {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    let mut reached_middle = false;
    for line in input.split('\n') {
        if line.is_empty() {
            reached_middle = true;
            continue;
        }
        if reached_middle {
            ids.push(line.parse().unwrap());
            continue;
        }
        let (start, end): (u64, u64) = line
            .split_once('-')
            .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
            .unwrap();
        ranges.push(start..=end);
    }

    Database { ranges, ids }
}

fn part1(input: &str) -> u64 {
    let mut result = 0;

    let database = parse_input(input);

    for id in database.ids {
        for range in &database.ranges {
            if range.contains(&id) {
                result += 1;
                break;
            }
        }
    }

    result
}

fn part2(input: &str) -> u64 {
    let mut database = parse_input(input);
    database.ranges.sort_by(|a, b| a.start().cmp(b.start()));
    database.ranges.reverse();

    let mut result = 0;

    let mut continous: Vec<RangeInclusive<u64>> = Vec::new();
    'outer: loop {
        let Some(popped) = database.ranges.pop() else {
            break;
        };

        let (mut start, end) = (*popped.start(), *popped.end());
        for range in &continous {
            if range.contains(&start) {
                start = range.end() + 1;
            }
            if start > end {
                continue 'outer;
            }
        }

        continous.push(start..=end);
    }

    for range in continous {
        result += range.end() - range.start() + 1;
    }

    result
}

fn bench_part1(c: &mut Criterion) {
    let input = fetch_aoc_input(5);

    let result = part1(&input);
    println!("day5 part1 result: {result}");

    c.bench_function("day5 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let input = fetch_aoc_input(5);

    let result = part2(&input);
    println!("day5 part2 result: {result}");

    c.bench_function("day5 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
