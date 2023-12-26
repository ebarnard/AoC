const A_TEST: &str = include_str!("../../inputs/1a-test.txt");
const B_TEST: &str = include_str!("../../inputs/1b-test.txt");
const REAL: &str = include_str!("../../inputs/1-real.txt");

fn main() {
    assert_eq!(part1(A_TEST), 142);
    assert_eq!(part1(REAL), 55607);
    assert_eq!(part2(B_TEST), 281);
    assert_eq!(part2(REAL), 55291);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers = || line.chars().filter(|c| c.is_numeric());
            let first = numbers().next().unwrap();
            let last = numbers().next_back().unwrap();
            (first as u32 - '0' as u32) * 10 + (last as u32 - '0' as u32)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    const WORDS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const DIGITS: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    input
        .lines()
        .map(|line| {
            let mut first_index = usize::MAX;
            let mut first_digit = 0;
            let mut last_index = 0usize;
            let mut last_digit = 0;

            for (index, text) in WORDS
                .into_iter()
                .enumerate()
                .chain(DIGITS.into_iter().enumerate())
            {
                if let Some(first_index_candidate) = line.find(text) {
                    if first_index_candidate <= first_index {
                        first_index = first_index_candidate;
                        first_digit = index as u32 + 1;
                    }
                }

                if let Some(last_index_candidate) = line.rfind(text) {
                    if last_index_candidate >= last_index {
                        last_index = last_index_candidate;
                        last_digit = index as u32 + 1;
                    }
                }
            }

            first_digit * 10 + last_digit
        })
        .sum()
}
