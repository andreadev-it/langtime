use chrono::prelude::*;
use nom::branch::alt;
use nom::IResult;
use crate::{Dialect, ParseConfig};

use crate::parsers::iso::parse_iso_date;
use crate::parsers::locale::{parse_dmy, named_dates, parse_mdy};
use crate::parsers::relative::{
    relative_date_past,
    relative_date_future, relative_weekdays, current_weekdays
};

pub fn dates(config: &ParseConfig) -> impl Fn(&str) -> IResult<&str, DateTime<Local>, ()> + '_ {
    move |input: &str| {
        let parse_dmy_or_mdy = match config.dialect {
            Dialect::UK => parse_dmy,
            Dialect::US => parse_mdy
        };

        let res = alt((
            parse_iso_date,
            parse_dmy_or_mdy,
            named_dates,
            relative_date_past,
            relative_date_future,
            relative_weekdays,
            current_weekdays
        ))(input)?;

        Ok(res)
    }
}
