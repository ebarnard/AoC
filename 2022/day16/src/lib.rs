use std::{collections::HashSet, fs, mem};

pub fn part1(input_path: &str) -> u32 {
    max_pressure_released::<1>(input_path, 30)
}

pub fn part2(input_path: &str) -> u32 {
    max_pressure_released::<2>(input_path, 26)
}

fn max_pressure_released<const N: usize>(input_path: &str, max_time: u32) -> u32 {
    let input = fs::read_to_string(input_path).unwrap();
    let (_, valves) = parser::parse(&input).unwrap();

    dbg!(&valves);

    // Make sure AA is first.
    let valve_map: Vec<_> = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.name == "AA")
        .chain(
            valves
                .iter()
                .enumerate()
                .filter(|(_, v)| v.name != "AA" && v.flow_rate > 0),
        )
        .map(|(i, _)| i)
        .collect();

    dbg!(&valve_map);

    let min_distances: Vec<Vec<_>> = valve_map
        .iter()
        .map(|i| {
            let distances = shortest_distances(&valves, *i);
            valve_map.iter().map(|i| distances[*i] + 1).collect()
        })
        .collect();

    dbg!(&min_distances);

    let flow_rates: Vec<_> = valve_map.iter().map(|i| valves[*i].flow_rate).collect();

    let mut max_pressure: u32 = 0;
    let initial_state = PathState {
        positions: [Position {
            valve: 0,
            remaining_time: max_time,
            pressure: 0,
            flow_rate: 0,
        }; N],
        to_visit: (1..valve_map.len()).fold(BitSet::EMPTY, |set, i| set.insert(i)),
    };
    visit(
        initial_state,
        &min_distances,
        &flow_rates,
        flow_rates.iter().sum(),
        &mut max_pressure,
    );

    max_pressure
}

fn visit<const N: usize>(
    current: PathState<N>,
    min_distances: &Vec<Vec<u32>>,
    flow_rates: &Vec<u32>,
    max_flow_rate: u32,
    max_pressure_lb: &mut u32,
) {
    fn check_final_pressure<const N: usize>(state: &PathState<N>, max_pressure_lb: &mut u32) {
        let final_pressure = state.positions.iter().fold(0, |acc, p| {
            acc + p.pressure + p.flow_rate * p.remaining_time
        });
        if final_pressure > *max_pressure_lb {
            println!(
                "Max pressure LB increasing from {} to {}",
                *max_pressure_lb, final_pressure
            );
            *max_pressure_lb = final_pressure;
        }
    }

    if current.to_visit == BitSet::EMPTY {
        check_final_pressure(&current, max_pressure_lb);
        return;
    }

    let (active_position_idx, active_position) = current
        .positions
        .iter()
        .enumerate()
        .max_by_key(|(_, p)| p.remaining_time)
        .unwrap();

    for valve in current.to_visit.into_iter() {
        let distance = min_distances[active_position.valve][valve];
        if distance >= active_position.remaining_time {
            check_final_pressure(&current, max_pressure_lb);
            continue;
        }

        // Move to the new valve.
        let next_position = Position {
            valve,
            remaining_time: active_position.remaining_time - distance,
            pressure: active_position.pressure + active_position.flow_rate * distance,
            flow_rate: active_position.flow_rate + flow_rates[valve],
        };

        let mut positions = current.positions;
        positions[active_position_idx] = next_position;
        let next = PathState {
            positions,
            to_visit: current.to_visit.remove(valve),
        };

        // Find an upper bound on the maximum pressure released by this path.
        // Imagine we can open all remaining valves as soon as we reach the destination valve.
        // If that final pressure is lower than the lower bound of the max final pressure then
        // discard this path.
        let max_pressure_ub = positions.iter().map(|p| p.pressure).sum::<u32>()
            + max_flow_rate * positions.iter().map(|p| p.remaining_time).max().unwrap();
        if max_pressure_ub < *max_pressure_lb {
            continue;
        }

        visit(
            next,
            min_distances,
            flow_rates,
            max_flow_rate,
            max_pressure_lb,
        );
    }
}

struct PathState<const N: usize> {
    positions: [Position; N],
    to_visit: BitSet,
}

#[derive(Copy, Clone)]
struct Position {
    valve: usize,
    remaining_time: u32,
    pressure: u32,
    flow_rate: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct BitSet(u64);

impl BitSet {
    const EMPTY: BitSet = BitSet(0);

    fn insert(self, valve: usize) -> BitSet {
        BitSet(self.0 | (1 << valve))
    }

    fn remove(self, valve: usize) -> BitSet {
        BitSet(self.0 & !(1 << valve))
    }

    fn into_iter(&self) -> BitSetIter {
        BitSetIter(self.0)
    }
}

struct BitSetIter(u64);

impl Iterator for BitSetIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let lowest_bit = self.0 & -(self.0 as i64) as u64;
        self.0 &= !lowest_bit;
        Some((usize::BITS - lowest_bit.leading_zeros() - 1) as usize)
    }
}

fn shortest_distances(valves: &[parser::Valve], start_idx: usize) -> Vec<u32> {
    let mut shortest_distances = vec![u32::MAX; valves.len()];
    let mut wavefront = HashSet::new();
    let mut next_wavefront = HashSet::new();

    shortest_distances[start_idx] = 0;
    wavefront.insert(start_idx);

    while !wavefront.is_empty() {
        for i in wavefront.drain() {
            let start_valve = &valves[i];
            for name in &start_valve.tunnels {
                let (j, _) = valves
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.name == *name)
                    .next()
                    .unwrap();
                if shortest_distances[j] > shortest_distances[i] + 1 {
                    shortest_distances[j] = shortest_distances[i] + 1;
                    next_wavefront.insert(j);
                }
            }
        }
        mem::swap(&mut wavefront, &mut next_wavefront);
    }

    shortest_distances
}

mod parser {
    #[derive(Debug)]
    pub struct Valve {
        pub name: String,
        pub flow_rate: u32,
        pub tunnels: Vec<String>,
    }

    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::complete::line_ending,
        combinator::{complete, map, map_res},
        multi::separated_list1,
        sequence::tuple,
        IResult,
    };

    pub fn parse(s: &str) -> IResult<&str, Vec<Valve>> {
        complete(separated_list1(line_ending, parse_valve))(s)
    }

    fn parse_valve(s: &str) -> IResult<&str, Valve> {
        map(
            tuple((
                tag("Valve "),
                take_while1(|c: char| c.is_ascii_alphabetic()),
                tag(" has flow rate="),
                map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
                    s.parse::<u32>()
                }),
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                parse_tunnels,
            )),
            |(_, name, _, flow_rate, _, tunnels)| Valve {
                name: name.to_owned(),
                flow_rate,
                tunnels: tunnels.into_iter().map(|t| t.to_owned()).collect(),
            },
        )(s)
    }

    fn parse_tunnels(s: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(tag(", "), take_while1(|c: char| c.is_ascii_alphabetic()))(s)
    }
}

#[test]
fn part1_test() {
    assert_eq!(part1("test.txt"), 1651);
}

#[test]
fn part1_real() {
    assert_eq!(part1("real.txt"), 1751);
}

#[test]
fn part2_test() {
    assert_eq!(part2("test.txt"), 1707);
}

#[test]
fn part2_real() {
    assert_eq!(part2("real.txt"), 2207);
}
