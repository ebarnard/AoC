use std::collections::HashSet;

const TEST: &str = include_str!("../../inputs/6-test.txt");
const REAL: &str = include_str!("../../inputs/6-real.txt");

fn main() {
    assert_eq!(part1(TEST), 41);
    assert_eq!(part1(REAL), 5551);
    assert_eq!(part2(TEST), 6);
    assert_eq!(part2(REAL), 1939);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u32 {
    let map = parse(input);
    let (x0, y0) = staring_pos(&map);

    let mut visited: HashSet<_> = [(x0, y0)].into_iter().collect();

    let mut state = (x0, y0, 0, -1);
    while let Some(next_state) = move_one_step(state, &map, None) {
        state = next_state;
        let (x, y, _, _) = state;
        visited.insert((x, y));
    }

    visited.len() as u32
}

fn part2(input: &str) -> u32 {
    let map = parse(input);
    let (x0, y0) = staring_pos(&map);

    let mut visited: HashSet<_> = [(x0, y0)].into_iter().collect();
    let mut possible_walls = HashSet::new();
    let mut loop_visited = HashSet::new();

    let mut state = (x0, y0, 0, -1);
    while let Some(next_state) = move_one_step(state, &map, None) {
        let (wx, wy, _, _) = next_state;

        if !visited.contains(&(wx, wy)) && !possible_walls.contains(&(wx, wy)) {
            loop_visited.clear();
            loop_visited.insert(state);

            let mut loop_state = state;
            while let Some(next_loop_state) = move_one_step(loop_state, &map, Some((wx, wy))) {
                loop_state = next_loop_state;
                if !loop_visited.insert(loop_state) {
                    possible_walls.insert((wx, wy));
                    break;
                }
            }
        }

        state = next_state;
        let (x, y, _, _) = state;
        visited.insert((x, y));
    }

    possible_walls.len() as u32
}

fn staring_pos(map: &[Vec<char>]) -> (u32, u32) {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &v)| (x, y, v)))
        .find(|&(_, _, v)| v == '^')
        .map(|(x, y, _)| (x as u32, y as u32))
        .unwrap()
}

fn move_one_step(
    (x, y, mut dx, mut dy): (u32, u32, i32, i32),
    map: &[Vec<char>],
    extra_wall: Option<(u32, u32)>,
) -> Option<(u32, u32, i32, i32)> {
    loop {
        let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));

        let &tile = map.get(ny as usize)?.get(nx as usize)?;
        if tile == '#' || Some((nx, ny)) == extra_wall {
            (dx, dy) = (-dy, dx);
            continue;
        }

        return Some((nx, ny, dx, dy));
    }
}
