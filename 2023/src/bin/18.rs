use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/18-test.txt");
const REAL: &str = include_str!("../../inputs/18-real.txt");

const CCW_TEST: &str = "R 3\nU 6\nL 2\nD 1\nR 1\nD 2\nL 2\nD 3";

fn main() {
    assert_eq!(part1(TEST), 62);
    assert_eq!(part1(CCW_TEST), 24);
    assert_eq!(part1(REAL), 48503);
    assert_eq!(part2(TEST), 952408144115);
    assert_eq!(part2(REAL), 148442153147147);
}

fn part1(input: &str) -> u64 {
    let plan: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let dir = parts.next().unwrap().chars().next().unwrap();
            let dist = parts.next().unwrap().parse().unwrap();
            (dir, dist)
        })
        .collect();

    calculate_area(&plan)
}

fn part2(input: &str) -> u64 {
    let plan: Vec<_> = input
        .lines()
        .map(|line| {
            let hex = line.split_whitespace().next_back().unwrap();
            let dir = match hex[7..8].chars().next().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!(),
            };
            let dist = u32::from_str_radix(&hex[2..7], 16).unwrap();
            (dir, dist)
        })
        .collect();

    calculate_area(&plan)
}

fn calculate_area(plan: &[(char, u32)]) -> u64 {
    let ccw = count_quarter_turns(&plan) > 0;

    let (x, y, area) = plan.iter().fold((0, 0, 1), |(x, y, area), &(dir, dist)| {
        let (x, y) = match dir {
            'U' => (x, y + dist as i32),
            'L' => (x - dist as i32, y),
            'D' => (x, y - dist as i32),
            'R' => (x + dist as i32, y),
            _ => panic!("unknown direction {dir}"),
        };

        let additional_area = match (dir, ccw) {
            ('D', true) | ('U', false) => i64::from(dist),
            ('U', true) | ('D', false) => 0,
            ('L', true) | ('R', false) => i64::from(y + 1) * i64::from(dist),
            ('R', true) | ('L', false) => -i64::from(y) * i64::from(dist),
            _ => unreachable!(),
        };

        (x, y, area + additional_area)
    });

    assert_eq!(x, 0);
    assert_eq!(y, 0);
    assert!(area > 0);

    area as u64
}

fn count_quarter_turns(plan: &[(char, u32)]) -> i32 {
    const CCW: [char; 4] = ['U', 'L', 'D', 'R'];

    plan.iter()
        .circular_tuple_windows()
        .fold(0, |quarter_turns, ((dir_a, _), (dir_b, _))| {
            if CCW
                .iter()
                .circular_tuple_windows()
                .any(|(x, y)| x == dir_a && y == dir_b)
            {
                quarter_turns + 1
            } else {
                quarter_turns - 1
            }
        })
}
