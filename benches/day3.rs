use aoc_2025::read_input;
use criterion::{Criterion, criterion_group, criterion_main};

fn part1(input: &Vec<String>) -> u32 {
    let mut result: u32 = 0;

    for bank in input {
        let mut highest_battery: (usize, u32) = (0, 0);

        let batteries = bank.chars().map(|c| c.to_digit(10).unwrap());
        for battery in batteries.clone().enumerate() {
            if battery.1 > highest_battery.1 {
                highest_battery = battery;
            }
        }

        let mut second_highest: u32 = 0;

        let is_last = highest_battery.0 == batteries.clone().count() - 1;

        if is_last {
            for battery in batteries.rev().enumerate() {
                if battery.0 == 0 {
                    continue;
                }
                if battery.1 > second_highest {
                    second_highest = battery.1;
                }
            }
        } else {
            for battery in batteries.enumerate() {
                if battery.0 <= highest_battery.0 {
                    continue;
                }
                if battery.1 > second_highest {
                    second_highest = battery.1;
                }
            }
        }

        result += if is_last {
            second_highest * 10 + highest_battery.1
        } else {
            highest_battery.1 * 10 + second_highest
        };
    }

    result
}

const fn part2() {
    //
}

fn bench_part1(c: &mut Criterion) {
    let input = read_input(3);

    let result = part1(&input);
    println!("day3 part1 result: {result}");

    c.bench_function("day3 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    // let input = read_input(3);

    // let result = part2(&input);
    // println!("day3 part2 result: {result}");

    c.bench_function("day3 part2", |b| b.iter(part2));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
