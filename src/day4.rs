use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn offset(x: usize, y: usize, dx: i32, dy: i32) -> Option<(usize, usize)> {
    let x = x as i32 + dx;
    let y = y as i32 + dy;

    if x < 0 || y < 0 {
        None
    } else {
        Some((x as usize, y as usize))
    }
}

fn check(puzzle: &[Vec<char>], needle: char, (x, y): (usize, usize)) -> bool {
    // println!("checking {},{} for {}", x, y, needle);
    puzzle
        .get(y)
        .and_then(|line| line.get(x))
        .map(|c| *c == needle)
        .unwrap_or_default()
}

fn check_o(puzzle: &Vec<Vec<char>>, needle: char, d: Option<(usize, usize)>) -> bool {
    d.map(|d| check(puzzle, needle, d)).unwrap_or_default()
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<Vec<char>>) -> i32 {
    let dirs = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut xmas_counter = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'X' {
                for d in dirs {
                    let m = offset(x, y, d.0, d.1)
                        .map(|m| check(input, 'M', m))
                        .unwrap_or_default();
                    let a = offset(x, y, d.0 * 2, d.1 * 2)
                        .map(|a| check(input, 'A', a))
                        .unwrap_or_default();
                    let s = offset(x, y, d.0 * 3, d.1 * 3)
                        .map(|s| check(input, 'S', s))
                        .unwrap_or_default();

                    if m && a && s {
                        // println!("xmas at {},{} in dir {:?}", x, y, d);
                        xmas_counter += 1;
                    }
                }
            }
        }
    }

    xmas_counter
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<Vec<char>>) -> i32 {
    let mut x_mas = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'A' {
                let lt = offset(x, y, -1, -1);
                let lb = offset(x, y, -1, 1);
                let rt = offset(x, y, 1, -1);
                let rb = offset(x, y, 1, 1);

                let top_sam = check_o(input, 'S', lt) && check_o(input, 'M', rb);
                let top_mas = check_o(input, 'M', lt) && check_o(input, 'S', rb);

                let bot_sam = check_o(input, 'S', lb) && check_o(input, 'M', rt);
                let bot_mas = check_o(input, 'M', lb) && check_o(input, 'S', rt);

                if (top_sam || top_mas) && (bot_sam || bot_mas) {
                    // println!("xmas at {},{}", x, y);
                    x_mas += 1;
                }
            }
        }
    }

    x_mas
}
