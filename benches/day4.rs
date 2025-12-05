use aoc_2025::fetch_aoc_input;
use criterion::{Criterion, criterion_group, criterion_main};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
struct Pos2D {
    x: i32,
    y: i32,
}

impl Pos2D {
    fn neighbours(&self) -> Vec<Self> {
        vec![
            Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            Self {
                x: self.x,
                y: self.y - 1,
            },
            Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Self {
                x: self.x - 1,
                y: self.y,
            },
            Self {
                x: self.x + 1,
                y: self.y,
            },
            Self {
                x: self.x - 1,
                y: self.y + 1,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
            Self {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

fn paper_neighbours(grid: &HashMap<Pos2D, bool>, pos: &Pos2D) -> usize {
    pos.neighbours()
        .iter()
        .filter(|p| grid.contains_key(p) && *grid.get(p).unwrap())
        .count()
}

fn parse_grid(input: &str) -> HashMap<Pos2D, bool> {
    let lines: Vec<String> = input.split('\n').map(str::to_string).collect();
    let mut grid = HashMap::with_capacity(input.len());

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                Pos2D {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                },
                c == '@',
            );
        }
    }

    grid
}

fn part1(input: &str) -> u64 {
    let grid = parse_grid(input);

    let mut result = 0;

    for (pos, is_paper) in &grid {
        if !is_paper {
            continue;
        }

        if paper_neighbours(&grid, pos) < 4 {
            result += 1;
        }
    }

    result
}

const fn part2(_input: &str) {
    //
}

fn bench_part1(c: &mut Criterion) {
    let input = fetch_aoc_input(4);

    let result = part1(&input);
    println!("day4 part1 result: {result}");

    c.bench_function("day4 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    let input = fetch_aoc_input(1);

    // let result = part2(&input);
    // println!("day4 part2 result: {result}");

    c.bench_function("day4 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
