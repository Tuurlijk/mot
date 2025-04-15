use crate::{
    model::EditField,
    ui::{Autocomplete, Shortcut, Shortcuts},
    AppModel,
};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap};
use ratatui::Frame;

/// Render the time entry edit form
pub fn render_time_entry_edit(model: &mut AppModel, area: Rect, frame: &mut Frame) {
    // Store the edit form area in the model for click detection
    model.edit_form_area = Some(area);

    // Clear previous field areas
    model.edit_state.field_areas.clear();

    let shortcuts = Shortcuts::new(vec![
        Shortcut::Pair("Tab", "Change focus"),
        Shortcut::Pair("Ctrl+S", "Save"),
        Shortcut::Pair("Esc", "Cancel"),
    ]);

    // Determine title based on whether we are creating or editing
    let title = if model.edit_state.entry_id.is_empty() {
        " Create Time Entry "
    } else {
        " Edit Time Entry "
    };

    // Create a nice block for the edit form
    let form_block = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .title_bottom(shortcuts.as_line())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(1, 1, 0, 0));

    // Calculate the inner area for form content
    let inner_area = form_block.inner(area);

    // Split the inner area into 8 sections for different form fields
    let chunks = Layout::vertical([
        Constraint::Length(1),  // Field label (Description)
        Constraint::Length(20), // Description field
        Constraint::Length(1),  // Spacer
        Constraint::Length(8),  // Project & Client fields (Increased height)
        Constraint::Length(1),  // Spacer
        Constraint::Length(3),  // Date & Time fields
        Constraint::Length(1),  // Spacer
        Constraint::Length(3),  // Buttons/Instructions
    ])
    .split(inner_area);

    let active_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(model.appearance.default_foreground_color_indexed))
        .title_style(
            Style::default()
                .fg(model.appearance.default_foreground_color_indexed)
                .bold(),
        )
        .padding(Padding::new(1, 0, 0, 0))
        .border_type(BorderType::Rounded);

    let inactive_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(model.appearance.default_foreground_color_dimmed_indexed))
        .border_type(BorderType::Rounded);

    // Description section
    let description_label = Paragraph::new("Description")
        .style(
            if model.edit_state.selected_field == EditField::Description {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            },
        )
        .block(
            Block::default()
                .borders(Borders::NONE)
                .padding(Padding::new(1, 0, 0, 0)),
        );
    frame.render_widget(description_label, chunks[0]);

    // Render the description field
    if model.edit_state.selected_field == EditField::Description {
        // Render editor in description field
        model.edit_state.editor.set_block(active_block.clone());
        frame.render_widget(&model.edit_state.editor, chunks[1]);
    } else {
        // Render static text
        let description_para = Paragraph::new(model.edit_state.description.clone())
            .block(inactive_block.clone())
            .wrap(Wrap { trim: true });
        frame.render_widget(description_para, chunks[1]);
    }
    // Store description field area
    model
        .edit_state
        .field_areas
        .insert(EditField::Description, chunks[1]);

    // Project & Contact section
    let chunks_row = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[3]);

    let contact_area = chunks_row[0];
    let project_area = chunks_row[1];

    // Store contact and project field areas
    model
        .edit_state
        .field_areas
        .insert(EditField::Contact, contact_area);
    model
        .edit_state
        .field_areas
        .insert(EditField::Project, project_area);

    // Contact selection
    let transform_contact_fn = |contact: &crate::moneybird::types::Contact| -> String {
        contact.company_name.clone().unwrap_or_default()
    };
    let contact_autocomplete_widget = Autocomplete::new(
        &mut model.edit_state.contact_autocomplete,
        transform_contact_fn,
    )
    .block(
        Block::default()
            .title(" ✏️ Contact ")
            .padding(Padding::new(1, 0, 0, 0)),
    )
    .input_style(Style::default().fg(model.appearance.default_foreground_color_indexed))
    .selected_style(Style::default().bg(Color::Green).fg(Color::White))
    .dropdown_style(Style::default())
    .placeholder("Type to search contacts...");

    if model.edit_state.selected_field == EditField::Contact {
        // Use active block when selected
        frame.render_widget(
            contact_autocomplete_widget.block(active_block.clone().title(" ✏️ Contact ")),
            contact_area,
        );
        // Set cursor position for the active contact field
        let input_len = model.edit_state.contact_autocomplete.input.chars().count() as u16;
        // Adjust for block padding (left: 1) and border (left: 1)
        let cursor_x = contact_area.x + 1 + 1 + input_len;
        // Adjust for block padding (top: 0) and border (top: 1)
        let cursor_y = contact_area.y + 1;
        frame.set_cursor_position((cursor_x, cursor_y));
    } else {
        // Use inactive block and potentially display the selected name when not active
        let contact_name = model.edit_state.contact_name.clone().unwrap_or_default();

        // Display static text within an inactive block
        let contact_display = if model.edit_state.contact_id.is_some() {
            contact_name
        } else {
            "Contact: Select contact...".to_string()
        };
        let contact_label = Paragraph::new(contact_display).block(
            inactive_block
                .clone()
                .title(" Contact ")
                .padding(Padding::new(1, 0, 0, 0)),
        );
        frame.render_widget(contact_label, contact_area);
    }

    let project_area = chunks_row[1];

    // Project selection
    let transform_project_fn = |project: &crate::moneybird::types::Project| -> String {
        project.name.clone().unwrap_or_default()
    };
    let project_autocomplete_widget = Autocomplete::new(
        &mut model.edit_state.project_autocomplete,
        transform_project_fn,
    )
    .block(
        Block::default()
            .title(" ✏️ Project ")
            .padding(Padding::new(1, 0, 0, 0)),
    )
    .input_style(Style::default().fg(model.appearance.default_foreground_color_indexed))
    .selected_style(Style::default().bg(Color::Green).fg(Color::White))
    .dropdown_style(Style::default())
    .placeholder("Type to search projects...");

    if model.edit_state.selected_field == EditField::Project {
        // Use active block when selected
        frame.render_widget(
            project_autocomplete_widget.block(active_block.clone().title(" ✏️ Project ")),
            project_area,
        );
        // Set cursor position for the active project field
        let input_len = model.edit_state.project_autocomplete.input.chars().count() as u16;
        // Adjust for block padding (left: 1) and border (left: 1)
        let cursor_x = project_area.x + 1 + 1 + input_len;
        // Adjust for block padding (top: 0) and border (top: 1)
        let cursor_y = project_area.y + 1;
        frame.set_cursor_position((cursor_x, cursor_y));
    } else {
        // Use inactive block and display the selected name when not active
        let project_name = model.edit_state.project_name.clone().unwrap_or_default();

        // Display static text within an inactive block
        let project_display = if model.edit_state.project_id.is_some() {
            project_name
        } else {
            "Project: Select project...".to_string()
        };
        let project_label = Paragraph::new(project_display).block(
            inactive_block
                .clone()
                .title(" Project ")
                .padding(Padding::new(1, 0, 0, 0)),
        );
        frame.render_widget(project_label, project_area);
    }

    // Date & Time fields
    let date_time_row = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(chunks[5]);

    // Store date and time field areas
    model
        .edit_state
        .field_areas
        .insert(EditField::StartTime, date_time_row[0]);
    model
        .edit_state
        .field_areas
        .insert(EditField::EndTime, date_time_row[1]);
    model
        .edit_state
        .field_areas
        .insert(EditField::StartDate, date_time_row[2]);
    model
        .edit_state
        .field_areas
        .insert(EditField::EndDate, date_time_row[3]);

    let selected_field = model.edit_state.selected_field;

    // Start time field
    let start_time_label = " Start time ";
    if selected_field == EditField::StartTime {
        model.edit_state.editor.set_block(
            active_block
                .clone()
                .title(format!(" ✏️ {}", start_time_label)),
        );
        frame.render_widget(&model.edit_state.editor, date_time_row[0]);
    } else {
        let widget = Paragraph::new(model.edit_state.start_time.clone())
            .block(inactive_block.clone().title(start_time_label));
        frame.render_widget(widget, date_time_row[0]);
    }

    // End time field
    let end_time_label = " End time ";
    if selected_field == EditField::EndTime {
        model.edit_state.editor.set_block(
            active_block
                .clone()
                .title(format!(" ✏️ {}", end_time_label)),
        );
        frame.render_widget(&model.edit_state.editor, date_time_row[1]);
    } else {
        let widget = Paragraph::new(model.edit_state.end_time.clone())
            .block(inactive_block.clone().title(end_time_label));
        frame.render_widget(widget, date_time_row[1]);
    }

    // Start date field
    let start_date_label = " Start date ";
    if selected_field == EditField::StartDate {
        model.edit_state.editor.set_block(
            active_block
                .clone()
                .title(format!(" ✏️ {}", start_date_label)),
        );
        frame.render_widget(&model.edit_state.editor, date_time_row[2]);
    } else {
        let widget = Paragraph::new(model.edit_state.start_date.clone())
            .block(inactive_block.clone().title(start_date_label));
        frame.render_widget(widget, date_time_row[2]);
    }

    // End date field
    let end_date_label = " End date ";
    if selected_field == EditField::EndDate {
        model.edit_state.editor.set_block(
            active_block
                .clone()
                .title(format!(" ✏️ {}", end_date_label)),
        );
        frame.render_widget(&model.edit_state.editor, date_time_row[3]);
    } else {
        let widget = Paragraph::new(model.edit_state.end_date.clone())
            .block(inactive_block.clone().title(end_date_label));
        frame.render_widget(widget, date_time_row[3]);
    }

    // Render the form's outer block
    frame.render_widget(form_block.clone(), area);
}
