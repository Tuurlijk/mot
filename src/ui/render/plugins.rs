use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, TableState, Wrap},
    Frame,
};
use rust_i18n::t;

use crate::{
    model::AppModel,
    plugin::PluginInfo,
    ui::{Shortcut, Shortcuts},
};

/// Render the plugins view
pub(crate) fn render_plugins(model: &mut AppModel, area: Rect, frame: &mut Frame) {
    let shortcuts = Shortcuts::new(vec![
        Shortcut::Pair("Esc", t!("ui_shortcut_back").as_ref()),
        Shortcut::Pair("q", t!("ui_shortcut_quit").as_ref()),
    ])
    .with_alignment(Alignment::Right)
    .with_label_style(model.appearance.default_style);

    // Create a block for the plugins view
    let block = model
        .appearance
        .default_block
        .clone()
        .title(format!(" {} ", t!("ui_plugins_title")))
        .title_bottom(shortcuts.as_line());

    // Apply the block to the area
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Get the list of loaded plugins from the plugin manager
    let plugins = if let Some(plugin_manager) = &model.plugin_manager {
        plugin_manager.list_plugins()
    } else {
        Vec::new()
    };

    // If no plugins are loaded, show a message
    if plugins.is_empty() {
        let no_plugins_text = Paragraph::new(Text::from(t!("ui_no_plugins_found")))
            .style(Style::default().fg(model.appearance.default_foreground_color_indexed))
            .block(model.appearance.default_block.clone());
        frame.render_widget(no_plugins_text, inner_area);
        return;
    }

    // Split the area vertically to include shortcuts at the bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(inner_area);

    let content_area = chunks[0];
    let shortcuts_area = chunks[1];

    // Split the content area into two sections: plugin list and plugin details
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(content_area);

    // Create the plugin list
    let items: Vec<ListItem> = plugins
        .iter()
        .enumerate()
        .map(|(_idx, plugin)| {
            let status = if plugin.initialized {
                Span::styled(
                    " [✓] ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(
                    " [✗] ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )
            };

            let name = Span::styled(&plugin.name, model.appearance.default_style);

            let version = Span::styled(
                format!(" (v{})", plugin.version),
                model.appearance.default_style.bold(),
            );

            // Create a Line from spans
            let spans = vec![status, name, version];
            ListItem::new(Line::from(spans))
        })
        .collect();

    let mut plugin_list = List::new(items)
        .block(
            model
                .appearance
                .default_block
                .clone()
                .title(format!(" {} ", t!("ui_plugins_list"))),
        )
        .highlight_style(
            Style::default().add_modifier(Modifier::REVERSED | Modifier::ITALIC | Modifier::BOLD),
        );

    // Set the selected item if applicable
    if let Some(selected_idx) = model.plugin_view_state.selected_index {
        plugin_list = plugin_list.highlight_symbol("> ");

        // We need to create a stateful widget for selection to work
        let mut state = ListState::default();
        state.select(Some(selected_idx));

        // Render the stateful list
        frame.render_stateful_widget(plugin_list, chunks[0], &mut state);
    } else {
        // Render the list without selection
        frame.render_widget(plugin_list, chunks[0]);
    }

    // Show plugin details in the second chunk if a plugin is selected
    let selected_plugin = match &model.plugin_view_state.selected_index {
        Some(idx) if *idx < plugins.len() => Some(&plugins[*idx]),
        _ => None,
    };

    if let Some(plugin) = selected_plugin {
        let description = plugin.description.as_deref().unwrap_or_default();

        // Format plugin details using Lines instead of Spans
        let mut detail_lines = Vec::new();

        // Name line
        let name_spans = vec![
            Span::styled(
                format!("{}: ", t!("ui_plugins_name")),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(&plugin.name),
        ];
        detail_lines.push(Line::from(name_spans));

        // Version line
        let version_spans = vec![
            Span::styled(
                format!("{}: ", t!("ui_plugins_version")),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(&plugin.version),
        ];
        detail_lines.push(Line::from(version_spans));

        // Status line
        let status_style = if plugin.initialized {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };

        let status_spans = vec![
            Span::styled(
                format!("{}: ", t!("ui_plugins_status")),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                if plugin.initialized {
                    t!("ui_plugins_initialized")
                } else {
                    t!("ui_plugins_not_initialized")
                },
                status_style,
            ),
        ];
        detail_lines.push(Line::from(status_spans));

        // Description header
        let desc_header = vec![Span::styled(
            format!("{}: ", t!("ui_plugins_description")),
            Style::default().add_modifier(Modifier::BOLD),
        )];
        detail_lines.push(Line::from(desc_header));

        // Description content
        detail_lines.push(Line::from(Span::raw(description)));

        let plugin_details = Paragraph::new(Text::from(detail_lines))
            .block(
                model
                    .appearance
                    .default_block
                    .clone()
                    .title(format!(" {} ", t!("ui_plugins_details"))),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(plugin_details, chunks[1]);
    }
}
