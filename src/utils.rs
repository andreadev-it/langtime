use chrono::{DateTime, LocalResult, Local, TimeZone, Datelike, Timelike};

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
