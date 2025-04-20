use crossterm::event::KeyCode;
use ratatui::style::Style;
use rust_i18n::t;
use tui_textarea::TextArea;

use crate::{
    api,
    api::{get_contacts_by_query, get_time_entries},
    config, datetime,
    event::Message,
    file,
    model::{AppModel, AutocompleteState, EditState, ImportState, TimeEntryForTable},
    ui::{self},
    RunningState,
};

// Import the EditField enum
use crate::model::EditField;

// --- Helper Functions for Selection Logic ---

/// Calculates the next selection index, preventing wrapping.
fn calculate_next_index(current_index_opt: Option<usize>, count: usize) -> Option<usize> {
    if count == 0 {
        return None; // No items to select
    }
    let current_index = current_index_opt.unwrap_or(0);
    if current_index < count - 1 {
        Some(current_index + 1)
    } else {
        current_index_opt // Stay at the last item
    }
}

/// Calculates the previous selection index, preventing wrapping.
fn calculate_previous_index(current_index_opt: Option<usize>, _count: usize) -> Option<usize> {
    let current_index = current_index_opt.unwrap_or(0);
    if current_index > 0 {
        Some(current_index - 1)
    } else {
        current_index_opt // Stay at the first item
    }
}

/// Validates if a given index is within the bounds of the item count.
fn validate_row_index(index: usize, count: usize) -> Option<usize> {
    if count > 0 && index < count {
        Some(index)
    } else {
        None
    }
}

// Helper function to update the EditState field based on the editor content
fn update_edit_field_from_editor(edit_state: &mut EditState) {
    match edit_state.selected_field {
        crate::model::EditField::Description => {
            let lines = edit_state.editor.lines();
            if !lines.is_empty() {
                edit_state.description = lines.join("\n");
            }
        }
        crate::model::EditField::StartDate => {
            if let Some(line) = edit_state.editor.lines().first() {
                edit_state.start_date = line.clone();
            }
        }
        crate::model::EditField::StartTime => {
            if let Some(line) = edit_state.editor.lines().first() {
                edit_state.start_time = line.clone();
            }
        }
        crate::model::EditField::EndDate => {
            if let Some(line) = edit_state.editor.lines().first() {
                edit_state.end_date = line.clone();
            }
        }
        crate::model::EditField::EndTime => {
            if let Some(line) = edit_state.editor.lines().first() {
                edit_state.end_time = line.clone();
            }
        }
        _ => {} // Other fields don't use the editor directly for updates in this way
    }
}

// Helper function to initialize the shared editor or autocomplete state for the selected field
fn initialize_editor_or_autocomplete(edit_state: &mut EditState) {
    edit_state.editor = TextArea::default(); // Clear editor for text fields
    match edit_state.selected_field {
        crate::model::EditField::Description => {
            edit_state.editor.insert_str(&edit_state.description);
        }
        crate::model::EditField::StartDate => {
            edit_state.editor.insert_str(&edit_state.start_date);
        }
        crate::model::EditField::StartTime => {
            edit_state.editor.insert_str(&edit_state.start_time);
        }
        crate::model::EditField::EndDate => {
            edit_state.editor.insert_str(&edit_state.end_date);
        }
        crate::model::EditField::EndTime => {
            edit_state.editor.insert_str(&edit_state.end_time);
        }
        crate::model::EditField::Project => {
            // Reset autocomplete state and clear input for new search
            edit_state.project_autocomplete.items.clear();
            edit_state.project_autocomplete.is_dropdown_visible = false;
            edit_state.project_autocomplete.list_state.select(None);
            edit_state.project_autocomplete.clear_input();
        }
        crate::model::EditField::Contact => {
            // Reset autocomplete state and clear input for new search
            edit_state.contact_autocomplete.items.clear();
            edit_state.contact_autocomplete.is_dropdown_visible = false;
            edit_state.contact_autocomplete.list_state.select(None);
            edit_state.contact_autocomplete.clear_input();
        }
    }
}

// Helper function to handle keypresses for autocomplete fields
fn handle_autocomplete_keypress<T: Clone>(
    autocomplete_state: &mut AutocompleteState<T>,
    key_code: KeyCode,
) {
    match key_code {
        KeyCode::Char(c) => {
            autocomplete_state.add_char(c);
            autocomplete_state.record_keypress();
        }
        KeyCode::Backspace => {
            autocomplete_state.remove_char();
            autocomplete_state.record_keypress();
        }
        _ => {} // Ignore other keys like arrows, enter, etc. here
    }
}

// Helper function to handle next/previous item selection in autocomplete
fn handle_autocomplete_navigation<T: Clone>(
    autocomplete_state: &mut AutocompleteState<T>,
    next: bool,
) {
    if next {
        autocomplete_state.select_next();
    } else {
        autocomplete_state.select_previous();
    }
}

// Helper function to handle clearing autocomplete input
fn handle_autocomplete_clear<T: Clone>(
    autocomplete_state: &mut AutocompleteState<T>,
) -> Option<Message> {
    autocomplete_state.clear_input();
    autocomplete_state.record_keypress(); // Record keypress after clearing
    Some(Message::AutocompleteRefresh) // Trigger refresh after clearing
}

// Helper function to handle selecting a project from autocomplete
fn handle_autocomplete_select_project(model: &mut AppModel) {
    // Determine which edit state to modify
    let is_import = model.import_state.active;
    let edit_state = if is_import {
        &mut model.import_state.edit_state
    } else {
        &mut model.edit_state
    };

    if let Some(selected_project) = edit_state.project_autocomplete.selected_item() {
        // Add selected project to the main project list if it's not already there
        if !model.projects.iter().any(|p| p.id == selected_project.id) {
            model.projects.push(selected_project.clone());
        }

        let project_id = selected_project.id.clone();
        let project_name = selected_project.name.clone().unwrap_or_default();

        // Update the correct edit state
        edit_state.project_id = project_id;
        edit_state.project_name = project_name.clone(); // Update display name too
        let autocomplete_state = &mut edit_state.project_autocomplete;
        autocomplete_state.input = project_name.clone();
        autocomplete_state.mark_searched(); // Prevent re-search
        autocomplete_state.is_dropdown_visible = false;
        autocomplete_state.items.clear();
        model.log_notice(t!("update_selected_project", project_name = project_name));
    }
}

