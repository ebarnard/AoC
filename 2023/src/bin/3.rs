const TEST: &str = include_str!("../../inputs/3-test.txt");
const REAL: &str = include_str!("../../inputs/3-real.txt");

#[derive(Debug)]
struct Number {
    line: usize,
    start: usize,
    end: usize,
    value: u32,
}

#[derive(Debug)]
struct Symbol {
    line: usize,
    col: usize,
}

fn main() {
    assert_eq!(part1(TEST), 4361);
    assert_eq!(part1(REAL), 546563);
    assert_eq!(part2(TEST), 467835);
    assert_eq!(part2(REAL), 91031374);
}

fn parse_numbers(lines: &[&str]) -> Vec<Number> {
    let mut numbers = Vec::new();
    for (line_index, line) in lines.iter().enumerate() {
        let mut chars = line.char_indices().peekable();

        while chars.peek().is_some() {
            while chars.peek().map(|(_, c)| !c.is_numeric()).unwrap_or(false) {
                chars.next();
            }

            let Some(&(start, _)) = chars.peek() else {
                continue;
            };
            while chars.peek().map(|(_, c)| c.is_numeric()).unwrap_or(false) {
                chars.next();
            }
            let end = chars.peek().map(|&(i, _)| i).unwrap_or(line.len());

            numbers.push(Number {
                line: line_index,
                start,
                end,
                value: line[start..end].parse().unwrap(),
            });
        }
    }
    numbers
}

fn parse_symbols(lines: &[&str], is_symbol: impl Fn(char) -> bool) -> Vec<Symbol> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.char_indices()
                .filter(|&(_, c)| is_symbol(c))
                .map(move |(i, _)| Symbol {
                    line: line_index,
                    col: i,
                })
        })
        .collect()
}

fn adjacent_symbols<'a>(
    number: &'a Number,
    symbols: &'a [Symbol],
) -> impl Iterator<Item = usize> + 'a {
    symbols
        .iter()
        .enumerate()
        .filter(|(_, symbol)| {
            (number.line.saturating_sub(1)..=number.line.saturating_add(1)).contains(&symbol.line)
                && (number.start.saturating_sub(1)..number.end.saturating_add(1))
                    .contains(&symbol.col)
        })
        .map(|(i, _)| i)
}

fn part1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let numbers = parse_numbers(&lines);
    let symbols = parse_symbols(&lines, |c| !c.is_numeric() && c != '.');

    // Check if number is adjacent to any symbol
    numbers
        .iter()
        .filter(|number| adjacent_symbols(number, &symbols).next().is_some())
        .map(|number| number.value)
        .sum()
}

fn part2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let numbers = parse_numbers(&lines);
    let symbols = parse_symbols(&lines, |c| c == '*');

    let mut numbers_adjacent_to_symbol = vec![Vec::new(); symbols.len()];
    for number in numbers.iter() {
        for symbol_index in adjacent_symbols(number, &symbols) {
            numbers_adjacent_to_symbol[symbol_index].push(number);
        }
    }

    numbers_adjacent_to_symbol
        .iter()
        .map(|numbers| match numbers.len() {
            0 | 1 => 0,
            2 => numbers[0].value * numbers[1].value,
            _ => panic!(),
        })
        .sum()
}
