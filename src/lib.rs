#![warn(missing_docs)]

//! Simple to use crate for converting english-spelled 
//! dates to chrono DateTimes.
//!
//! This crate does a similar job to what [`chrono_english`]
//! already does, but using [`nom`] parsers. This has two
//! advantages:
//! - new formats are easier to add
//! - combination of date and time formats are declarative
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
    let mut alt_parse = alt((
        times,
        full_datetime,
        dates,
        relative_time_past,
        relative_time_future,
    ));

    match alt_parse(input) {
        Ok((_tail, dt)) => Ok(dt),
        Err(_) => Err(NotParsable)
    }
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
