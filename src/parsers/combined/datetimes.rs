use chrono::prelude::*;
use nom::IResult;
use nom::branch::alt;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::bytes::complete::tag;

use crate::parsers::iso::parse_iso;
use crate::parsers::combined::dates;
use crate::parsers::combined::times;
use crate::utils::join_date_time;

pub fn full_datetime(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let (tail, dt) = alt((
        parse_iso,
        map_res(
            tuple((
                dates,
                tag(" at "),
                times
            )),
            |(date, _, time)| join_date_time(date, time)
        )
    ))(input)?;

    Ok((tail, dt))
}
