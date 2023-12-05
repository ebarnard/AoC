use std::{cmp, fs};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part1(input_path: &str) -> u32 {
    let world = parse(input_path);
    let height = world.len();
    let width = world[0].len();
    let (time, _) = min_time_to_traverse(world, (0, 0), (width - 1, height - 1));
    time
}

pub fn part2(input_path: &str) -> u32 {
    let world = parse(input_path);
    let height = world.len();
    let width = world[0].len();
    let entry = (0, 0);
    let exit = (width - 1, height - 1);
    let (time_a, world) = min_time_to_traverse(world, entry, exit);
    let (time_b, world) = min_time_to_traverse(world, exit, entry);
    let (time_c, _) = min_time_to_traverse(world, entry, exit);
    time_a + time_b + time_c
}

fn min_time_to_traverse(
    mut input_world: Vec<Vec<Vec<Direction>>>,
    entry: (usize, usize),
    exit: (usize, usize),
) -> (u32, Vec<Vec<Vec<Direction>>>) {
    let mut input_distances: Vec<Vec<_>> = input_world
        .iter()
        .map(|row| row.iter().map(|_| u32::MAX).collect())
        .collect();

    // Can always start on top left (0, -1).

    let height = input_world.len();
    let width = input_world[0].len();

    print_world(&input_world);

    for t in 1.. {
        let output_world = sim_step(&input_world);
        let mut output_distances = input_distances.clone();
        for y in 0..height {
            for x in 0..width {
                output_distances[y][x] = u32::MAX;
            }
        }

        // For each cell of the input, see if we can move to an output cell.
        for y in 0..height {
            for x in 0..width {
                //if input_world[y][x].is_empty() {
                //    continue;
                //}

                let dist = input_distances[y][x].saturating_add(1);

                // Stay still
                min_assign(&mut output_distances[y][x], dist);

                // Up
                if y > 0 {
                    if output_world[y - 1][x].is_empty() {
                        min_assign(&mut output_distances[y - 1][x], dist);
                    } else {
                        output_distances[y - 1][x] = u32::MAX;
                    }
                }

                // Down
                if y < height - 1 {
                    if output_world[y + 1][x].is_empty() {
                        min_assign(&mut output_distances[y + 1][x], dist);
                    } else {
                        output_distances[y + 1][x] = u32::MAX;
                    }
                }

                // Left
                if x > 0 {
                    if output_world[y][x - 1].is_empty() {
                        min_assign(&mut output_distances[y][x - 1], dist);
                    } else {
                        output_distances[y][x - 1] = u32::MAX;
                    }
                }

                // Right
                if x < width - 1 {
                    if output_world[y][x + 1].is_empty() {
                        min_assign(&mut output_distances[y][x + 1], dist);
                    } else {
                        output_distances[y][x + 1] = u32::MAX;
                    }
                }
            }
        }

        // Entry
        if output_world[entry.1][entry.0].is_empty() {
            min_assign(&mut output_distances[entry.1][entry.0], t);
        }

        // Exit
        if output_world[exit.1][exit.0].is_empty() && output_distances[exit.1][exit.0] < u32::MAX {
            return (
                output_distances[exit.1][exit.0] + 1,
                sim_step(&output_world),
            );
        }

        print_world(&output_world);
        print_dist(&output_distances);

        input_world = output_world;
        input_distances = output_distances;
    }

    unreachable!();
}

fn min_assign(slot: &mut u32, val: u32) {
    *slot = cmp::min(*slot, val);
}

fn print_world(world: &Vec<Vec<Vec<Direction>>>) {
    let width = world[0].len() as i32;

    print!("#.");
    for _ in 0..width {
        print!("#");
    }
    println!();

    for row in world {
        print!("#");
        for item in row {
            if item.len() == 0 {
                print!(".");
            } else if item.len() == 1 {
                match item[0] {
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                    Direction::Right => print!(">"),
                }
            } else {
                print!("{}", item.len());
            }
        }
        println!("#");
    }

    for _ in 0..width {
        print!("#");
    }
    println!(".#");
}

fn print_dist(dist: &Vec<Vec<u32>>) {
    let width = dist[0].len() as i32;

    print!("#.");
    for _ in 0..width {
        print!("#");
    }
    println!();

    for row in dist {
        print!("#");
        for item in row {
            if *item > 9 {
                print!("X");
            } else {
                print!("{}", item);
            }
        }
        println!("#");
    }

    for _ in 0..width {
        print!("#");
    }
    println!(".#");
}

fn parse(input_path: &str) -> Vec<Vec<Vec<Direction>>> {
    let input = fs::read_to_string(input_path).unwrap();
    input
        .lines()
        .take(input.lines().count() - 1)
        .skip(1)
        .map(|line| {
            line.chars()
                .take(line.chars().count() - 1)
                .skip(1)
                .map(|c| match c {
                    '.' => Vec::new(),
                    '^' => vec![Direction::Up],
                    'v' => vec![Direction::Down],
                    '<' => vec![Direction::Left],
                    '>' => vec![Direction::Right],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn sim_step(input_world: &Vec<Vec<Vec<Direction>>>) -> Vec<Vec<Vec<Direction>>> {
    fn wrap(x: usize, diff: i64, max: usize) -> usize {
        (x as i64 + diff).rem_euclid(max as i64) as usize
    }

    let height = input_world.len();
    let width = input_world[0].len();

    let mut output_world = input_world.clone();
    for y in 0..height {
        for x in 0..width {
            let cell = &mut output_world[y][x];
            cell.clear();

            // Look up
            cell.extend(
                input_world[wrap(y, -1, height)][x]
                    .iter()
                    .copied()
                    .filter(|s| *s == Direction::Down),
            );

            // Look down
            cell.extend(
                input_world[wrap(y, 1, height)][x]
                    .iter()
                    .copied()
                    .filter(|s| *s == Direction::Up),
            );

            // Look left
            cell.extend(
                input_world[y][wrap(x, -1, width)]
                    .iter()
                    .copied()
                    .filter(|s| *s == Direction::Right),
            );

            // Look right
            cell.extend(
                input_world[y][wrap(x, 1, width)]
                    .iter()
                    .copied()
                    .filter(|s| *s == Direction::Left),
            );
        }
    }
    output_world
}

#[test]
fn part1_test1() {
    assert_eq!(part1("test-1.txt"), 10);
}

#[test]
fn part1_test2() {
    assert_eq!(part1("test-2.txt"), 18);
}

#[test]
fn part1_real() {
    assert_eq!(part1("real.txt"), 283);
}

#[test]
fn part2_test2() {
    assert_eq!(part2("test-2.txt"), 54);
}

#[test]
fn part2_real() {
    assert_eq!(part2("real.txt"), 883);
}
