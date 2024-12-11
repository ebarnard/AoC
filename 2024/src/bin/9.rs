use std::iter;

const TEST: &str = include_str!("../../inputs/9-test.txt");
const REAL: &str = include_str!("../../inputs/9-real.txt");

fn main() {
    assert_eq!(part1(TEST), 1928);
    assert_eq!(part1(REAL), 6382875730645);
    assert_eq!(part2(TEST), 2858);
    assert_eq!(part2(REAL), 6420913943576);
}

fn parse(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| {
            let mut dst = [0; 4];
            let dst = c.encode_utf8(&mut dst);
            dst.parse().unwrap()
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let map = parse(input);
    let mut blocks = map_to_blocks(&map);

    let mut head = 0;
    let mut tail_incl = blocks.len() - 1;

    while head < tail_incl {
        if blocks[head].is_some() {
            head += 1;
        } else if blocks[tail_incl].is_none() {
            tail_incl -= 1;
        } else {
            blocks.swap(head, tail_incl);
            head += 1;
            tail_incl -= 1;
        }
    }

    checksum(&blocks)
}

fn part2(input: &str) -> u64 {
    let map = parse(input);
    let mut blocks = map_to_blocks(&map);

    let mut tail = blocks.len();
    while tail > 0 {
        let Some((file_head, file_len)) = find_next_file_back(&blocks[..tail]) else {
            break;
        };

        if let Some(free_head) = find_next_free_fwd(&blocks[..file_head], file_len) {
            for (i, j) in (file_head..).zip(free_head..).take(file_len) {
                blocks.swap(i, j);
            }
        };

        tail = file_head;
    }

    checksum(&blocks)
}

fn map_to_blocks(map: &[u8]) -> Vec<Option<u32>> {
    map.iter()
        .enumerate()
        .flat_map(|(i, &n)| {
            let id = if i % 2 == 0 { Some(i as u32 / 2) } else { None };
            iter::repeat_n(id, usize::from(n))
        })
        .collect()
}

fn checksum(blocks: &[Option<u32>]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| x.map(|x| (i, x)))
        .map(|(i, n)| i as u64 * n as u64)
        .sum()
}

fn find_next_file_back(blocks: &[Option<u32>]) -> Option<(usize, usize)> {
    let mut iter = blocks
        .iter()
        .enumerate()
        .rev()
        .skip_while(|(_, id)| id.is_none())
        .peekable();

    let &(tail_incl, id) = iter.peek()?;
    let (head, _) = iter.take_while(|&(_, d)| d == id).last().unwrap();

    Some((head, tail_incl - head + 1))
}

fn find_next_free_fwd(blocks: &[Option<u32>], len: usize) -> Option<usize> {
    let mut free = 0;
    for (i, &id) in blocks.iter().enumerate() {
        if id.is_some() {
            free = 0;
        } else {
            free += 1;
        }

        if free == len {
            return Some(i + 1 - free);
        }
    }
    None
}
