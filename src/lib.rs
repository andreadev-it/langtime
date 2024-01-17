use chrono::prelude::*;
use nom::branch::alt;
use parsers::combined::{full_datetime, dates, times};
use parsers::relative::{time_ago, date_ago, in_time, in_date};

mod parsers;
mod utils;

pub fn parse(input: &str) -> Result<DateTime<Local>, nom::Err<()>> {
    let mut alt_parse = alt((
        times,
        full_datetime,
        dates,
        time_ago,
        date_ago,
        in_time,
        in_date
    ));

    let (_tail, dt) = alt_parse(&input)?;

    Ok(dt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let dt = parse("2024-12-23").unwrap();
        assert!(
            dt.day() == 23 &&
            dt.month() == 12 &&
            dt.year() == 2024
        );
    }
}
