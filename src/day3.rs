use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for m in re.captures_iter(input) {
        let a: i32 = m.get(1).unwrap().as_str().parse().unwrap();
        let b: i32 = m.get(2).unwrap().as_str().parse().unwrap();

        sum += a * b;
    }

    sum
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut enable = true;
    let mut sum = 0;
    for m in re.captures_iter(input) {
        let f = m.get(0).unwrap().as_str();

        if f.starts_with("do(") {
            enable = true;
        } else if f.starts_with("don't(") {
            enable = false;
        } else if enable {
            let a: i32 = m.get(1).unwrap().as_str().parse().unwrap();
            let b: i32 = m.get(2).unwrap().as_str().parse().unwrap();

            sum += a * b;
        }
    }

    sum
}
