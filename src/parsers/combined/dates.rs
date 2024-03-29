use chrono::prelude::*;
use nom::branch::alt;
use nom::IResult;

use crate::parsers::iso::parse_iso_date;
use crate::parsers::locale::{
    parse_dmy,
    named_dates
};
use crate::parsers::relative::{
    relative_date_past,
    relative_date_future, relative_weekdays, current_weekdays
};

pub fn dates(input: &str) -> IResult<&str, DateTime<Local>, ()> {
    let res = alt((
        parse_iso_date,
        parse_dmy,
        named_dates,
        relative_date_past,
        relative_date_future,
        relative_weekdays,
        current_weekdays
    ))(input)?;

    Ok(res)
}
