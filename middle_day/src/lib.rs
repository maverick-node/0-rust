use chrono::{NaiveDate};
use chrono::Duration;
use chrono::Datelike;
pub fn middle_day(year: u32) -> Option<chrono::Weekday>  {

if year% 2 != 0{
  return None;
}
    let date = NaiveDate::from_ymd_opt(year.try_into().unwrap(), 1, 1).unwrap();
    let middle = date + Duration::days(182);

    Some(middle.weekday())
}


