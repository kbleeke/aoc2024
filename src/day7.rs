use aoc_runner_derive::{aoc, aoc_generator};

struct Equation {
    left: i64,
    right: Vec<i64>,
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();

            let left = left.parse().unwrap();
            let right = right
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            Equation { left, right }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &Vec<Equation>) -> i64 {
    let mut total_sum: i64 = 0;
    for eq in input {
        let n = eq.right.len() - 1;
        let bits = 1 << n;

        println!("{bits}");

        for ops in 0..bits {
            let mut ops_str = String::new();
            let mut digits = eq.right.iter().copied().enumerate();
            let mut sum = digits.next().unwrap().1;
            for (i, c) in digits {
                let op = ops & (1 << (n - i)) != 0;
                if op {
                    ops_str += "+";
                    sum += c;
                } else {
                    ops_str += "*";
                    sum *= c;
                }

                if sum > eq.left {
                    break;
                }
            }

            if sum == eq.left {
                println!(
                    "{} == {:?} with ops {} {:b}",
                    eq.left, eq.right, ops_str, ops
                );
                total_sum += eq.left;
                break;
            } else {
                // println!("{} != {:?} with ops {} {:b}", sum, eq.right, ops_str, ops);
            }
        }
    }

    total_sum
}

#[aoc(day7, part2)]
fn part2(input: &Vec<Equation>) -> i64 {
    let mut total_sum: i64 = 0;
    for eq in input {
        let n = eq.right.len() - 1;

        for ops in generate_permutations(&[0, 1, 2], n) {
            // let mut ops_str = String::new();
            let mut digits = eq.right.iter().copied();
            let mut sum = digits.next().unwrap();

            for (digit, op) in digits.zip(ops.iter().copied()) {
                if op == 0 {
                    sum *= digit;
                    // ops_str += " *";
                } else if op == 1 {
                    sum = format!("{sum}{digit}").parse().unwrap();
                    // ops_str += " ||";
                } else if op == 2 {
                    sum += digit;
                    // ops_str += " +";
                }

                if sum > eq.left {
                    break;
                }
            }

            if sum == eq.left {
                // println!(
                //     "{} == {:?} with ops {} {:?}",
                //     eq.left, eq.right, ops_str, ops
                // );
                total_sum += eq.left;
                break;
            } else {
                // println!("{} != {:?} with ops {} {:b}", sum, eq.right, ops_str, ops);
            }
        }
    }

    total_sum
}

// thanks claude
fn generate_permutations(digits: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    // Pre-compute the total number of permutations
    let total_permutations = digits.len().pow(k as u32);

    for i in 0..total_permutations {
        let mut current_permutation = Vec::with_capacity(k);

        // Convert index to base-n representation
        let mut remaining = i;
        for _ in 0..k {
            let digit_index = remaining % digits.len();
            current_permutation.push(digits[digit_index]);
            remaining /= digits.len();
        }

        result.push(current_permutation);
    }

    result
}
