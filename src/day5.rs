use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

struct Puzzle {
    rules: HashMap<i32, Vec<i32>>,
    updates: Vec<Vec<i32>>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Puzzle {
    let mut parts = input.split("\n\n");

    let rules = parts.next().unwrap();
    let updates = parts.next().unwrap();

    let mut rulesmap = HashMap::<i32, Vec<i32>>::new();
    for rule in rules.lines() {
        let (left, right) = rule.split_once("|").unwrap();
        let left = left.parse().unwrap();
        let right = right.parse().unwrap();

        rulesmap.entry(right).or_default().push(left);
    }

    let updates = updates
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    Puzzle {
        rules: rulesmap,
        updates,
    }
}

#[aoc(day5, part1)]
fn part1(input: &Puzzle) -> i32 {
    let mut sum = 0;
    'updateloop: for update in input.updates.iter() {
        let allset: HashSet<i32> = update.iter().copied().collect();
        let mut prevset = HashSet::<i32>::new();

        for x in update {
            if let Some(requirements) = input.rules.get(x) {
                for req in requirements {
                    if allset.contains(req) && !prevset.contains(req) {
                        println!("update {:?}", update);
                        println!("not in order because it contains {req} but not before {x}");
                        continue 'updateloop;
                    }
                }
            }
            prevset.insert(*x);
        }

        let imiddle = update.len() / 2;
        let middle = update[imiddle];
        println!("middle: {imiddle} => {middle}");
        sum += middle;
    }
    sum
}
