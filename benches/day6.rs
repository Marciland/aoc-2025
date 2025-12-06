use aoc_2025::fetch_aoc_input;
use criterion::{Criterion, criterion_group, criterion_main};

struct Problem {
    numbers: Vec<u64>,
    operator: String,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operator.as_str() {
            "*" => {
                let mut result = 1;

                for number in &self.numbers {
                    result *= number;
                }

                result
            }
            "+" => {
                let mut result = 0;

                for number in &self.numbers {
                    result += number;
                }

                result
            }
            _ => panic!("invalid operator"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Problem> {
    let lines: Vec<_> = input.split('\n').collect();

    let mut problems: Vec<Problem> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        for (x_index, value) in line
            .split(' ')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .enumerate()
        {
            if index == 0 {
                problems.push(Problem {
                    numbers: Vec::with_capacity(lines.len() - 1),
                    operator: String::new(),
                });
            }

            if index == lines.len() - 1 {
                problems[x_index].operator = value.to_string();
            } else {
                problems[x_index].numbers.push(value.parse().unwrap());
            }
        }
    }

    problems
}

fn part1(input: &str) -> u64 {
    let problems = parse_input(input);

    let mut result = 0;

    for problem in problems {
        result += problem.solve();
    }

    result
}

const fn part2(_input: &str) {
    //
}

fn bench_part1(c: &mut Criterion) {
    let input = fetch_aoc_input(6);

    let result = part1(&input);
    println!("day6 part1 result: {result}");

    c.bench_function("day6 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    // let input = fetch_aoc_input(6);
    let input = "123 328  51 64
45 64  387 23
6 98  215 314
*   +   *   +  "
        .to_owned();
    // let result = part2(&input);
    // println!("day6 part2 result: {result}");

    c.bench_function("day6 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
