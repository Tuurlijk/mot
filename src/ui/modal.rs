use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;
use rust_i18n::t;
use std::borrow::Cow;

use super::{Shortcut, Shortcuts};

/// Modal type for different visual styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalType {
    Info,
    Error,
    Warning,
    Confirm,
}

impl ModalType {
    /// Get the emoji character for this modal type
    pub fn emoji(&self) -> &'static str {
        match self {
            ModalType::Info => "🔔",
            ModalType::Error => "⛔",
            ModalType::Warning => "⚠️",
            ModalType::Confirm => "✅",
        }
    }
}

/// Modal data structure for rendering
#[derive(Clone)]
pub struct ModalData {
    pub title: String,
    pub message: String,
    pub modal_type: ModalType,
    pub buttons: Option<Shortcuts>, // Optional buttons to show
    pub id: Option<String>,         // Optional identifier for action handling
    pub on_confirm: Option<crate::event::Message>, // Message to send when confirmed
    pub on_cancel: Option<crate::event::Message>, // Message to send when canceled
}

impl Default for ModalData {
    fn default() -> Self {
        Self {
            title: String::new(),
            message: String::new(),
            modal_type: ModalType::Info,
            buttons: None,
            id: None,
            on_confirm: None,
            on_cancel: None,
        }
    }
}

/// Helper function to show a modal message in the UI
pub fn show_modal(model: &mut crate::AppModel, modal: ModalData) {
    // Push the modal onto the stack
    model.modal_stack.push(modal);

    // Dim the background when a modal is displayed
    model.appearance.default_style =
        Style::default().fg(model.appearance.default_foreground_color_dimmed_indexed);
}

/// Helper function to show an error message in the UI
pub fn show_error(model: &mut crate::AppModel, message: impl Into<Cow<'static, str>>) {
    show_modal(
        model,
        ModalData {
            title: t!("modal_error_title").to_string(),
            message: message.into().to_string(),
            modal_type: ModalType::Error,
            id: Some("error".to_string()),
            ..Default::default()
        },
    );
}

/// Helper function to show a connection error message in the UI
pub fn show_connection_error(model: &mut crate::AppModel, message: impl Into<Cow<'static, str>>) {
    show_modal(
        model,
        ModalData {
            title: t!("modal_connection_error_title").to_string(),
            message: message.into().to_string(),
            modal_type: ModalType::Error,
            id: Some("connection_error".to_string()),
            ..Default::default()
        },
    );
}

/// Helper function to show a confirmation modal
pub fn show_confirmation(
    model: &mut crate::AppModel,
    title: impl Into<Cow<'static, str>>,
    message: impl Into<Cow<'static, str>>,
    on_confirm: Option<crate::event::Message>,
    on_cancel: Option<crate::event::Message>,
) {
    show_modal(
        model,
        ModalData {
            title: title.into().to_string(),
            message: message.into().to_string(),
            modal_type: ModalType::Confirm,
            buttons: Some(
                Shortcuts::new(vec![
                    Shortcut::Pair("Esc", t!("modal_cancel").as_ref()),
                    Shortcut::Pair("Enter", t!("modal_ok").as_ref()),
                ])
                .with_key_style(
                    model
                        .appearance
                        .default_style
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .with_label_style(model.appearance.default_style),
            ),
            id: Some("confirmation".to_string()),
            on_confirm,
            on_cancel,
        },
    );
}

/// Helper function to show an information modal
pub fn show_info(
    model: &mut crate::AppModel,
    id: &str,
    title: impl Into<Cow<'static, str>>,
    message: impl Into<Cow<'static, str>>,
) {
    show_modal(
        model,
        ModalData {
            title: title.into().to_string(),
            message: message.into().to_string(),
            modal_type: ModalType::Info,
            id: Some(id.to_string()),
            ..Default::default()
        },
    );
}

/// Render a modal dialog with different styles based on type
pub fn render_modal(model: &crate::AppModel, frame: &mut Frame) {
    // Check if there are any modals to render
    if let Some(modal_data) = model.modal_stack.top() {
        // Calculate a centered area for the modal
        let area = frame.area();
        let modal_width = std::cmp::min(80, area.width.saturating_sub(4));
        let modal_height = std::cmp::min(20, area.height.saturating_sub(4));

        // Calculate the modal position (centered)
        let modal_area = Rect::new(
            (area.width - modal_width) / 2,
            (area.height - modal_height) / 2,
            modal_width,
            modal_height,
        );

        // Create a clear background for the modal
        let clear = Clear;
        frame.render_widget(clear, modal_area);

        // Set border color based on modal type
        let border_color = match modal_data.modal_type {
            ModalType::Error => Color::Red,
            ModalType::Warning => Color::Yellow,
            ModalType::Info => Color::Blue,
            ModalType::Confirm => Color::Blue,
        };

        // Prepare the bottom instructions text
        let dismiss_shortcut = Shortcuts::new(vec![Shortcut::Pair(
            "Enter",
            t!("modal_press_enter_to_dismiss").as_ref(),
        )])
        .with_key_style(
            model
                .appearance
                .default_style
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .with_label_style(model.appearance.default_style);
        let instructions = match &modal_data.buttons {
            Some(buttons) => buttons.as_line(),
            _ => dismiss_shortcut.as_line(),
        };

        let default_modal_style = Style::default().fg(Color::Indexed(
            crate::ui::color::rgb_to_indexed(model.appearance.default_foreground_color),
        ));

        // Create the modal widget
        let modal_widget = Paragraph::new(modal_data.message.clone())
            .block(
                Block::default()
                    .title(format!(
                        " {} {} ",
                        modal_data.modal_type.emoji(),
                        modal_data.title
                    ))
                    .title_alignment(Alignment::Center)
                    .title_bottom(instructions)
                    .title_style(default_modal_style)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(
                        Style::default()
                            .fg(border_color)
                            .add_modifier(Modifier::BOLD),
                    )
                    .style(default_modal_style),
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        // Render the modal
        frame.render_widget(modal_widget, modal_area);
    }
}
