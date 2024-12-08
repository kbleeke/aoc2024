use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Map {
    width: i64,
    grid: Vec<char>,
}

impl Map {
    fn i_to_c(&self, i: usize) -> (i64, i64) {
        let x = i as i64 % self.width;
        let y = i as i64 / self.width;
        (x, y)
    }

    #[allow(dead_code)]
    fn c_to_i(&self, (x, y): (i64, i64)) -> usize {
        (y * self.width + x) as usize
    }

    fn c_in_bounds(&self, (x, y): (i64, i64)) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height()
    }

    fn height(&self) -> i64 {
        self.grid.len() as i64 / self.width
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Map {
    let width = input.lines().next().unwrap().len();

    Map {
        width: width as i64,
        grid: input.lines().flat_map(|line| line.chars()).collect(),
    }
}

#[aoc(day8, part1)]
fn part1(input: &Map) -> i64 {
    let mut freq = HashMap::<char, Vec<(i64, i64)>>::new();
    let mut antinodes = HashSet::<(i64, i64)>::new();

    for (i, &c) in input.grid.iter().enumerate() {
        if c != '.' {
            freq.entry(c).or_default().push(input.i_to_c(i));
        }
    }

    for (_freq, nodes) in freq.iter() {
        for (a, b) in nodes.iter().tuple_combinations() {
            let (ax, ay) = a;
            let (bx, by) = b;

            let ab = (bx - ax, by - ay);

            let anti_plus = (bx + ab.0, by + ab.1);
            let anti_minus = (ax - ab.0, ay - ab.1);

            if input.c_in_bounds(anti_plus) {
                antinodes.insert(anti_plus);
            }
            if input.c_in_bounds(anti_minus) {
                antinodes.insert(anti_minus);
            }
        }
    }

    antinodes.len() as i64
}

#[aoc(day8, part2)]
fn part2(input: &Map) -> i64 {
    let mut freq = HashMap::<char, Vec<(i64, i64)>>::new();
    let mut antinodes = HashSet::<(i64, i64)>::new();

    for (i, &c) in input.grid.iter().enumerate() {
        if c != '.' {
            freq.entry(c).or_default().push(input.i_to_c(i));
        }
    }

    for (_freq, nodes) in freq.iter() {
        for (a, b) in nodes.iter().tuple_combinations() {
            let (ax, ay) = *a;
            let (bx, by) = *b;

            let ab = (bx - ax, by - ay);

            // println!("line {a:?} => {b:?} vector {ab:?}");

            {
                let mut anti_plus = (bx, by);
                loop {
                    // println!("checking {anti_plus:?}");
                    if input.c_in_bounds(anti_plus) {
                        antinodes.insert(anti_plus);
                        anti_plus = (anti_plus.0 + ab.0, anti_plus.1 + ab.1)
                    } else {
                        break;
                    }
                }
            }
            {
                let mut anti_minus = (ax, ay);
                loop {
                    if input.c_in_bounds(anti_minus) {
                        antinodes.insert(anti_minus);
                        anti_minus = (anti_minus.0 - ab.0, anti_minus.1 - ab.1)
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len() as i64
}
