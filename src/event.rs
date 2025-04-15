use crate::{ui, AppModel};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEventKind};
use std::time::Duration;

#[derive(PartialEq, Clone)]
pub enum Message {
    AutocompleteClearInput,
    AutocompleteKeyPress(KeyEvent),
    AutocompleteNextItem,
    AutocompletePreviousItem,
    AutocompleteRefresh,
    AutocompleteResultsContact(Vec<crate::moneybird::types::Contact>),
    AutocompleteResultsProject(Vec<crate::moneybird::types::Project>),
    AutocompleteSelect,

    ConfirmModal(String),
    DismissModal(String, bool),

    EditTimeEntry,
    EditTimeEntryCancel,
    EditTimeEntryFieldClick(crate::model::EditField),
    EditTimeEntryKeyPress(KeyEvent),
    EditTimeEntryNextField,
    EditTimeEntryPreviousField,
    EditTimeEntrySave,
    EditTimeEntrySelectContact,
    EditTimeEntrySelectProject,

    ExecuteDeleteTimeEntry(String),
    ExecuteExport,

    Quit,

    TimeEntryClearSearch,
    TimeEntryCreate,
    TimeEntryCurrentWeek,
    TimeEntryDelete,
    TimeEntryExport,
    TimeEntryNextWeek,
    TimeEntryPreviousWeek,
    TimeEntryRefresh,
    TimeEntrySearchHide,
    TimeEntrySearchKeyPress(KeyEvent),
    TimeEntrySearchShow,
    TimeEntrySelectNext,
    TimeEntrySelectPrevious,
    TimeEntrySelectRow(usize),

    ToggleLogPanel,

    UserConfirmSelection,
    UserSelectNext,
    UserSelectPrevious,
}

// Implement PartialEq for the Message enum to help with Contact Vec comparison
impl PartialEq for crate::moneybird::types::Contact {
    fn eq(&self, other: &Self) -> bool {
        // Compare by ID if available, otherwise by name
        match (&self.id, &other.id) {
            (Some(id1), Some(id2)) => id1 == id2,
            _ => self.company_name == other.company_name,
        }
    }
}

// Implement PartialEq for the Message enum to help with Project Vec comparison
impl PartialEq for crate::moneybird::types::Project {
    fn eq(&self, other: &Self) -> bool {
        // Compare by ID if available, otherwise by name
        match (&self.id, &other.id) {
            (Some(id1), Some(id2)) => id1 == id2,
            _ => self.name == other.name,
        }
    }
}

// Revert handle_event to synchronous
pub fn handle_event(model: &mut AppModel) -> Result<Option<Message>> {
    let poll_timeout = Duration::from_millis(100); // Check every 100ms for debounce timeout

    // Original synchronous poll
    if event::poll(poll_timeout)? {
        // Original synchronous read
        match event::read()? {
            Event::Key(key) => {
                if key.kind == event::KeyEventKind::Press || key.kind == event::KeyEventKind::Repeat
                {
                    // Check if we're in a modal cooldown period
                    if model.modal_interaction.is_in_cooldown() {
                        model.log_debug("Ignoring key press during modal cooldown period");
                        return Ok(None);
                    }

                    // Only debounce Tab, BackTab, and some other keys to fix double-tab etc. issues
                    // All other keys are processed normally
                    let target_keys = &[
                        KeyCode::Backspace,
                        KeyCode::Delete,
                        KeyCode::Enter,
                        KeyCode::Tab,
                        KeyCode::BackTab,
                    ];
                    if !debounce(model, key.code, Some(target_keys)) {
                        return Ok(None);
                    }

                    // handle_key is synchronous, call it directly
                    Ok(handle_key(key, model))
                } else {
                    Ok(None)
                }
            }
            Event::Mouse(mouse) => {
                // handle_mouse is synchronous, call it directly
                Ok(handle_mouse(mouse, model))
            }
            _ => Ok(None), // Ignore other event types
        }
    } else {
        // Poll timed out - check for autocomplete debounce completion
        if model.edit_state.active {
            let should_refresh = match model.edit_state.selected_field {
                crate::model::EditField::Project => model
                    .edit_state
                    .project_autocomplete
                    .check_debounce_timeout(),
                crate::model::EditField::Contact => model
                    .edit_state
                    .contact_autocomplete
                    .check_debounce_timeout(),
                _ => false, // Not in an autocomplete field
            };

            if should_refresh {
                model.log_debug(format!(
                    "Debounce timeout reached for {:?}, triggering refresh.",
                    model.edit_state.selected_field
                ));
                // Return Ok(Some(...)) directly
                return Ok(Some(Message::AutocompleteRefresh));
            }
        }
        Ok(None)
    }
}

