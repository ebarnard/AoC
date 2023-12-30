use std::{collections::HashMap, mem};

use aoc2023::nom::{alphanum, character};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    combinator::complete,
    multi::separated_list0,
    sequence::{preceded, tuple},
};

const TEST: &str = include_str!("../../inputs/20-test.txt");
const REAL: &str = include_str!("../../inputs/20-real.txt");

type ParsedModule<'a> = (char, &'a str, Vec<&'a str>);

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    init_state: ModuleState,
    conns_in: HashMap<&'a str, usize>,
    conns_out: Vec<&'a str>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ModuleState {
    Broadcast,
    FlipFlop(bool),
    Conjunction(u32),
}

fn main() {
    assert_eq!(part1(TEST), 11687500);
    assert_eq!(part1(REAL), 861743850);
    assert_eq!(part2(REAL), 247023644760071);
}

fn part1(input: &str) -> u32 {
    let parsed = parse(input);
    let modules = convert(&parsed);

    let mut states = modules.iter().map(|(&k, v)| (k, v.init_state)).collect();
    let (mut highs, mut lows) = (0, 0);
    for _ in 0..1000 {
        press_button(&modules, &mut states, |_, _, high| match high {
            true => highs += 1,
            false => lows += 1,
        });
    }

    lows * highs
}

fn part2(input: &str) -> u64 {
    // The puzzle input consists of four 12-bit counters which each reset after
    // a different number of cycles (button presses).
    //
    // The number of cycles to before reset is controlled by which bits connect
    // into the central conjunction module of each counter.
    //
    // The result is the LCM of number of cycles to reset for all counters.
    // As all the cycle counts are prime, we just multiply the values.

    let parsed = parse(input);
    let modules = convert(&parsed);

    let (final_inverter,) = modules
        .values()
        .filter(|v| v.conns_out.contains(&"rx"))
        .collect_tuple()
        .expect("rx node must have a single input");

    let mut cycle_lengths: HashMap<_, _> = modules
        .values()
        .filter(|v| v.conns_out.contains(&final_inverter.name))
        .map(|m| (m.name, None))
        .collect();

    let mut states = modules.iter().map(|(&k, v)| (k, v.init_state)).collect();
    for i in 0..4096 {
        press_button(&modules, &mut states, |from, _, high| {
            match (cycle_lengths.get_mut(from), high) {
                (Some(cycles), true) => *cycles = Some(i),
                _ => (),
            }
        });
    }

    cycle_lengths.values().map(|c| 1 + c.unwrap()).product()
}

fn parse(input: &str) -> Vec<ParsedModule> {
    let module = tuple((
        character,
        alphanum,
        preceded(tag(" -> "), separated_list0(tag(", "), alphanum)),
    ));
    let modules = separated_list0(tag("\n"), module);

    let (s, parsed) = complete(modules)(input).unwrap();
    assert_eq!(s.len(), 0);
    parsed
}

fn convert<'a>(parsed: &[ParsedModule<'a>]) -> HashMap<&'a str, Module<'a>> {
    parsed
        .iter()
        .map(|(ty, name, conns)| Module {
            name,
            init_state: match ty {
                'b' => ModuleState::Broadcast,
                '%' => ModuleState::FlipFlop(false),
                '&' => ModuleState::Conjunction(0),
                _ => panic!(),
            },
            conns_in: parsed
                .iter()
                .filter(|(_, _, c)| c.contains(name))
                .enumerate()
                .map(|(i, &(_, n, _))| (n, i))
                .collect(),
            conns_out: conns.clone(),
        })
        .map(|module| (module.name, module))
        .collect()
}

fn press_button<'a>(
    modules: &HashMap<&'a str, Module<'a>>,
    states: &mut HashMap<&'a str, ModuleState>,
    mut pulse_cb: impl FnMut(&str, &str, bool),
) {
    let mut pulses = Vec::new();
    let mut next_pulses = Vec::new();

    // The `b` was stolen to indicate the type.
    pulses.push(("button", "roadcaster", false));
    while !pulses.is_empty() {
        for (from, to, high) in pulses.drain(..) {
            pulse_cb(&from, &to, high);

            let Some(module) = modules.get(to) else {
                continue;
            };
            let state = states.get_mut(to).unwrap();
            if let Some(high) = module.handle(state, &from, high) {
                for &conn in module.conns_out.iter() {
                    next_pulses.push((to, conn, high));
                }
            };
        }
        mem::swap(&mut pulses, &mut next_pulses);
    }
}

impl<'a> Module<'a> {
    fn handle(&self, state: &mut ModuleState, from: &str, high: bool) -> Option<bool> {
        match state {
            ModuleState::Broadcast => Some(high),
            ModuleState::FlipFlop(_) if high => None,
            ModuleState::FlipFlop(state) => {
                *state = !*state;
                Some(*state)
            }
            ModuleState::Conjunction(state) => {
                let index = self.conns_in[from];
                *state = (*state & !(1 << index)) | ((high as u32) << index);
                Some(state.count_ones() as usize != self.conns_in.len())
            }
        }
    }
}
