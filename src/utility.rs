use crate::models::EntryEntity;

pub fn today() -> chrono::NaiveDate {
    use chrono::Datelike;

    let now = chrono::Utc::now()
        .with_timezone(&chrono_tz::Europe::Stockholm)
        .naive_local();

    chrono::NaiveDate::from_ymd(now.year(), now.month(), now.day())
}

pub fn now() -> chrono::NaiveTime {
    use chrono::Timelike;

    let now = chrono::Utc::now()
        .with_timezone(&chrono_tz::Europe::Stockholm)
        .naive_local();

    chrono::NaiveTime::from_hms(now.hour(), now.minute(), now.second())
}

#[derive(Serialize)]
pub struct Duration {
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

impl Duration {
    pub fn new(duration: chrono::Duration) -> Duration {
        let mut temp_duration = duration;
        let hours = temp_duration.num_hours();
        temp_duration = temp_duration - chrono::Duration::hours(hours);

        let minutes = temp_duration.num_minutes();
        temp_duration = temp_duration - chrono::Duration::minutes(minutes);

        let seconds = temp_duration.num_seconds();

        Duration {
            hours: hours,
            minutes: minutes,
            seconds: seconds,
        }
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hours, self.minutes, self.seconds)
    }
}

#[derive(Serialize)]
pub struct EntryContext {
    pub id: i32,
    pub work_date: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration: Option<Duration>,
}

impl From<EntryEntity> for EntryContext {
    fn from(entity: EntryEntity) -> Self {
        match entity.end_time {
            Some(end_time) => EntryContext {
                id: entity.id,
                work_date: entity.work_date.to_string(),
                start_time: entity.start_time.to_string(),
                end_time: Some(end_time.to_string()),
                duration: Some(Duration::new(end_time - entity.start_time)),
            },
            None => EntryContext {
                id: entity.id,
                work_date: entity.work_date.to_string(),
                start_time: entity.start_time.to_string(),
                end_time: None,
                duration: None,
            },
        }
    }
}