fn handle_key(key: event::KeyEvent, model: &mut AppModel) -> Option<Message> {
    // Toggle log panel key pressed, this works in any mode
    if key.code == KeyCode::F(12) {
        model.log_notice("F12: Toggle log panel");
        return Some(Message::ToggleLogPanel);
    }

    // --- User Selection Mode Handling ---
    if model.user_selection_active {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => return Some(Message::UserSelectPrevious),
            KeyCode::Down | KeyCode::Char('j') => return Some(Message::UserSelectNext),
            KeyCode::Enter => return Some(Message::UserConfirmSelection),
            KeyCode::Char('q') => return Some(Message::Quit), // Allow quitting
            _ => return None,                                 // Ignore other keys in this mode
        }
    }

    // Modal handling is done next
    if !model.modal_stack.is_empty() {
        // Record the modal interaction time
        model.modal_interaction.record_interaction();

        // Get information about the current modal before logging
        let modal_info = model
            .modal_stack
            .top()
            .map(|modal| (modal.id.clone().unwrap_or_default(), modal.modal_type));

        match key.code {
            KeyCode::Enter | KeyCode::Char('y') => {
                if let Some((modal_id, modal_type)) = modal_info {
                    model.log_debug(format!(
                        "Confirming modal: {} with key: {:?}",
                        modal_id, key
                    ));
                    if modal_type == ui::ModalType::Confirm {
                        return Some(Message::ConfirmModal(modal_id));
                    } else {
                        return Some(Message::DismissModal(modal_id, false));
                    }
                }
                return None;
            }
            KeyCode::Esc | KeyCode::Char('n') => {
                if let Some((modal_id, _)) = modal_info {
                    model.log_debug(format!(
                        "Cancelling modal: {} with key: {:?}",
                        modal_id, key
                    ));
                    return Some(Message::DismissModal(modal_id, true));
                }
                return Some(Message::DismissModal(String::new(), true));
            }
            KeyCode::Char(' ') | KeyCode::Tab | KeyCode::BackTab => {
                if let Some((modal_id, _)) = modal_info {
                    model.log_debug(format!(
                        "Dismissing modal: {} with key: {:?}",
                        modal_id, key
                    ));
                    return Some(Message::DismissModal(modal_id, false));
                }
                return Some(Message::DismissModal(String::new(), false));
            }
            _ => return None,
        }
    }

    // --- Refactored Edit State Key Handling ---
    if model.edit_state.active {
        match key.code {
            // --- Global Edit Keys ---
            KeyCode::Char('s') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                Some(Message::EditTimeEntrySave)
            }
            KeyCode::Tab => Some(Message::EditTimeEntryNextField),
            KeyCode::BackTab => Some(Message::EditTimeEntryPreviousField),

            // --- Keys with Field-Dependent Behavior ---
            KeyCode::Enter => {
                match model.edit_state.selected_field {
                    crate::model::EditField::Description => {
                        if key.modifiers.contains(event::KeyModifiers::SHIFT) {
                            Some(Message::EditTimeEntryKeyPress(key)) // Let the textarea handle it
                        } else {
                            Some(Message::EditTimeEntryNextField) // Default Enter: move to next field
                        }
                    }
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        let is_dropdown_visible = if model.edit_state.selected_field
                            == crate::model::EditField::Project
                        {
                            model.edit_state.project_autocomplete.is_dropdown_visible
                        } else {
                            model.edit_state.contact_autocomplete.is_dropdown_visible
                        };
                        if is_dropdown_visible {
                            Some(Message::AutocompleteSelect)
                        } else {
                            Some(Message::EditTimeEntryNextField) // Default Enter: move to next field
                        }
                    }
                    _ => Some(Message::EditTimeEntryNextField), // Default for other fields
                }
            }
            KeyCode::Up => {
                match model.edit_state.selected_field {
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        let is_dropdown_visible = if model.edit_state.selected_field
                            == crate::model::EditField::Project
                        {
                            model.edit_state.project_autocomplete.is_dropdown_visible
                        } else {
                            model.edit_state.contact_autocomplete.is_dropdown_visible
                        };
                        if is_dropdown_visible {
                            Some(Message::AutocompletePreviousItem)
                        } else {
                            // No action if dropdown not visible for now
                            None
                        }
                    }
                    _ => Some(Message::EditTimeEntryKeyPress(key)),
                }
            }
            KeyCode::Down => {
                match model.edit_state.selected_field {
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        let is_dropdown_visible = if model.edit_state.selected_field
                            == crate::model::EditField::Project
                        {
                            model.edit_state.project_autocomplete.is_dropdown_visible
                        } else {
                            model.edit_state.contact_autocomplete.is_dropdown_visible
                        };
                        if is_dropdown_visible {
                            Some(Message::AutocompleteNextItem)
                        } else {
                            // Trigger refresh if dropdown not visible and input exists?
                            // Or maybe do nothing?
                            None // Doing nothing for now
                        }
                    }
                    _ => Some(Message::EditTimeEntryKeyPress(key)),
                }
            }
            KeyCode::Esc => {
                match model.edit_state.selected_field {
                    crate::model::EditField::Project => {
                        // Handle Esc specifically for Project field
                        if model.edit_state.project_autocomplete.is_dropdown_visible {
                            model.edit_state.project_autocomplete.is_dropdown_visible = false;
                            None // Just hide dropdown
                        } else {
                            Some(Message::EditTimeEntryCancel) // Esc cancels edit if dropdown hidden
                        }
                    }
                    crate::model::EditField::Contact => {
                        // Handle Esc specifically for Contact field
                        if model.edit_state.contact_autocomplete.is_dropdown_visible {
                            model.edit_state.contact_autocomplete.is_dropdown_visible = false;
                            None // Just hide dropdown
                        } else {
                            Some(Message::EditTimeEntryCancel) // Esc cancels edit if dropdown hidden
                        }
                    }
                    _ => Some(Message::EditTimeEntryCancel), // Default Esc: cancel edit mode
                }
            }
            KeyCode::Char('u') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                match model.edit_state.selected_field {
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        Some(Message::AutocompleteClearInput)
                    }
                    _ => Some(Message::EditTimeEntryKeyPress(key)), // Pass Ctrl+U to standard editor fields
                }
            }
            KeyCode::Char(_) | KeyCode::Backspace => {
                match model.edit_state.selected_field {
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        Some(Message::AutocompleteKeyPress(key))
                    }
                    _ => Some(Message::EditTimeEntryKeyPress(key)), // Pass character/backspace to standard editor fields
                }
            }
            // Catch-all for any other keys (e.g., F-keys not handled above, Delete, Home, End etc.)
            _ => {
                // Pass to standard editor fields if appropriate, otherwise ignore
                match model.edit_state.selected_field {
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        model.log_debug(format!(
                            "Ignoring unhandled key ({:?}) in project/contact field",
                            key.code
                        ));
                        None // Ignore in Project/Contact fields as they don't use the shared editor
                    }
                    _ => Some(Message::EditTimeEntryKeyPress(key)), // Pass to editor for Description, Dates, Times
                }
            }
        }
    // --- End Refactored Edit State ---
    } else {
        // --- Handling for Non-Edit State ---
        if model.search_state.active {
            match key.code {
                KeyCode::Char('u') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                    Some(Message::TimeEntryClearSearch)
                }
                KeyCode::Char('x') => Some(Message::TimeEntryExport),
                KeyCode::F(4) => Some(Message::EditTimeEntry),
                KeyCode::Up => Some(Message::TimeEntrySelectPrevious),
                KeyCode::Down => Some(Message::TimeEntrySelectNext),
                KeyCode::Left => Some(Message::TimeEntryPreviousWeek),
                KeyCode::Right => Some(Message::TimeEntryNextWeek),
                KeyCode::Esc => Some(Message::TimeEntrySearchHide),
                _ => Some(Message::TimeEntrySearchKeyPress(key)),
            }
        } else {
            match key.code {
                KeyCode::Char('x') => Some(Message::TimeEntryExport),
                KeyCode::F(4) => Some(Message::EditTimeEntry),
                KeyCode::Char('h') | KeyCode::Left => Some(Message::TimeEntryPreviousWeek),
                KeyCode::Char('j') | KeyCode::Down => Some(Message::TimeEntrySelectNext),
                KeyCode::Char('k') | KeyCode::Up => Some(Message::TimeEntrySelectPrevious),
                KeyCode::Char('l') | KeyCode::Right => Some(Message::TimeEntryNextWeek),
                KeyCode::Char('q') => Some(Message::Quit),
                KeyCode::Char('e') | KeyCode::Char(' ') | KeyCode::Enter => {
                    Some(Message::EditTimeEntry)
                }
                KeyCode::Char('c') => Some(Message::TimeEntryCreate),
                KeyCode::Char('t') => Some(Message::TimeEntryCurrentWeek),
                KeyCode::Char('r') => Some(Message::TimeEntryRefresh),
                KeyCode::Char('f') | KeyCode::Char('/') => Some(Message::TimeEntrySearchShow),
                KeyCode::Char('d') | KeyCode::Delete => Some(Message::TimeEntryDelete),
                _ => None,
            }
        }
    }
}

