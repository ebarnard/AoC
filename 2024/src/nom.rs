use std::fmt::Debug;

use nom::{
    bytes::complete::take_while1,
    combinator::{complete, eof},
    error::ParseError,
    sequence::terminated,
    IResult, Parser,
};

pub fn parse_all<'a, O, E, F>(input: &'a str, parser: F) -> O
where
    E: ParseError<&'a str> + Debug,
    F: Parser<&'a str, O, E>,
{
    let (_, parsed) = complete(terminated(parser, eof))(input).unwrap();
    parsed
}

pub fn uint8(s: &str) -> IResult<&str, u8> {
    let (s, number) = take_while1(char::is_numeric)(s)?;
    Ok((s, number.parse().unwrap()))
}

pub fn uint32(s: &str) -> IResult<&str, u32> {
    let (s, number) = take_while1(char::is_numeric)(s)?;
    Ok((s, number.parse().unwrap()))
}

pub fn uint64(s: &str) -> IResult<&str, u64> {
    let (s, number) = take_while1(char::is_numeric)(s)?;
    Ok((s, number.parse().unwrap()))
}

pub fn ws(s: &str) -> IResult<&str, &str> {
    take_while1(|c| c == ' ')(s)
}
