use chrono::prelude::*;
use nom::branch::alt;
use nom::IResult;
use crate::{Dialect, ParseConfig};

use crate::parsers::iso::parse_iso_date;
use crate::parsers::locale::{named_dates, named_months, parse_dmy, parse_mdy, parse_my, spelled_dates_uk, spelled_dates_us};
use crate::parsers::relative::{
    relative_date_past,
    relative_date_future, relative_weekdays, current_weekdays
};

pub fn dates(config: &ParseConfig) -> impl Fn(&str) -> IResult<&str, DateTime<Local>, ()> + '_ {
    move |input: &str| {
        let parse_uk_or_us = match config.dialect {
            Dialect::UK => parse_dmy,
            Dialect::US => parse_mdy
        };

        let parse_spelled_uk_or_us = match config.dialect {
            Dialect::UK => spelled_dates_uk,
            Dialect::US => spelled_dates_us
        };

        let res = alt((
            parse_iso_date,
            parse_uk_or_us,
            parse_spelled_uk_or_us,
            parse_my,
            named_dates,
            named_months,
            relative_date_past,
            relative_date_future,
            relative_weekdays,
            current_weekdays
        ))(input)?;

        Ok(res)
    }
}
