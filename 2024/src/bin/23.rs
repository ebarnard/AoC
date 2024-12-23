use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::alpha1, multi::separated_list1,
    sequence::separated_pair,
};

use aoc2024::nom::parse_all;

const TEST: &str = include_str!("../../inputs/23-test.txt");
const REAL: &str = include_str!("../../inputs/23-real.txt");

fn main() {
    assert_eq!(part1(TEST), 7);
    assert_eq!(part1(REAL), 1512);
    assert_eq!(part2(TEST), "co,de,ka,ta");
    assert_eq!(part2(REAL), "ac,ed,fh,kd,lf,mb,om,pe,qt,uo,uy,vr,wg");
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    let pair = separated_pair(alpha1::<_, ()>, tag("-"), alpha1);
    let lines = separated_list1(tag("\n"), pair);
    parse_all(input, lines)
}

fn part1(input: &str) -> u32 {
    let pairs = parse(input);

    let mut graph = HashMap::new();
    for (a, b) in pairs {
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let mut cliques = HashSet::new();
    for (n, connected) in graph.iter() {
        for (a, b) in connected.iter().tuple_combinations() {
            if graph[a].contains(b) {
                let mut key = [n, a, b];
                key.sort();
                cliques.insert(key);
            }
        }
    }

    cliques
        .iter()
        .filter(|c| c.iter().any(|n| n.starts_with('t')))
        .count() as u32
}

fn part2(input: &str) -> String {
    let pairs = parse(input);

    let mut graph = HashMap::new();
    for (a, b) in pairs {
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let max_cliques = bron_kerbosch(
        &graph,
        HashSet::new(),
        graph.keys().copied().collect(),
        HashSet::new(),
    );

    let max_clique = max_cliques.iter().max_by_key(|c| c.len()).unwrap();

    max_clique.into_iter().sorted().join(",")
}

fn bron_kerbosch<'a, 'b>(
    graph: &'a HashMap<&'b str, HashSet<&'b str>>,
    r: HashSet<&'b str>,
    mut p: HashSet<&'b str>,
    mut x: HashSet<&'b str>,
) -> Vec<HashSet<&'b str>> {
    if p.is_empty() && x.is_empty() {
        return vec![r];
    }

    let mut max_cliques = Vec::new();

    for v in p.clone() {
        let mut nr = r.clone();
        nr.insert(v);

        let np = p.intersection(&graph[v]).copied().collect();
        let nx = x.intersection(&graph[v]).copied().collect();

        max_cliques.extend_from_slice(&bron_kerbosch(graph, nr, np, nx));

        p.remove(v);
        x.insert(v);
    }

    max_cliques
}
