use crate::{
    model::AppModel,
    ui::{Shortcut, Shortcuts},
};
use ratatui::{
    layout::{Alignment, Constraint, Rect},
    prelude::*,
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};
use rust_i18n::t;

pub fn render_user_selection(model: &mut AppModel, area: Rect, frame: &mut Frame) {
    let shortcuts = Shortcuts::new(vec![
        Shortcut::Pair("Enter", t!("ui_shortcut_select_user").as_ref()),
        Shortcut::Pair("Esc", t!("ui_shortcut_exit_user_selection").as_ref()),
    ])
    .with_alignment(Alignment::Right)
    .with_label_style(model.appearance.default_style);

    let header_cells = [t!("ui_user_id"), t!("ui_user_name"), t!("ui_user_email")];
    let header = Row::new(header_cells)
        .style(model.appearance.default_style.add_modifier(Modifier::BOLD))
        .height(1)
        .bottom_margin(1);

    let rows = model.users.iter().map(|user| {
        let item = [
            user.id.clone().unwrap_or_default(),
            user.name.clone().unwrap_or_default(),
            user.email.clone().unwrap_or_default(),
        ];
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.clone()));
        Row::new(cells)
            .height(height as u16)
            .style(model.appearance.default_style)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(38), // ID width
            Constraint::Length(30), // Name width
            Constraint::Fill(1),    // Email width (fills remaining)
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(" {} ", t!("ui_select_default_user")))
            .title_alignment(Alignment::Center)
            .title_bottom(shortcuts.as_line())
            .style(model.appearance.default_style),
    )
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED | Modifier::ITALIC | Modifier::BOLD));

    // Use a mutable borrow of the state
    let mut table_state = model.user_selection_state.clone();

    frame.render_stateful_widget(table, area, &mut table_state);

    // Update the model's state after rendering
    // model.user_selection_state = table_state; // No need to write back, we just read it here
}
