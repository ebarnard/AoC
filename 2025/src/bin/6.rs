use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/6-test.txt");
const REAL: &str = include_str!("../../inputs/6-real.txt");

fn main() {
    assert_eq!(part1(TEST), 4277556);
    assert_eq!(part1(REAL), 6299564383938);
    assert_eq!(part2(TEST), 3263827);
    assert_eq!(part2(REAL), 11950004808442);
}

fn part1(input: &str) -> u64 {
    let lines = input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .collect_vec();

    let num_rows = lines.len() - 1;
    let num_cols = lines[0].len();

    (0..num_cols)
        .map(|col| {
            let op = lines[num_rows][col];
            let values = lines
                .iter()
                .take(num_rows)
                .map(|line| line[col].parse::<u64>().unwrap());

            match op {
                "*" => values.product::<u64>(),
                "+" => values.sum::<u64>(),
                _ => unreachable!(),
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let lines = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let num_rows = lines.len() - 1;
    let num_cols = lines[0].len();

    let mut col = 0;
    let mut numbers = Vec::new();
    let mut sum = 0;
    while col < num_cols {
        let op = lines[num_rows][col];
        while col < num_cols {
            let num = (0..num_rows)
                .map(|row| lines[row][col])
                .filter(|&c| !c.is_whitespace())
                .collect::<String>();

            col += 1;

            if num.is_empty() {
                break;
            }

            numbers.push(num.parse::<u64>().unwrap());
        }

        sum += match op {
            '*' => numbers.drain(..).product::<u64>(),
            '+' => numbers.drain(..).sum::<u64>(),
            _ => unreachable!(),
        };
    }
    sum
}
