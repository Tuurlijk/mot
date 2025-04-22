use crate::ui::Shortcut;
use crate::{datetime, ui, ui::Shortcuts, AppModel, TimeEntryForTable};
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use ratatui::{symbols, Frame};
use rust_i18n::t;

/// Get the display icon for a time entry
fn get_time_entry_icon(time_entry: &TimeEntryForTable) -> String {
    if let Some(custom_icon) = &time_entry.icon {
        // Use custom icon from plugin manifest if available
        custom_icon.clone()
    } else if time_entry.source.to_lowercase() == "moneybird" {
        // Use blue circle for Moneybird 
        "🔵".to_string()
    } else if let Some(plugin_name) = &time_entry.plugin_name {
        // Use the plugin name for consistent icons
        ui::get_default_icon(plugin_name)
    } else {
        // Use a default icon for unmatched entries
        "❓".to_string()
    }
}

pub fn render_time_entry_detail(model: &AppModel, area: Rect, frame: &mut Frame) {
    let shortcuts = Shortcuts::new(vec![
        Shortcut::Trio("◀", t!("ui_shortcut_week").as_ref(), "▶"),
        Shortcut::Pair("t", t!("ui_shortcut_this_week").as_ref()),
        Shortcut::Pair("f", t!("ui_shortcut_filter").as_ref()),
        Shortcut::Pair("c", t!("ui_shortcut_create").as_ref()),
        Shortcut::Pair("e", t!("ui_shortcut_edit").as_ref()),
        Shortcut::Pair("i", t!("ui_shortcut_import").as_ref()),
        Shortcut::Pair("d", t!("ui_shortcut_delete").as_ref()),
        Shortcut::Pair("p", t!("ui_shortcut_plugins").as_ref()),
        Shortcut::Pair("x", t!("ui_shortcut_export").as_ref()),
        Shortcut::Pair("q", t!("ui_shortcut_quit").as_ref()),
    ])
    .with_alignment(Alignment::Right)
    .with_label_style(model.appearance.default_style.add_modifier(Modifier::BOLD));

    let collapsed_top_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        bottom_right: symbols::line::ROUNDED_BOTTOM_RIGHT,
        bottom_left: symbols::line::ROUNDED_BOTTOM_LEFT,
        ..symbols::border::PLAIN
    };

    let detail_block = model
        .appearance
        .default_block
        .clone()
        .border_set(collapsed_top_border_set)
        .title_alignment(Alignment::Left)
        .title_bottom(shortcuts.as_line());

    // Check if we have any items to display and a valid selection
    if model.time_entries_for_table.is_empty() || model.time_entry_table_state.selected().is_none()
    {
        // Render an empty detail view with a message
        let message = if model.time_entries_for_table.is_empty() {
            let week_desc = datetime::get_week_description(
                model.week_offset,
                &model
                    .administration
                    .time_zone
                    .clone()
                    .unwrap_or_else(|| "UTC".to_string()),
                &model.config.week_starts_on,
            );
            t!("ui_detail_no_entries", week_description = week_desc).to_string()
        } else {
            t!("ui_detail_no_selection").to_string()
        };

        let detail = Paragraph::new(message).block(detail_block.clone());
        frame.render_widget(detail, area);
        return;
    }

    let selected_idx = model.time_entry_table_state.selected().unwrap_or(0);

    // Ensure the selected index is valid
    if selected_idx >= model.time_entries_for_table.len() {
        // Render an empty detail view with error message
        let detail = Paragraph::new(t!("ui_detail_invalid_selection")).block(detail_block.clone());
        frame.render_widget(detail, area);
        return; // Return early
    }

    let selected_item = &model.time_entries_for_table[selected_idx];

    // Get the icon to display
    let icon_display = get_time_entry_icon(selected_item);

    let client_project = vec![
        selected_item.customer.clone().green(),
        Span::from(" - "),
        selected_item.project.clone().green(),
    ];

    let description = selected_item.description.clone();

    // Split the description by newlines and create a Line for each part
    let description_lines: Vec<Line> = description
        .split('\n')
        .map(|line| Line::from(line.to_string()))
        .collect();

    let admin_timezone_str = model
        .administration
        .time_zone
        .clone()
        .unwrap_or_else(|| "UTC".to_string());

    let (hours, minutes) =
        datetime::calculate_duration(&selected_item.started_at, &selected_item.ended_at);

    let total_time_style = Style::default().bold().yellow();
    let total_time = datetime::format_duration(hours, minutes, total_time_style);

    let mut times = vec![
        Span::from(crate::datetime::format_date_from_time_entry(
            selected_item.clone(),
            &admin_timezone_str,
        ))
        .blue(),
        Span::from("  "),
    ];
    times.extend(total_time);
    times.push(Span::from("  "));
    times.push(Span::from(
        crate::datetime::format_time_range_from_time_entry(
            selected_item.clone(),
            &admin_timezone_str,
        ),
    ));

    let mut title_spans = vec![
        Span::from(" "),
        Span::from(icon_display),
        Span::from(" "),
    ];
    title_spans.extend(client_project.clone());
    
    // Add plugin information if not from Moneybird
    if selected_item.source.to_lowercase() != "moneybird" {
        if let Some(plugin_name) = &selected_item.plugin_name {
            title_spans.push(Span::from(" ("));
            title_spans.push(Span::from(plugin_name).italic());
            title_spans.push(Span::from(")"));
        }
    }
    
    title_spans.push(Span::from(" "));

    let mut detail_lines: Vec<Line> = vec![Line::from(times), Line::from("")];
    detail_lines.extend(description_lines);

    let detail = Paragraph::new(Text::from(detail_lines))
        .block(detail_block.title(Line::from(title_spans)))
        .wrap(Wrap { trim: true });
    frame.render_widget(detail, area);
}
