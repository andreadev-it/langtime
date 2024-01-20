use chrono::{
    DateTime,
    LocalResult,
    Local,
    TimeZone,
    Datelike,
    Timelike,
    Weekday
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
        "monday"   => Ok(0),
        "tuesday"  => Ok(1),
        "wednsday" => Ok(2),
        "thursday" => Ok(3),
        "friday"   => Ok(4),
        "saturday" => Ok(5),
        "sunday"   => Ok(6),
        _ => Err(())
    }
}
