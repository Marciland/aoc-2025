use aoc_2025::{Pos2D, fetch_aoc_input};
use criterion::{Criterion, criterion_group, criterion_main};
use std::collections::HashMap;

struct Manifold {
    start: Pos2D,
    diagram: HashMap<Pos2D, bool>,
}

fn parse_input(input: &str) -> Manifold {
    let mut diagram = HashMap::new();

    let mut start = Pos2D { x: -100, y: -100 };
    for (y_index, line) in input.split('\n').enumerate() {
        for (x_index, char) in line.chars().enumerate() {
            if char == 'S' {
                start.x = i32::try_from(x_index).unwrap();
                start.y = i32::try_from(y_index).unwrap();
            }
            diagram.insert(
                Pos2D {
                    x: i32::try_from(x_index).unwrap(),
                    y: i32::try_from(y_index).unwrap(),
                },
                char == '^',
            );
        }
    }

    assert!(!start.x.is_negative() && !start.y.is_negative());

    Manifold { start, diagram }
}

fn run_beam_and_count_splits(
    start: Pos2D,
    diagram: &HashMap<Pos2D, bool>,
    visited: &mut Vec<Pos2D>,
) -> u64 {
    let mut splits = 0;
    let mut current = start;

    loop {
        visited.push(current.clone());
        let below = current.below();
        let Some(splitter_below) = diagram.get(&below) else {
            break; // beam ended
        };
        if visited.contains(&below) {
            break; // beam merged
        }
        if *splitter_below {
            splits += 1;

            let left = below.left();
            if !visited.contains(&left) {
                splits += run_beam_and_count_splits(left, diagram, visited);
            }

            let right = below.right();
            if !visited.contains(&right) {
                splits += run_beam_and_count_splits(right, diagram, visited);
            }
            break; // beam splitted
        }
        current = below;
    }

    splits
}

fn part1(input: &str) -> u64 {
    let manifold = parse_input(input);

    run_beam_and_count_splits(manifold.start, &manifold.diagram, &mut Vec::new())
}

const fn part2(_input: &str) {
    //
}

fn bench_part1(c: &mut Criterion) {
    let input = fetch_aoc_input(7);

    let result = part1(&input);
    println!("day7 part1 result: {result}");

    c.bench_function("day7 part1", |b| b.iter(|| part1(&input)));
}

fn bench_part2(c: &mut Criterion) {
    // let input = fetch_aoc_input(1);
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
        .to_string();

    // let result = part2(&input);
    // println!("day7 part2 result: {result}");

    c.bench_function("day7 part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
