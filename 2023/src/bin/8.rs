use aoc2023::nom::alphanum;
use nom::{
    bytes::complete::tag,
    combinator::{complete, map},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
};

const TEST_A: &str = include_str!("../../inputs/8a-test.txt");
const TEST_B: &str = include_str!("../../inputs/8b-test.txt");
const REAL: &str = include_str!("../../inputs/8-real.txt");

#[derive(Debug)]
struct Network {
    directions: String,
    nodes: Vec<Node>,
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Network {
    fn advance(&self, node_id: &str, direction: char) -> &str {
        let node = self.nodes.iter().find(|node| node.id == node_id).unwrap();
        match direction {
            'L' => &node.left,
            'R' => &node.right,
            _ => unreachable!(),
        }
    }
}

fn main() {
    assert_eq!(part1(TEST_A), 6);
    assert_eq!(part1(REAL), 16343);
    assert_eq!(part2(TEST_B), 6);
    assert_eq!(part2(REAL), 15299095336639);
}

fn parse(s: &str) -> Network {
    let node = map(
        tuple((
            alphanum,
            tag(" = ("),
            alphanum,
            tag(", "),
            alphanum,
            tag(")"),
        )),
        |(id, _, left, _, right, _)| Node {
            id: id.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        },
    );

    let nodes = separated_list1(tag("\n"), node);

    let network = map(
        separated_pair(alphanum, tag("\n\n"), nodes),
        |(diretions, nodes)| Network {
            directions: diretions.to_owned(),
            nodes,
        },
    );

    let (_, network) = complete(network)(s).unwrap();
    network
}

fn part1(input: &str) -> u32 {
    let network = parse(input);

    network
        .directions
        .chars()
        .cycle()
        .scan("AAA", |id, direction| {
            *id = network.advance(*id, direction);
            if *id == "ZZZ" {
                None
            } else {
                Some(())
            }
        })
        .count() as u32
        + 1
}

fn part2(input: &str) -> u64 {
    // The input defines multiple DFAs such that each starting node only visits
    // a single terminating node given the input.
    //
    // A DFA with a repeating input must be periodic with the period a multiple
    // of the input length.
    //
    // We must find the number of steps to get from each starting node to each
    // terminating node, and the cycle length to get back to the terminating
    // node, giving:
    //
    // steps_to_terminator + num_cycles * terminator_cycle_length = answer
    // a_i                 + m_i        * b_i                     = c
    //
    // In this case it turns out that a_i == b_i.

    let network = parse(input);

    let start_nodes = network.nodes.iter().filter(|node| node.id.ends_with('A'));

    let steps: Vec<_> = start_nodes
        .map(|node| steps_to_any_terminating_node(&network, &node.id))
        .collect();

    // Brute force a solution
    for i in 0.. {
        let candidate = steps[0].0 + steps[0].1 * i;
        if steps
            .iter()
            .all(|&(a, b)| candidate > a && (candidate - a) % b == 0)
        {
            return candidate;
        }
    }

    unreachable!()
}

fn steps_to_any_terminating_node(network: &Network, start_id: &str) -> (u64, u64) {
    let mut directions = network.directions.chars().cycle();

    // Find number of steps from starting to terminating node.
    let mut id = start_id;
    let mut steps_to_term = 0;
    while !id.ends_with('Z') {
        id = network.advance(id, directions.next().unwrap());
        steps_to_term += 1;
    }
    let terminating_id = id;

    // Find cycle period to get back to terminating node.
    for cycle_period in 1.. {
        id = network.advance(id, directions.next().unwrap());

        // This is only guarenteed to be periodic if the cycle period is a
        // multiple of the instruction length, but this works for the input we
        // have been given.
        if id == terminating_id {
            println!(
                "{}: steps to terminator = {}, cycle period = {}",
                start_id, steps_to_term, cycle_period
            );
            return (steps_to_term, cycle_period);
        } else if id.ends_with('Z') {
            panic!(
                "{}: multiple terminators reachable from starting node",
                start_id
            );
        }
    }

    unreachable!()
}
