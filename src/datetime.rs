use crate::model::TimeEntryForTable;
use chrono::{DateTime, Datelike, ParseError, Timelike, Utc, Weekday};
use chrono_tz::Tz;
use ratatui::{style::Style, text::Span};
use rust_i18n::t;

/// Parse an RFC3339 string into a DateTime
fn parse_rfc3339(date_str: &str) -> Result<DateTime<chrono::FixedOffset>, ParseError> {
    DateTime::parse_from_rfc3339(date_str)
}

/// Get a Tz object from a timezone string, falling back to UTC if invalid
fn get_timezone(timezone: &str) -> Tz {
    timezone.parse::<Tz>().unwrap_or(chrono_tz::UTC)
}

/// Format a DateTime using the given format string and timezone
fn format_with_timezone(
    date: &DateTime<chrono::FixedOffset>,
    format_str: &str,
    timezone: &str,
) -> String {
    let tz = get_timezone(timezone);
    let local_date = date.with_timezone(&tz);
    local_date.format(format_str).to_string()
}

/// Parse an RFC3339 string and format it with the given format string and timezone
pub fn parse_and_format(date_str: &str, format_str: &str, timezone: &str) -> String {
    match parse_rfc3339(date_str) {
        Ok(date) => format_with_timezone(&date, format_str, timezone),
        Err(_) => t!("dt_invalid_date").to_string(),
    }
}

/// Format an RFC3339 date for human-readable display
pub fn format_date(date_str: &str, timezone: &str) -> String {
    parse_and_format(date_str, "%a %d %b %Y", timezone)
}

/// Format an RFC3339 date as YYYY-MM-DD
pub fn format_date_iso(date_str: &str, timezone: &str) -> String {
    parse_and_format(date_str, "%Y-%m-%d", timezone)
}

/// Format an RFC3339 time for human-readable display
pub fn format_time(date_str: &str, timezone: &str) -> String {
    parse_and_format(date_str, "%H:%M", timezone)
}

/// Format an RFC3339 datetime for human-readable display
pub fn format_datetime(date_str: &str, timezone: &str) -> String {
    parse_and_format(date_str, "%a %d %b %Y %H:%M", timezone)
}

/// Format a date range for display
pub fn format_date_range(start_date: &str, end_date: &str, timezone: &str) -> String {
    let start_formatted = format_date(start_date, timezone);
    let end_formatted = format_date(end_date, timezone);
    format!("{} to {}", start_formatted, end_formatted)
}

/// Convert a weekday string to a chrono Weekday
pub fn string_to_weekday(weekday_str: &str) -> Weekday {
    match weekday_str.to_lowercase().as_str() {
        "monday" => Weekday::Mon,
        "tuesday" => Weekday::Tue,
        "wednesday" => Weekday::Wed,
        "thursday" => Weekday::Thu,
        "friday" => Weekday::Fri,
        "saturday" => Weekday::Sat,
        "sunday" => Weekday::Sun,
        _ => Weekday::Mon, // Default to Monday
    }
}

/// Get the start of the week containing the given datetime
pub fn get_week_start(date: DateTime<Tz>, week_starts_on: Weekday) -> DateTime<Tz> {
    // Calculate days from the specified week start
    // Example: if today is Wednesday and week_starts_on is Monday, days_from_week_start = 2
    let current_weekday = date.weekday();
    let days_from_week_start = (7 + current_weekday.num_days_from_monday() as i64
        - week_starts_on.num_days_from_monday() as i64)
        % 7;

    // Start of day (midnight)
    let start_of_day = date
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();

    // Go back to the start of the week
    start_of_day - chrono::Duration::days(days_from_week_start)
}

/// Get the end of the week containing the given datetime
pub fn get_week_end(week_start: DateTime<Tz>) -> DateTime<Tz> {
    // End of the week is 6 days and 23:59:59 after the start
    week_start
        + chrono::Duration::days(6)
        + chrono::Duration::hours(23)
        + chrono::Duration::minutes(59)
        + chrono::Duration::seconds(59)
}

