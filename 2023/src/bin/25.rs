use std::iter;

use aoc2023::nom::alphanum;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::complete, multi::separated_list0, sequence::separated_pair,
};
use osqp::{CscMatrix, Problem, Settings, Status};

const TEST: &str = include_str!("../../inputs/25-test.txt");
const REAL: &str = include_str!("../../inputs/25-real.txt");

fn main() {
    assert_eq!(part1(TEST, "cmg", "ntq"), 54);
    assert_eq!(part1(REAL, "dlc", "qmt"), 558376);
}

fn parse(input: &str) -> Vec<(&str, Vec<&str>)> {
    let node = separated_pair(alphanum, tag(": "), separated_list0(tag(" "), alphanum));
    let graph = separated_list0(tag("\n"), node);

    let (s, parsed) = complete(graph)(input).unwrap();
    assert_eq!(s.len(), 0);
    parsed
}

fn part1(input: &str, start: &str, end: &str) -> u32 {
    let parsed = parse(input);

    let nodes = parsed
        .iter()
        .flat_map(|(a, n)| iter::once(a).chain(n))
        .unique()
        .collect_vec();
    let edges = parsed
        .iter()
        .flat_map(|(a, n)| n.iter().map(|b| (*a, *b)))
        .collect_vec();

    // Indices 0 to (parsed.len() - 1) correspond to nodes
    // Indicies after that correspond to edges
    let objective = iter::repeat(0.0)
        .take(nodes.len())
        .chain(iter::repeat(1.0).take(edges.len()))
        .collect_vec();

    let node_idx = |name: &str| nodes.iter().position(|&&n| n == name).unwrap();

    let mut constraints = Vec::new();
    let mut min = Vec::new();
    let mut max = Vec::new();

    // Start node in in S
    for i in 0..nodes.len() {
        let mut constraint = vec![0.0; objective.len()];
        constraint[i] = 1.0;
        constraints.push(constraint);
        min.push(0.0);
        max.push(f64::INFINITY);
    }

    // Edge nodes split graph
    for (i, (a, b)) in edges.iter().enumerate() {
        let mut constraint = vec![0.0; objective.len()];
        constraint[nodes.len() + i] = 1.0;
        constraint[node_idx(a)] = -1.0;
        constraint[node_idx(b)] = 1.0;
        constraints.push(constraint);
        min.push(0.0);
        max.push(f64::INFINITY);

        let mut constraint = vec![0.0; objective.len()];
        constraint[nodes.len() + i] = 1.0;
        constraint[node_idx(a)] = 1.0;
        constraint[node_idx(b)] = -1.0;
        constraints.push(constraint);
        min.push(0.0);
        max.push(f64::INFINITY);

        // Edge node is positive
        let mut constraint = vec![0.0; objective.len()];
        constraint[nodes.len() + i] = 1.0;
        constraints.push(constraint);
        min.push(0.0);
        max.push(f64::INFINITY);
    }

    let n = objective.len();
    #[allow(non_snake_case)]
    let P = CscMatrix::from(iter::repeat(iter::repeat(&0.0).take(n)).take(n)).into_upper_tri();

    let settings = Settings::default().eps_prim_inf(1e-5).eps_abs(1e-5);

    let mut prob = Problem::new(P, &objective, &constraints, &min, &max, &settings)
        .expect("failed to setup problem");

    min[node_idx(start)] = 1.0;
    max[node_idx(start)] = 1.0;
    min[node_idx(end)] = 0.0;
    max[node_idx(end)] = 0.0;

    prob.update_bounds(&min, &max);

    let Status::Solved(solution) = prob.solve() else {
        panic!()
    };

    let s = solution.x()[..nodes.len()]
        .iter()
        .filter(|&v| v.abs() < 1e-3)
        .count() as u32;
    let e = solution.x()[..nodes.len()]
        .iter()
        .filter(|&v| (v - 1.0).abs() < 1e-3)
        .count() as u32;
    assert_eq!(s + e, nodes.len() as u32);

    let edge_cuts = solution.x()[nodes.len()..]
        .iter()
        .enumerate()
        .filter(|&(_, v)| (v - 1.0).abs() < 1e-3)
        .map(|(i, _)| edges[i])
        .collect_vec();

    dbg!(edge_cuts);

    s * e
}
