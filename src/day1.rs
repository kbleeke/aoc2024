use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Numbers = Vec<(i32, i32)>;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<(i32, i32)> {
    let mut v = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let l = iter.next().unwrap();
        let r = iter.next().unwrap();

        v.push((
            l.trim().parse::<i32>().unwrap(),
            r.trim().parse::<i32>().unwrap(),
        ));
    }

    v
}

#[aoc(day1, part1)]
pub fn part1(input: &Numbers) -> i32 {
    let mut left: Vec<_> = input.iter().map(|(l, _)| *l).collect();
    let mut right: Vec<_> = input.iter().map(|(_, r)| *r).collect();

    left.sort_unstable();
    right.sort_unstable();

    let sum = left
        .iter()
        .zip(&right)
        .fold(0, |acc, (l, r)| acc + (l - r).abs());

    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &Numbers) -> i32 {
    let left: Vec<_> = input.iter().map(|(l, _)| *l).collect();
    let mut right = HashMap::with_capacity(left.len());

    for r in input.iter().map(|(_, r)| *r) {
        *right.entry(r).or_insert(0) += 1;
    }

    let sum = left
        .iter()
        .fold(0, |acc, l| acc + l * right.get(l).copied().unwrap_or(0));

    sum
}
