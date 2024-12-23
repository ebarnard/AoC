// Depth first search of possible button presses to get from X on number pad to Y on number pad.
// Abort when steps > min_steps
// Abort when retuning to an already visited button (that's got to be bad)?
// Abort when invalid
// End up with a map of min presses from X to Y

use std::{collections::HashMap, u32};

use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/21-test.txt");
const REAL: &str = include_str!("../../inputs/21-real.txt");

// Dir 0 -> right, 1 -> up, 2 -> left, 3 -> down
const RIGHT: u8 = 0;
const UP: u8 = 1;
const LEFT: u8 = 2;
const DOWN: u8 = 3;

const A: u8 = 10;
const D_R: u8 = 0;
const D_U: u8 = 1;
const D_L: u8 = 2;
const D_D: u8 = 3;

fn main() {
    let min_steps_from_to: HashMap<_, _> = (0..=A).map(|pos| (pos, min_steps_from(pos))).collect();

    assert_eq!(part1(TEST, &min_steps_from_to), 126384);
    assert_eq!(part1(REAL, &min_steps_from_to), 206798);
}

fn parse(input: &str) -> Vec<(u32, Vec<u8>)> {
    input
        .lines()
        .map(|line| {
            (
                line[..line.len() - 1].parse().unwrap(),
                line.chars()
                    .map(|c| {
                        if c == 'A' {
                            A
                        } else {
                            let mut dst = [0; 4];
                            let dst = c.encode_utf8(&mut dst);
                            dst.parse().unwrap()
                        }
                    })
                    .collect(),
            )
        })
        .collect()
}

fn part1(input: &str, min_steps_from_to: &HashMap<u8, HashMap<u8, u32>>) -> u64 {
    let codes = parse(input);
    let mut score = 0;
    for (num, code) in codes {
        let mut dist = 0;
        for (a, b) in [&A].into_iter().chain(&code).tuple_windows() {
            dist += min_steps_from_to[a][b];
        }
        println!("{}{}{}{} {dist}", code[0], code[1], code[2], code[3]);
        score += num as u64 * dist as u64;
    }
    score
}

fn min_steps_from(from_keypad_pos: u8) -> HashMap<u8, u32> {
    let mut visited = HashMap::new();
    let mut min_steps = HashMap::new();

    let state = State {
        keypad_pos: from_keypad_pos,
        dirpad_1_pos: A,
        dirpad_2_pos: A,
    };

    min_steps_from_rec(state, 0, &mut min_steps, &mut visited);
    min_steps
}

fn min_steps_from_rec(
    state: State,
    steps: u32,
    steps_to_press: &mut HashMap<u8, u32>,
    steps_to_visit: &mut HashMap<State, u32>,
) {
    let min_steps = steps_to_visit.entry(state).or_insert(u32::MAX);
    if steps >= *min_steps {
        return;
    }
    *min_steps = steps;

    [RIGHT, UP, LEFT, DOWN, A]
        .into_iter()
        .filter_map(|action| state_step(state, action))
        .for_each(|(new_state, pressed)| {
            if pressed {
                let min_steps_to_press = steps_to_press
                    .entry(new_state.keypad_pos)
                    .or_insert(u32::MAX);
                if steps + 1 < *min_steps_to_press {
                    *min_steps_to_press = steps + 1;
                }
            } else {
                min_steps_from_rec(new_state, steps + 1, steps_to_press, steps_to_visit)
            }
        });
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    keypad_pos: u8,
    dirpad_1_pos: u8,
    dirpad_2_pos: u8,
}

fn state_step(mut state: State, action: u8) -> Option<(State, bool)> {
    if action == A {
        if state.dirpad_2_pos == A {
            if state.dirpad_1_pos == A {
                // Keypad button pressed
                return Some((state, true));
            } else {
                state.keypad_pos = numpad_step(state.keypad_pos, state.dirpad_1_pos)?;
            }
        } else {
            state.dirpad_1_pos = dirpad_step(state.dirpad_1_pos, state.dirpad_2_pos)?;
        }
    } else {
        state.dirpad_2_pos = dirpad_step(state.dirpad_2_pos, action)?;
    }
    Some((state, false))
}

fn dirpad_step(pos: u8, dir: u8) -> Option<u8> {
    Some(match (pos, dir) {
        (D_R, UP) => A,
        (D_R, LEFT) => D_D,
        (D_U, RIGHT) => A,
        (D_U, DOWN) => D_D,
        (D_L, RIGHT) => D_D,
        (D_D, RIGHT) => D_R,
        (D_D, UP) => D_U,
        (D_D, LEFT) => D_L,
        (A, LEFT) => D_U,
        (A, DOWN) => D_R,
        (_, RIGHT | UP | LEFT | DOWN) => return None,
        _ => panic!(),
    })
}

fn numpad_step(pos: u8, dir: u8) -> Option<u8> {
    Some(match (pos, dir) {
        (0, RIGHT) => A,
        (0, UP) => 2,
        (1, RIGHT) => 2,
        (1, UP) => 4,
        (2, RIGHT) => 3,
        (2, UP) => 5,
        (2, LEFT) => 1,
        (2, DOWN) => 0,
        (3, UP) => 6,
        (3, LEFT) => 2,
        (3, DOWN) => A,
        (4, RIGHT) => 5,
        (4, UP) => 7,
        (4, DOWN) => 1,
        (5, RIGHT) => 6,
        (5, UP) => 8,
        (5, LEFT) => 4,
        (5, DOWN) => 2,
        (6, UP) => 9,
        (6, LEFT) => 5,
        (6, DOWN) => 3,
        (7, RIGHT) => 8,
        (7, DOWN) => 4,
        (8, RIGHT) => 9,
        (8, LEFT) => 7,
        (8, DOWN) => 5,
        (9, LEFT) => 8,
        (9, DOWN) => 6,
        (A, UP) => 3,
        (A, LEFT) => 0,
        (_, RIGHT | UP | LEFT | DOWN) => return None,
        _ => panic!(),
    })
}
