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
    ui::{self, Shortcut, Shortcuts},
};

/// Helper function to get an icon for a plugin
fn get_plugin_icon(plugin: &PluginInfo, default_style: Style) -> Span {
    let icon_text = if let Some(icon) = &plugin.icon {
        icon.clone()
    } else {
        // Use the centralized function for default icons
        ui::get_default_icon(&plugin.name)
    };
    
    Span::styled(format!("{} ", icon_text), default_style)
}

/// Get a status indicator span based on plugin initialization state
fn get_status_span(initialized: bool) -> Span<'static> {
    if initialized {
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
    }
}

/// Create a detail line with label and value
fn create_detail_line<'a>(label: &'a str, value: &'a str, label_style: Style, value_style: Style) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("{}: ", label), label_style),
        Span::styled(value, value_style),
    ])
}

/// Render the plugins view
pub(crate) fn render_plugins(model: &mut AppModel, area: Rect, frame: &mut Frame) {
    let shortcuts = Shortcuts::new(vec![
        Shortcut::Pair("Esc", t!("ui_shortcut_back").as_ref()),
        Shortcut::Pair("q", t!("ui_shortcut_quit").as_ref()),
    ])
    .with_alignment(Alignment::Right)
    .with_label_style(model.appearance.default_style.add_modifier(Modifier::BOLD));

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
        .constraints([Constraint::Min(10)])
        .split(inner_area);

    let content_area = chunks[0];

    // Split the content area into two sections: plugin list and plugin details
    let list_detail_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(content_area);

    // Store the list area for mouse events
    model.plugin_list_area = Some(list_detail_chunks[0]);

    // Render the plugin list (statefully) and details
    render_plugin_list(model, frame, list_detail_chunks[0], &plugins);
    render_plugin_details(model, frame, list_detail_chunks[1], &plugins);
}

/// Render the list of plugins in the left panel (always statefully)
fn render_plugin_list(model: &mut AppModel, frame: &mut Frame, area: Rect, plugins: &[PluginInfo]) {
    // Create the plugin list items
    let items: Vec<ListItem> = plugins
        .iter()
        .map(|plugin| {
            let status = get_status_span(plugin.initialized);
            let icon = get_plugin_icon(plugin, model.appearance.default_style);
            let name = Span::styled(&plugin.name, model.appearance.default_style);
            let version = Span::styled(
                format!(" (v{})", plugin.version),
                model.appearance.default_style.bold(),
            );

            // Create a Line from spans, including icon
            ListItem::new(Line::from(vec![status, icon, name, version]))
        })
        .collect();

    let plugin_list = List::new(items)
        .block(
            model
                .appearance
                .default_block
                .clone()
                .title(format!(" {} ", t!("ui_plugins_list"))),
        )
        .highlight_style(
            Style::default().add_modifier(Modifier::REVERSED | Modifier::ITALIC | Modifier::BOLD),
        )
        .highlight_symbol("> "); // Always show highlight symbol

    // Render the stateful list using the ListState from the model
    frame.render_stateful_widget(plugin_list, area, &mut model.plugin_view_state.plugin_list_state);
}

/// Render the details of the selected plugin in the right panel
fn render_plugin_details(model: &mut AppModel, frame: &mut Frame, area: Rect, plugins: &[PluginInfo]) {
    // Use the synced selected_index from the model state
    let selected_plugin = match model.plugin_view_state.selected_index {
        Some(idx) if idx < plugins.len() => Some(&plugins[idx]),
        _ => None,
    };

    if let Some(plugin) = selected_plugin {
        let description = plugin.description.as_deref().unwrap_or_default();
        let bold_style = Style::default().add_modifier(Modifier::BOLD);
        let mut detail_lines = Vec::new();

        // Get the icon as a string
        let icon_display = if let Some(icon) = &plugin.icon {
            icon.clone()
        } else {
            // Use the centralized function for default icons
            ui::get_default_icon(&plugin.name)
        };

        // Create translated labels as local variables to avoid temporary value issues
        let icon_label = t!("ui_plugins_icon");
        let name_label = t!("ui_plugins_name");
        let version_label = t!("ui_plugins_version");
        let status_label = t!("ui_plugins_status");
        let description_label = t!("ui_plugins_description");
        
        // Status text
        let status_text = if plugin.initialized {
            t!("ui_plugins_initialized")
        } else {
            t!("ui_plugins_not_initialized")
        };

        // Add detail lines
        detail_lines.push(create_detail_line(
            &icon_label,
            &icon_display,
            bold_style,
            Style::default(),
        ));
        
        detail_lines.push(create_detail_line(
            &name_label, 
            &plugin.name, 
            bold_style, 
            Style::default()
        ));
        
        detail_lines.push(create_detail_line(
            &version_label, 
            &plugin.version, 
            bold_style, 
            Style::default()
        ));

        // Status line with special coloring
        let status_style = if plugin.initialized {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };
        
        detail_lines.push(create_detail_line(
            &status_label,
            &status_text,
            bold_style,
            status_style,
        ));

        // Description header and content
        detail_lines.push(create_detail_line(
            &description_label,
            "",
            bold_style,
            Style::default(),
        ));
        
        detail_lines.push(Line::from(Span::raw(description)));

        // Render the details widget
        let plugin_details = Paragraph::new(Text::from(detail_lines))
            .block(
                model
                    .appearance
                    .default_block
                    .clone()
                    .title(format!(" {} ", t!("ui_plugins_details"))),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(plugin_details, area);
    } else {
        // Optional: Render a placeholder if no plugin is selected
        let placeholder_text = Paragraph::new(t!("ui_plugins_select_prompt"))
            .alignment(Alignment::Center)
            .block(
                model
                    .appearance
                    .default_block
                    .clone()
                    .title(format!(" {} ", t!("ui_plugins_details"))),
            );
        frame.render_widget(placeholder_text, area);
    }
}
