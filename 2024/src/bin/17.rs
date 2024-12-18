use nom::{
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
};

use aoc2024::nom::{parse_all, uint64, uint8};

const TEST_A: &str = include_str!("../../inputs/17a-test.txt");
const TEST_B: &str = include_str!("../../inputs/17b-test.txt");
const REAL: &str = include_str!("../../inputs/17-real.txt");

fn main() {
    assert_eq!(part1(TEST_A), [4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    assert_eq!(part1(REAL), [7, 6, 1, 5, 3, 1, 4, 2, 6]);
    assert_eq!(part2(TEST_B), 117440);
    assert_eq!(part2(REAL), 164541017976509);
}

fn parse(input: &str) -> (u64, u64, u64, Vec<u8>) {
    let register = |name| {
        delimited(
            tuple((tag("Register "), tag(name), tag(": "))),
            uint64,
            tag("\n"),
        )
    };

    let instructions = preceded(tag("\nProgram: "), separated_list1(tag(","), uint8));

    let state = tuple((register("A"), register("B"), register("C"), instructions));

    parse_all(input, state)
}

fn part1(input: &str) -> Vec<u8> {
    let (a, b, c, instructions) = parse(input);
    interpret(&instructions, a, b, c)
}

fn part2(input: &str) -> u64 {
    // The instructions in `REAL` are equivalent to this loop.
    //
    // while (a != 0)
    // {
    //     b = a & 0b111
    //     b = b ^ 1
    //     c = a >> b
    //     b = b ^ 0b101
    //     b = b ^ c
    //     a = a >> 3
    //     output(b & 0b111)
    // }
    //
    // Each loop iteration depends only on the bottom 10 bits of register A and
    // register A is shifted left by 3 on every loop iteration.

    let (_, _, _, instructions) = parse(input);
    let instructions_no_jnz = &instructions[..instructions.len() - 2];

    outputs_to_a(instructions_no_jnz, 0, 10, &instructions).unwrap()
}

fn interpret(instructions: &[u8], mut a: u64, mut b: u64, mut c: u64) -> Vec<u8> {
    let mut ip = 0;
    let mut output = Vec::new();
    while (ip as usize + 1) < instructions.len() {
        let opcode = instructions[ip as usize];
        let operand = instructions[ip as usize + 1];
        let combo = combo(operand, a, b, c);

        match opcode {
            // adv
            0 => a = a >> combo,
            // bxl
            1 => b = b ^ u64::from(operand),
            // bst
            2 => b = combo & 0b111,
            // jnz
            3 if a == 0 => (),
            3 => {
                ip = operand;
                continue;
            }
            // bxc
            4 => b = b ^ c,
            // out
            5 => output.push((combo & 0b111) as u8),
            // bdv
            6 => b = a >> combo,
            // cdv
            7 => c = a >> combo,
            _ => panic!(),
        }

        ip += 2;
    }

    output
}

fn combo(operand: u8, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0..=3 => operand.into(),
        4 => a,
        5 => b,
        6 => c,
        _ => panic!(),
    }
}

fn outputs_to_a(instructions: &[u8], a_high: u64, a_low_bits: u64, outputs: &[u8]) -> Option<u64> {
    let Some((&target_output, remaining_outputs)) = outputs.split_last() else {
        return Some(a_high);
    };

    for a_low in 0..(1 << a_low_bits) {
        let a_test = (a_high << a_low_bits) | a_low;
        let output = interpret(instructions, a_test, 0, 0);

        assert_eq!(output.len(), 1);
        if output[0] == target_output {
            if let Some(a) = outputs_to_a(instructions, a_test, 3, remaining_outputs) {
                return Some(a);
            }
        }
    }

    None
}
