use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/15-test.txt");
const REAL: &str = include_str!("../../inputs/15-real.txt");

fn main() {
    assert_eq!(part1(TEST), 10092);
    assert_eq!(part1(REAL), 1413675);
    assert_eq!(part2(TEST), 9021);
    assert_eq!(part2(REAL), 1399772);
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let map = map.lines().map(|line| line.chars().collect()).collect();
    let moves = moves.chars().filter(|c| !c.is_whitespace()).collect();

    (map, moves)
}

fn part1(input: &str) -> u32 {
    let (mut map, moves) = parse(input);

    let (mut x, mut y) = staring_pos(&map);
    map[y][x] = '.';

    for dir in moves {
        assert_eq!(map[y][x], '.');

        let (dx, dy) = dxdy(dir);
        if part1_try_move(&mut map, x, y, dx, dy) {
            (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        }
    }

    gps_sum(&map, 'O')
}

fn part1_try_move(map: &mut [Vec<char>], x: usize, y: usize, dx: isize, dy: isize) -> bool {
    let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));

    match (map[y][x], map[ny][nx]) {
        ('.', '.') => return true,
        ('.' | 'O', 'O') => {
            if !part1_try_move(map, nx, ny, dx, dy) {
                return false;
            }
        }
        ('O', '.') => (),
        ('.' | 'O', '#') => return false,
        (a, b) => panic!("{a} {b}"),
    }

    assert_eq!(map[ny][nx], '.');
    if map[y][x] != '.' {
        map[y][x] = '.';
        map[ny][nx] = 'O';
    }
    true
}

fn part2(input: &str) -> u32 {
    let (map, moves) = parse(input);

    let mut map = map
        .into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let (mut x, mut y) = staring_pos(&map);
    map[y][x] = '.';

    for dir in moves {
        assert_eq!(map[y][x], '.');

        let (dx, dy) = dxdy(dir);

        let moved = if dx == 0 {
            part2_try_move_ud(&mut map, x, y, dy, true)
        } else {
            part2_try_move_lr(&mut map, x, y, dx)
        };

        if moved {
            (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        }
    }

    gps_sum(&map, '[')
}

fn part2_try_move_lr(map: &mut [Vec<char>], x: usize, y: usize, dx: isize) -> bool {
    let (nx, ny) = (x.wrapping_add_signed(dx), y);

    match (map[y][x], map[ny][nx]) {
        ('.', '.') => return true,
        ('.' | '[' | ']', '[' | ']') => {
            if !part2_try_move_lr(map, nx, ny, dx) {
                return false;
            }
        }
        ('[' | ']', '.') => (),
        ('.' | '[' | ']', '#') => return false,
        _ => panic!(),
    }

    assert_eq!(map[ny][nx], '.');
    if map[y][x] != '.' {
        let v = map[y][x];
        map[y][x] = '.';
        map[ny][nx] = v;
    }
    true
}

fn part2_try_move_ud(map: &mut [Vec<char>], x: usize, y: usize, dy: isize, act: bool) -> bool {
    let (nx, ny) = (x, y.wrapping_add_signed(dy));

    match (map[y][x], map[ny][nx]) {
        ('.', '.') => return true,
        ('.' | '[' | ']', v @ ('[' | ']')) => {
            let (nx1, nx2) = if v == '[' { (nx, nx + 1) } else { (nx - 1, nx) };
            if !part2_try_move_ud(map, nx1, ny, dy, false)
                || !part2_try_move_ud(map, nx2, ny, dy, false)
            {
                return false;
            }
            if act {
                assert!(part2_try_move_ud(map, nx1, ny, dy, true));
                assert!(part2_try_move_ud(map, nx2, ny, dy, true));
            }
        }
        ('[' | ']', '.') => (),
        ('.' | '[' | ']', '#') => return false,
        _ => panic!(),
    }

    if act {
        assert_eq!(map[ny][nx], '.');

        if map[y][x] != '.' {
            assert!(matches!(map[y][x], '[' | ']'));
            let v = map[y][x];
            map[y][x] = '.';
            map[ny][nx] = v;
        }
    }
    true
}

fn staring_pos(map: &[Vec<char>]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &v)| (x, y, v)))
        .find(|&(_, _, v)| v == '@')
        .map(|(x, y, _)| (x, y))
        .unwrap()
}

fn dxdy(dir: char) -> (isize, isize) {
    match dir {
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        '^' => (0, -1),
        _ => panic!(),
    }
}

fn gps_sum(map: &[Vec<char>], c: char) -> u32 {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &v)| v == c)
                .map(move |(x, _)| x as u32 + y as u32 * 100)
        })
        .sum()
}
