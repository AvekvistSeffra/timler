use super::schema::entries;
use chrono::{NaiveDate, NaiveTime};

#[derive(Queryable, Clone, Copy)]
pub struct EntryEntity {
    pub id: i32,
    pub work_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: Option<NaiveTime>,
}

impl std::fmt::Display for EntryEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.end_time {
            Some(end_time) => write!(
                f,
                "[{}]: start: {}, end: {}",
                self.work_date, self.start_time, end_time
            ),
            None => write!(f, "[{}]: start: {}", self.work_date, self.start_time),
        }
    }
}

#[derive(Insertable)]
#[table_name = "entries"]
pub struct Entry {
    pub work_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: Option<NaiveTime>,
}
