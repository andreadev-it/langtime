
use nom::{Parser, IResult};
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while, take_while_m_n};
use nom::combinator::map_res;

pub fn year(input: &str) -> IResult<&str, i32, ()> {
    map_res(
        take_while(|c| char::is_digit(c, 10)),
        |y: &str| y.parse::<i32>()
    ).parse(input)
}

pub fn month1(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take_while_m_n(1, 2, |c| char::is_digit(c, 10)),
        |m: &str| m.parse::<u32>()
    ).parse(input)
}

pub fn month2(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take(2u8),
        |m: &str| m.parse::<u32>()
    ).parse(input)
}

pub fn day1(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take_while_m_n(1, 2, |c| char::is_digit(c, 10)),
        |d: &str| d.parse::<u32>()
    ).parse(input)
}

pub fn day2(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take(2u8),
        |d: &str| d.parse::<u32>()
    ).parse(input)
}

/* Parse the hours taking either 1 or 2 bytes */
pub fn hour1(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take_while_m_n(1, 2, |c| char::is_digit(c, 10)),
        |h: &str| h.parse::<u32>()
    ).parse(input)
}

/* Parse the hours taking 2 bytes */
pub fn hour2(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take(2u8),
        |h: &str| h.parse::<u32>()
    ).parse(input)
}

pub fn minute1(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take_while_m_n(1, 2, |c| char::is_digit(c, 10)),
        |m: &str| m.parse::<u32>()
    ).parse(input)
}

pub fn minute2(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take(2u8),
        |m: &str| m.parse::<u32>()
    ).parse(input)
}

pub fn second1(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take_while_m_n(1, 2, |c| char::is_digit(c, 10)),
        |s: &str| s.parse::<u32>()
    ).parse(input)
}

pub fn second2(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take(2u8),
        |s: &str| s.parse::<u32>()
    ).parse(input)
}

#[allow(dead_code)]
pub fn millisecond(input: &str) -> IResult<&str, u32, ()> {
    map_res(
        take(3u8),
        |ms: &str| ms.parse::<u32>()
    ).parse(input)
}

pub fn weekday(input: &str) -> IResult<&str, &str, ()> {
    alt((
        alt((
            tag("monday"),
            tag("tuesday"),
            tag("wednesday"),
            tag("thursday"),
            tag("friday"),
            tag("saturday"),
            tag("sunday")
        )),
        alt((
            tag("mon"),
            tag("tue"),
            tag("wed"),
            tag("thu"),
            tag("fri"),
            tag("sat"),
            tag("sun")
        ))
    )).parse(input)
}

pub fn month_name(input: &str) -> IResult<&str, &str, ()> {
    alt((
        alt((
            tag("january"),
            tag("february"),
            tag("march"),
            tag("april"),
            tag("may"),
            tag("june"),
            tag("july"),
            tag("august"),
            tag("september"),
            tag("october"),
            tag("november"),
            tag("december")
        )),
        alt((
            tag("jan"),
            tag("feb"),
            tag("mar"),
            tag("apr"),
            tag("jun"),
            tag("jul"),
            tag("aug"),
            tag("sep"),
            tag("oct"),
            tag("nov"),
            tag("dec")
        ))
    )).parse(input)
}
