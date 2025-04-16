use crate::{datetime, AppModel};
use rust_i18n::t;
use std::fs::File;
use std::io::Write;

/// Export time entries based on command-line arguments
pub async fn handle_export_command(
    model: &mut AppModel,
    week_arg: String,
    query_arg: String,
) -> color_eyre::Result<()> {
    println!("{}", t!("file_exporting_time_entries", week = week_arg));

    // Get administration timezone, default to UTC if not set
    let admin_timezone_str = model
        .administration
        .time_zone
        .clone()
        .unwrap_or_else(|| "UTC".to_string());

    // Get current week number to calculate offset
    let (current_week_num, _) = datetime::get_week_number(
        model.week_offset,
        &admin_timezone_str,
        &model.config.week_starts_on,
    );

    let target_week: i32;
    // Calculate week offset based on requested week number
    if week_arg == "current week" {
        target_week = current_week_num;
        model.week_offset = 0;
    } else {
        target_week = week_arg.parse::<i32>().unwrap_or(current_week_num);
        model.week_offset = target_week - current_week_num;
    }

    // Fetch time entries for the specified week
    println!("{}", t!("file_fetching_time_entries", week = target_week));
    crate::api::get_time_entries(model).await;

    // Generate the filename with week number and date range
    let filename = generate_export_filename(model, Some(target_week));
    println!("{}", t!("file_exporting_to_file", filename = filename));

    // Apply filter if query is provided
    let original_entries = model.time_entries_for_table.clone();
    let mut filtered = false;

    if !query_arg.is_empty() {
        println!("{}", t!("file_filtering_with_query", query = query_arg));

        // Set up the search state with the query
        model.search_state.active = true;
        model.search_state.text_input.insert_str(&query_arg);
        model.filter_items();
        filtered = true;
    }

    // Export the data
    let result = export_time_entries_to_csv(model, &filename);

    // Restore original entries if we applied filtering
    if filtered {
        model.time_entries_for_table = original_entries;
        model.search_state.active = false;
    }

    // Handle result
    match result {
        Ok(()) => {
            println!(
                "{}",
                t!(
                    "file_export_success",
                    count = model.time_entries_for_table.len(),
                    filename = filename
                )
            );
            Ok(())
        }
        Err(err) => {
            eprintln!("{}", t!("file_export_error", error = err));
            Err(color_eyre::eyre::eyre!(t!(
                "file_export_failed",
                error = err
            )))
        }
    }
}

/// Export time entries to a CSV file
pub fn export_time_entries_to_csv(
    model: &AppModel,
    filename: &str,
) -> color_eyre::Result<(), String> {
    // Create a new file
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(e) => return Err(t!("file_create_error", error = e).to_string()),
    };

    // Write CSV header
    match file.write_all(b"Date,Start Time,End Time,Duration,Client,Project,Description\n") {
        Ok(_) => (),
        Err(e) => return Err(t!("file_write_header_error", error = e).to_string()),
    }

    // Write CSV data for each time entry
    for entry in &model.time_entries_for_table {
        // Get administration timezone, default to UTC if not set
        let admin_timezone_str = model
            .administration
            .time_zone
            .clone()
            .unwrap_or_else(|| "UTC".to_string());

        // Format the date in ISO format (YYYY-MM-DD)
        let date = datetime::format_date_iso(&entry.started_at, &admin_timezone_str);

        // Format the start and end times
        let start_time = datetime::format_time(&entry.started_at, &admin_timezone_str);
        let end_time = datetime::format_time(&entry.ended_at, &admin_timezone_str);

        // Calculate duration
        let (hours, minutes) = datetime::calculate_duration(&entry.started_at, &entry.ended_at);
        let duration = format!("{:02}:{:02}", hours, minutes);

        // Format CSV line (with proper escaping for any fields that might contain commas)
        let line = format!(
            "{},{},{},{},\"{}\",\"{}\",\"{}\"\n",
            date,
            start_time,
            end_time,
            duration,
            entry.customer.replace("\"", "\"\""), // Escape quotes in CSV
            entry.project.replace("\"", "\"\""),
            entry.description.replace("\"", "\"\"").replace("\n", " ") // Replace newlines
        );

        // Write the line to the file
        if let Err(e) = file.write_all(line.as_bytes()) {
            return Err(t!("file_write_data_error", error = e).to_string());
        }
    }

    Ok(())
}

/// Generate a filename with week number and date range
pub fn generate_export_filename(model: &AppModel, override_week_num: Option<i32>) -> String {
    // Get administration timezone, default to UTC if not set
    let admin_timezone_str = model
        .administration
        .time_zone
        .clone()
        .unwrap_or_else(|| "UTC".to_string());

    // Get week number and year - use override if provided, otherwise calculate from offset
    let (week_num, year) = if let Some(week) = override_week_num {
        let (_, year) = datetime::get_week_number(
            model.week_offset,
            &admin_timezone_str,
            &model.config.week_starts_on,
        );
        (week, year)
    } else {
        datetime::get_week_number(
            model.week_offset,
            &admin_timezone_str,
            &model.config.week_starts_on,
        )
    };

    // Get the week date range for the filename
    let (start, end) = datetime::calculate_week_range(
        model.week_offset,
        &admin_timezone_str,
        &model.config.week_starts_on,
    );

    // Format dates in a file-friendly format (YYYYMMDD)
    let start_date = start.format("%Y%m%d");
    let end_date = end.format("%Y%m%d");

    // Assemble the filename
    format!(
        "work_week_{}_{}_{}_{}.csv",
        week_num, year, start_date, end_date
    )
}
