use aoc2023::nom::uint32;
use itertools::{iproduct, Itertools};
use nom::{
    bytes::complete::tag,
    combinator::{complete, map},
    multi::separated_list0,
    sequence::{separated_pair, tuple},
};

const TEST: &str = include_str!("../../inputs/22-test.txt");
const REAL: &str = include_str!("../../inputs/22-real.txt");

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Brick {
    // Field ordering is critical for Ord implementation
    z_min: usize,
    z_max: usize,
    y_min: usize,
    y_max: usize,
    x_min: usize,
    x_max: usize,
}

fn main() {
    assert_eq!(part1(TEST), 5);
    assert_eq!(part1(REAL), 441);
    assert_eq!(part2(TEST), 7);
    assert_eq!(part2(REAL), 80778);
}

fn parse(input: &str) -> Vec<Brick> {
    let point = || {
        map(
            tuple((uint32, tag(","), uint32, tag(","), uint32)),
            |(x, _, y, _, z)| [x, y, z],
        )
    };
    let bricks = separated_list0(tag("\n"), separated_pair(point(), tag("~"), point()));

    let (_, parsed) = complete(bricks)(input).unwrap();

    parsed
        .iter()
        .map(|&([x0, y0, z0], [x1, y1, z1])| Brick {
            z_min: z0.min(z1) as usize,
            z_max: z0.max(z1) as usize,
            y_min: y0.min(y1) as usize,
            y_max: y0.max(y1) as usize,
            x_min: x0.min(x1) as usize,
            x_max: x0.max(x1) as usize,
        })
        .sorted()
        .collect()
}

fn part1(input: &str) -> u32 {
    let bricks = parse(input);

    let bricks = drop_bricks(&bricks, None).sorted().collect_vec();

    (0..bricks.len())
        .filter(|&i| {
            bricks[i + 1..]
                .iter()
                .zip(drop_bricks(&bricks, Some(i)).skip(i))
                .all(|(&a, b)| a == b)
        })
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let bricks = parse(input);

    let bricks = drop_bricks(&bricks, None).sorted().collect_vec();

    (0..bricks.len())
        .map(|i| {
            bricks[i + 1..]
                .iter()
                .zip(drop_bricks(&bricks, Some(i)).skip(i))
                .filter(|&(&a, b)| a != b)
                .count() as u32
        })
        .sum::<u32>()
}

fn drop_bricks<'a>(bricks: &'a [Brick], except: Option<usize>) -> impl Iterator<Item = Brick> + 'a {
    // Bricks must be sorted by minimum Z coordinate.
    bricks.windows(2).all(|w| w[1] >= w[0]);

    let z_max = bricks.iter().map(|brick| brick.z_max).max().unwrap_or(0);
    let y_max = bricks.iter().map(|brick| brick.y_max).max().unwrap_or(0);
    let x_max = bricks.iter().map(|brick| brick.x_max).max().unwrap_or(0);
    let mut cell_filled = vec![vec![vec![false; x_max + 1]; y_max + 1]; z_max + 1];

    // Try to move brick downwards until we hit another brick.
    bricks
        .iter()
        .enumerate()
        .filter(move |&(i, _)| Some(i) != except)
        .map(move |(_, &b)| {
            let moves_down = max_moves_down(&cell_filled, b);
            let z_min = b.z_min - moves_down;
            let z_max = b.z_max - moves_down;

            for (x, y, z) in iproduct!(b.x_min..=b.x_max, b.y_min..=b.y_max, z_min..=z_max) {
                cell_filled[z][y][x] = true;
            }

            Brick { z_min, z_max, ..b }
        })
}

fn max_moves_down(cell_filled: &Vec<Vec<Vec<bool>>>, b: Brick) -> usize {
    (1..b.z_min)
        .rev()
        .take_while(|&z| {
            iproduct!(b.x_min..=b.x_max, b.y_min..=b.y_max).all(|(x, y)| !cell_filled[z][y][x])
        })
        .count()
}
