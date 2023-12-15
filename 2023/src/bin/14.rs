use std::collections::HashMap;

const TEST: &str = include_str!("../../inputs/14-test.txt");
const REAL: &str = include_str!("../../inputs/14-real.txt");

fn main() {
    assert_eq!(part1(TEST), 136);
    assert_eq!(part1(REAL), 109596);
    assert_eq!(part2(TEST), 64);
    assert_eq!(part2(REAL), 96105);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u32 {
    let mut map = parse(input);
    tilt(&mut map, 0, -1);
    total_load(&map)
}

fn part2(input: &str) -> u32 {
    let mut map = parse(input);

    let mut history = HashMap::new();
    let mut i = 0;
    let (cycle_start, cycle_len) = loop {
        history.insert(map.clone(), i);

        spin_cycle(&mut map);

        if let Some(cycle_start) = history.get(&map) {
            let cycle_len = i + 1 - cycle_start;
            break (cycle_start, cycle_len);
        }

        i += 1;
    };

    let final_cycle_pos = cycle_start + (1000000000 - cycle_start) % cycle_len;
    println!(
        "cycle start: {}, cycle length: {}, final cycle pos {}",
        cycle_start, cycle_len, final_cycle_pos
    );

    let (final_map, _) = history
        .iter()
        .find(|&(_, &pos)| pos == final_cycle_pos)
        .unwrap();

    total_load(final_map)
}

fn spin_cycle(map: &mut Vec<Vec<char>>) {
    tilt(map, 0, -1); // North
    tilt(map, -1, 0); // West
    tilt(map, 0, 1); // South
    tilt(map, 1, 0); // East
}

fn tilt(map: &mut Vec<Vec<char>>, dx: i32, dy: i32) {
    loop {
        let mut any_moved = false;

        for j in 0.max(-dy)..(map.len() as i32 + 0.min(-dy)) {
            let (j, v) = (j as usize, (j + dy) as usize);

            for i in 0.max(-dx)..(map[0].len() as i32 + 0.min(-dx)) {
                let (i, u) = (i as usize, (i + dx) as usize);

                if map[j][i] == 'O' && map[v][u] == '.' {
                    map[j][i] = '.';
                    map[v][u] = 'O';
                    any_moved = true;
                }
            }
        }

        if !any_moved {
            break;
        }
    }
}

fn total_load(map: &Vec<Vec<char>>) -> u32 {
    map.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| (i + 1) * row.iter().filter(|&&c| c == 'O').count())
        .sum::<usize>() as u32
}
