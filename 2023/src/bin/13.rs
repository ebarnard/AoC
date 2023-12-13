const TEST: &str = include_str!("../../inputs/13-test.txt");
const REAL: &str = include_str!("../../inputs/13-real.txt");

fn main() {
    assert_eq!(part1(TEST), 405);
    assert_eq!(part1(REAL), 32723);
    assert_eq!(part2(TEST), 400);
    assert_eq!(part2(REAL), 34536);
}

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|map| map.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn part1(input: &str) -> u32 {
    let maps = parse(input);
    score_reflections(&maps, 0)
}

fn part2(input: &str) -> u32 {
    let maps = parse(input);
    score_reflections(&maps, 1)
}

fn score_reflections(maps: &Vec<Vec<Vec<char>>>, difference_count: u32) -> u32 {
    maps.iter()
        .map(|map| {
            let reflected_rows = (1..map.len())
                .filter(|&i| row_reflection_difference_count(map, i) == difference_count);

            let reflected_columns = (1..map[0].len())
                .filter(|&j| column_reflection_difference_count(map, j) == difference_count);

            // Each pattern contains only a single reflection
            let num_reflected_rows = reflected_rows.clone().count();
            let num_reflected_columns = reflected_columns.clone().count();
            assert!(num_reflected_rows + num_reflected_columns == 1);

            reflected_rows.sum::<usize>() as u32 * 100 + reflected_columns.sum::<usize>() as u32
        })
        .sum()
}

fn row_reflection_difference_count(map: &Vec<Vec<char>>, row: usize) -> u32 {
    let (top, bottom) = map.split_at(row);
    top.iter()
        .rev()
        .zip(bottom)
        .map(|(a, b)| a.iter().zip(b).filter(|(x, y)| x != y).count() as u32)
        .sum()
}

fn column_reflection_difference_count(map: &Vec<Vec<char>>, col: usize) -> u32 {
    map.iter()
        .map(|row| {
            let (left, right) = row.split_at(col);
            left.iter().rev().zip(right).filter(|(a, b)| a != b).count() as u32
        })
        .sum()
}
