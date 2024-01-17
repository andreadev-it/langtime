use chrono::{DateTime, Local, Duration};
use nom::{Parser, IResult};
use nom::character::complete::{
    digit1,
    space1
};
use nom::combinator::{map_res, opt};
use nom::sequence::tuple;
use nom::multi::many1;
use nom::branch::alt;
use nom::bytes::complete::tag;

pub fn time_ago(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, (data, _)) = tuple((
        many1(
            tuple((
                map_res(digit1, |s: &str| s.parse::<usize>()),
                space1,
                alt((
                    tag("hour"),
                    tag("minute"),
                    tag("second")
                )),
                opt(tag("s")),
                opt(alt( (tag(" and "), tag(", ")) ))
            ))
        ),
        tag(" ago")
    )).parse(input)?;

    let mut seconds = 0;

    for (amount, _, timing, _, _) in data {
        match timing {
            "hour" => seconds += amount * 60 * 60,
            "minute" => seconds += amount * 60,
            "second" => seconds += amount,
            _ => ()
        }
    }

    let mut dt = Local::now();
    dt = dt - Duration::seconds(seconds as i64);

    Ok((tail, dt))
}

pub fn date_ago(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, (data, _)) = tuple((
        many1(
            tuple((
                map_res(digit1, |s: &str| s.parse::<i64>()),
                space1,
                alt((
                    tag("day"),
                    tag("week"),
                    tag("month"),
                    tag("year")
                )),
                opt(tag("s")),
                opt(alt( (tag(" and "), tag(", ")) ))
            ))
        ),
        tag(" ago")
    )).parse(input)?;

    let mut dt = Local::now();

    for (amount, _, timing, _, _) in data {
        match timing {
            "day" => dt -= Duration::days(amount),
            "week" => dt -= Duration::weeks(amount),
            "month" => dt -= Duration::weeks(4 * amount),
            "year" => dt -= Duration::days(365 * amount),
            _ => ()
        }
    }

    Ok((tail, dt))
}

pub fn in_time(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, (_, _, data)) = tuple((
        tag("in"),
        space1,
        many1(
            tuple((
                map_res(digit1, |s: &str| s.parse::<i64>()),
                space1,
                alt((
                    tag("hour"),
                    tag("minute"),
                    tag("second")
                )),
                opt(tag("s")),
                opt(alt( (tag(" and "), tag(", ")) ))
            ))
        )
    )).parse(input)?;

    let mut cur = Local::now();

    for (amount, _, timing, _, _) in data {
        match timing {
            "hour" => cur += Duration::seconds(60 * 60 * amount),
            "minute" => cur += Duration::seconds(60 * amount),
            "second" => cur += Duration::seconds(amount),
            _ => ()
        };
    }

    Ok((tail, cur))
}

pub fn in_date(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, (_, _, data)) = tuple((
        tag("in"),
        space1,
        many1(
            tuple((
                map_res(digit1, |s: &str| s.parse::<i64>()),
                space1,
                alt((
                    tag("day"),
                    tag("week"),
                    tag("month"),
                    tag("year")
                )),
                opt(tag("s")),
                opt(alt( (tag(" and "), tag(", ")) ))
            ))
        )
    )).parse(input)?;

    let mut cur = Local::now();

    for (amount, _, timing, _, _) in data {
        match timing {
            "day" => cur += Duration::days(amount),
            "week" => cur += Duration::weeks(amount),
            "month" => cur += Duration::weeks(4 * amount),
            "year" => cur += Duration::days(365 * amount),
            _ => ()
        };
    }

    Ok((tail, cur))
}

