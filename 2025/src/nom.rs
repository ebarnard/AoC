use std::fmt::Debug;

use nom::{
    IResult, Parser,
    bytes::complete::take_while1,
    combinator::{complete, eof},
    error::ParseError,
    sequence::terminated,
};

pub fn parse_all<'a, O, E, F>(input: &'a str, parser: F) -> O
where
    E: ParseError<&'a str> + Debug,
    F: Parser<&'a str, Output = O, Error = E>,
{
    let (_, parsed) = complete(terminated(parser, eof)).parse(input).unwrap();
    parsed
}

pub fn int32(s: &str) -> IResult<&str, i32> {
    let (s, number) = take_while1(|c| char::is_numeric(c) || c == '-')(s)?;
    Ok((s, number.parse().unwrap()))
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
