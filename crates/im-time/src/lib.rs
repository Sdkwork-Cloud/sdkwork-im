use std::time::{SystemTime, UNIX_EPOCH};

pub fn utc_now_rfc3339_millis() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format_unix_timestamp_millis(duration.as_millis())
}

pub fn format_unix_timestamp_millis(epoch_millis: u128) -> String {
    let seconds = (epoch_millis / 1000) as i64;
    let millis = (epoch_millis % 1000) as u32;
    format_unix_timestamp_parts(seconds, millis)
}

fn format_unix_timestamp_parts(epoch_seconds: i64, millis: u32) -> String {
    let days = epoch_seconds.div_euclid(86_400);
    let seconds_of_day = epoch_seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;

    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z")
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_unix_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_param = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_param + 2) / 5 + 1;
    let month = month_param + if month_param < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };

    (year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::format_unix_timestamp_millis;

    #[test]
    fn test_formats_epoch_with_millisecond_precision() {
        assert_eq!(format_unix_timestamp_millis(0), "1970-01-01T00:00:00.000Z");
        assert_eq!(
            format_unix_timestamp_millis(1_744_635_600_123),
            "2025-04-14T13:00:00.123Z"
        );
    }

    #[test]
    fn test_lexicographic_order_matches_time_order() {
        let earlier = format_unix_timestamp_millis(1_744_635_600_123);
        let later = format_unix_timestamp_millis(1_744_635_605_456);

        assert!(earlier < later);
    }
}
