use crate::ui;
use crate::{datetime, AppModel, TimeEntryForTable};
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Row, Table};
use ratatui::Frame;
use rust_i18n::t;

/// Get the display icon for a time entry
fn get_time_entry_icon(time_entry: &TimeEntryForTable) -> String {
    if let Some(custom_icon) = &time_entry.icon {
        // Use custom icon from plugin manifest if available
        custom_icon.clone()
    } else if time_entry.source.to_lowercase() == "moneybird" {
        // Use blue circle for Moneybird
        "🐦".to_string()
    } else if let Some(plugin_name) = &time_entry.plugin_name {
        // Use the plugin name for consistent icons
        ui::get_default_icon(plugin_name)
    } else {
        // Use a default icon for unmatched entries
        "❓".to_string()
    }
}

pub fn render_time_entries_table(model: &mut AppModel, area: Rect, frame: &mut Frame) {
    // Store the table area for mouse click handling
    model.table_area = Some(area);

    let header_cols = vec![
        "".to_string(), // Empty header for the icon column
        t!("ui_table_header_date").to_string(),
        t!("ui_table_header_time").to_string(),
        t!("ui_table_header_client").to_string(),
        t!("ui_table_header_project").to_string(),
        t!("ui_table_header_description").to_string(),
    ];
    let header = Row::new(header_cols)
        .style(model.appearance.default_style.add_modifier(Modifier::BOLD))
        .height(1);

    // Get administration timezone, default to UTC if not set
    let admin_timezone_str = model
        .administration
        .time_zone
        .clone()
        .unwrap_or_else(|| "UTC".to_string());

    let (week_num, year) = datetime::get_week_number(
        model.week_offset,
        &admin_timezone_str,
        &model.config.week_starts_on,
    );

    // Calculate the colmun widths
    let client_width = model
        .time_entries_for_table
        .iter()
        .map(|time_entry| time_entry.customer.len())
        .max()
        .unwrap_or(0);
    let project_width = model
        .time_entries_for_table
        .iter()
        .map(|time_entry| time_entry.project.len())
        .max()
        .unwrap_or(0);

    // Get relative week description
    let week_relative = datetime::get_title_week_description(model.week_offset);

    // Calculate total time - keep everything in u64 to prevent overflow
    let total_minutes = model
        .time_entries_for_table
        .iter()
        .filter(|time_entry| time_entry.source.to_lowercase() == "moneybird") // Filter for Moneybird entries
        .fold(0_u64, |acc, time_entry| {
            let (hours, minutes) =
                datetime::calculate_duration(&time_entry.started_at, &time_entry.ended_at);
            acc + (hours * 60 + minutes)
        });

    // Convert to hours and minutes for display
    let total_hours = total_minutes / 60;
    let total_minutes = total_minutes % 60;

    let total_time_style = Style::default().bold().yellow();
    let total_time_str = datetime::format_duration(total_hours, total_minutes, total_time_style);

    let title_week_prefix = t!("ui_table_title_week");
    let title_separator = t!("ui_table_title_separator");
    let mut title_spans = vec![
        Span::from(title_week_prefix.to_string()),
        week_num.to_string().bold().green(),
        Span::from(" "),
        Span::from(year.to_string()),
        Span::from(title_separator.to_string()),
        Span::from(week_relative),
        Span::from(title_separator.to_string()),
    ];

    title_spans.extend(total_time_str);
    title_spans.push(Span::from(" "));

    // Get the currently selected index for calculating distance
    let selected_idx = model.time_entry_table_state.selected().unwrap_or(0);

    // Create table rows from time entries
    let rows: Vec<Row> = model
        .time_entries_for_table
        .iter()
        .enumerate()
        .map(|(idx, time_entry)| {
            // Calculate distance from selected row to apply gradient
            let distance = if idx > selected_idx {
                idx - selected_idx
            } else {
                selected_idx - idx
            };

            // Apply gradient styling based on distance and color support
            let row_style = crate::ui::gradient_color(
                // Use crate::ui::gradient_color
                distance,
                idx == selected_idx,
                model.appearance.color_support,
                model.appearance.default_foreground_color,
                model.appearance.color_mode,
            );

            // Get a properly formatted set of time entries
            // Use crate::datetime::format_date_from_time_entry and crate::datetime::format_time_range_from_time_entry
            let date = crate::datetime::format_date_from_time_entry(
                time_entry.clone(),
                &admin_timezone_str,
            );
            let time = crate::datetime::format_time_range_from_time_entry(
                time_entry.clone(),
                &admin_timezone_str,
            );

            // Get the icon for this time entry
            let icon = get_time_entry_icon(time_entry);

            Row::new(vec![
                icon,
                date,
                time,
                time_entry.customer.clone(),
                time_entry.project.clone(),
                time_entry.description.clone().replace("\n", " "),
            ])
            .style(row_style)
        })
        .collect();

    let widths = [
        Constraint::Length(2),                    // Icon column (small fixed width)
        Constraint::Length(10),                   // Date (YYYY-MM-DD)
        Constraint::Length(11),                   // Time range (HH:MM-HH:MM)
        Constraint::Length(client_width as u16),  // Client name
        Constraint::Length(project_width as u16), // Project name
        Constraint::Fill(1),                      // Description (fills remaining space)
    ];

    // If table is empty, render empty state
    if model.time_entries_for_table.is_empty() {
        let empty_message = t!("ui_table_empty_state");
        let empty_state = Paragraph::new(empty_message)
            .alignment(Alignment::Center)
            .block(
                model
                    .appearance
                    .default_block
                    .clone()
                    .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                    .title(Line::from(title_spans))
                    .title_alignment(Alignment::Center)
                    .padding(Padding::new(1, 1, 0, 0)),
            );

        frame.render_widget(empty_state, area);
    } else {
        // Create the table widget
        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(
                Style::default()
                    .add_modifier(Modifier::REVERSED | Modifier::ITALIC | Modifier::BOLD),
            )
            .block(
                model
                    .appearance
                    .default_block
                    .clone()
                    .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                    .padding(Padding::new(1, 1, 0, 0))
                    .title(Line::from(title_spans.clone())) // Clone needed here
                    .title_alignment(Alignment::Center),
            );

        frame.render_stateful_widget(table, area, &mut model.time_entry_table_state);
    }
}
