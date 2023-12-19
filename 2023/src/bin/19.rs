use std::collections::HashMap;

use aoc2023::nom::{alphanum, character, uint32};
use nom::{
    bytes::complete::tag,
    combinator::{complete, map},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
};

const TEST: &str = include_str!("../../inputs/19-test.txt");
const REAL: &str = include_str!("../../inputs/19-real.txt");

#[derive(Debug)]
struct Workflow {
    name: String,
    comparisons: Vec<Comparison>,
    final_action: String,
}

#[derive(Debug)]
struct Comparison {
    attr: char,
    op: char,
    value: u32,
    action: String,
}

#[derive(Clone, Copy, Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn main() {
    assert_eq!(part1(TEST), 19114);
    assert_eq!(part1(REAL), 367602);
    assert_eq!(part2(TEST), 167409079868000);
    assert_eq!(part2(REAL), 125317461667458);
}

fn parse(s: &str) -> (Vec<Workflow>, Vec<Part>) {
    let action = alphanum;

    let comparison = map(
        tuple((character, character, uint32, tag(":"), action)),
        |(attr, op, value, _, action)| Comparison {
            attr,
            op,
            value,
            action: action.to_owned(),
        },
    );

    let workflow = map(
        pair(
            alphanum,
            delimited(
                tag("{"),
                pair(many0(terminated(comparison, tag(","))), action),
                tag("}"),
            ),
        ),
        |(name, (comparisons, final_action))| Workflow {
            name: name.to_owned(),
            comparisons,
            final_action: final_action.to_owned(),
        },
    );

    let part = map(
        delimited(
            tag("{"),
            tuple((
                preceded(tag("x="), uint32),
                preceded(tag(",m="), uint32),
                preceded(tag(",a="), uint32),
                preceded(tag(",s="), uint32),
            )),
            tag("}"),
        ),
        |(x, m, a, s)| Part { x, m, a, s },
    );

    let input = separated_pair(
        separated_list1(tag("\n"), workflow),
        tag("\n\n"),
        separated_list1(tag("\n"), part),
    );

    let (_, parsed) = complete(input)(s).unwrap();
    parsed
}

fn part1(input: &str) -> u32 {
    let (workflows, parts) = parse(input);
    let workflows: HashMap<_, _> = workflows.into_iter().map(|w| (w.name.clone(), w)).collect();

    parts.iter().fold(0, |rating, &part| {
        if run_action(part, "in", &workflows) {
            rating + part.x + part.m + part.a + part.s
        } else {
            rating
        }
    })
}

fn run_action(part: Part, action: &str, workflows: &HashMap<String, Workflow>) -> bool {
    let workflow = match action {
        "A" => return true,
        "R" => return false,
        workflow => &workflows[workflow],
    };

    for comp in workflow.comparisons.iter() {
        let v = *get_attr(&mut part.clone(), comp.attr);

        if match comp.op {
            '>' => v > comp.value,
            '<' => v < comp.value,
            _ => panic!(),
        } {
            return run_action(part, &comp.action, workflows);
        }
    }

    run_action(part, &workflow.final_action, workflows)
}

fn part2(input: &str) -> u64 {
    let (workflows, _) = parse(input);
    let workflows: HashMap<_, _> = workflows.into_iter().map(|w| (w.name.clone(), w)).collect();

    accepted_combinations(
        Part {
            x: 1,
            m: 1,
            a: 1,
            s: 1,
        },
        Part {
            x: 4000,
            m: 4000,
            a: 4000,
            s: 4000,
        },
        "in",
        &workflows,
    )
}

fn accepted_combinations(
    mut min_part: Part,
    mut max_part: Part,
    action: &str,
    workflows: &HashMap<String, Workflow>,
) -> u64 {
    let workflow = match action {
        "A" => {
            return between(min_part.x, max_part.x)
                * between(min_part.m, max_part.m)
                * between(min_part.a, max_part.a)
                * between(min_part.s, max_part.s)
        }
        "R" => return 0,
        workflow => &workflows[workflow],
    };

    let mut combinations = 0;
    for comp in workflow.comparisons.iter() {
        let min_value = *get_attr(&mut min_part, comp.attr);
        let max_value = *get_attr(&mut max_part, comp.attr);
        assert!(min_value <= max_value);

        if (comp.op == '>' && min_value > comp.value) || (comp.op == '<' && max_value < comp.value)
        {
            // All parts pass comparison.
            return combinations
                + accepted_combinations(min_part, max_part, &comp.action, workflows);
        } else if comp.op == '>' && max_value > comp.value {
            // Some parts pass gt comparison.
            let mut min_passing_part = min_part;
            *get_attr(&mut min_passing_part, comp.attr) = comp.value + 1;
            combinations +=
                accepted_combinations(min_passing_part, max_part, &comp.action, workflows);

            *get_attr(&mut max_part, comp.attr) = comp.value;
        } else if comp.op == '<' && min_value < comp.value {
            // Some parts pass lt comparison.
            let mut max_passing_part = max_part;
            *get_attr(&mut max_passing_part, comp.attr) = comp.value - 1;
            combinations +=
                accepted_combinations(min_part, max_passing_part, &comp.action, workflows);

            *get_attr(&mut min_part, comp.attr) = comp.value;
        }
    }

    combinations + accepted_combinations(min_part, max_part, &workflow.final_action, workflows)
}

fn get_attr(part: &mut Part, attr: char) -> &mut u32 {
    match attr {
        'x' => &mut part.x,
        'm' => &mut part.m,
        'a' => &mut part.a,
        's' => &mut part.s,
        _ => panic!(),
    }
}

fn between(min: u32, max: u32) -> u64 {
    u64::from(max - min + 1)
}
