use crate::{model::LogSeverity, AppModel};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap};
use ratatui::Frame;

/// Render the log panel with latest entries at the top
pub fn render_log_panel(model: &AppModel, area: Rect, frame: &mut Frame) {
    // Create a block for the log panel
    let log_block = Block::default()
        .title(" Log ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(1, 1, 0, 0))
        .style(model.appearance.default_style);

    // Create a text with all log entries
    let text: Vec<Line> = model
        .log_entries
        .iter()
        .rev() // Reverse to show newest at the top
        .map(|entry| {
            // Format the timestamp
            let timestamp = entry.timestamp.format("%H:%M:%S%.3f").to_string();

            // Choose color and icon based on severity
            let (severity_color, severity_icon) = match entry.severity {
                LogSeverity::Debug => (Color::DarkGray, "🐞"),
                LogSeverity::Notice => (Color::Blue, "💡"),
                LogSeverity::Success => (Color::Green, "✅"),
                LogSeverity::Warning => (Color::Yellow, "⚠️"),
                LogSeverity::Error => (Color::Red, "⛔"),
            };

            // Create a styled line for each log entry
            Line::from(vec![
                Span::styled(
                    format!("{} ", timestamp),
                    Style::default().fg(severity_color),
                ),
                Span::styled(
                    format!("{} ", severity_icon),
                    Style::default().fg(severity_color),
                ),
                Span::raw(&entry.message),
            ])
        })
        .collect();

    // Create the paragraph widget with the text
    let log_paragraph = Paragraph::new(text)
        .block(log_block)
        .wrap(Wrap { trim: true });

    // Render the log panel
    frame.render_widget(log_paragraph, area);
}
