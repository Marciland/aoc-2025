use aoc_2025::read_input;
use criterion::{Criterion, criterion_group, criterion_main};

const START: i16 = 50;

fn part1(input: &Vec<String>) -> u16 {
    let mut current = START;
    let mut result = 0;

    for l in input {
        let (direction, distance_str) = l.split_at(1);
        let distance = distance_str.parse::<i16>().unwrap();

        match direction {
            "L" => {
                current -= distance;
            }
            "R" => {
                current += distance;
            }
            _ => panic!(),
        }

        current %= 100;

        if current == 0 {
            result += 1;
        }
    }

    result
}

const fn part2(_input: &Vec<String>) {
    //
}

fn bench_part1(c: &mut Criterion) {
    let input = read_input(1);

    let result = part1(&input);
    println!("day1 part1 result: {result}");

    c.bench_function("day1 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let input = read_input(1);

    // let result = part2(&input);
    // println!("day1 part2 result: {result}");

    c.bench_function("day1 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
