use chrono::{
    DateTime,
    LocalResult,
    Local,
    TimeZone,
    Datelike,
    Timelike,
    Weekday,
    Duration
};

pub fn extract_datetime(res: LocalResult<DateTime<Local>>) -> Result<DateTime<Local>, nom::Err<()>> {
    match res {
        LocalResult::Single(dt) => Ok(dt),
        LocalResult::Ambiguous(_start, _end) => Err(nom::Err::<()>::Error(())),
        LocalResult::None => Err(nom::Err::<()>::Error(()))
    }
}

pub fn join_date_time(date: DateTime<Local>, time: DateTime<Local>) -> Result<DateTime<Local>, nom::Err<()>> {
    let dt_opt = Local.with_ymd_and_hms(
        date.year(),
        date.month(),
        date.day(), 
        time.hour(), 
        time.minute(), 
        time.second()
    );

    extract_datetime(dt_opt)
}

pub fn weekday_to_int(day: Weekday) -> i64 {
    match day {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6
    }
}

pub fn weekday_string_to_int(day: &str) -> Result<i64, ()> {
    match day {
        "monday"    => Ok(0),
        "mon"       => Ok(0),
        "tuesday"   => Ok(1),
        "tue"       => Ok(1),
        "wednesday" => Ok(2),
        "wed"       => Ok(2),
        "thursday"  => Ok(3),
        "thu"       => Ok(3),
        "friday"    => Ok(4),
        "fri"       => Ok(4),
        "saturday"  => Ok(5),
        "sat"       => Ok(5),
        "sunday"    => Ok(6),
        "sun"       => Ok(6),
        _ => Err(())
    }
}

pub fn month_string_to_int(month: &str) -> Result<u32, ()> {
    match month {
        "january"   => Ok(1),
        "jan"       => Ok(1),
        "february"  => Ok(2),
        "feb"       => Ok(2),
        "march"     => Ok(3),
        "mar"       => Ok(3),
        "april"     => Ok(4),
        "apr"       => Ok(4),
        "may"       => Ok(5),
        "june"      => Ok(6),
        "jun"       => Ok(6),
        "july"      => Ok(7),
        "jul"       => Ok(7),
        "august"    => Ok(8),
        "aug"       => Ok(8),
        "september" => Ok(9),
        "sep"       => Ok(9),
        "october"   => Ok(10),
        "oct"       => Ok(10),
        "november"  => Ok(11),
        "nov"       => Ok(11),
        "december"  => Ok(12),
        "dec"       => Ok(12),
        _ => Err(())
    }
}

pub fn end_of_month(day: DateTime<Local>) -> Result<DateTime<Local>, nom::Err<()>> {
    let mut month = day.month();
    let mut year = day.year();

    month += 1;

    if month > 12 {
        month = 1;
        year += 1;
    }

    let next_month = extract_datetime(Local.with_ymd_and_hms(year, month, 1, 23, 59, 59))?;

    Ok(next_month - Duration::days(1))
}

pub fn month_future(date: DateTime<Local>, amount: u32) -> Result<DateTime<Local>, nom::Err<()>> {
    let mut month = date.month();
    let mut year = date.year();
    let mut day = date.day();

    month += amount;
    while month > 12 {
        month -= 12;
        year += 1;
    }

    let future_month = extract_datetime(Local.with_ymd_and_hms(year, month, 1, 0, 0, 0))?;

    let eonm = end_of_month(future_month)?; // End Of Next Month
    let eonm_day = eonm.day();

    if day > eonm_day {
        day = eonm_day;
    }

    Ok(extract_datetime(Local.with_ymd_and_hms(
        year, 
        month, 
        day, 
        date.hour(), 
        date.minute(), 
        date.second()
    ))?)
}

pub fn year_future(date: DateTime<Local>, amount: i32) -> Result<DateTime<Local>, nom::Err<()>> {
    let year = date.year();
    let mut day = date.day();

    let next_year_first_of_month = extract_datetime(Local.with_ymd_and_hms(
        year + amount, 
        date.month(), 
        1, 
        date.hour(), 
        date.minute(), 
        date.second()
    ))?;

    // All this just to check for leap years
    let eom = end_of_month(next_year_first_of_month)?;
    if day > eom.day() {
        day = eom.day();
    }

    Ok(extract_datetime(Local.with_ymd_and_hms(
        year + amount as i32,
        date.month(),
        day,
        date.hour(),
        date.minute(),
        date.second()
    ))?)
}

pub fn month_past(date: DateTime<Local>, amount: i32) -> Result<DateTime<Local>, nom::Err<()>> {
    let mut month = date.month() as i32;
    let mut year = date.year();
    let mut day = date.day();

    month -= amount;
    while month <= 0 {
        month += 12;
        year -= 1;
    }

    let last_month = extract_datetime(Local.with_ymd_and_hms(year, month as u32, 1, 0, 0, 0))?;

    let eolm = end_of_month(last_month)?; // End Of Last Month
    let eolm_day = eolm.day();

    if day > eolm_day {
        day = eolm_day;
    }

    Ok(extract_datetime(Local.with_ymd_and_hms(
        year, 
        month as u32,
        day, 
        date.hour(), 
        date.minute(), 
        date.second()
    ))?)
}

pub fn year_past(date: DateTime<Local>, amount: i32) -> Result<DateTime<Local>, nom::Err<()>> {
    let year = date.year();
    let mut day = date.day();

    let last_year_first_of_month = extract_datetime(Local.with_ymd_and_hms(
        year - amount, 
        date.month(), 
        1, 
        date.hour(), 
        date.minute(), 
        date.second()
    ))?;

    // All this just to check for leap years
    let eom = end_of_month(last_year_first_of_month)?;
    if day > eom.day() {
        day = eom.day();
    }

    Ok(extract_datetime(Local.with_ymd_and_hms(
        year - amount,
        date.month(),
        day,
        date.hour(),
        date.minute(),
        date.second()
    ))?)
}
