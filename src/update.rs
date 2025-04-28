use crossterm::event::KeyCode;
use ratatui::style::Style;
use rust_i18n::t;
use std::fs;
use toml::Value;
use tui_textarea::TextArea;

use crate::{
    api,
    api::{get_contacts_by_query, get_time_entries},
    config, datetime,
    event::Message,
    file,
    model::{AppModel, AutocompleteState, EditField, EditState, EditType, TimeEntryForTable},
    moneybird::types::{Contact, Project, TimeEntry, User},
    plugin::{PluginManager, PluginTimeEntry},
    ui::{self},
    RunningState,
};

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
    // Get a reference to the edit state
    let edit_state = &mut model.edit_state;

    if let Some(selected_project) = edit_state.project_autocomplete.selected_item() {
        // Add selected project to the main project list if it's not already there
        if !model.projects.iter().any(|p| p.id == selected_project.id) {
            model.projects.push(selected_project.clone());
        }

        let project_id = selected_project.id.clone();
        let project_name = selected_project.name.clone().unwrap_or_default();

        // Update the edit state
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
    // Get a reference to the edit state
    let edit_state = &mut model.edit_state;

    if let Some(selected_contact) = edit_state.contact_autocomplete.selected_item() {
        // Add selected contact to the main contact list if it's not already there
        if !model.contacts.iter().any(|c| c.id == selected_contact.id) {
            model.contacts.push(selected_contact.clone());
        }

        let contact_id = selected_contact.id.clone();
        let contact_name = selected_contact.company_name.clone().unwrap_or_default();

        // Update the edit state
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
    // Get the query and min chars once to avoid multiple borrows
    let query = model.edit_state.project_autocomplete.input.clone();
    let min_chars = model.edit_state.project_autocomplete.min_chars_to_search;

    // If the query is not long enough, don't search
    if query.len() < min_chars {
        // Update with empty results
        model.edit_state.project_autocomplete.update_items(vec![]);
        return None;
    }

    // Mark state as loading, set searched flag
    model.edit_state.project_autocomplete.mark_searched();

    // Create a filtered list of projects
    let filtered_projects = model
        .projects
        .iter()
        .filter(|project| {
            if let Some(name) = &project.name {
                name.to_lowercase().contains(&query.to_lowercase())
            } else {
                false
            }
        })
        .cloned()
        .collect::<Vec<_>>();

    // Update project autocomplete with filtered projects
    model
        .edit_state
        .project_autocomplete
        .update_items(filtered_projects);

    // Get the count of items after update
    let items_count = model.edit_state.project_autocomplete.items.len();

    model.log_debug(format!(
        "Updated project items: {} results for query '{}'",
        items_count, query
    ));

    None // Return None as we've already updated the state
}

// Helper function to refresh contact autocomplete suggestions (API call)
async fn handle_autocomplete_refresh_contact(model: &mut AppModel) -> Option<Message> {
    // Get the query and min chars once to avoid multiple borrows
    let query = model.edit_state.contact_autocomplete.input.clone();
    let min_chars = model.edit_state.contact_autocomplete.min_chars_to_search;

    // If the query is not long enough, don't search
    if query.len() < min_chars {
        // Update with empty results
        model.edit_state.contact_autocomplete.update_items(vec![]);
        return None;
    }

    // Log before making API call
    model.log_debug(format!("Searching contacts with query: '{}'", query));

    // Get admin ID properly
    let admin_id = model.administration.id.clone().unwrap_or_default();
    // Make the API call
    let contacts_result = get_contacts_by_query(&model.client, &admin_id, &query).await;

    match contacts_result {
        Ok(contacts) => {
            let count = contacts.len();
            model.log_debug(format!("Contact search returned {} results", count));
            model.edit_state.contact_autocomplete.update_items(contacts);
        }
        Err(err) => {
            model.log_error(format!("Contact search failed: {}", err));
            model.edit_state.contact_autocomplete.update_items(vec![]);
        }
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
async fn initialize_time_entry_import(model: &mut AppModel) -> Option<Message> {
    // Get the currently selected entry from the time entries table
    let selected_entry = if let Some(selected_index) = model.time_entry_table_state.selected() {
        if selected_index < model.time_entries_for_table.len() {
            model.time_entries_for_table[selected_index].clone()
        } else {
            return None;
        }
    } else {
        return None;
    };

    // Check if this is a plugin entry (not from Moneybird)
    if selected_entry.plugin_name.is_none() && selected_entry.source == "moneybird" {
        // This is a Moneybird entry, can't import it
        ui::show_error(model, t!("update_cant_import_moneybird_entry"));
        return None;
    }

    // Store the original entry for reference
    model.edit_state.original_entry = Some(selected_entry.clone());

    // Initialize edit state with data from the selected entry
    let mut edit_state = EditState::default();

    // Set fields from the plugin entry
    edit_state.description = selected_entry.description.clone();

    // Parse start_date and start_time from started_at
    let admin_timezone = model
        .administration
        .time_zone
        .clone()
        .unwrap_or_else(|| "UTC".to_string());
    if let (Some(start_date), Some(start_time)) =
        crate::datetime::parse_datetime_for_edit(&selected_entry.started_at, &admin_timezone)
    {
        edit_state.start_date = start_date;
        edit_state.start_time = start_time;
    }

    // Parse end_date and end_time from ended_at
    if let (Some(end_date), Some(end_time)) =
        crate::datetime::parse_datetime_for_edit(&selected_entry.ended_at, &admin_timezone)
    {
        edit_state.end_date = end_date;
        edit_state.end_time = end_time;
    }

    // --- Match Contact ---
    let contact_name_from_plugin = selected_entry.customer.clone();
    model.log_debug(format!(
        "Contact name from plugin: {}",
        contact_name_from_plugin
    ));
    let mut matched_contact: Option<Contact> = None;
    if !contact_name_from_plugin.is_empty() {
        // 1. Check local cache first
        for contact in &model.contacts {
            if let Some(company_name) = &contact.company_name {
                if company_name.to_lowercase() == contact_name_from_plugin.to_lowercase() {
                    matched_contact = Some(contact.clone());
                    model.log_notice(t!(
                        "update_matched_contact_local",
                        contact_name = company_name.clone()
                    ));
                    break;
                }
            }
        }

        // 2. If no local match, try API lookup
        if matched_contact.is_none() {
            model.log_debug(format!(
                "No local contact match for '{}', querying API...",
                contact_name_from_plugin
            ));
            let admin_id = model.administration.id.clone().unwrap_or_default();
            // Use a cloned client for the async call to avoid borrow issues
            let client = model.client.clone();
            match api::get_contacts_by_query(&client, &admin_id, &contact_name_from_plugin).await {
                Ok(mut api_contacts) => {
                    if api_contacts.len() == 1 {
                        // Exact match found via API
                        let api_contact = api_contacts.remove(0);
                        matched_contact = Some(api_contact.clone());
                        model.log_notice(t!(
                            "update_matched_contact_api",
                            contact_name = api_contact.company_name.unwrap_or_default()
                        ));
                        // Optionally add to local cache?
                        // model.contacts.push(api_contact);
                    } else if api_contacts.is_empty() {
                        model.log_notice(t!(
                            "update_no_contact_match_api",
                            contact_name = contact_name_from_plugin.clone()
                        ));
                    } else {
                        model.log_notice(t!(
                            "update_multiple_contact_match_api",
                            count = api_contacts.len(),
                            contact_name = contact_name_from_plugin.clone()
                        ));
                    }
                }
                Err(e) => {
                    model.log_error(format!(
                        "API error querying contact '{}': {}",
                        contact_name_from_plugin, e
                    ));
                }
            }
        }
    }

    if matched_contact.is_none() && !contact_name_from_plugin.is_empty() {
        // Log final no-match only if API lookup also failed or wasn't applicable
        if model.contacts.iter().all(|c| {
            c.company_name.as_deref().unwrap_or_default().to_lowercase()
                != contact_name_from_plugin.to_lowercase()
        }) {
            model.log_notice(t!(
                "update_no_contact_match",
                contact_name = contact_name_from_plugin.clone()
            ));
        }
    }

    // --- Match Project ---
    let project_name_from_plugin = selected_entry.project.clone();
    let mut matched_project: Option<Project> = None;
    if !project_name_from_plugin.is_empty() {
        for project in &model.projects {
            if let Some(name) = &project.name {
                if name.to_lowercase() == project_name_from_plugin.to_lowercase() {
                    matched_project = Some(project.clone());
                    model.log_notice(t!("update_matched_project", project_name = name.clone()));
                    break;
                }
            }
        }
    }

    if matched_project.is_none() && !project_name_from_plugin.is_empty() {
        model.log_notice(t!(
            "update_no_project_match",
            project_name = project_name_from_plugin.clone()
        ));
    }

    // Set EditState fields based on matching results
    if let Some(contact) = matched_contact {
        edit_state.contact_id = contact.id;
        edit_state.contact_name = contact.company_name.unwrap_or_default();
    } else {
        edit_state.contact_id = None;
        edit_state.contact_name = contact_name_from_plugin; // Show the original name if no match
    }

    if let Some(project) = matched_project {
        edit_state.project_id = project.id;
        edit_state.project_name = project.name.unwrap_or_default();
    } else {
        edit_state.project_id = None;
        edit_state.project_name = project_name_from_plugin; // Show the original name if no match
    }

    // Log the import action
    model.log_notice(t!(
        "update_importing_time_entry",
        description = selected_entry.description.clone(),
        source = selected_entry.source.clone()
    ));

    edit_state.edit_type = EditType::Import;
    edit_state.active = true;
    edit_state.original_entry = Some(selected_entry);

    // Set the default field to Description
    edit_state.selected_field = crate::model::EditField::Description;

    // Initialize the editor component with the description
    edit_state.editor = TextArea::default();
    edit_state.editor.insert_str(&edit_state.description);

    // Make sure project and contact autocomplete fields are properly set up
    // Always initialize the input with the name we decided to display
    edit_state.project_autocomplete.input = edit_state.project_name.clone();
    edit_state.contact_autocomplete.input = edit_state.contact_name.clone();

    // Set the model's edit state
    model.edit_state = edit_state;

    Some(Message::None)
}

/// Check if the model's import state is active
fn is_import_active(model: &AppModel) -> bool {
    model.edit_state.active && model.edit_state.is_import_mode()
}

// --- Helper function to get the active mutable EditState ---
fn get_active_edit_state_mut(model: &mut AppModel) -> Option<&mut EditState> {
    if model.edit_state.active {
        Some(&mut model.edit_state)
    } else {
        None
    }
}

/// Formats a time input string. If it's a valid hour (0-23), formats as HH:00.
/// Otherwise, returns the original string.
fn format_time_input(time_str: &str) -> String {
    // Trim whitespace for cleaner parsing
    let trimmed_str = time_str.trim();
    
    // Try parsing as just an hour
    match trimmed_str.parse::<u8>() {
        Ok(hour) if hour < 24 => {
            // Valid hour, format as HH:00
            format!("{:02}:00", hour)
        }
        _ => {
            // Not a simple hour or parsing failed, return original string
            // We assume it might already be HH:MM or invalid
            time_str.to_string()
        }
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
            let mut edit_state = EditState::default();
            edit_state.edit_type = EditType::Create;
            edit_state.active = true;
            edit_state.time_entry_id = None;
            edit_state.editor = TextArea::default();
            edit_state.selected_field = crate::model::EditField::Description;
            let admin_timezone_str = model
                .administration
                .time_zone
                .clone()
                .unwrap_or_else(|| "UTC".to_string());
            let admin_tz = admin_timezone_str
                .parse::<chrono_tz::Tz>()
                .unwrap_or(chrono_tz::UTC);
            let now = chrono::Utc::now().with_timezone(&admin_tz);
            edit_state.start_date = now.format("%Y-%m-%d").to_string();
            edit_state.start_time = now.format("%H:%M").to_string();
            let end_time_default = now + chrono::Duration::hours(1);
            edit_state.end_date = end_time_default.format("%Y-%m-%d").to_string();
            edit_state.end_time = end_time_default.format("%H:%M").to_string();
            initialize_editor_or_autocomplete(&mut edit_state);
            model.edit_state = edit_state;
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
            let was_import = model.edit_state.is_import_mode();
            model.edit_state = EditState::default();

            if was_import {
                Some(Message::PluginViewActivate)
            } else {
                Some(Message::None)
            }
        }
        Message::EditSave => {
            let was_import = model.edit_state.is_import_mode();
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
                            edit_state.edit_type == EditType::Create,
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

                if is_creating || was_import {
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
                        model.edit_state = EditState::default();
                    }
                }
            }
            next_message
        }

        Message::EditCancel => {
            let was_import = model.edit_state.is_import_mode();
            model.edit_state = EditState::default();

            if was_import {
                Some(Message::PluginViewActivate)
            } else {
                Some(Message::None)
            }
        }
        Message::EditTimeEntryNextField => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                let field_before_move = edit_state.selected_field;
                update_edit_field_from_editor(edit_state);
                
                // Format the time field if focus is leaving StartTime or EndTime
                match field_before_move {
                    EditField::StartTime => {
                        edit_state.start_time = format_time_input(&edit_state.start_time);
                    }
                    EditField::EndTime => {
                        edit_state.end_time = format_time_input(&edit_state.end_time);
                    }
                    _ => {}
                }

                const FIELD_ORDER: &[crate::model::EditField] = &[
                    crate::model::EditField::Description,
                    crate::model::EditField::Contact,
                    crate::model::EditField::Project,
                    crate::model::EditField::StartTime,
                    crate::model::EditField::EndTime,
                    crate::model::EditField::StartDate,
                    crate::model::EditField::EndDate,
                ];
                if let Some(current_index) = FIELD_ORDER
                    .iter()
                    .position(|&field| field == edit_state.selected_field)
                {
                    let next_index = (current_index + 1) % FIELD_ORDER.len();
                    edit_state.selected_field = FIELD_ORDER[next_index];
                    initialize_editor_or_autocomplete(edit_state);
                }
            }
            None
        }
        Message::EditTimeEntryPreviousField => {
            if let Some(edit_state) = get_active_edit_state_mut(model) {
                let field_before_move = edit_state.selected_field;
                update_edit_field_from_editor(edit_state);
                
                // Format the time field if focus is leaving StartTime or EndTime
                match field_before_move {
                    EditField::StartTime => {
                        edit_state.start_time = format_time_input(&edit_state.start_time);
                    }
                    EditField::EndTime => {
                        edit_state.end_time = format_time_input(&edit_state.end_time);
                    }
                    _ => {}
                }

                const FIELD_ORDER: &[crate::model::EditField] = &[
                    crate::model::EditField::Description,
                    crate::model::EditField::Contact,
                    crate::model::EditField::Project,
                    crate::model::EditField::StartTime,
                    crate::model::EditField::EndTime,
                    crate::model::EditField::StartDate,
                    crate::model::EditField::EndDate,
                ];
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
            if model.edit_state.active {
                // Check if regular edit is active
                if let Some(edit_state) = get_active_edit_state_mut(model) {
                    // Get read-only state to check field
                    match edit_state.selected_field {
                        crate::model::EditField::Project => {
                            handle_autocomplete_select_project(model)
                        }
                        crate::model::EditField::Contact => {
                            handle_autocomplete_select_contact(model)
                        }
                        _ => {}
                    }
                }
            }
            None
        }
        Message::AutocompleteRefresh => {
            if model.edit_state.active {
                // Check if regular edit is active
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
                initialize_time_entry_import(model).await
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

        Message::None => None,
        Message::PluginViewActivate => {
            model.plugin_view_state.active = true;
            None
        }

        Message::DebugPluginResponse(plugin_name) => {
            model.log_notice(t!("debugging_plugin", name = plugin_name.clone()));

            // Get the current date and a date 24 hours later for testing
            let now = chrono::Utc::now();
            let tomorrow = now + chrono::Duration::hours(24);

            if let Some(plugin_manager) = &mut model.plugin_manager {
                match plugin_manager
                    .debug_plugin_response(&plugin_name, &now, &tomorrow)
                    .await
                {
                    Ok(result_json) => {
                        // Create a diagnostic message to show the developer
                        let diagnostic_modal_id = "plugin_debug_response";
                        let diagnostic_title = t!("plugin_debug_result", name = plugin_name);

                        // Create a formatted version of the response with line numbers
                        let formatted_response = if result_json.trim().is_empty() {
                            t!("plugin_returned_empty_response").to_string()
                        } else {
                            // Add helpful diagnostic info for common issues
                            let mut diagnostic_info = String::new();

                            // The debug_plugin_response method only returns the result field,
                            // so we're analyzing the time entries array, not the JSON-RPC envelope
                            if result_json.trim().starts_with("[") {
                                // This is good - we expect an array of time entries
                                match serde_json::from_str::<Vec<PluginTimeEntry>>(&result_json) {
                                    Ok(entries) => {
                                        diagnostic_info.push_str(&t!(
                                            "plugin_debug_valid_entries",
                                            count = entries.len().to_string()
                                        ));
                                        diagnostic_info.push_str("\n\n");

                                        // Check for required fields in entries
                                        if !entries.is_empty() {
                                            for (i, entry) in entries.iter().enumerate() {
                                                let mut field_issues = Vec::new();

                                                if entry.id.is_empty() {
                                                    field_issues.push("id");
                                                }
                                                if entry.description.is_empty() {
                                                    field_issues.push("description");
                                                }
                                                if entry.started_at.is_empty() {
                                                    field_issues.push("started_at");
                                                }
                                                if entry.ended_at.is_empty() {
                                                    field_issues.push("ended_at");
                                                }

                                                if !field_issues.is_empty() {
                                                    diagnostic_info.push_str(&format!(
                                                        "Entry #{}: Missing required fields: {}\n",
                                                        i + 1,
                                                        field_issues.join(", ")
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        diagnostic_info.push_str(&t!(
                                            "plugin_error_invalid_time_entries",
                                            error = e.to_string()
                                        ));
                                        diagnostic_info.push_str("\n\n");
                                    }
                                }
                            } else if !result_json.trim().starts_with("{") {
                                // Not a valid JSON object or array
                                diagnostic_info.push_str(&t!("plugin_error_invalid_json_response"));
                                diagnostic_info.push_str("\n\n");
                            } else {
                                // Got a single object instead of an array - probably an error
                                diagnostic_info.push_str(&t!("plugin_warning_expected_array"));
                                diagnostic_info.push_str("\n\n");
                            }

                            // Format with line numbers and add the diagnostic info
                            let mut formatted = String::new();
                            if !diagnostic_info.is_empty() {
                                formatted.push_str(&diagnostic_info);
                                formatted.push_str("---\n");
                            }

                            for (i, line) in result_json.lines().enumerate() {
                                formatted.push_str(&format!("{:03}: {}\n", i + 1, line));
                            }

                            formatted
                        };

                        ui::show_modal(
                            model,
                            crate::ui::ModalData {
                                title: diagnostic_title.to_string(),
                                message: formatted_response,
                                modal_type: crate::ui::ModalType::Info,
                                id: Some(diagnostic_modal_id.to_string()),
                                ..Default::default()
                            },
                        );
                    }
                    Err(err) => {
                        let error_msg = t!(
                            "plugin_debug_failed",
                            name = plugin_name,
                            error = err.to_string()
                        )
                        .to_string();
                        model.log_error(error_msg.clone());
                        ui::show_error(model, error_msg);
                    }
                }
            } else {
                ui::show_error(model, t!("plugin_manager_not_available"));
            }

            None
        }
        Message::PluginToggleActivation => {
            let selected_index_opt = model.plugin_view_state.selected_index;
            // Get necessary info before the mutable borrow for logging
            let plugins_dir_opt = model
                .plugin_manager
                .as_ref()
                .map(|pm| pm.plugins_dir().to_path_buf());

            if let (Some(selected_index), Some(plugins_dir)) = (selected_index_opt, plugins_dir_opt)
            {
                // We need plugin_manager again for list_plugins, so re-borrow immutably
                if let Some(plugin_manager) = model.plugin_manager.as_ref() {
                    let plugins = plugin_manager.list_plugins();
                    if selected_index < plugins.len() {
                        let plugin_info = &plugins[selected_index];
                        let plugin_name = plugin_info.name.clone();

                        // Now log, borrowing model mutably
                        model
                            .log_notice(format!("Toggling activation for plugin: {}", plugin_name));

                        // Find the plugin directory using the pre-fetched plugins_dir
                        let plugin_dir_result = find_plugin_directory(&plugins_dir, &plugin_name);

                        match plugin_dir_result {
                            Ok(plugin_dir) => {
                                let config_path = plugin_dir.join("config.toml");
                                match toggle_plugin_config_enabled(&config_path) {
                                    Ok(new_state) => {
                                        let status_msg = if new_state {
                                            t!("plugin_toggle_enabled", name = plugin_name)
                                        } else {
                                            t!("plugin_toggle_disabled", name = plugin_name)
                                        };
                                        model.log_success(status_msg.clone());
                                        ui::show_info(
                                            model,
                                            "plugin_toggle",
                                            t!("success").to_string(),
                                            status_msg,
                                        );
                                    }
                                    Err(e) => {
                                        let error_msg = t!(
                                            "plugin_toggle_error",
                                            name = plugin_name,
                                            error = e.to_string()
                                        );
                                        model.log_error(error_msg.clone());
                                        ui::show_error(model, error_msg);
                                    }
                                }
                            }
                            Err(e) => {
                                let error_msg = t!(
                                    "plugin_toggle_find_dir_error",
                                    name = plugin_name,
                                    error = e.to_string()
                                );
                                model.log_error(error_msg.clone());
                                ui::show_error(model, error_msg);
                            }
                        }
                    }
                }
            }
            None // No further message needed
        }
    }
}

// --- Helper function to find plugin directory (similar to debug logic) ---
fn find_plugin_directory(
    plugins_dir: &std::path::Path,
    plugin_name: &str,
) -> Result<std::path::PathBuf, String> {
    // Search logic remains largely the same, but takes plugins_dir directly
    if let Ok(entries) = fs::read_dir(plugins_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("manifest.toml");
                if manifest_path.exists() {
                    if let Ok(content) = fs::read_to_string(&manifest_path) {
                        if let Ok(manifest_value) = content.parse::<Value>() {
                            if let Some(plugin_table) =
                                manifest_value.get("plugin").and_then(|p| p.as_table())
                            {
                                // Use as_str() instead of as_string()
                                if let Some(name_val) =
                                    plugin_table.get("name").and_then(|n| n.as_str())
                                {
                                    if name_val == plugin_name {
                                        return Ok(path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Fallback: try direct name match (less reliable)
    let direct_path = plugins_dir.join(plugin_name);
    if direct_path.is_dir() {
        Ok(direct_path)
    } else {
        Err(format!(
            "Could not find directory for plugin '{}'",
            plugin_name
        ))
    }
}

// --- Helper function to toggle the 'enabled' key in a TOML config file ---
fn toggle_plugin_config_enabled(config_path: &std::path::Path) -> Result<bool, String> {
    // Read the config file content
    let content = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read config file {:?}: {}", config_path, e))?;

    // Parse the TOML content
    let mut config_value = content
        .parse::<Value>()
        .map_err(|e| format!("Failed to parse TOML from {:?}: {}", config_path, e))?;

    // Get the current enabled state, default to false if missing or not a boolean
    let current_enabled = config_value
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Toggle the state
    let new_enabled_state = !current_enabled;

    // Update the value in the TOML table
    if let Some(table) = config_value.as_table_mut() {
        table.insert("enabled".to_string(), Value::Boolean(new_enabled_state));
    } else {
        return Err(format!(
            "Config file {:?} is not a valid TOML table",
            config_path
        ));
    }

    // Serialize the updated TOML back to a string
    let updated_content = toml::to_string_pretty(&config_value)
        .map_err(|e| format!("Failed to serialize updated TOML: {}", e))?;

    // Write the updated content back to the file
    fs::write(config_path, updated_content)
        .map_err(|e| format!("Failed to write updated config to {:?}: {}", config_path, e))?;

    Ok(new_enabled_state)
}
