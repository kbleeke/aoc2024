use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<i32>>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> i32 {
    input
        .iter()
        .filter(|line| {
            let ascending = line[1] > line[0];
            line.windows(2).all(|w| {
                let dist = if ascending { w[1] - w[0] } else { w[0] - w[1] };
                (1..=3).contains(&dist)
            })
        })
        .count() as i32
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> i32 {
    input
        .iter()
        .filter(|line| {
            let ascending = line[1] > line[0];
            line.windows(2).all(|w| {
                let dist = if ascending { w[1] - w[0] } else { w[0] - w[1] };
                (1..=3).contains(&dist)
            })
        })
        .count() as i32
}
