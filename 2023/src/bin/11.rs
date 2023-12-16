const TEST: &str = include_str!("../../inputs/11-test.txt");
const REAL: &str = include_str!("../../inputs/11-real.txt");

fn main() {
    assert_eq!(part1(TEST), 374);
    assert_eq!(part1(REAL), 10231178);
    assert_eq!(part2(TEST), 82000210);
    assert_eq!(part2(REAL), 622120986954);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u64 {
    let map = parse(input);
    sum_galaxy_l1_distances(&map, 2)
}

fn part2(input: &str) -> u64 {
    let map = parse(input);
    sum_galaxy_l1_distances(&map, 1000000)
}

fn sum_galaxy_l1_distances(map: &Vec<Vec<char>>, expansion: u64) -> u64 {
    let x_distances = distances_from_origin(
        (0..map[0].len()).map(|i| map.iter().all(|row| row[i] == '.')),
        expansion,
    );

    let y_distances = distances_from_origin(
        (0..map.len()).map(|i| map[i].iter().all(|&c| c == '.')),
        expansion,
    );

    let galaxies: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let pairs = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| galaxies[(i + 1)..].iter().map(move |&b| (a, b)));

    pairs
        .map(|((ax, ay), (bx, by))| {
            let dx = x_distances[ax.max(bx)] - x_distances[ax.min(bx)];
            let dy = y_distances[ay.max(by)] - y_distances[ay.min(by)];
            dx + dy
        })
        .sum()
}

fn distances_from_origin(items: impl Iterator<Item = bool>, expansion: u64) -> Vec<u64> {
    items
        .scan(0, |dist, is_empty| {
            let current_dist = *dist;
            *dist += if is_empty { expansion } else { 1 };
            Some(current_dist)
        })
        .collect()
}
