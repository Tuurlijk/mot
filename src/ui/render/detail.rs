use crate::ui::Shortcut;
use crate::{datetime, ui::Shortcuts, AppModel};
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use ratatui::{symbols, Frame};

pub fn render_time_entry_detail(model: &AppModel, area: Rect, frame: &mut Frame) {
    let shortcuts = Shortcuts::new(vec![
        Shortcut::Trio("◀", "week", "▶"),
        Shortcut::Pair("t", "this week"),
        Shortcut::Pair("f", "filter"),
        Shortcut::Pair("c", "create"),
        Shortcut::Pair("e", "edit"),
        Shortcut::Pair("d", "delete"),
        Shortcut::Pair("x", "export"),
        Shortcut::Pair("q", "quit"),
    ])
    .with_alignment(Alignment::Right)
    .with_key_style(
        model
            .appearance
            .default_style
            .green()
            .add_modifier(Modifier::BOLD),
    )
    .with_label_style(model.appearance.default_style);

    let collapsed_top_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        bottom_right: symbols::line::ROUNDED_BOTTOM_RIGHT,
        bottom_left: symbols::line::ROUNDED_BOTTOM_LEFT,
        ..symbols::border::PLAIN
    };

    let detail_block = Block::default()
        .borders(Borders::ALL)
        .border_set(collapsed_top_border_set)
        .title_alignment(Alignment::Left)
        .title_bottom(shortcuts.as_line())
        .padding(Padding::new(1, 1, 0, 0))
        .style(model.appearance.default_style);

    // Check if we have any items to display and a valid selection
    if model.time_entries_for_table.is_empty() || model.time_entry_table_state.selected().is_none()
    {
        // Render an empty detail view with a message
        let message = if model.time_entries_for_table.is_empty() {
            format!(
                "No time entries found for this week.\n\n{}",
                datetime::get_week_description(
                    model.week_offset,
                    &model
                        .administration
                        .time_zone
                        .clone()
                        .unwrap_or_else(|| "UTC".to_string()),
                    &model.config.week_starts_on
                )
            )
        } else {
            "No item selected.".to_string()
        };

        let detail = Paragraph::new(message).block(detail_block.clone());
        frame.render_widget(detail, area);
        return;
    }

    let selected_idx = model.time_entry_table_state.selected().unwrap_or(0);

    // Ensure the selected index is valid
    if selected_idx >= model.time_entries_for_table.len() {
        // Render an empty detail view with error message
        let detail = Paragraph::new("Invalid selection index.").block(detail_block.clone());
        frame.render_widget(detail, area);
        return; // Return early
    }

    let selected_item = &model.time_entries_for_table[selected_idx];

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

    let mut title_spans = vec![Span::from(" ")];
    title_spans.extend(client_project.clone());
    title_spans.push(Span::from(" "));

    let mut detail_lines: Vec<Line> = vec![Line::from(times), Line::from("")];
    detail_lines.extend(description_lines);

    let detail = Paragraph::new(Text::from(detail_lines))
        .block(detail_block.title(Line::from(title_spans)))
        .wrap(Wrap { trim: true });
    frame.render_widget(detail, area);
}
