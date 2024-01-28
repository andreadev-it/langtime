use chrono::prelude::*;
use nom::IResult;
use nom::branch::alt;

use crate::parsers::iso::parse_iso_time;
use crate::parsers::locale::{
    parse_time,
    parse_time_ampm, parse_time_spelled
};

pub fn times(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let res = alt((
        parse_iso_time,
        parse_time,
        parse_time_ampm,
        parse_time_spelled
    ))(input)?;

    Ok(res)
}