fn handle_mouse(mouse: event::MouseEvent, model: &mut AppModel) -> Option<Message> {
    if !model.modal_stack.is_empty() {
        // Ignore mouse events when a modal is open
        return None;
    }

    // Handle clicks in edit mode
    if model.edit_state.active && mouse.kind == MouseEventKind::Down(event::MouseButton::Left) {
        let mouse_pos = ratatui::layout::Position {
            x: mouse.column,
            y: mouse.row,
        };

        // Check if click is on any of the stored field areas
        for (&field, &area) in &model.edit_state.field_areas {
            if area.contains(mouse_pos) {
                model.log_debug(format!("Click detected on field: {:?}", field));
                return Some(Message::EditTimeEntryFieldClick(field));
            }
        }

        // Click was in edit mode but not on any field
        return None;
    }

    // Handle normal mode clicks time entry table
    match mouse.kind {
        MouseEventKind::ScrollDown => Some(Message::TimeEntrySelectPrevious),
        MouseEventKind::ScrollUp => Some(Message::TimeEntrySelectNext),
        MouseEventKind::Down(event::MouseButton::Left) => {
            if let Some(table_area) = model.table_area {
                // Check if the click is within the table's bounds
                if table_area.contains(ratatui::layout::Position {
                    x: mouse.column,
                    y: mouse.row,
                }) {
                    // Adjust for table header (1 row) and top border (1 row)
                    let relative_row = mouse.row.saturating_sub(table_area.y + 2);
                    let selected_index =
                        model.time_entry_table_state.offset() + relative_row as usize;

                    // Ensure the calculated index is within the bounds of the data
                    if selected_index < model.time_entries_for_table.len() {
                        model.log_debug(format!(
                            "Mouse click detected on row: {}. Selected index: {}",
                            relative_row, selected_index
                        ));
                        return Some(Message::TimeEntrySelectRow(selected_index));
                    }
                }
            }
            None // Click was outside table area
        }
        _ => None, // Ignore other mouse events
    }
}

// Helper function to debounce key presses
// Returns true if the key should be processed, false if it should be ignored
// If target_keys is provided, debouncing only applies to those keys
fn debounce(model: &mut AppModel, key_code: KeyCode, target_keys: Option<&[KeyCode]>) -> bool {
    // If target_keys is provided, only debounce those specific keys
    if let Some(keys) = target_keys {
        if !keys.contains(&key_code) {
            // Not a targeted key, so process it without debouncing
            return true;
        }
    }

    // Check if we're in a cooldown period
    if model.key_debounce.is_in_cooldown() {
        // We're in a cooldown period, so ignore this keypress
        return false;
    }

    // Record this keypress for future debouncing
    model.key_debounce.record_keypress();

    // We're not in a cooldown period, so process this keypress
    true
}
