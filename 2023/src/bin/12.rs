use std::collections::HashMap;

const TEST: &str = include_str!("../../inputs/12-test.txt");
const REAL: &str = include_str!("../../inputs/12-real.txt");

fn main() {
    assert_eq!(part1(TEST), 21);
    assert_eq!(part1(REAL), 7670);
    assert_eq!(part2(TEST), 525152);
    assert_eq!(part2(REAL), 157383940585037);
}

fn parse(input: &str) -> Vec<(&str, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let (conditions, broken_runs) = line.split_once(' ').unwrap();
            assert_eq!(conditions.is_ascii(), true);
            let broken_runs = broken_runs.split(',').map(|n| n.parse().unwrap()).collect();
            (conditions, broken_runs)
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let rows = parse(input);
    rows.iter()
        .map(|(conditions, broken_runs)| {
            count_valid_arrangements(conditions, broken_runs, &mut Default::default())
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let rows = parse(input);

    let unfolded = rows.into_iter().map(|(conditions, broken_runs)| {
        (
            (0..5)
                .map(|_| &conditions[..])
                .collect::<Vec<_>>()
                .join("?"),
            (0..5)
                .flat_map(|_| broken_runs.iter().copied())
                .collect::<Vec<_>>(),
        )
    });

    unfolded
        .map(|(conditions, broken_runs)| {
            count_valid_arrangements(&conditions, &broken_runs, &mut Default::default())
        })
        .sum()
}

fn count_valid_arrangements<'a, 'b>(
    conditions: &'a str,
    broken_runs: &'a [u32],
    memoised: &'b mut HashMap<(&'a str, &'a [u32]), u64>,
) -> u64 {
    if let Some(&count) = memoised.get(&(conditions, broken_runs)) {
        return count;
    }

    let count = if let Some((&run, remaining_runs)) = broken_runs.split_first() {
        let run = run.try_into().unwrap();

        (0..(conditions.len().saturating_sub(run - 1)))
            .map(|i| {
                if conditions[..i].contains('#') || conditions[i..][..run].contains('.') {
                    0
                } else if i + run == conditions.len() {
                    count_valid_arrangements("", remaining_runs, memoised)
                } else if &conditions[i + run..][..1] == "#" {
                    0
                } else {
                    count_valid_arrangements(&conditions[i + run + 1..], remaining_runs, memoised)
                }
            })
            .sum()
    } else {
        if conditions.contains('#') {
            0
        } else {
            1
        }
    };

    memoised.insert((conditions, broken_runs), count);

    count
}
