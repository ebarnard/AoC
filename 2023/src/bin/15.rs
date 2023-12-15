const TEST: &str = include_str!("../../inputs/15-test.txt");
const REAL: &str = include_str!("../../inputs/15-real.txt");

fn main() {
    assert_eq!(part1(TEST), 1320);
    assert_eq!(part1(REAL), 513158);
    assert_eq!(part2(TEST), 145);
    assert_eq!(part2(REAL), 200277);
}

fn parse(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

fn part1(input: &str) -> u32 {
    let steps = parse(input);
    steps.iter().map(|step| hash(step) as u32).sum()
}

fn part2(input: &str) -> u32 {
    let steps = parse(input);

    let mut buckets = [(); 256].map(|_| Vec::new());

    for step in steps {
        let (label, focal_length) = if let Some((label, "")) = step.split_once('-') {
            (label, None)
        } else if let Some((label, focal_length)) = step.split_once('=') {
            (label, Some(focal_length.parse::<u32>().unwrap()))
        } else {
            panic!("bad instruction: {step}");
        };

        let bucket = &mut buckets[hash(label) as usize];
        match (bucket.iter().position(|&(l, _)| l == label), focal_length) {
            (Some(idx), Some(focal_length)) => bucket[idx] = (label, focal_length),
            (None, Some(focal_length)) => bucket.push((label, focal_length)),
            (Some(idx), None) => {
                bucket.remove(idx);
            }
            (None, None) => (),
        }
    }

    buckets
        .iter()
        .enumerate()
        .flat_map(|(i, bucket)| {
            bucket
                .iter()
                .enumerate()
                .map(move |(j, &(_, focal_length))| (i as u32 + 1) * (j as u32 + 1) * focal_length)
        })
        .sum()
}

fn hash(input: &str) -> u8 {
    input
        .chars()
        .fold(0u8, |hash, c| ((hash as u32 + c as u32) * 17 % 256) as u8)
}
