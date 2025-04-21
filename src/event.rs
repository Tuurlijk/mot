use crate::{ui, AppModel};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use rust_i18n::t;
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

    // Debug message for plugin development
    DebugPluginResponse(String),

    EditTimeEntry,
    EditTimeEntryCancel,
    EditTimeEntryFieldClick(crate::model::EditField),
    EditTimeEntryKeyPress(KeyEvent),
    EditTimeEntryNextField,
    EditTimeEntryPreviousField,
    EditTimeEntrySelectContact,
    EditTimeEntrySelectProject,
    
    // EditSave and EditCancel are used for both regular edit and import
    EditSave,
    EditCancel,

    ExecuteDeleteTimeEntry(String),
    ExecuteExport,

    None, // Placeholder for no action needed

    PluginViewActivate, // Activate the plugin view
    PluginViewShow,
    PluginViewHide,
    PluginViewSelectNext,
    PluginViewSelectPrevious,
    PluginViewSelectRow(usize),

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

    // Import a plugin time entry to Moneybird
    ImportTimeEntry,
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
                        model.log_debug(t!("event_ignore_modal_cooldown"));
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
                model.log_debug(t!(
                    "event_debounce_timeout_reached",
                    field = format!("{:?}", model.edit_state.selected_field)
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
        model.log_notice(t!("event_toggle_log_panel"));
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

    // --- Plugin View Mode Handling ---
    if model.plugin_view_state.active {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => return Some(Message::PluginViewSelectPrevious),
            KeyCode::Down | KeyCode::Char('j') => return Some(Message::PluginViewSelectNext),
            KeyCode::Esc | KeyCode::Char('p') => return Some(Message::PluginViewHide),
            KeyCode::Char('q') => return Some(Message::Quit), // Allow quitting
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Debug selected plugin
                if let Some(idx) = model.plugin_view_state.selected_index {
                    let plugins = if let Some(manager) = &model.plugin_manager {
                        manager.list_plugins()
                    } else {
                        vec![]
                    };
                    
                    if idx < plugins.len() {
                        let plugin_name = plugins[idx].name.clone();
                        return Some(Message::DebugPluginResponse(plugin_name));
                    }
                }
                return None;
            },
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
                    model.log_debug(t!(
                        "event_confirming_modal",
                        modal_id = modal_id,
                        key = format!("{:?}", key)
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
                    model.log_debug(t!(
                        "event_cancelling_modal",
                        modal_id = modal_id,
                        key = format!("{:?}", key)
                    ));
                    return Some(Message::DismissModal(modal_id, true));
                }
                return Some(Message::DismissModal(String::new(), true));
            }
            KeyCode::Char(' ') | KeyCode::Tab | KeyCode::BackTab => {
                if let Some((modal_id, _)) = modal_info {
                    model.log_debug(t!(
                        "event_dismissing_modal",
                        modal_id = modal_id,
                        key = format!("{:?}", key)
                    ));
                    return Some(Message::DismissModal(modal_id, false));
                }
                return Some(Message::DismissModal(String::new(), false));
            }
            _ => return None,
        }
    }

    // --- Refactored Edit State Key Handling (Regular Edit or Import Edit) ---
    if model.edit_state.active {
        // Check if we're in import mode
        let is_import = model.edit_state.is_import_mode();
        let edit_state = &model.edit_state;

        match key.code {
            // --- Global Edit Keys ---
            KeyCode::Char('s') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                Some(Message::EditSave)
            }
            KeyCode::Enter => {
                // Check if we are in an autocomplete field with the dropdown visible
                match edit_state.selected_field {
                    crate::model::EditField::Project => {
                        if edit_state.project_autocomplete.is_dropdown_visible {
                            Some(Message::AutocompleteSelect)
                        } else {
                            Some(Message::EditSave) // Save if dropdown not visible
                        }
                    }
                    crate::model::EditField::Contact => {
                        if edit_state.contact_autocomplete.is_dropdown_visible {
                            Some(Message::AutocompleteSelect)
                        } else {
                            Some(Message::EditSave) // Save if dropdown not visible
                        }
                    }
                    _ => Some(Message::EditSave), // Save for all other fields
                }
            }
            KeyCode::Tab => Some(Message::EditTimeEntryNextField),
            KeyCode::BackTab => Some(Message::EditTimeEntryPreviousField),

            // --- Keys with Field-Dependent Behavior ---
            KeyCode::Up => {
                match edit_state.selected_field {
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        let is_dropdown_visible = if edit_state.selected_field == crate::model::EditField::Project {
                            edit_state.project_autocomplete.is_dropdown_visible
                        } else {
                            edit_state.contact_autocomplete.is_dropdown_visible
                        };
                        if is_dropdown_visible {
                            Some(Message::AutocompletePreviousItem)
                        } else {
                            None // No action if dropdown not visible
                        }
                    }
                    _ => Some(Message::EditTimeEntryKeyPress(key)),
                }
            }
            KeyCode::Down => {
                 match edit_state.selected_field {
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        let is_dropdown_visible = if edit_state.selected_field == crate::model::EditField::Project {
                            edit_state.project_autocomplete.is_dropdown_visible
                        } else {
                            edit_state.contact_autocomplete.is_dropdown_visible
                        };
                        if is_dropdown_visible {
                            Some(Message::AutocompleteNextItem)
                        } else {
                             // Maybe trigger refresh if dropdown not visible and input exists?
                             // For now, do nothing.
                            None
                        }
                    }
                    _ => Some(Message::EditTimeEntryKeyPress(key)),
                }
            }
            KeyCode::Esc => Some(Message::EditCancel),
            KeyCode::Char('u') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                match edit_state.selected_field {
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        Some(Message::AutocompleteClearInput)
                    }
                    _ => Some(Message::EditTimeEntryKeyPress(key)),
                }
            }
            KeyCode::Char(_) | KeyCode::Backspace => {
                 match edit_state.selected_field {
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        Some(Message::AutocompleteKeyPress(key))
                    }
                    _ => Some(Message::EditTimeEntryKeyPress(key)),
                }
            }
            // Catch-all for other keys
            _ => {
                 match edit_state.selected_field {
                    crate::model::EditField::Project | crate::model::EditField::Contact => {
                        model.log_debug(t!("event_ignoring_unhandled_key", key = format!("{:?}", key.code)));
                        None
                    }
                    _ => Some(Message::EditTimeEntryKeyPress(key)), // Pass to editor for other fields
                }
            }
        }
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
                KeyCode::Char('p') => Some(Message::PluginViewShow),
                KeyCode::Char('i') => Some(Message::ImportTimeEntry),
                KeyCode::Char('q') => Some(Message::Quit),
                KeyCode::Char('e') | KeyCode::Char(' ') | KeyCode::Enter => Some(Message::EditTimeEntry),
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

        // Get the field areas from the edit state
        let field_areas = &model.edit_state.field_areas;

        // Check if click is on any of the stored field areas
        for (&field, &area) in field_areas {
            if area.contains(mouse_pos) {
                model.log_debug(t!(
                    "update_debug_click_field",
                    field = format!("{:?}", field)
                ));
                return Some(Message::EditTimeEntryFieldClick(field));
            }
        }

        // Click was in edit mode but not on any field
        return None;
    }

    // Handle plugin view mouse events
    if model.plugin_view_state.active {
        match mouse.kind {
            MouseEventKind::ScrollDown => return Some(Message::PluginViewSelectPrevious),
            MouseEventKind::ScrollUp => return Some(Message::PluginViewSelectNext),
            MouseEventKind::Down(event::MouseButton::Left) => {
                if let Some(list_area) = model.plugin_list_area {
                    if list_area.contains(ratatui::layout::Position { x: mouse.column, y: mouse.row }) {
                        // Adjust for list border/padding if necessary (assuming 1 row top border/padding)
                        let relative_row = mouse.row.saturating_sub(list_area.y + 1);
                        // Calculate index based on scroll offset and relative row
                        let selected_index = model.plugin_view_state.plugin_list_state.offset() + relative_row as usize;

                        // TODO: Check if index is within bounds (requires knowing list length here)
                        //       We might need to pass list length or handle bounds check in update.rs
                        model.log_debug(t!(
                            "event_mouse_click_detected_plugin",
                            row = relative_row.to_string(),
                            index = selected_index.to_string()
                        ));
                        return Some(Message::PluginViewSelectRow(selected_index));
                    }
                }
                return None; // Click outside list area
            }
            _ => return None, // Ignore other mouse events in plugin view
        }
    }

    // Handle user selection view mouse events
    if model.user_selection_active {
        match mouse.kind {
            MouseEventKind::ScrollDown => return Some(Message::UserSelectPrevious),
            MouseEventKind::ScrollUp => return Some(Message::UserSelectNext),
            MouseEventKind::Down(event::MouseButton::Left) => {
                return None;
            }
            _ => return None, // Ignore other mouse events in user selection view
        }
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
                        model.log_debug(t!(
                            "event_mouse_click_detected",
                            row = relative_row.to_string(),
                            index = selected_index.to_string()
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