/// Calculate the week range (start and end) for the current week, adjusted by week_offset
pub fn calculate_week_range(
    week_offset: i32,
    timezone: &str,
    week_starts_on: &str,
) -> (DateTime<Tz>, DateTime<Tz>) {
    // Parse the week start day
    let week_start_day = string_to_weekday(week_starts_on);

    // Get the current date in the specified timezone
    let tz = get_timezone(timezone);
    let now = Utc::now().with_timezone(&tz);

    // Get the start of the current week
    let current_week_start = get_week_start(now, week_start_day);

    // Apply the week offset
    let target_week_start = if week_offset == 0 {
        current_week_start
    } else {
        current_week_start + chrono::Duration::weeks(week_offset as i64)
    };

    // Calculate the end of the target week
    let target_week_end = get_week_end(target_week_start);

    (target_week_start, target_week_end)
}

/// Format the week range for API calls (returns strings in RFC3339 format)
pub fn get_week_range_strings(
    week_offset: i32,
    timezone: &str,
    week_starts_on: &str,
) -> (String, String) {
    let (start, end) = calculate_week_range(week_offset, timezone, week_starts_on);
    (start.to_rfc3339(), end.to_rfc3339())
}

/// Get a human-readable description of the current week range
pub fn get_week_description(week_offset: i32, timezone: &str, week_starts_on: &str) -> String {
    let (start, end) = calculate_week_range(week_offset, timezone, week_starts_on);

    t!(
        "dt_week_description",
        week_desc = get_title_week_description(week_offset),
        start_date = start.format("%a %d %b %Y").to_string(),
        end_date = end.format("%a %d %b %Y").to_string()
    )
    .to_string()
}

/// Get the ISO week number for the given week offset
pub fn get_week_number(week_offset: i32, timezone: &str, week_starts_on: &str) -> (i32, i32) {
    let (start, _) = calculate_week_range(week_offset, timezone, week_starts_on);

    // Get the ISO week number and year
    let iso_week = start.iso_week();
    (iso_week.week() as i32, iso_week.year())
}

/// Get a relative week description for title display
pub fn get_title_week_description(week_offset: i32) -> String {
    if week_offset == 0 {
        t!("dt_this_week").to_string()
    } else if week_offset == -1 {
        t!("dt_last_week").to_string()
    } else if week_offset < -1 {
        let weeks_ago = format!(
            "{} {}",
            week_offset.abs(),
            t!("dt_weeks_ago").replace("%{count}", "").trim()
        );
        weeks_ago
    } else if week_offset == 1 {
        t!("dt_next_week").to_string()
    } else {
        let weeks_from_now = format!(
            "{} {}",
            week_offset,
            t!("dt_weeks_from_now").replace("%{count}", "").trim()
        );
        weeks_from_now
    }
}

/// Calculate the duration between two RFC3339 timestamps
pub fn calculate_duration(started_at: &str, ended_at: &str) -> (u64, u64) {
    let start = DateTime::parse_from_rfc3339(started_at).ok();
    let end = DateTime::parse_from_rfc3339(ended_at).ok();

    match (start, end) {
        (Some(start), Some(end)) => {
            let duration = end - start;
            let total_minutes = duration.num_minutes();
            
            // Handle invalid dates (end before start) or unrealistically large durations
            if total_minutes < 0 || total_minutes > 10000 * 60 { // Cap at 10,000 hours (over a year)
                return (0, 0);
            }
            
            let hours = total_minutes / 60;
            let minutes = total_minutes % 60;

            (hours as u64, minutes as u64)
        }
        _ => (0, 0), // Return zero duration if there's a parsing error
    }
}

/// Get a formatted duration string for display
pub fn format_duration(hours: u64, minutes: u64, style: Style) -> Vec<Span<'static>> {
    let hour_label = t!("dt_duration_hour");
    let minute_label = t!("dt_duration_minute");

    if hours >= 1 && minutes >= 1 {
        vec![
            Span::raw(format!("{}", hours)).style(style),
            Span::raw(format!("{} ", hour_label)),
            Span::raw(format!("{}", minutes)).style(style),
            Span::raw(minute_label.to_string()),
        ]
    } else if hours >= 1 {
        vec![
            Span::raw(format!("{}", hours)).style(style),
            Span::raw(hour_label.to_string()),
        ]
    } else if minutes >= 1 {
        vec![
            Span::raw(format!("{}", minutes)).style(style),
            Span::raw(minute_label.to_string()),
        ]
    } else {
        vec![
            Span::raw("0").style(style),
            Span::raw(minute_label.to_string()),
        ]
    }
}