// Helper function to handle selecting a contact from autocomplete
fn handle_autocomplete_select_contact(model: &mut AppModel) {
    // Determine which edit state to modify
    let is_import = model.import_state.active;
    let edit_state = if is_import {
        &mut model.import_state.edit_state
    } else {
        &mut model.edit_state
    };

    if let Some(selected_contact) = edit_state.contact_autocomplete.selected_item() {
        // Add selected contact to the main contact list if it's not already there
        if !model.contacts.iter().any(|c| c.id == selected_contact.id) {
            model.contacts.push(selected_contact.clone());
        }

        let contact_id = selected_contact.id.clone();
        let contact_name = selected_contact.company_name.clone().unwrap_or_default();

        // Update the correct edit state
        edit_state.contact_id = contact_id;
        edit_state.contact_name = contact_name.clone(); // Update display name too
        let autocomplete_state = &mut edit_state.contact_autocomplete;
        autocomplete_state.input = contact_name.clone();
        autocomplete_state.mark_searched(); // Prevent re-search
        autocomplete_state.is_dropdown_visible = false;
        autocomplete_state.items.clear();
        model.log_notice(t!("update_selected_contact", contact_name = contact_name));
    }
}

// Helper function to refresh project autocomplete suggestions (local filter)
async fn handle_autocomplete_refresh_project(model: &mut AppModel) -> Option<Message> {
    // Determine which edit state to use
    let is_import = model.import_state.active;
    let (query, min_chars) = {
        let edit_state = if is_import {
            &mut model.import_state.edit_state
        } else {
            &mut model.edit_state
        };
        (
            edit_state.project_autocomplete.input.clone(),
            edit_state.project_autocomplete.min_chars_to_search,
        )
    }; // edit_state borrow ends here

    if query.is_empty() || query.len() < min_chars {
        // Re-borrow edit_state mutably for update
        let edit_state = if is_import {
            &mut model.import_state.edit_state
        } else {
            &mut model.edit_state
        };
        edit_state.project_autocomplete.update_items(vec![]);
        // Borrow model immutably for logging after mutable borrow is released
        model.log_debug(format!(
            "Query '{}' too short or empty, cleared project items.",
            query
        ));
    } else {
        // Mark searched in a separate mutable borrow scope
        {
            let edit_state = if is_import {
                &mut model.import_state.edit_state
            } else {
                &mut model.edit_state
            };
            edit_state.project_autocomplete.mark_searched();
        } // edit_state borrow ends here

        // Borrow model immutably for logging
        model.log_debug(t!("update_filtering_local_projects", query = query));

        // Use the main model.projects list for filtering (immutable borrow)
        let filtered_projects: Vec<_> = model
            .projects
            .iter()
            .filter(|p| {
                if let Some(name) = &p.name {
                    name.to_lowercase().contains(&query.to_lowercase())
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        let items_count = filtered_projects.len();

        // Update items in a separate mutable borrow scope
        {
            let edit_state = if is_import {
                &mut model.import_state.edit_state
            } else {
                &mut model.edit_state
            };
            edit_state
                .project_autocomplete
                .update_items(filtered_projects);
        } // edit_state borrow ends here

        // Log after the mutable borrow is released
        model.log_debug(format!(
            "Local filter found {} projects.",
            items_count // Use the stored count
        ));
    }

    None // Return None as we've already updated the state
}

// Helper function to refresh contact autocomplete suggestions (API call)
async fn handle_autocomplete_refresh_contact(model: &mut AppModel) -> Option<Message> {
    // Determine which edit state to use
    let is_import = model.import_state.active;
    let (query, min_chars) = {
        let edit_state = if is_import {
            &mut model.import_state.edit_state
        } else {
            &mut model.edit_state
        };
        (
            edit_state.contact_autocomplete.input.clone(),
            edit_state.contact_autocomplete.min_chars_to_search,
        )
    }; // edit_state borrow ends here

    if query.is_empty() || query.len() < min_chars {
        // Re-borrow edit_state mutably for update
        {
            let edit_state = if is_import {
                &mut model.import_state.edit_state
            } else {
                &mut model.edit_state
            };
            edit_state.contact_autocomplete.update_items(vec![]);
        } // edit_state borrow ends here

        // Borrow model immutably for logging after mutable borrow is released
        model.log_debug(format!(
            "Query '{}' too short or empty, cleared contact items.",
            query
        ));
    } else {
        // Set loading and mark searched in a separate mutable borrow scope
        {
            let edit_state = if is_import {
                &mut model.import_state.edit_state
            } else {
                &mut model.edit_state
            };
            edit_state.contact_autocomplete.is_loading = true;
            edit_state.contact_autocomplete.mark_searched();
        } // edit_state borrow ends here

        // Log before API call (immutable borrow)
        model.log_debug(format!(
            "Calling API to search contacts for query: '{}'",
            query
        ));

        // API call parameters remain the same (immutable borrows of model parts)
        let client = model.client.clone();
        let admin_id = model.administration.id.clone().unwrap_or_default();

        // Perform the API call
        let api_result = get_contacts_by_query(&client, &admin_id, &query).await;

        // Process result and update state in a new mutable borrow scope
        let mut items_count = 0;
        match api_result {
            Ok(contacts) => {
                items_count = contacts.len();
                // Update the correct edit state
                {
                    let edit_state = if is_import {
                        &mut model.import_state.edit_state
                    } else {
                        &mut model.edit_state
                    };
                    edit_state.contact_autocomplete.update_items(contacts);
                } // edit_state borrow ends here

                // Log success (immutable borrow)
                model.log_debug(format!(
                    "API search succeeded. Found {} contacts.",
                    items_count
                ));
            }
            Err(err) => {
                let error_msg =
                    t!("update_api_search_contacts_failed", error = err.to_string()).to_string();
                // Show error uses immutable borrow temporarily
                ui::show_error(model, error_msg.clone());
                // Update the correct edit state
                {
                    let edit_state = if is_import {
                        &mut model.import_state.edit_state
                    } else {
                        &mut model.edit_state
                    };
                    edit_state.contact_autocomplete.update_items(vec![]);
                } // edit_state borrow ends here
            }
        }
        // Update loading state in a final mutable borrow scope
        {
            let edit_state = if is_import {
                &mut model.import_state.edit_state
            } else {
                &mut model.edit_state
            };
            edit_state.contact_autocomplete.is_loading = false;
        } // edit_state borrow ends here
    }

    None // Return None as we've already updated the state
}

// Helper function to handle exporting time entries to CSV
fn handle_export(model: &mut AppModel) {
    if model.time_entries_for_table.is_empty() {
        ui::show_error(model, t!("no_time_entries_to_export").to_string());
        return;
    }

    let filename = file::generate_export_filename(model, None);
    model.log_debug(t!("update_log_starting_export").to_string());

    match file::export_time_entries_to_csv(model, &filename) {
        Ok(()) => {
            model.log_debug(t!("update_log_export_success_modal").to_string());
            ui::show_info(
                model,
                "export_success",
                t!("export_success").to_string(),
                t!("update_export_success", filename = filename).to_string(),
            );
        }
        Err(err) => {
            model.log_error(format!("Export failed: {}", err));
            ui::show_error(
                model,
                t!("update_failed_to_export", error = err.to_string()),
            );
        }
    }
}

// Helper function to handle common logic for closing modals (confirm/dismiss)
fn handle_modal_close(
    model: &mut AppModel,
    modal_id_from_message: String,
    is_confirm: bool,
) -> Option<Message> {
    if model.modal_stack.is_empty() {
        return None;
    }

    let mut next_message_candidate: Option<Message> = None;
    let mut should_pop = false;

    // Phase 1: Check condition using immutable borrow
    if let Some(modal) = model.modal_stack.top() {
        let current_modal_id = modal.id.clone().unwrap_or_default();
        if modal_id_from_message.is_empty() || modal_id_from_message == current_modal_id {
            // ID matches, determine potential next message and mark for popping
            should_pop = true;
            next_message_candidate = if is_confirm {
                modal.on_confirm.clone()
            } else {
                modal.on_cancel.clone()
            };
        }
    } // Immutable borrow of model.modal_stack ends here

    // Phase 2: Perform mutable actions if needed
    if should_pop {
        model.modal_stack.pop(); // Now it's safe to pop

        // Reset style only if the stack is now empty
        if model.modal_stack.is_empty() {
            model.appearance.default_style =
                Style::default().fg(model.appearance.default_foreground_color_indexed);
        }

        // Process the next message candidate
        match next_message_candidate {
            Some(Message::TimeEntryRefresh) => {
                // Return Refresh for the main loop to handle asynchronously
                Some(Message::TimeEntryRefresh)
            }
            Some(Message::ExecuteExport) => {
                // Call the export handler directly for synchronous action
                handle_export(model);
                None // handle_export shows its own modals
            }
            // Any other message returned from on_confirm/on_cancel
            other_message => other_message,
        }
    } else {
        // ID didn't match or stack became empty between checks (unlikely but safe)
        None
    }
}

/// Initialize an import from a plugin time entry to Moneybird
fn initialize_time_entry_import(model: &mut AppModel) -> Option<Message> {
    // Make sure we have a selected entry
    if model.time_entries_for_table.is_empty() || model.time_entry_table_state.selected().is_none()
    {
        return None;
    }

    let selected_idx = model.time_entry_table_state.selected().unwrap();
    // Clone the entry to avoid borrowing conflicts
    let selected_entry = model.time_entries_for_table[selected_idx].clone();

    // Only allow importing of plugin entries (not Moneybird entries)
    if selected_entry.source.to_lowercase() == "moneybird" {
        model.log_notice(t!("update_cant_import_moneybird_entry"));
        return None;
    }

    // Log that we're importing the entry
    model.log_notice(t!(
        "update_importing_time_entry",
        description = selected_entry.description.clone(),
        source = selected_entry.source.clone()
    ));

    // Store the original entry in the import state
    model.import_state.original_entry = Some(selected_entry.clone());

    // Initialize edit state for the import
    let mut edit_state = EditState::default();
    edit_state.active = true;
    edit_state.is_create_mode = true; // Import is technically creating a new entry in MB
    edit_state.description = selected_entry.description.clone();
    // Initialize names here, might be overwritten if match found
    edit_state.project_name = selected_entry.project.clone();
    edit_state.contact_name = selected_entry.customer.clone();
    edit_state.time_entry_id = None; // Always creating a new entry on import

    // Parse and set date/time fields from the original entry
    let admin_timezone = model
        .administration
        .time_zone
        .clone()
        .unwrap_or_else(|| "UTC".to_string());

    if let (Some(start_date), Some(start_time)) =
        datetime::parse_datetime_for_edit(&selected_entry.started_at, &admin_timezone)
    {
        edit_state.start_date = start_date;
        edit_state.start_time = start_time;
    }

    if let (Some(end_date), Some(end_time)) =
        datetime::parse_datetime_for_edit(&selected_entry.ended_at, &admin_timezone)
    {
        edit_state.end_date = end_date;
        edit_state.end_time = end_time;
    }

    // Set up editor with the description text
    edit_state.editor = TextArea::new(vec![edit_state.description.clone()]);
    edit_state.selected_field = EditField::Description;

    // Try to match the contact with Moneybird entities
    if !model.contacts.is_empty() {
        let contact_name_lower = selected_entry.customer.to_lowercase();
        for contact in &model.contacts {
            let name = crate::ui::format_contact_name(contact).to_lowercase();
            if name == contact_name_lower
                || name.contains(&contact_name_lower)
                || contact_name_lower.contains(&name)
            {
                let matched_name = crate::ui::format_contact_name(contact);
                edit_state.contact_id = contact.id.clone();
                edit_state.contact_name = matched_name.clone();
                // Set autocomplete input and mark as searched
                edit_state.contact_autocomplete.input = matched_name.clone();
                edit_state.contact_autocomplete.mark_searched();
                model.log_notice(format!("Matched contact: {}", edit_state.contact_name));
                break;
            }
        }
    }

    // Try to match the project with Moneybird entities
    if !model.projects.is_empty() {
        let project_name_lower = selected_entry.project.to_lowercase();
        for project in &model.projects {
            let name = project.name.clone().unwrap_or_default().to_lowercase();
            if name == project_name_lower
                || name.contains(&project_name_lower)
                || project_name_lower.contains(&name)
            {
                let matched_name = project.name.clone().unwrap_or_default();
                edit_state.project_id = project.id.clone();
                edit_state.project_name = matched_name.clone();
                // Set autocomplete input and mark as searched
                edit_state.project_autocomplete.input = matched_name.clone();
                edit_state.project_autocomplete.mark_searched();
                model.log_notice(format!("Matched project: {}", edit_state.project_name));
                break;
            }
        }
    }

    // Show notifications for unmatched entities (after attempting matches)
    if edit_state.contact_id.is_none() {
        ui::show_error(
            model,
            t!(
                "update_no_contact_match",
                contact_name = selected_entry.customer.clone()
            ),
        );
    }

    if edit_state.project_id.is_none() {
        ui::show_error(
            model,
            t!(
                "update_no_project_match",
                project_name = selected_entry.project.clone()
            ),
        );
    }

    // Set the edit state in the import state
    model.import_state.edit_state = edit_state;
    model.import_state.active = true;

    None
}

/// Check if the model's import state is active
fn is_import_active(model: &AppModel) -> bool {
    model.import_state.active
}

// --- Helper function to get the active mutable EditState ---
fn get_active_edit_state_mut(model: &mut AppModel) -> Option<&mut EditState> {
    if model.import_state.active {
        Some(&mut model.import_state.edit_state)
    } else if model.edit_state.active {
        Some(&mut model.edit_state)
    } else {
        None
    }
}

/// Process a message and update the model state
pub(crate) async fn update(model: &mut AppModel, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running_state = RunningState::Done;
            None
        }

        // --- Time Entry Navigation/Refresh ---
        Message::TimeEntryPreviousWeek => {
            model.week_offset -= 1;
            model.log_notice(format!(
                "Navigating to previous week (offset: {})",
                model.week_offset
            ));
            Some(Message::TimeEntryRefresh)
        }
        Message::TimeEntryNextWeek => {
            model.week_offset += 1;
            model.log_notice(format!(
                "Navigating to next week (offset: {})",
                model.week_offset
            ));
            Some(Message::TimeEntryRefresh)
        }
        Message::TimeEntryCurrentWeek => {
            model.week_offset = 0;
            model.log_notice(t!("update_log_navigating_current_week").to_string());
            Some(Message::TimeEntryRefresh)
        }
        Message::TimeEntryRefresh => {
            model.log_notice(t!("update_log_manual_refresh").to_string());
            get_time_entries(model).await;
            model.log_success(t!("update_time_entries_refreshed").to_string());
            None
        }
        Message::TimeEntrySelectNext => {
            let count = model.time_entries_for_table.len();
            if let Some(next_index) =
                calculate_next_index(model.time_entry_table_state.selected(), count)
            {
                model.time_entry_table_state.select(Some(next_index));
            }
            None
        }
        Message::TimeEntrySelectPrevious => {
            let count = model.time_entries_for_table.len();
            if let Some(prev_index) =
                calculate_previous_index(model.time_entry_table_state.selected(), count)
            {
                model.time_entry_table_state.select(Some(prev_index));
            }
            None
        }
        Message::TimeEntrySelectRow(index) => {
            let count = model.time_entries_for_table.len();
            if let Some(valid_index) = validate_row_index(index, count) {
                model.time_entry_table_state.select(Some(valid_index));
            } else {
                model.log_warning(format!(
                    "TimeEntrySelectRow: Index {} out of bounds ({})",
                    index, count
                ));
            }
            None
        }
        Message::TimeEntrySearchShow => {
            model.search_state.active = true;
            model.search_state.text_input = TextArea::default();
            None
        }
        Message::TimeEntrySearchHide => {
            model.search_state.active = false;
            model.time_entries_for_table = model.time_entries_for_table_backup.clone();
            model.ensure_valid_selection();
            None
        }
        Message::TimeEntryClearSearch => {
            if model.search_state.active {
                model.search_state.text_input = TextArea::default();
                model.filter_items();
            }
            None
        }
        Message::TimeEntrySearchKeyPress(key) => {
            if model.search_state.active {
                model.search_state.text_input.input(key);
                model.filter_items();
            }
            None
        }
        Message::TimeEntryCreate => {
            model.log_notice(t!("update_log_initiating_create").to_string());
            model.edit_state = EditState::default();
            model.edit_state.active = true;
            model.edit_state.time_entry_id = None;
            model.edit_state.is_create_mode = true;
            model.edit_state.editor = TextArea::default();
            model.edit_state.selected_field = crate::model::EditField::Description;
            let admin_timezone_str = model
                .administration
                .time_zone
                .clone()
                .unwrap_or_else(|| "UTC".to_string());
            let admin_tz = admin_timezone_str
                .parse::<chrono_tz::Tz>()
                .unwrap_or(chrono_tz::UTC);
            let now = chrono::Utc::now().with_timezone(&admin_tz);
            model.edit_state.start_date = now.format("%Y-%m-%d").to_string();
            model.edit_state.start_time = now.format("%H:%M").to_string();
            let end_time_default = now + chrono::Duration::hours(1);
            model.edit_state.end_date = end_time_default.format("%Y-%m-%d").to_string();
            model.edit_state.end_time = end_time_default.format("%H:%M").to_string();
            initialize_editor_or_autocomplete(&mut model.edit_state);
            None
        }
        Message::TimeEntryExport => {
            if !model.time_entries_for_table.is_empty() {
                ui::show_confirmation(
                    model,
                    t!("export").to_string(),
                    t!("do_you_want_to_export_the_selected_time_entries_to_csv").to_string(),
                    Some(Message::ExecuteExport),
                    None,
                );
            } else {
                ui::show_error(model, t!("no_time_entries_to_export").to_string());
            }
            None
        }
        Message::TimeEntryDelete => {
            if let Some(selected_idx) = model.time_entry_table_state.selected() {
                if selected_idx < model.time_entries_for_table.len() {
                    let entry_description = model.time_entries_for_table[selected_idx]
                        .description
                        .clone();
                    let entry_id = model.time_entries_for_table[selected_idx].id.clone();

                    ui::show_confirmation(
                        model,
                        t!("delete_time_entry").to_string(),
                        format!(
                            "{}\n\"{}\"?",
                            t!("confirm_delete_prompt"),
                            entry_description
                        ),
                        Some(Message::ExecuteDeleteTimeEntry(entry_id)),
                        None,
                    );
                }
            }
            None
        }

        Message::TimeEntrySearchKeyPress(key) => {
            if model.search_state.active {
                model.search_state.text_input.input(key);
                model.filter_items();
            }
            None
        }

        Message::EditTimeEntry => {
            if let Some(selected_idx) = model.time_entry_table_state.selected() {
                if selected_idx < model.time_entries_for_table.len() {
                    let selected_entry = &model.time_entries_for_table[selected_idx];
                    let original_entry = model
                        .time_entries
                        .iter()
                        .find(|e| e.id.clone().unwrap_or_default() == selected_entry.id)
                        .cloned();

                    if let Some(orig_entry) = original_entry {
                        let admin_timezone_str = model
                            .administration
                            .time_zone
                            .clone()
                            .unwrap_or_else(|| "UTC".to_string());
                        let start_datetime = datetime::parse_iso_datetime(
                            &selected_entry.started_at,
                            &admin_timezone_str,
                        );
                        let end_datetime = datetime::parse_iso_datetime(
                            &selected_entry.ended_at,
                            &admin_timezone_str,
                        );

                        if let (Some(start), Some(end)) = (start_datetime, end_datetime) {
                            model.edit_state.active = true;
                            model.edit_state.time_entry_id = Some(selected_entry.id.clone());

                            let start_date_str = start.format("%Y-%m-%d").to_string();
                            let start_time_str = start.format("%H:%M").to_string();
                            let end_date_str = end.format("%Y-%m-%d").to_string();
                            let end_time_str = end.format("%H:%M").to_string();

                            model.edit_state.description = selected_entry.description.clone();
                            model.edit_state.start_date = start_date_str;
                            model.edit_state.start_time = start_time_str;
                            model.edit_state.end_date = end_date_str;
                            model.edit_state.end_time = end_time_str;

                            model.edit_state.project_id = orig_entry.project_id.clone();
                            model.edit_state.project_name = orig_entry
                                .project
                                .clone()
                                .unwrap_or_default()
                                .name
                                .clone()
                                .unwrap_or_default();
                            model.edit_state.contact_id = orig_entry.contact_id.clone();
                            model.edit_state.contact_name = orig_entry
                                .contact
                                .clone()
                                .unwrap_or_default()
                                .company_name
                                .clone()
                                .unwrap_or_default();

                            model.edit_state.editor = TextArea::default();
                            model
                                .edit_state
                                .editor
                                .insert_str(&model.edit_state.description);
                            model.edit_state.selected_field = crate::model::EditField::Description;
                            model.log_notice(t!(
                                "update_editing_time_entry",
                                entry_id = selected_entry.id.clone()
                            ));
                        } else {
                            model.log_error(t!("failed_to_parse_time_entry_dates").to_string());
                        }
                    } else {
                        model.log_error(t!("could_not_find_original_entry").to_string());
                    }
                }
            }
            None
        }

        Message::EditTimeEntryCancel => {
            if model.edit_state.active {
                model.edit_state.active = false;
                model.log_notice(t!("canceled_editing").to_string());
            }
            None
        }
        Message::EditSave => {
            let was_import = model.import_state.active;
            let mut next_message: Option<Message> = None;

            // Get timezone string outside the mutable borrow scope
            let admin_timezone_str = model
                .administration
                .time_zone
                .clone()
                .unwrap_or_else(|| "UTC".to_string());

            let mut prepared_data: Option<(
                crate::moneybird::types::TimeEntry,
                bool,
                Option<String>,
                String,
            )> = None;
            let mut validation_failed = false;

            // --- Scope for mutable borrow of edit_state ---
            {
                if let Some(edit_state) = get_active_edit_state_mut(model) {
                    update_edit_field_from_editor(edit_state);

                    // Check description validity first
                    if edit_state.description.is_empty() {
                        validation_failed = true;
                    } else {
                        // If valid, prepare the data within this scope
                        let data = edit_state.try_into_time_entry(&admin_timezone_str); // Use pre-fetched timezone
                        prepared_data = Some((
                            data,
                            edit_state.is_create_mode,
                            edit_state.time_entry_id.clone(),
                            edit_state.description.clone(),
                        ));
                    }
                } else {
                    // No active edit state, nothing to save
                    return None;
                }
            } // Mutable borrow of edit_state (and model) ends here

            // --- Handle validation failure outside the borrow scope ---
            if validation_failed {
                ui::show_error(model, t!("update_description_required"));
                return None;
            }

            // --- Proceed only if data was prepared successfully ---
            if let Some((time_entry_data, is_creating, entry_id_opt, description)) = prepared_data {
                // Get immutable borrows or clones needed for API call
                let admin_id = model.administration.id.clone().unwrap_or_default();
                let client = model.client.clone();
                let user_id = model.config.user_id.clone().unwrap_or_default();

                if is_creating {
                    // Log using immutable borrow of model
                    model.log_notice(format!("Creating new time entry: {}", description));
                    let endpoint = "time_entries.json";
                    crate::api::log_debug_curl(model, endpoint, "POST");

                    // API Call uses cloned client
                    match crate::api::create_time_entry(
                        &client,
                        &admin_id,
                        &user_id,
                        time_entry_data,
                    )
                    .await
                    {
                        Ok(_created_entry) => {
                            let success_msg = if was_import {
                                t!("update_import_success")
                            } else {
                                t!("time_entry_was_created_successfully")
                            };
                            model.log_success(success_msg.clone());
                            ui::show_info(
                                model,
                                "create_success",
                                t!("success").to_string(),
                                success_msg.to_string(),
                            );
                            next_message = Some(Message::TimeEntryRefresh);
                        }
                        Err(err) => {
                            let error_msg =
                                t!("update_failed_create_time_entry", error = err.to_string())
                                    .to_string();
                            model.log_error(error_msg.clone());
                            ui::show_error(model, error_msg);
                            return None;
                        }
                    }
                } else if let Some(entry_id) = entry_id_opt {
                    model.log_notice(t!(
                        "updating_time_entry_notice",
                        entry_id = entry_id.clone(),
                        description = description
                    ));
                    let endpoint = format!("time_entries/{}.json", entry_id);
                    crate::api::log_debug_curl(model, &endpoint, "PATCH");

                    match crate::api::update_time_entry_by_id(
                        &client,
                        &admin_id,
                        &entry_id,
                        time_entry_data,
                    )
                    .await
                    {
                        Ok(_updated_entry) => {
                            model.log_success(t!(
                                "time_entry_updated_successfully",
                                id = entry_id.clone()
                            ));
                            ui::show_info(
                                model,
                                "update_success",
                                t!("success").to_string(),
                                t!("time_entry_was_updated_successfully").to_string(),
                            );
                            next_message = Some(Message::TimeEntryRefresh);
                        }
                        Err(err) => {
                            let error_msg =
                                t!("update_failed_update_time_entry", error = err.to_string())
                                    .to_string();
                            model.log_error(error_msg.clone());
                            ui::show_error(model, error_msg);
                            return None;
                        }
                    }
                }

                // --- Reset State (only on success) ---
                if next_message.is_some() {
                    if was_import {
                        model.import_state = ImportState::default();
                    } else {
                        model.edit_state = EditState::default();
                    }
                }
            }
            next_message
        }

        // --- Add the EditCancel arm here ---
        Message::EditCancel => {
            let was_import = model.import_state.active;
            let mut state_reset = false;
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                edit_state.active = false;
                state_reset = true;
            }
            if state_reset {
                if was_import {
                    model.import_state = ImportState::default();
                    model.log_notice(t!("canceled_import").to_string());
                } else {
                    model.log_notice(t!("canceled_editing").to_string());
                    model.edit_state = EditState::default();
                }
            }
            None
        }
        // --- End EditCancel arm ---
        Message::EditTimeEntryNextField => {
            // Get the right edit state based on whether we're in import mode or not
            let edit_state = if model.import_state.active {
                &mut model.import_state.edit_state
            } else if model.edit_state.active {
                &mut model.edit_state
            } else {
                return None; // Not in edit mode, ignore
            };

            const FIELD_ORDER: &[crate::model::EditField] = &[
                crate::model::EditField::Description,
                crate::model::EditField::Contact,
                crate::model::EditField::Project,
                crate::model::EditField::StartTime,
                crate::model::EditField::EndTime,
                crate::model::EditField::StartDate,
                crate::model::EditField::EndDate,
            ];
            update_edit_field_from_editor(edit_state);
            if let Some(current_index) = FIELD_ORDER
                .iter()
                .position(|&field| field == edit_state.selected_field)
            {
                let next_index = (current_index + 1) % FIELD_ORDER.len();
                edit_state.selected_field = FIELD_ORDER[next_index];
                initialize_editor_or_autocomplete(edit_state);
            }
            None
        }
        Message::EditTimeEntryPreviousField => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                const FIELD_ORDER: &[crate::model::EditField] = &[
                    crate::model::EditField::Description,
                    crate::model::EditField::Contact,
                    crate::model::EditField::Project,
                    crate::model::EditField::StartTime,
                    crate::model::EditField::EndTime,
                    crate::model::EditField::StartDate,
                    crate::model::EditField::EndDate,
                ];
                update_edit_field_from_editor(edit_state);
                if let Some(current_index) = FIELD_ORDER
                    .iter()
                    .position(|&field| field == edit_state.selected_field)
                {
                    let prev_index = if current_index == 0 {
                        FIELD_ORDER.len() - 1
                    } else {
                        current_index - 1
                    };
                    edit_state.selected_field = FIELD_ORDER[prev_index];
                    initialize_editor_or_autocomplete(edit_state);
                }
            }
            None
        }
        Message::EditTimeEntryFieldClick(field) => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                update_edit_field_from_editor(edit_state);
                edit_state.selected_field = field;
                initialize_editor_or_autocomplete(edit_state);
                model.log_debug(t!(
                    "update_debug_click_field",
                    field = format!("{:?}", field)
                ));
            }
            None
        }
        Message::EditTimeEntryKeyPress(key) => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                if !matches!(
                    edit_state.selected_field,
                    EditField::Project | EditField::Contact
                ) {
                    edit_state.editor.input(key);
                    update_edit_field_from_editor(edit_state);
                }
            }
            None
        }
        Message::EditTimeEntrySelectProject => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                if edit_state.selected_field == crate::model::EditField::Project {
                    model.log_notice(t!("update_log_project_select_nyi").to_string());
                }
            }
            None
        }
        Message::EditTimeEntrySelectContact => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                if edit_state.selected_field == crate::model::EditField::Contact {
                    model.log_notice(t!("update_log_contact_select_nyi").to_string());
                }
            }
            None
        }

        // --- Autocomplete Handling ---
        Message::AutocompleteKeyPress(key) => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                match edit_state.selected_field {
                    crate::model::EditField::Project => {
                        handle_autocomplete_keypress(&mut edit_state.project_autocomplete, key.code)
                    }
                    crate::model::EditField::Contact => {
                        handle_autocomplete_keypress(&mut edit_state.contact_autocomplete, key.code)
                    }
                    _ => return None,
                }
                return Some(Message::AutocompleteRefresh);
            }
            None
        }
        Message::AutocompleteSelect => {
            if model.import_state.active || model.edit_state.active {
                // Check if any edit state is active
                // Helpers now check internal state, just call them based on field
                if let Some(edit_state) = get_active_edit_state_mut(model) {
                    // Get read-only state to check field
                    match edit_state.selected_field {
                        crate::model::EditField::Project => {
                            handle_autocomplete_select_project(model)
                        } // Pass mutable model
                        crate::model::EditField::Contact => {
                            handle_autocomplete_select_contact(model)
                        } // Pass mutable model
                        _ => {}
                    }
                }
            }
            None
        }
        Message::AutocompleteRefresh => {
            if model.import_state.active || model.edit_state.active {
                // Check if any edit state is active
                if let Some(edit_state) = get_active_edit_state_mut(model) {
                    // Get read-only state to check field
                    match edit_state.selected_field {
                        crate::model::EditField::Project => {
                            return handle_autocomplete_refresh_project(model).await
                        }
                        crate::model::EditField::Contact => {
                            return handle_autocomplete_refresh_contact(model).await
                        }
                        _ => {}
                    }
                }
            }
            None
        }
        Message::AutocompleteNextItem => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                match edit_state.selected_field {
                    crate::model::EditField::Project => {
                        handle_autocomplete_navigation(&mut edit_state.project_autocomplete, true)
                    }
                    crate::model::EditField::Contact => {
                        handle_autocomplete_navigation(&mut edit_state.contact_autocomplete, true)
                    }
                    _ => {}
                }
            }
            None
        }
        Message::AutocompletePreviousItem => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                match edit_state.selected_field {
                    crate::model::EditField::Project => {
                        handle_autocomplete_navigation(&mut edit_state.project_autocomplete, false)
                    }
                    crate::model::EditField::Contact => {
                        handle_autocomplete_navigation(&mut edit_state.contact_autocomplete, false)
                    }
                    _ => {}
                }
            }
            None
        }
        Message::AutocompleteClearInput => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                match edit_state.selected_field {
                    crate::model::EditField::Project => {
                        if let Some(msg) =
                            handle_autocomplete_clear(&mut edit_state.project_autocomplete)
                        {
                            return Some(msg);
                        } // handle_autocomplete_clear now returns Option<Message>
                    }
                    crate::model::EditField::Contact => {
                        if let Some(msg) =
                            handle_autocomplete_clear(&mut edit_state.contact_autocomplete)
                        {
                            return Some(msg);
                        }
                    }
                    _ => {}
                }
            }
            None
        }
        Message::AutocompleteResultsProject(projects) => {
            // This message might need adjustment depending on whether it's for regular or import state
            // For now, assume it updates the regular edit state's autocomplete
            if model.edit_state.active {
                // Only update if regular edit is active?
                model.edit_state.project_autocomplete.update_items(projects);
                model.log_debug(
                    t!(
                        "updated_project_suggestions",
                        count = model.edit_state.project_autocomplete.items.len()
                    )
                    .to_string(),
                );
            } // Maybe update import state too?
            None
        }
        Message::AutocompleteResultsContact(contacts) => {
            // Similar to Project results, assume regular edit state for now
            if model.edit_state.active {
                model.edit_state.contact_autocomplete.update_items(contacts);
                model.log_debug(
                    t!(
                        "updated_contact_suggestions",
                        count = model.edit_state.contact_autocomplete.items.len()
                    )
                    .to_string(),
                );
            }
            None
        }

        // --- Plugin View Handling ---
        Message::PluginViewShow => {
            model.plugin_view_state.active = true;
            if let Some(plugin_manager) = &model.plugin_manager {
                let plugins = plugin_manager.list_plugins();
                if !plugins.is_empty() {
                    if model
                        .plugin_view_state
                        .plugin_list_state
                        .selected()
                        .is_none()
                        || model
                            .plugin_view_state
                            .plugin_list_state
                            .selected()
                            .unwrap_or(0)
                            >= plugins.len()
                    {
                        model.plugin_view_state.plugin_list_state.select(Some(0));
                        model.plugin_view_state.selected_index = Some(0);
                    } else {
                        model.plugin_view_state.selected_index =
                            model.plugin_view_state.plugin_list_state.selected();
                    }
                } else {
                    model.plugin_view_state.plugin_list_state.select(None);
                    model.plugin_view_state.selected_index = None;
                }
            }
            None
        }
        Message::PluginViewHide => {
            model.plugin_view_state.active = false;
            None
        }
        Message::PluginViewSelectNext => {
            if let Some(plugin_manager) = &model.plugin_manager {
                let plugins = plugin_manager.list_plugins();
                let count = plugins.len();
                if let Some(next_index) = calculate_next_index(
                    model.plugin_view_state.plugin_list_state.selected(),
                    count,
                ) {
                    model
                        .plugin_view_state
                        .plugin_list_state
                        .select(Some(next_index));
                    model.plugin_view_state.selected_index = Some(next_index);
                }
            }
            None
        }
        Message::PluginViewSelectPrevious => {
            if let Some(plugin_manager) = &model.plugin_manager {
                let plugins = plugin_manager.list_plugins();
                let count = plugins.len();
                if let Some(prev_index) = calculate_previous_index(
                    model.plugin_view_state.plugin_list_state.selected(),
                    count,
                ) {
                    model
                        .plugin_view_state
                        .plugin_list_state
                        .select(Some(prev_index));
                    model.plugin_view_state.selected_index = Some(prev_index);
                }
            }
            None
        }
        Message::PluginViewSelectRow(index) => {
            if let Some(plugin_manager) = &model.plugin_manager {
                let plugins = plugin_manager.list_plugins();
                let count = plugins.len();
                if let Some(valid_index) = validate_row_index(index, count) {
                    model
                        .plugin_view_state
                        .plugin_list_state
                        .select(Some(valid_index));
                    model.plugin_view_state.selected_index = Some(valid_index);
                } else {
                    model.log_warning(format!(
                        "PluginViewSelectRow: Index {} out of bounds ({} plugins)",
                        index, count
                    ));
                }
            }
            None
        }

        // --- Import Handling ---
        Message::ImportTimeEntry => {
            if !model.edit_state.active && !is_import_active(model) {
                initialize_time_entry_import(model)
            } else {
                None
            }
        }

        // --- Modal Handling ---
        Message::ConfirmModal(modal_id) => handle_modal_close(model, modal_id, true),
        Message::DismissModal(modal_id, is_cancel) => {
            handle_modal_close(model, modal_id, !is_cancel)
        }

        // --- Execution Actions (triggered by modals) ---
        Message::ExecuteExport => {
            handle_export(model);
            None
        }
        Message::ExecuteDeleteTimeEntry(entry_id) => {
            model.log_notice(t!(
                "update_deleting_time_entry",
                entry_id = entry_id.clone()
            ));
            let admin_id = model.administration.id.clone().unwrap_or_default();
            let client = model.client.clone();
            let delete_result =
                crate::api::delete_time_entry_by_id(&client, &admin_id, &entry_id).await;
            match delete_result {
                Ok(_) => {
                    model.log_success(t!("update_time_entry_deleted", entry_id = entry_id.clone()));
                    ui::show_info(
                        model,
                        "delete_success",
                        t!("success").to_string(),
                        t!("time_entry_was_deleted_successfully").to_string(),
                    );
                    // Trigger refresh after successful deletion
                    return Some(Message::TimeEntryRefresh);
                }
                Err(err) => {
                    let error_msg =
                        t!("update_failed_delete_time_entry", error = err.to_string()).to_string();
                    model.log_error(error_msg.clone());
                    ui::show_error(model, error_msg);
                }
            }
            None
        }
        Message::UserSelectNext => {
            let count = model.users.len();
            if let Some(next_index) =
                calculate_next_index(model.user_selection_state.selected(), count)
            {
                model.user_selection_state.select(Some(next_index));
            }
            None
        }
        Message::UserSelectPrevious => {
            let count = model.users.len();
            if let Some(prev_index) =
                calculate_previous_index(model.user_selection_state.selected(), count)
            {
                model.user_selection_state.select(Some(prev_index));
            }
            None
        }
        Message::UserConfirmSelection => {
            let mut selected_user_id: Option<String> = None;
            let mut log_no_id_error = false;
            if model.user_selection_active {
                if let Some(selected_index) = model.user_selection_state.selected() {
                    if selected_index < model.users.len() {
                        if let Some(id) = &model.users[selected_index].id {
                            selected_user_id = Some(id.clone());
                        } else {
                            log_no_id_error = true;
                        }
                    }
                }
            }
            if let Some(user_id) = selected_user_id {
                model.log_notice(t!("update_selected_user_id", user_id = user_id.clone()));
                model.config.user_id = Some(user_id.clone());
                match config::save_configuration(&model.config) {
                    Ok(_) => {
                        model.log_success(t!("update_config_saved_success").to_string());
                        model.user_selection_active = false;
                        Some(Message::TimeEntryRefresh)
                    }
                    Err(e) => {
                        let error_msg =
                            t!("update_failed_save_config", error = e.to_string()).to_string();
                        model.log_error(error_msg.clone());
                        ui::show_error(model, error_msg);
                        None
                    }
                }
            } else if model.user_selection_active {
                if log_no_id_error {
                    model.log_error(t!("error_user_no_id").to_string());
                    ui::show_error(model, t!("error_user_no_id").to_string());
                } else {
                    model.log_warning(t!("warning_confirm_no_selection").to_string());
                }
                None
            } else {
                None
            }
        }

        // --- Log Panel ---
        Message::ToggleLogPanel => {
            model.log_panel_state.visible = !model.log_panel_state.visible;
            if model.log_panel_state.visible {
                model.log_debug(t!("update_log_panel_opened").to_string());
            } else {
                model.log_debug(t!("update_log_panel_closed").to_string());
            }
            None
        }
    }
}
