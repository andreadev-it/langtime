use chrono::prelude::*;
use chrono::Duration;
use nom::branch::alt;
use nom::character::complete::space0;
use nom::sequence::tuple;
use nom::{Parser, IResult};
use nom::bytes::complete::tag;
use nom::combinator::opt;

use crate::parsers::generic::*;
use crate::utils::extract_datetime;

pub fn parse_dmy(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, day) = day1(input)?;
    let (tail, _) = tag("/").parse(tail)?;
    let (tail, month) = month1(tail)?;
    let (tail, _) = tag("/").parse(tail)?;
    let (tail, year) = year(tail)?;

    let dt_opt = Local.with_ymd_and_hms(year, month, day, 0, 0, 0);

    let dt = extract_datetime(dt_opt)?;

    Ok((tail, dt))
}

#[allow(dead_code)]
pub fn parse_mdy(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, month) = month1(input)?;
    let (tail, _) = tag("/").parse(tail)?;
    let (tail, day) = day1(tail)?;
    let (tail, _) = tag("/").parse(tail)?;
    let (tail, year) = year(tail)?;

    let dt_opt = Local.with_ymd_and_hms(year, month, day, 0, 0, 0);

    let dt = extract_datetime(dt_opt)?;

    Ok((tail, dt))
}

pub fn named_dates(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, data) = alt((
        tag("yesterday"),
        tag("tomorrow")
    )).parse(input)?;

    let cur = Local::now().round_subsecs(0);

    match data {
        "yesterday" => Ok((tail, cur - Duration::days(1))),
        "tomorrow" => Ok((tail, cur + Duration::days(1))),
        _ => Err(nom::Err::<()>::Error(()))
    }
}

pub fn parse_time(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, data) = tuple((
        hour1,
        tag(":"),
        minute1,
        opt(
            tuple((
                tag(":"),
                second1,
            ))
        )
    )).parse(input)?;

    let (hour, _, minute, opt_sec) = data;

    let mut second = 0;

    if let Some((_, sec)) = opt_sec {
        second = sec;
    }

    let now = Local::now().round_subsecs(0);
    let dt_opt = Local.with_ymd_and_hms(
        now.year(),
        now.month(),
        now.day(),
        hour,
        minute,
        second
    );

    let dt = extract_datetime(dt_opt)?;

    Ok((tail, dt))
}

pub fn parse_time_ampm(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, data) = tuple((
        hour1,
        opt(
            tuple((
                tag(":"),
                minute1,
                opt(
                    tuple((
                        tag(":"),
                        second1,
                    ))
                )
            ))
        ),
        space0,
        alt((
            tag("a.m."),
            tag("am"),
            tag("p.m."),
            tag("pm")
        ))
    )).parse(input)?;

    let (hour, opt_min_sec, _, ampm) = data;

    if hour > 12 {
        return Err(nom::Err::<()>::Error(()));
    }

    let mut hour = hour;

    match ampm {
        "a.m." | "am" => {
            if hour == 12 { hour = 0 }
        },
        "p.m." | "pm" => {
            if hour < 12 { hour += 12 }
        },
        _ => ()
    }

    let mut minute = 0;
    let mut second = 0;

    match opt_min_sec {
        Some((_, mins, None)) => minute = mins,
        Some((_, mins, Some((_, secs)))) => {
            minute = mins;
            second = secs;
        },
        None => ()
    };

    let now = Local::now().round_subsecs(0);
    let dt_opt = Local.with_ymd_and_hms(
        now.year(),
        now.month(),
        now.day(),
        hour,
        minute,
        second
    );

    let dt = extract_datetime(dt_opt)?;

    Ok((tail, dt))
}
