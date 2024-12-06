use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct Cell {
    obstacle: bool,
}

#[derive(Clone)]
struct Map {
    width: i32,
    grid: Vec<Cell>,
    player: (i32, i32),
}

impl Map {
    fn get(&self, x: i32, y: i32) -> Option<&Cell> {
        if x < 0 || y < 0 || x >= self.width {
            return None;
        }

        let index = (y * self.width + x) as usize;
        self.grid.get(index)
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Map {
    let width = input.lines().next().unwrap().len();
    let mut grid = Vec::<Cell>::new();
    let mut player = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let obstacle = c == '#';
            let is_player = c == '^';
            if is_player {
                player = (x as i32, y as i32);
            }

            grid.push(Cell { obstacle });
        }
    }

    Map {
        width: width as i32,
        grid,
        player,
    }
}

fn part1_pass(input: &Map) -> HashSet<(i32, i32)> {
    let mut dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)].into_iter().cycle();
    let mut current_dir = dirs.next().unwrap();
    let mut player = input.player;

    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert(player);

    loop {
        let (x, y) = (player.0 + current_dir.0, player.1 + current_dir.1);
        if let Some(cell) = input.get(x, y) {
            if cell.obstacle {
                current_dir = dirs.next().unwrap();
            } else {
                player = (x, y);
                visited.insert((x, y));
            }
        } else {
            break;
        }
    }
    visited
}

#[aoc(day6, part1)]
fn part1(input: &Map) -> i32 {
    let visited = part1_pass(input);

    visited.len() as i32
}

#[aoc(day6, part2)]
fn part2(input: &Map) -> i32 {
    let visited = part1_pass(input);

    let mut sum = 0;
    for i in 0..input.grid.len() {
        let mut dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)].into_iter().cycle();
        let mut current_dir = dirs.next().unwrap();
        let mut player = input.player;
        let mut input = input.clone();
        if !visited.contains(&(i as i32 % input.width, i as i32 / input.width)) {
            continue;
        } else {
            input.grid[i].obstacle = true;
        }

        let mut collided = HashSet::<((i32, i32), (i32, i32))>::new();

        let escaped = loop {
            let (x, y) = (player.0 + current_dir.0, player.1 + current_dir.1);
            if let Some(cell) = input.get(x, y) {
                if cell.obstacle {
                    // println!("collided with ({x}, {y})",);
                    current_dir = dirs.next().unwrap();
                    let fresh = collided.insert(((x, y), player));
                    if !fresh {
                        // println!("cycle found at cell ({x}, {y})",);
                        break false;
                    }
                } else {
                    player = (x, y);
                }
            } else {
                break true;
            }
        };
        if !escaped {
            sum += 1;
        }
    }

    sum
}
