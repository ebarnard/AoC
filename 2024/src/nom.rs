use nom::{
    bytes::complete::{take_while, take_while1},
    IResult,
};

pub fn uint32(s: &str) -> IResult<&str, u32> {
    let (s, number) = take_while1(char::is_numeric)(s)?;
    Ok((s, number.parse().unwrap()))
}

pub fn ws(s: &str) -> IResult<&str, &str> {
    take_while(|c| c == ' ')(s)
}
