use chrono::{DateTime, Local, Duration, Datelike, SubsecRound};
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

use crate::utils::{weekday_string_to_int, weekday_to_int, next_month, next_year, last_month, last_year};

pub fn relative_time_past(input: &str) -> IResult<&str, DateTime<Local>, ()> {
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

    let mut dt = Local::now().round_subsecs(0);
    dt -= Duration::seconds(seconds as i64);

    Ok((tail, dt))
}

pub fn relative_date_past(input: &str) -> IResult<&str, DateTime<Local>, ()> {
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

    let mut dt = Local::now().round_subsecs(0);

    for (amount, _, timing, _, _) in data {
        match timing {
            "day" => dt -= Duration::days(amount),
            "week" => dt -= Duration::weeks(amount),
            "month" => dt = last_month(dt)?,
            "year" => dt = last_year(dt)?,
            _ => ()
        }
    }

    Ok((tail, dt))
}

pub fn relative_time_future(input: &str) -> IResult<&str, DateTime<Local>, ()> {
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

    let mut cur = Local::now().round_subsecs(0);

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

pub fn relative_date_future(input: &str) -> IResult<&str, DateTime<Local>, ()> {
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

    let mut cur = Local::now().round_subsecs(0);

    for (amount, _, timing, _, _) in data {
        match timing {
            "day" => cur += Duration::days(amount),
            "week" => cur += Duration::weeks(amount),
            "month" => cur = next_month(cur)?,
            "year" => cur = next_year(cur)?,
            _ => ()
        };
    }

    Ok((tail, cur))
}

pub fn relative_weekdays(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, (rel, _, day)) = tuple((
        alt((
            tag("next"),
            tag("last")
        )),
        space1,
        alt((
            tag("monday"),
            tag("tuesday"),
            tag("wednsday"),
            tag("thursday"),
            tag("friday"),
            tag("saturday"),
            tag("sunday")
        ))
    )).parse(input)?;

    let dt = Local::now().round_subsecs(0);

    let to = weekday_string_to_int(day);

    if to.is_err() {
        return Err(nom::Err::<()>::Error(()));
    }

    let to = to.unwrap();

    let from = weekday_to_int(dt.weekday());

    // Unwrap is safe because of previous check
    let days_diff = match rel {
        "next" => 7 + (to - from),
        "last" => if to >= from {
            - (7 + (from - to))
            } else {
                to - from
            },
        _ => -1 // this is impossible, the nom parser will error
    };

    let result = dt + Duration::days(days_diff);

    Ok((tail, result))
}

pub fn current_weekdays(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, (_, day)) = tuple((
        opt(tag("this ")),
        alt((
            tag("monday"),
            tag("tuesday"),
            tag("wednsday"),
            tag("thursday"),
            tag("friday"),
            tag("saturday"),
            tag("sunday")
        ))
    )).parse(input)?;

    let dt = Local::now().round_subsecs(0);
    let cur_weekday = weekday_to_int(dt.weekday());

    let to = weekday_string_to_int(day);

    if to.is_err() {
        return Err(nom::Err::<()>::Error(()));
    }

    let to = to.unwrap();

    if cur_weekday >= to {
        return Err(nom::Err::<()>::Error(()));
    }

    let result = dt + Duration::days(to - cur_weekday);

    Ok((tail, result))
}
