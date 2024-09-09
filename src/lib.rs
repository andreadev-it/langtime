#![warn(missing_docs)]

//! Easy to use crate for converting english-spelled 
//! dates to chrono DateTimes.
//!
//! This crate does a similar job to what [`chrono_english`]
//! already does, but using [`nom`] parsers. This has two
//! advantages:
//! - new formats are easier to add
//! - combinations of date and time formats are declarative
//!
//! Right now, this crate only exposes one function: `parse`.
//! This function should never panic, but since this library
//! is still under development, if you find anything wrong
//! or if it ever panics, please write an issue on the
//! [`github repo`].
//!
//! # Example
//! ```rust
//! match langtime::parse("yesterday at 9pm") {
//!     Ok(datetime) => println!("{:?}", datetime),
//!     Err(_) => println!("Cannot parse input as a date")
//! }
//! ```
//!
//! [`chrono_english`]: https://docs.rs/chrono-english/latest/chrono_english/
//! [`nom`]: https://docs.rs/nom/latest/nom/
//! [`github repo`]: https://github.com/andreadev-it/langtime

use chrono::prelude::*;
use nom::branch::alt;
use parsers::combined::{full_datetime, dates, times};
use parsers::relative::{
    relative_time_past,
    relative_time_future,
};
use errors::NotParsable;

mod parsers;
mod utils;
mod errors;

/// This function will take a string as an input
/// and try to parse it into a valid Datetime with
/// the local timezone.
pub fn parse(input: &str) -> Result<DateTime<Local>, NotParsable> {
    let config = ParseConfig::default();

    parse_with_config(input, &config)
}

/// This function takes an input string and tries
/// to parse it into a valid Datetime with the local
/// timezone. It also takes a configuration to set
/// the desired english dialect, or to decide whether
/// the string has to be matched in full, or just partially.
pub fn parse_with_config(input: &str, config: &ParseConfig) -> Result<DateTime<Local>, NotParsable> {
    let input = input.trim().to_lowercase();

    let mut alt_parse = alt((
        times,
        full_datetime(config),
        dates(config),
        relative_time_past,
        relative_time_future,
    ));

    match alt_parse(&input) {
        Ok((tail, dt)) => {
            if tail != "" && config.full_string_match == true {
                return Err(NotParsable)
            }
            Ok(dt)
        },
        Err(_) => Err(NotParsable)
    }
}

/// A list of english dialects that will influence
/// how the parser will convert the input string.
/// For example, using mm-dd-yyyy instead of dd-mm-yyyy.
///
#[derive(Eq, PartialEq)]
pub enum Dialect {
    /// US dialect (mm-dd-yyyy)
    US,
    /// UK dialect (dd-mm-yyyy)
    UK
}

/// The configuration for the langtime parse function
pub struct ParseConfig {
    /// The english dialect to use
    dialect: Dialect,
    /// Whether to match the whole string or not
    full_string_match: bool
}

impl Default for ParseConfig {
    fn default() -> Self {
        ParseConfig {
            dialect: Dialect::UK,
            full_string_match: false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_date() {
        let dt = parse("2024-12-23").unwrap();
        assert!(
            dt.day() == 23 &&
            dt.month() == 12 &&
            dt.year() == 2024
        );
    }

    #[test]
    fn test_iso_datetime() {
        let dt = parse("2024-06-05T07:02:24").unwrap();
        assert!(
            dt.day() == 5 &&
            dt.month() == 6 &&
            dt.year() == 2024 &&
            dt.hour() == 07 &&
            dt.minute() == 02 &&
            dt.second() == 24
        );
    }

    #[test]
    fn test_dialect_uk() {
        let dt = parse("12/06/2024").unwrap();

        assert!(
            dt.day() == 12 &&
            dt.month() == 6 &&
            dt.year() == 2024
        );
    }

    #[test]
    fn test_dialect_us() {
        let config = ParseConfig {
            dialect: Dialect::US,
            full_string_match: false
        };

        let dt = parse_with_config("12/06/2024", &config).unwrap();

        assert!(
            dt.day() == 6 &&
            dt.month() == 12 &&
            dt.year() == 2024
        );
    }

    #[test]
    fn test_match_full_text() {
        let mut config = ParseConfig::default();
        config.full_string_match = true;

        let text = "12/06/2024 is the date";

        let dt_res = parse_with_config(text, &config);

        assert!(dt_res.is_err());

        let dt = parse(text).unwrap();

        assert!(
            dt.day() == 12 &&
            dt.month() == 06 &&
            dt.year() == 2024
        );
    }
}
