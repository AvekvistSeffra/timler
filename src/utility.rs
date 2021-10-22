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
            hours,
            minutes,
            seconds,
        }
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", -self.hours, -self.minutes, -self.seconds)
    }
}