/// Parse an RFC3339 string into a DateTime in the specified timezone
pub fn parse_iso_datetime(date_str: &str, timezone: &str) -> Option<DateTime<Tz>> {
    match parse_rfc3339(date_str) {
        Ok(date) => {
            let tz = get_timezone(timezone);
            Some(date.with_timezone(&tz))
        }
        Err(_) => None,
    }
}

/// Format a date from a time entry for display
pub(crate) fn format_date_from_time_entry(time_entry: TimeEntryForTable, timezone: &str) -> String {
    format_date(&time_entry.started_at, timezone)
}

/// Format a time range from a time entry for display
pub(crate) fn format_time_range_from_time_entry(
    time_entry: TimeEntryForTable,
    timezone: &str,
) -> String {
    let start = &time_entry.started_at;
    let end = &time_entry.ended_at;

    format!(
        "{}-{}",
        format_time(start, timezone),
        format_time(end, timezone)
    )
}

/// Parse a datetime string into separate date and time components for the edit form
pub fn parse_datetime_for_edit(datetime_str: &str, timezone: &str) -> (Option<String>, Option<String>) {
    // Parse the datetime string
    match parse_iso_datetime(datetime_str, timezone) {
        Some(dt) => {
            // Format the date as YYYY-MM-DD
            let date = dt.format("%Y-%m-%d").to_string();
            // Format the time as HH:MM
            let time = dt.format("%H:%M").to_string();
            (Some(date), Some(time))
        }
        None => (None, None)
    }
}

