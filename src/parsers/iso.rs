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
    fn parse_iso_time_test() {
        let (_, dt) = parse_iso_time("08:20").unwrap();
        assert!(
            dt.hour() == 8 &&
            dt.minute() == 20
        );

        let (_, dt) = parse_iso_time("08:20:10").unwrap();
        assert!(
            dt.hour() == 8 &&
            dt.minute() == 20 &&
            dt.second() == 10
        );
    }

    #[test]
    fn parse_iso_date_test() {
        let (_, dt) = parse_iso_date("2024-10-23").unwrap();
        assert!(
            dt.year() == 2024 &&
            dt.month() == 10 &&
            dt.day() == 23
        );
    }

    #[test]
    fn parse_wrong_iso_date() {
        let mut results = vec![];
        results.push(parse_iso_date("12/12/2024"));
        results.push(parse_iso_date("2024-2-1"));
        results.push(parse_iso_date("2024-33-33"));

        assert!(
            results.into_iter().all(|r| r.is_err())
        );
    }

    #[test]
    fn parse_wrong_iso_time() {
        let mut results = vec![];
        results.push(parse_iso_time("1:2"));
        results.push(parse_iso_time("10"));

        assert!(
            results.iter().all(|r| r.is_err()),
        );
    }
}
