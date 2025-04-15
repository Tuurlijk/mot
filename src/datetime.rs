use crate::model::TimeEntryForTable;
use chrono::{DateTime, Datelike, ParseError, Timelike, Utc, Weekday};
use chrono_tz::Tz;
use ratatui::{style::Style, text::Span};

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
        Err(_) => "Invalid date".to_string(),
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

    format!(
        "{}: {} to {}",
        get_title_week_description(week_offset),
        start.format("%a %d %b %Y"),
        end.format("%a %d %b %Y")
    )
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
        "this week".to_string()
    } else if week_offset == -1 {
        "last week".to_string()
    } else if week_offset < -1 {
        format!("{} weeks ago", week_offset.abs())
    } else if week_offset == 1 {
        "next week".to_string()
    } else {
        format!("{} weeks from now", week_offset)
    }
}

/// Calculate the duration between two RFC3339 timestamps
pub fn calculate_duration(started_at: &str, ended_at: &str) -> (u32, u32) {
    let start = DateTime::parse_from_rfc3339(started_at).ok();
    let end = DateTime::parse_from_rfc3339(ended_at).ok();

    match (start, end) {
        (Some(start), Some(end)) => {
            let duration = end - start;
            let total_minutes = duration.num_minutes();
            let hours = total_minutes / 60;
            let minutes = total_minutes % 60;

            (hours as u32, minutes as u32)
        }
        _ => (0, 0), // Return zero duration if there's a parsing error
    }
}

/// Get a formatted duration string for display
pub fn format_duration(hours: u32, minutes: u32, style: Style) -> Vec<Span<'static>> {
    if hours >= 1 && minutes >= 1 {
        vec![
            Span::raw(format!("{}", hours)).style(style),
            Span::raw("h "),
            Span::raw(format!("{}", minutes)).style(style),
            Span::raw("m"),
        ]
    } else if hours >= 1 {
        vec![Span::raw(format!("{}", hours)).style(style), Span::raw("h")]
    } else if minutes >= 1 {
        vec![
            Span::raw(format!("{}", minutes)).style(style),
            Span::raw("m"),
        ]
    } else {
        vec![Span::raw("0").style(style), Span::raw("m")]
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