/// Format date and time components into an ISO8601 datetime string
pub fn format_datetime_from_edit(date: &str, time: &str) -> String {
    format!("{}T{}:00Z", date, time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_string_to_weekday() {
        assert_eq!(string_to_weekday("monday"), Weekday::Mon);
        assert_eq!(string_to_weekday("Monday"), Weekday::Mon);
        assert_eq!(string_to_weekday("MONDAY"), Weekday::Mon);
        assert_eq!(string_to_weekday("tuesday"), Weekday::Tue);
        assert_eq!(string_to_weekday("wednesday"), Weekday::Wed);
        assert_eq!(string_to_weekday("thursday"), Weekday::Thu);
        assert_eq!(string_to_weekday("friday"), Weekday::Fri);
        assert_eq!(string_to_weekday("saturday"), Weekday::Sat);
        assert_eq!(string_to_weekday("sunday"), Weekday::Sun);

        // Default case should return Monday
        assert_eq!(string_to_weekday("invalid"), Weekday::Mon);
        assert_eq!(string_to_weekday(""), Weekday::Mon);
    }

    #[test]
    fn test_get_week_start() {
        // Test with Monday as the start of the week
        let tz = chrono_tz::Europe::Amsterdam;

        // Create a Wednesday (2023-08-09) date
        let wednesday = tz.with_ymd_and_hms(2023, 8, 9, 12, 0, 0).unwrap();

        // Get the start of the week (should be Monday 2023-08-07)
        let week_start = get_week_start(wednesday, Weekday::Mon);

        assert_eq!(week_start.year(), 2023);
        assert_eq!(week_start.month(), 8);
        assert_eq!(week_start.day(), 7);
        assert_eq!(week_start.hour(), 0);
        assert_eq!(week_start.minute(), 0);
        assert_eq!(week_start.second(), 0);
        assert_eq!(week_start.weekday(), Weekday::Mon);

        // Test with Sunday as the start of the week
        let week_start_sunday = get_week_start(wednesday, Weekday::Sun);

        assert_eq!(week_start_sunday.year(), 2023);
        assert_eq!(week_start_sunday.month(), 8);
        assert_eq!(week_start_sunday.day(), 6);
        assert_eq!(week_start_sunday.hour(), 0);
        assert_eq!(week_start_sunday.minute(), 0);
        assert_eq!(week_start_sunday.second(), 0);
        assert_eq!(week_start_sunday.weekday(), Weekday::Sun);
    }

    #[test]
    fn test_get_week_end() {
        let tz = chrono_tz::Europe::Amsterdam;

        // Create a Monday (2023-08-07) date at midnight
        let monday = tz.with_ymd_and_hms(2023, 8, 7, 0, 0, 0).unwrap();

        // Get the end of the week (should be Sunday 2023-08-13 at 23:59:59)
        let week_end = get_week_end(monday);

        assert_eq!(week_end.year(), 2023);
        assert_eq!(week_end.month(), 8);
        assert_eq!(week_end.day(), 13);
        assert_eq!(week_end.hour(), 23);
        assert_eq!(week_end.minute(), 59);
        assert_eq!(week_end.second(), 59);
        assert_eq!(week_end.weekday(), Weekday::Sun);
    }

    #[test]
    fn test_calculate_week_range() {
        // Test with current date set to 2023-08-09 (a Wednesday)
        // and Monday as the start of the week

        // We'll mock the current time by using a specific week offset
        // Let's test with week_offset = 0 (current week)
        let (start, end) = calculate_week_range(0, "Europe/Amsterdam", "monday");

        // The current week should be correct (we can only assert it's a Monday and Sunday)
        assert_eq!(start.weekday(), Weekday::Mon);
        assert_eq!(end.weekday(), Weekday::Sun);
        assert_eq!(end.hour(), 23);
        assert_eq!(end.minute(), 59);
        assert_eq!(end.second(), 59);

        // Test with a different week_offset
        let (prev_start, prev_end) = calculate_week_range(-1, "Europe/Amsterdam", "monday");

        // The previous week should be 7 days before the current week
        assert_eq!(prev_start.weekday(), Weekday::Mon);
        assert_eq!(prev_end.weekday(), Weekday::Sun);
        assert_eq!((start - prev_start).num_days(), 7);
        assert_eq!((end - prev_end).num_days(), 7);

        // Test with a different week_starts_on
        let (sunday_start, _) = calculate_week_range(0, "Europe/Amsterdam", "sunday");
        assert_eq!(sunday_start.weekday(), Weekday::Sun);
    }

    #[test]
    fn test_get_title_week_description() {
        rust_i18n::set_locale("en"); // Ensure tests run with English locale
                                     // Test different week offsets
        assert_eq!(get_title_week_description(0), "this week");
        assert_eq!(get_title_week_description(-1), "last week");

        // For variable week numbers, verify the format is correct
        let result = get_title_week_description(-2);
        assert!(
            result.starts_with("2"),
            "Expected result '{}' to start with '2'",
            result
        );
        assert!(
            result.contains("weeks ago"),
            "Expected result '{}' to contain 'weeks ago'",
            result
        );

        let result = get_title_week_description(-5);
        assert!(
            result.starts_with("5"),
            "Expected result '{}' to start with '5'",
            result
        );
        assert!(
            result.contains("weeks ago"),
            "Expected result '{}' to contain 'weeks ago'",
            result
        );

        assert_eq!(get_title_week_description(1), "next week");

        let result = get_title_week_description(3);
        assert!(
            result.starts_with("3"),
            "Expected result '{}' to start with '3'",
            result
        );
        assert!(
            result.contains("weeks from now"),
            "Expected result '{}' to contain 'weeks from now'",
            result
        );
    }

    #[test]
    fn test_calculate_duration() {
        // 1 hour difference
        let duration = calculate_duration("2023-08-09T10:00:00+02:00", "2023-08-09T11:00:00+02:00");
        assert_eq!(duration, (1, 0), "Should be 1 hour, 0 minutes");

        // 1 hour and 30 minutes
        let duration = calculate_duration("2023-08-09T10:00:00+02:00", "2023-08-09T11:30:00+02:00");
        assert_eq!(duration, (1, 30), "Should be 1 hour, 30 minutes");

        // Different day
        let duration = calculate_duration("2023-08-09T23:30:00+02:00", "2023-08-10T00:30:00+02:00");
        assert_eq!(duration, (1, 0), "Should be 1 hour, 0 minutes");

        // Different timezone
        let duration = calculate_duration("2023-08-09T10:00:00+02:00", "2023-08-09T11:00:00+03:00");
        assert_eq!(
            duration,
            (0, 0),
            "Should be 0 hours, 0 minutes - same time in different zones"
        );

        // Invalid dates
        let duration = calculate_duration("invalid", "2023-08-09T11:00:00+02:00");
        assert_eq!(
            duration,
            (0, 0),
            "Should return zero for invalid start date"
        );

        let duration = calculate_duration("2023-08-09T11:00:00+02:00", "invalid");
        assert_eq!(duration, (0, 0), "Should return zero for invalid end date");
    }

    #[test]
    fn test_format_duration() {
        rust_i18n::set_locale("en"); // Ensure tests run with English locale
        let style = Style::default();

        // Test hour and minutes
        let formatted = format_duration(2, 30, style);
        assert_eq!(formatted.len(), 4);
        assert_eq!(formatted[0].content, "2");
        assert_eq!(formatted[1].content, "h ");
        assert_eq!(formatted[2].content, "30");
        assert_eq!(formatted[3].content, "m");

        // Test hours only
        let formatted = format_duration(3, 0, style);
        assert_eq!(formatted.len(), 2);
        assert_eq!(formatted[0].content, "3");
        assert_eq!(formatted[1].content, "h");

        // Test minutes only
        let formatted = format_duration(0, 45, style);
        assert_eq!(formatted.len(), 2);
        assert_eq!(formatted[0].content, "45");
        assert_eq!(formatted[1].content, "m");

        // Test zero duration
        let formatted = format_duration(0, 0, style);
        assert_eq!(formatted.len(), 2);
        assert_eq!(formatted[0].content, "0");
        assert_eq!(formatted[1].content, "m");
    }

    #[test]
    fn test_parse_and_format() {
        rust_i18n::set_locale("en"); // Ensure tests run with English locale
                                     // Test valid RFC3339 date
        let formatted = parse_and_format(
            "2023-08-09T10:30:00+02:00",
            "%Y-%m-%d %H:%M",
            "Europe/Amsterdam",
        );
        assert_eq!(formatted, "2023-08-09 10:30");

        // Test invalid date
        let formatted = parse_and_format("invalid", "%Y-%m-%d", "Europe/Amsterdam");
        assert_eq!(formatted, "Invalid date");
    }

    #[test]
    fn test_parse_iso_datetime() {
        // Test valid date
        let parsed = parse_iso_datetime("2023-08-09T10:30:00+02:00", "Europe/Amsterdam");
        assert!(parsed.is_some());

        if let Some(dt) = parsed {
            assert_eq!(dt.year(), 2023);
            assert_eq!(dt.month(), 8);
            assert_eq!(dt.day(), 9);
            assert_eq!(dt.hour(), 10);
            assert_eq!(dt.minute(), 30);
        }

        // Test invalid date
        let parsed = parse_iso_datetime("invalid", "Europe/Amsterdam");
        assert!(parsed.is_none());
    }

    #[test]
    fn test_format_functions() {
        let date_str = "2023-08-09T10:30:00+02:00";
        let timezone = "Europe/Amsterdam";

        // Test format_date
        assert_eq!(format_date(date_str, timezone), "Wed 09 Aug 2023");

        // Test format_date_iso
        assert_eq!(format_date_iso(date_str, timezone), "2023-08-09");

        // Test format_time
        assert_eq!(format_time(date_str, timezone), "10:30");

        // Test format_datetime
        assert_eq!(format_datetime(date_str, timezone), "Wed 09 Aug 2023 10:30");
    }

    #[test]
    fn test_get_week_number() {
        // Set a known fixed date for testing ISO week numbers
        // Jan 1, 2023 was in ISO week 52 of 2022
        let (week, year) = get_week_number(0, "Europe/Amsterdam", "monday");

        // We can't assert the exact week number since it depends on the current date
        // but we can verify that it's within a reasonable range
        assert!(
            (1..=53).contains(&week),
            "Week number should be between 1-53"
        );

        // Test the previous week
        let (prev_week, prev_year) = get_week_number(-1, "Europe/Amsterdam", "monday");

        // If we're in week 1, the previous week might be in the previous year
        if week == 1 {
            assert!(
                prev_week == 52 || prev_week == 53,
                "Previous week should be 52 or 53"
            );
            assert_eq!(prev_year, year - 1, "Previous year should be one less");
        } else {
            assert_eq!(prev_week, week - 1, "Previous week should be one less");
            assert_eq!(prev_year, year, "Years should match");
        }
    }
}
