use std::{
    collections::{hash_map::Entry, HashMap},
    mem,
};

const TEST: &str = include_str!("../../inputs/23-test.txt");
const REAL: &str = include_str!("../../inputs/23-real.txt");

type Point = (usize, usize);

fn main() {
    assert_eq!(part1(TEST), 94);
    assert_eq!(part1(REAL), 2170);
    assert_eq!(part2(TEST), 154);
    assert_eq!(part2(REAL), 6502);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u32 {
    let map = parse(input);
    max_route_distance(&map)
}

fn part2(input: &str) -> u32 {
    let map = parse(&input.replace(">", ".").replace("v", "."));
    max_route_distance(&map)
}

fn max_route_distance(map: &Vec<Vec<char>>) -> u32 {
    let nodes = map_to_graph(map);
    assert!(nodes.len() < 64);

    let mut max_distances = vec![0; nodes.len()];
    let mut wavefront: HashMap<_, _> = [((0, 0u64), 0)].into_iter().collect();
    let mut next_wavefront = HashMap::new();
    while !wavefront.is_empty() {
        for ((i, visited), distance) in wavefront.drain() {
            assert_eq!((visited >> i) & 1, 0);
            let visited = visited | (1 << i);

            let max_distance = &mut max_distances[i];
            *max_distance = (*max_distance).max(distance);

            for &(j, distance_to_next) in nodes[i].iter() {
                if (visited >> j) & 1 == 1 {
                    continue;
                }

                let next_distance = next_wavefront.entry((j, visited)).or_insert(0);
                *next_distance = (*next_distance).max(distance + distance_to_next);
            }
        }
        mem::swap(&mut wavefront, &mut next_wavefront);
    }

    max_distances[1]
}

fn map_to_graph(map: &Vec<Vec<char>>) -> Vec<Vec<(usize, u32)>> {
    let mut nodes = vec![Vec::new(); 2];

    let start = (1, 0);
    let end = (map.len() - 2, map[0].len() - 1);
    let mut pos_to_node = [(start, 0), (end, 1)].into_iter().collect();

    follow_path(&map, (1, 1), (0, 1), false, 0, &mut nodes, &mut pos_to_node);

    nodes
}

fn follow_path(
    map: &Vec<Vec<char>>,
    (mut x, mut y): Point,
    (mut dx, mut dy): (isize, isize),
    mut downhill: bool,
    prev_node: usize,
    nodes: &mut Vec<Vec<(usize, u32)>>,
    pos_to_node: &mut HashMap<Point, usize>,
) {
    const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    let mut steps = 0;
    let valid_moves = loop {
        steps += 1;

        let mut valid_moves = DIRECTIONS.into_iter().filter_map(|(ndx, ndy)| {
            if dx * ndx < 0 || dy * ndy < 0 {
                return None;
            }
            let (nx, ny) = (x.checked_add_signed(ndx)?, y.checked_add_signed(ndy)?);
            let tile = *map.get(ny)?.get(nx)?;
            let downhill = match (ndx, ndy, tile) {
                (_, _, '#') | (0, -1, 'v') | (-1, 0, '>') => return None,
                (0, 1, 'v') | (1, 0, '>') => true,
                _ => false,
            };
            Some((nx, ny, ndx, ndy, downhill))
        });

        // Keep following the path until we come to a junction.
        let num_valid_moves = valid_moves.clone().count();
        if matches!(num_valid_moves, 0 | 2..) {
            break valid_moves;
        }

        let d;
        (x, y, dx, dy, d) = valid_moves.next().unwrap();
        downhill |= d;
    };

    // If first time visiting this node, follow paths from this node.
    let node = match pos_to_node.entry((x, y)) {
        Entry::Occupied(entry) => *entry.get(),
        Entry::Vacant(entry) => {
            let node = nodes.len();
            entry.insert(node);
            nodes.push(Vec::new());

            for (nx, ny, dx, dy, d) in valid_moves {
                follow_path(map, (nx, ny), (dx, dy), d, node, nodes, pos_to_node);
            }

            node
        }
    };

    // Add path from previous node to node.
    nodes[prev_node].push((node, steps));

    // Add path from node to previous node if not uphill.
    if !downhill {
        nodes[node].push((prev_node, steps));
    }
}
