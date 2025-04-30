use crate::ui::{Shortcut, Shortcuts};
use crate::AppModel;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Stylize};
use ratatui::widgets::{Block, Borders, Padding};
use ratatui::{symbols, Frame};
use rust_i18n::t;

pub fn render_search(model: &mut AppModel, area: Rect, frame: &mut Frame) {
    let collapsed_top_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        ..symbols::border::PLAIN
    };

    let shortcuts = Shortcuts::new(vec![
        Shortcut::Pair("🔍", t!("ui_shortcut_filter").as_ref()),
        Shortcut::Pair("Esc", t!("ui_shortcut_exit").as_ref()),
        Shortcut::Pair("Ctrl+U", t!("ui_shortcut_clear").as_ref()),
    ])
    .with_alignment(Alignment::Left)
    .with_label_style(model.appearance.default_style.add_modifier(Modifier::BOLD));

    let block = Block::default()
        .padding(Padding {
            left: 1,
            right: 0,
            top: 0,
            bottom: 0,
        })
        .border_set(collapsed_top_border_set)
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .title(shortcuts.as_line())
        .style(model.appearance.default_style);

    let inner_area = block.inner(area);

    frame.render_widget(block, area);

    frame.render_widget(&model.search_state.text_input, inner_area);
}
