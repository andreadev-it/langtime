use chrono::prelude::*;
use nom::{Parser, IResult};
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::tuple;

use crate::parsers::generic::*;
use crate::utils::extract_datetime;

pub fn parse_iso_date(input: &str) -> IResult<&str, DateTime<chrono::Local>, ()> {
    let (tail, (year, _, month, _, day)) = tuple((
        year,
        tag("-"),
        month2,
        tag("-"),
        day2
    )).parse(input)?;

    let dt_opt = Local.with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0);
    
    let dt = extract_datetime(dt_opt)?;

    Ok((tail, dt))
}

pub fn parse_iso_time(input: &str) -> IResult<&str, DateTime<chrono::Local>, ()> {
    let (tail, data) = tuple((
        hour2,
        tag(":"),
        minute2,
        opt(
            tuple((
                tag(":"),
                second1,
            ))
        )
    )).parse(input)?;

    let (hour, _, minute, opt_sec) = data;

    let mut second = 0;

    match opt_sec {
        Some((_, sec)) => second = sec,
        None => ()
    };

    let now = Local::now();
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

pub fn parse_iso(input: &str) -> IResult<&str, DateTime<chrono::Local>, ()> {
    let (tail, date) = parse_iso_date(input)?;
    let (tail, _) = tag("T").parse(tail)?;
    let (tail, time) = parse_iso_time(tail)?;

    let dt_opt = Local.with_ymd_and_hms(
        date.year(),
        date.month(),
        date.day(),
        time.hour(),
        time.minute(),
        time.second()
    );

    let dt = extract_datetime(dt_opt)?;

    Ok((tail, dt))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_iso_date_test() {
    }

    #[test]
    fn parse_iso_time_test() {
    }

    #[test]
    #[should_panic]
    fn parse_wrong_iso_date() {
            parse_iso_date("12/12/2024").unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_wrong_iso_time() {
        parse_iso_time("05.12.32:000").unwrap();
    }
}
