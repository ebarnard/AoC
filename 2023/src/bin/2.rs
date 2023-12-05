use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::complete,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

const TEST: &str = include_str!("../../inputs/2-test.txt");
const REAL: &str = include_str!("../../inputs/2-real.txt");

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Debug, Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    assert_eq!(part1(TEST), 8);
    assert_eq!(part1(REAL), 2879);
    assert_eq!(part2(TEST), 2286);
    assert_eq!(part2(REAL), 65122);
}

fn parse(s: &str) -> Vec<Game> {
    fn game(s: &str) -> IResult<&str, Game> {
        let (s, (id, draws)) = separated_pair(
            preceded(tag("Game "), uinteger),
            tag(": "),
            separated_list1(tag("; "), draw),
        )(s)?;

        Ok((s, Game { id, draws }))
    }

    fn draw(s: &str) -> IResult<&str, Draw> {
        let (s, counts) = separated_list1(
            tag(", "),
            separated_pair(
                uinteger,
                tag(" "),
                alt((tag("red"), tag("green"), tag("blue"))),
            ),
        )(s)?;

        let mut draw = Draw::default();
        for (count, colour) in counts {
            match colour {
                "red" => draw.red = count,
                "green" => draw.green = count,
                "blue" => draw.blue = count,
                _ => panic!(),
            }
        }

        Ok((s, draw))
    }

    fn uinteger(s: &str) -> IResult<&str, u32> {
        let (s, number) = take_while1(char::is_numeric)(s)?;
        Ok((s, number.parse().unwrap()))
    }

    let (_, games) = complete(separated_list1(tag("\n"), game))(s).unwrap();

    games
}

fn part1(input: &str) -> u32 {
    let games = parse(input);

    games
        .into_iter()
        .map(|game| {
            if game
                .draws
                .into_iter()
                .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
            {
                game.id
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let games = parse(input);

    games
        .into_iter()
        .map(|game| {
            let max_red = game.draws.iter().map(|draw| draw.red).max().unwrap();
            let max_green = game.draws.iter().map(|draw| draw.green).max().unwrap();
            let max_blue = game.draws.iter().map(|draw| draw.blue).max().unwrap();

            max_red * max_green * max_blue
        })
        .sum()
}
