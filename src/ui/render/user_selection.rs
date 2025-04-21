use crate::{
    model::AppModel,
    ui::{Shortcut, Shortcuts},
};
use ratatui::{
    layout::{Alignment, Constraint, Rect},
    prelude::*,
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table, List, ListItem},
};
use rust_i18n::t;

pub fn render_user_selection(model: &mut AppModel, area: Rect, frame: &mut Frame) {
    let shortcuts = Shortcuts::new(vec![
        Shortcut::Pair("Enter", t!("ui_shortcut_select_user").as_ref()),
        Shortcut::Pair("Esc", t!("ui_shortcut_exit_user_selection").as_ref()),
    ])
    .with_alignment(Alignment::Right)
    .with_label_style(model.appearance.default_style.add_modifier(Modifier::BOLD));

    // Create ListItems instead of Rows
    let items: Vec<ListItem> = model.users.iter().map(|user| {
        let name = user.name.clone().unwrap_or_default();
        let email = user.email.clone().unwrap_or_default();
        let id = user.id.clone().unwrap_or_default();
        // Format the user info into a single line for the list
        let line = Line::from(vec![
            Span::styled(format!("{:<38}", id), Style::default()), // Pad ID
            Span::raw(" | "),
            Span::styled(format!("{:<30}", name), Style::default().bold()), // Pad Name
            Span::raw(" | "),
            Span::styled(email, Style::default().italic()),
        ]);
        ListItem::new(line).style(model.appearance.default_style)
    }).collect();

    // Create a List widget
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(format!(" {} ", t!("ui_select_default_user")))
                .title_alignment(Alignment::Center)
                .title_bottom(shortcuts.as_line())
                .style(model.appearance.default_style),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED | Modifier::ITALIC | Modifier::BOLD))
        .highlight_symbol("> "); // Add a highlight symbol

    // Render the List widget statefully using model.user_selection_state (which is ListState)
    frame.render_stateful_widget(list, area, &mut model.user_selection_state);
}
