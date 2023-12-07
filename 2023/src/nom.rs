use nom::{
    bytes::complete::{take_while, take_while1},
    error::ParseError,
    sequence::delimited,
    IResult,
};

pub fn uint32(s: &str) -> IResult<&str, u32> {
    let (s, number) = take_while1(char::is_numeric)(s)?;
    Ok((s, number.parse().unwrap()))
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(take_while(|c| c == ' '), inner, take_while(|c| c == ' '))
}
