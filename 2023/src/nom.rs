use nom::{
    bytes::complete::{take, take_while, take_while1},
    combinator::map,
    error::ParseError,
    sequence::delimited,
    IResult,
};

pub fn uint32(s: &str) -> IResult<&str, u32> {
    let (s, number) = take_while1(char::is_numeric)(s)?;
    Ok((s, number.parse().unwrap()))
}

pub fn uint64(s: &str) -> IResult<&str, u64> {
    let (s, number) = take_while1(char::is_numeric)(s)?;
    Ok((s, number.parse().unwrap()))
}

pub fn int128(s: &str) -> IResult<&str, i128> {
    let (s, number) = take_while1(|c: char| c.is_numeric() || c == '-')(s)?;
    Ok((s, number.parse().unwrap()))
}

pub fn alphanum(s: &str) -> IResult<&str, &str> {
    take_while1(|c| char::is_ascii_alphanumeric(&c))(s)
}

pub fn character(s: &str) -> IResult<&str, char> {
    map(take(1usize), |s: &str| s.chars().next().unwrap())(s)
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(take_while(|c| c == ' '), inner, take_while(|c| c == ' '))
}
