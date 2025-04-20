use crossterm::event::KeyCode;
use ratatui::style::Style;
use rust_i18n::t;
use tui_textarea::TextArea;

use crate::{
    api::{get_contacts_by_query, get_time_entries},
    config, datetime,
    event::Message,
    file,
    model::{AppModel, AutocompleteState, EditState, TimeEntryForTable},
    ui::{self},
    RunningState,
    api,
};

// Import the EditField enum
use crate::model::EditField;

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
    if let Some(selected_project) = model.edit_state.project_autocomplete.selected_item() {
        // Add selected project to the project list on model.projects if it's not already there
        if !model.projects.iter().any(|p| p.id == selected_project.id) {
            model.projects.push(selected_project.clone());
        }

        let project_id = selected_project.id.clone();
        let project_name = selected_project.name.clone().unwrap_or_default();
        model.edit_state.project_id = project_id;
        let autocomplete_state = &mut model.edit_state.project_autocomplete;
        autocomplete_state.input = project_name.clone();
        autocomplete_state.mark_searched(); // Prevent re-search
        autocomplete_state.is_dropdown_visible = false;
        autocomplete_state.items.clear();
        model.log_notice(t!("update_selected_project", project_name = project_name));
    }
}

// Helper function to handle selecting a contact from autocomplete
fn handle_autocomplete_select_contact(model: &mut AppModel) {
    if let Some(selected_contact) = model.edit_state.contact_autocomplete.selected_item() {
        // Add selected contact to the contact list on model.contacts if it's not already there
        if !model.contacts.iter().any(|c| c.id == selected_contact.id) {
            model.contacts.push(selected_contact.clone());
        }

        let contact_id = selected_contact.id.clone();
        let contact_name = selected_contact.company_name.clone().unwrap_or_default();
        model.edit_state.contact_id = contact_id;
        let autocomplete_state = &mut model.edit_state.contact_autocomplete;
        autocomplete_state.input = contact_name.clone();
        autocomplete_state.mark_searched(); // Prevent re-search
        autocomplete_state.is_dropdown_visible = false;
        autocomplete_state.items.clear();
        model.log_notice(t!("update_selected_contact", contact_name = contact_name));
    }
}

// Helper function to refresh project autocomplete suggestions (local filter)
async fn handle_autocomplete_refresh_project(model: &mut AppModel) -> Option<Message> {
    let query = model.edit_state.project_autocomplete.input.clone();
    let min_chars = model.edit_state.project_autocomplete.min_chars_to_search;

    if query.is_empty() || query.len() < min_chars {
        model.edit_state.project_autocomplete.update_items(vec![]);
        model.log_debug(format!(
            "Query '{}' too short or empty, cleared project items.",
            query
        ));
    } else {
        model.edit_state.project_autocomplete.mark_searched();
        model.log_debug(t!("update_filtering_local_projects", query = query));

        let filtered_projects = model
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

        model
            .edit_state
            .project_autocomplete
            .update_items(filtered_projects);
        model.log_debug(format!(
            "Local filter found {} projects.",
            model.edit_state.project_autocomplete.items.len()
        ));
    }
    
    None // Return None as we've already updated the state
}

// Helper function to refresh contact autocomplete suggestions (API call)
async fn handle_autocomplete_refresh_contact(model: &mut AppModel) -> Option<Message> {
    let query = model.edit_state.contact_autocomplete.input.clone();
    let min_chars = model.edit_state.contact_autocomplete.min_chars_to_search;

    if query.is_empty() || query.len() < min_chars {
        model.edit_state.contact_autocomplete.update_items(vec![]);
        model.log_debug(format!(
            "Query '{}' too short or empty, cleared contact items.",
            query
        ));
    } else {
        model.edit_state.contact_autocomplete.is_loading = true;
        model.edit_state.contact_autocomplete.mark_searched();
        model.log_debug(format!(
            "Calling API to search contacts for query: '{}'",
            query
        ));

        let client = model.client.clone();
        let admin_id = model.administration.id.clone().unwrap_or_default();

        match get_contacts_by_query(&client, &admin_id, &query).await {
            Ok(contacts) => {
                model.edit_state.contact_autocomplete.update_items(contacts);
                model.log_debug(format!(
                    "API search succeeded. Found {} contacts.",
                    model.edit_state.contact_autocomplete.items.len()
                ));
            }
            Err(err) => {
                let error_msg =
                    t!("update_api_search_contacts_failed", error = err.to_string()).to_string();
                ui::show_error(model, error_msg.clone());
                model.edit_state.contact_autocomplete.update_items(vec![]);
            }
        }
        model.edit_state.contact_autocomplete.is_loading = false;
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
    if model.time_entries_for_table.is_empty() || model.time_entry_table_state.selected().is_none() {
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
    model.log_notice(t!("update_importing_time_entry", 
                     description = selected_entry.description.clone(),
                     source = selected_entry.source.clone()));
    
    // Store the original entry in the import state
    model.import_state.original_entry = Some(selected_entry.clone());
    
    // Initialize edit state for the import
    let mut edit_state = EditState::default();
    edit_state.active = true;
    edit_state.is_create_mode = true;
    edit_state.description = selected_entry.description.clone();
    edit_state.project_name = selected_entry.project.clone();
    edit_state.contact_name = selected_entry.customer.clone();
    edit_state.time_entry_id = Some(selected_entry.id.clone());
    
    // Parse and set date/time fields from the original entry
    let admin_timezone = model.administration.time_zone.clone().unwrap_or_else(|| "UTC".to_string());
    
    if let (Some(start_date), Some(start_time)) = 
        datetime::parse_datetime_for_edit(&selected_entry.started_at, &admin_timezone) {
        edit_state.start_date = start_date;
        edit_state.start_time = start_time;
    }
    
    if let (Some(end_date), Some(end_time)) = 
        datetime::parse_datetime_for_edit(&selected_entry.ended_at, &admin_timezone) {
        edit_state.end_date = end_date;
        edit_state.end_time = end_time;
    }
    
    // Set up editor with the description text
    edit_state.editor = TextArea::new(vec![edit_state.description.clone()]);
    edit_state.selected_field = EditField::Description;
    
    // Try to match the project and contact with Moneybird entities
    if !model.contacts.is_empty() {
        // Try to find matching contact by name
        let contact_name_lower = selected_entry.customer.to_lowercase();
        
        for contact in &model.contacts {
            let name = crate::ui::format_contact_name(contact).to_lowercase();
            if name == contact_name_lower || name.contains(&contact_name_lower) || contact_name_lower.contains(&name) {
                edit_state.contact_id = contact.id.clone();
                edit_state.contact_name = crate::ui::format_contact_name(contact);
                // Log before moving to next phase
                model.log_notice(format!("Matched contact: {}", edit_state.contact_name.clone()));
                break;
            }
        }
    }
    
    if !model.projects.is_empty() {
        // Try to find matching project by name
        let project_name_lower = selected_entry.project.to_lowercase();
        
        for project in &model.projects {
            let name = project.name.clone().unwrap_or_default().to_lowercase();
            if name == project_name_lower || name.contains(&project_name_lower) || project_name_lower.contains(&name) {
                edit_state.project_id = project.id.clone();
                edit_state.project_name = project.name.clone().unwrap_or_default();
                // Log before moving to next phase
                model.log_notice(format!("Matched project: {}", edit_state.project_name.clone()));
                break;
            }
        }
    }
    
    // Show notifications for unmatched entities
    if edit_state.contact_id.is_none() {
        ui::show_error(
            model, 
            t!("update_no_contact_match", contact_name = selected_entry.customer.clone())
        );
    }
    
    if edit_state.project_id.is_none() {
        ui::show_error(
            model, 
            t!("update_no_project_match", project_name = selected_entry.project.clone())
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

async fn handle_import_save(model: &mut AppModel) -> Option<Message> {
    // Get user ID from configuration
    let user_id = match &model.config.user_id {
        Some(id) => id.clone(),
        None => {
            ui::show_error(model, t!("update_missing_user_id"));
            return None;
        }
    };

    // Get administration ID
    let administration_id = match &model.administration.id {
        Some(id) => id.clone(),
        None => {
            ui::show_error(model, t!("update_missing_admin_id"));
            return None;
        }
    };

    // Get edit state from import state
    let edit_state = &model.import_state.edit_state;
    
    // Make sure description is not empty (the only required field)
    if edit_state.description.is_empty() {
        ui::show_error(model, t!("update_description_required"));
        return None;
    }

    // Create a time entry from the edit state
    let time_entry = crate::moneybird::types::TimeEntry {
        id: None, // No ID for new entries
        administration_id: Some(administration_id.clone()),
        contact_id: edit_state.contact_id.clone(), // This is optional
        contact: None,
        created_at: None,
        description: Some(edit_state.description.clone()),
        ended_at: Some(datetime::format_datetime_from_edit(
            &edit_state.end_date,
            &edit_state.end_time,
        )),
        events: Vec::new(),
        notes: Vec::new(),
        paused_duration: None,
        project_id: edit_state.project_id.clone(), // This is optional
        project: None,
        started_at: Some(datetime::format_datetime_from_edit(
            &edit_state.start_date,
            &edit_state.start_time,
        )),
        updated_at: None,
        user_id: Some(user_id.clone()),
        billable: Some(true), // Default to billable
    };

    // Create the time entry in Moneybird
    match api::create_time_entry(&model.client, &administration_id, &user_id, time_entry).await {
        Ok(created_entry) => {
            // Log success
            let success_msg = t!("update_import_success");
            model.log_success(success_msg.clone());
            ui::show_info(model, "create_success", success_msg.to_string(), success_msg.to_string());
            
            // Reset the import state
            model.import_state.active = false;
            model.import_state.original_entry = None;
            
            // Refresh time entries to include the newly created one
            Some(Message::TimeEntryRefresh)
        }
        Err(err) => {
            // Log failure
            let error_msg = t!("update_failed_create_time_entry", error = err.to_string());
            model.log_error(error_msg.clone());
            ui::show_error(model, error_msg);
            None
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
            if model.time_entries_for_table.is_empty() {
                return None;
            }

            let current_index = model.time_entry_table_state.selected().unwrap_or(0);
            if current_index < model.time_entries_for_table.len() - 1 {
                model.time_entry_table_state.select(Some(current_index + 1));
            }
            None
        }

        Message::TimeEntrySelectPrevious => {
            if model.time_entries_for_table.is_empty() {
                return None;
            }

            let current_index = model.time_entry_table_state.selected().unwrap_or(0);
            if current_index > 0 {
                model.time_entry_table_state.select(Some(current_index - 1));
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

        Message::ConfirmModal(modal_id) => handle_modal_close(model, modal_id, true),

        Message::DismissModal(modal_id, is_cancel) => {
            handle_modal_close(model, modal_id, !is_cancel)
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

        Message::EditTimeEntrySave => {
            if model.edit_state.active {
                update_edit_field_from_editor(&mut model.edit_state);

                let admin_timezone_str = model
                    .administration
                    .time_zone
                    .clone()
                    .unwrap_or_else(|| "UTC".to_string());
                let time_entry_data = model.edit_state.try_into_time_entry(&admin_timezone_str);

                let is_creating = model.edit_state.time_entry_id.is_none();
                let admin_id = model.administration.id.clone().unwrap_or_default();
                let client = model.client.clone();
                let user_id = model.config.user_id.clone().unwrap_or_default();

                // Make sure description is not empty
                if model.edit_state.description.is_empty() {
                    ui::show_error(model, t!("update_description_required"));
                    return None;
                }

                if is_creating {
                    model.log_notice(format!(
                        "Creating new time entry: {}",
                        model.edit_state.description
                    ));
                    let endpoint = "time_entries.json";
                    crate::api::log_debug_curl(model, endpoint, "POST");

                    match crate::api::create_time_entry(
                        &client,
                        &admin_id,
                        &user_id,
                        time_entry_data,
                    )
                    .await
                    {
                        Ok(created_entry) => {
                            model.log_success(
                                t!(
                                    "time_entry_created_successfully",
                                    id = created_entry.id.clone().unwrap_or_default()
                                )
                                .to_string(),
                            );
                            ui::show_info(
                                model,
                                "create_success",
                                t!("success").to_string(),
                                t!("time_entry_was_created_successfully").to_string(),
                            );
                            get_time_entries(model).await;
                        }
                        Err(err) => {
                            let error_msg =
                                t!("update_failed_create_time_entry", error = err.to_string())
                                    .to_string();
                            model.log_error(error_msg.clone());
                            crate::ui::show_error(model, error_msg);
                        }
                    }
                } else if let Some(entry_id) = &model.edit_state.time_entry_id {
                    let entry_id = entry_id.clone();
                    model.log_notice(t!(
                        "updating_time_entry_notice",
                        entry_id = entry_id.clone(),
                        description = model.edit_state.description.clone()
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
                        Ok(updated_entry) => {
                            if let Some(index) = model
                                .time_entries
                                .iter()
                                .position(|entry| entry.id == Some(entry_id.clone()))
                            {
                                if let Some(table_index) = model
                                    .time_entries_for_table
                                    .iter()
                                    .position(|t_entry| t_entry.id == entry_id)
                                {
                                    model.time_entries[index] = updated_entry.clone();
                                    model.time_entries_for_table[table_index] = TimeEntryForTable {
                                        id: updated_entry.id.clone().unwrap_or_default(),
                                        customer: updated_entry
                                            .contact
                                            .clone()
                                            .unwrap_or_default()
                                            .company_name
                                            .clone()
                                            .unwrap_or_default(),
                                        project: updated_entry
                                            .project
                                            .clone()
                                            .unwrap_or_default()
                                            .name
                                            .clone()
                                            .unwrap_or_default(),
                                        description: updated_entry
                                            .description
                                            .clone()
                                            .unwrap_or_default(),
                                        started_at: updated_entry
                                            .started_at
                                            .clone()
                                            .unwrap_or_default(),
                                        ended_at: updated_entry
                                            .ended_at
                                            .clone()
                                            .unwrap_or_default(),
                                        billable: updated_entry.billable.unwrap_or_default(),
                                        source: "moneybird".to_string(),
                                        icon: None,
                                    };
                                    if let Some(backup_index) = model
                                        .time_entries_for_table_backup
                                        .iter()
                                        .position(|t_entry| t_entry.id == entry_id)
                                    {
                                        model.time_entries_for_table_backup[backup_index] =
                                            model.time_entries_for_table[table_index].clone();
                                    }
                                }
                            }
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
                        }
                        Err(err) => {
                            let error_msg =
                                t!("update_failed_update_time_entry", error = err.to_string())
                                    .to_string();
                            model.log_error(error_msg.clone());
                            crate::ui::show_error(model, error_msg);
                        }
                    }
                }
                model.edit_state = EditState::default();
            }
            None
        }

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
                let prev_index = if current_index == 0 {
                    FIELD_ORDER.len() - 1
                } else {
                    current_index - 1
                };
                edit_state.selected_field = FIELD_ORDER[prev_index];
                initialize_editor_or_autocomplete(edit_state);
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

        Message::ExecuteExport => {
            handle_export(model);
            None // handle_export shows its own modals
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
                    get_time_entries(model).await;
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

        Message::EditTimeEntrySelectProject => {
            if model.edit_state.active
                && model.edit_state.selected_field == crate::model::EditField::Project
            {
                model.log_notice(t!("update_log_project_select_nyi").to_string());
            }
            None
        }

        Message::EditTimeEntrySelectContact => {
            if model.edit_state.active
                && model.edit_state.selected_field == crate::model::EditField::Contact
            {
                model.log_notice(t!("update_log_contact_select_nyi").to_string());
            }
            None
        }

        Message::ToggleLogPanel => {
            model.log_panel_state.visible = !model.log_panel_state.visible;
            if model.log_panel_state.visible {
                model.log_debug(t!("update_log_panel_opened").to_string());
            } else {
                model.log_debug(t!("update_log_panel_closed").to_string());
            }
            None
        }

        Message::AutocompleteKeyPress(key) => {
            // Get the right edit state based on whether we're in import mode or not
            let edit_state = if model.import_state.active {
                &mut model.import_state.edit_state
            } else if model.edit_state.active {
                &mut model.edit_state
            } else {
                return None; // Not in edit mode, ignore
            };

            match edit_state.selected_field {
                crate::model::EditField::Project => handle_autocomplete_keypress(
                    &mut edit_state.project_autocomplete,
                    key.code,
                ),
                crate::model::EditField::Contact => handle_autocomplete_keypress(
                    &mut edit_state.contact_autocomplete,
                    key.code,
                ),
                _ => return None,
            }
            Some(Message::AutocompleteRefresh)
        }

        Message::AutocompleteSelect => {
            // Get the right edit state 
            if model.import_state.active {
                let edit_state = &mut model.import_state.edit_state;
                match edit_state.selected_field {
                    crate::model::EditField::Project => {
                        if let Some(selected_project) = edit_state.project_autocomplete.selected_item() {
                            // Add selected project to the project list if it's not already there
                            if !model.projects.iter().any(|p| p.id == selected_project.id) {
                                model.projects.push(selected_project.clone());
                            }
            
                            let project_id = selected_project.id.clone();
                            let project_name = selected_project.name.clone().unwrap_or_default();
                            edit_state.project_id = project_id;
                            edit_state.project_autocomplete.input = project_name.clone();
                            edit_state.project_autocomplete.mark_searched();
                            edit_state.project_autocomplete.is_dropdown_visible = false;
                            edit_state.project_autocomplete.items.clear();
                            model.log_notice(t!("update_selected_project", project_name = project_name));
                        }
                    },
                    crate::model::EditField::Contact => {
                        if let Some(selected_contact) = edit_state.contact_autocomplete.selected_item() {
                            // Add selected contact to the contact list if it's not already there
                            if !model.contacts.iter().any(|c| c.id == selected_contact.id) {
                                model.contacts.push(selected_contact.clone());
                            }
            
                            let contact_id = selected_contact.id.clone();
                            let contact_name = selected_contact.company_name.clone().unwrap_or_default();
                            edit_state.contact_id = contact_id;
                            edit_state.contact_autocomplete.input = contact_name.clone();
                            edit_state.contact_autocomplete.mark_searched();
                            edit_state.contact_autocomplete.is_dropdown_visible = false;
                            edit_state.contact_autocomplete.items.clear();
                            model.log_notice(t!("update_selected_contact", contact_name = contact_name));
                        }
                    },
                    _ => {}
                }
            } else if model.edit_state.active {
                match model.edit_state.selected_field {
                    crate::model::EditField::Project => handle_autocomplete_select_project(model),
                    crate::model::EditField::Contact => handle_autocomplete_select_contact(model),
                    _ => {}
                }
            }
            None
        }

        Message::AutocompleteRefresh => {
            if model.import_state.active {
                let query: String;
                let min_chars: usize;
                
                {
                    let edit_state = &mut model.import_state.edit_state;
                    match edit_state.selected_field {
                        EditField::Project => {
                            query = edit_state.project_autocomplete.input.clone();
                            min_chars = edit_state.project_autocomplete.min_chars_to_search;
                            edit_state.project_autocomplete.mark_searched();
                        },
                        EditField::Contact => {
                            // Contact field uses a different async approach
                            return handle_autocomplete_refresh_contact(model).await;
                        },
                        _ => return None,
                    }
                }
                
                // Now outside the mutable borrow on edit_state
                if query.is_empty() || query.len() < min_chars {
                    // Log first, before getting a mutable ref to edit_state again
                    model.log_debug(format!(
                        "Query '{}' too short or empty, cleared project items.",
                        query
                    ));
                    model.import_state.edit_state.project_autocomplete.update_items(vec![]);
                } else {
                    // Log first
                    model.log_debug(t!("update_filtering_local_projects", query = query.clone()));
                    
                    // Filter projects before getting a mutable ref to edit_state again
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
                    
                    // Get length for logging before updating edit_state
                    let projects_len = filtered_projects.len();
                    
                    // Update items
                    model.import_state.edit_state.project_autocomplete.update_items(filtered_projects);
                    
                    // Log after update and with pre-calculated length
                    model.log_debug(format!(
                        "Local filter found {} projects.",
                        projects_len
                    ));
                }
                None
            } else if model.edit_state.active {
                match model.edit_state.selected_field {
                    crate::model::EditField::Project => {
                        return handle_autocomplete_refresh_project(model).await;
                    }
                    crate::model::EditField::Contact => {
                        return handle_autocomplete_refresh_contact(model).await;
                    }
                    _ => {}
                }
                None
            } else {
                None
            }
        }

        Message::AutocompleteNextItem => {
            // Get the right edit state based on whether we're in import mode or not
            let edit_state = if model.import_state.active {
                &mut model.import_state.edit_state
            } else if model.edit_state.active {
                &mut model.edit_state
            } else {
                return None; // Not in edit mode, ignore
            };

            match edit_state.selected_field {
                crate::model::EditField::Project => {
                    handle_autocomplete_navigation(&mut edit_state.project_autocomplete, true)
                }
                crate::model::EditField::Contact => {
                    handle_autocomplete_navigation(&mut edit_state.contact_autocomplete, true)
                }
                _ => {}
            }
            None
        }

        Message::AutocompletePreviousItem => {
            // Get the right edit state based on whether we're in import mode or not
            let edit_state = if model.import_state.active {
                &mut model.import_state.edit_state
            } else if model.edit_state.active {
                &mut model.edit_state
            } else {
                return None; // Not in edit mode, ignore
            };

            match edit_state.selected_field {
                crate::model::EditField::Project => handle_autocomplete_navigation(
                    &mut edit_state.project_autocomplete,
                    false,
                ),
                crate::model::EditField::Contact => handle_autocomplete_navigation(
                    &mut edit_state.contact_autocomplete,
                    false,
                ),
                _ => {}
            }
            None
        }

        Message::AutocompleteClearInput => {
            // Get the right edit state based on whether we're in import mode or not
            let edit_state = if model.import_state.active {
                &mut model.import_state.edit_state
            } else if model.edit_state.active {
                &mut model.edit_state
            } else {
                return None; // Not in edit mode, ignore
            };

            match edit_state.selected_field {
                crate::model::EditField::Project => {
                    handle_autocomplete_clear(&mut edit_state.project_autocomplete)
                }
                crate::model::EditField::Contact => {
                    handle_autocomplete_clear(&mut edit_state.contact_autocomplete)
                }
                _ => None,
            }
        }

        Message::AutocompleteResultsProject(projects) => {
            model.edit_state.project_autocomplete.update_items(projects);
            model.log_debug(
                t!(
                    "updated_project_suggestions",
                    count = model.edit_state.project_autocomplete.items.len()
                )
                .to_string(),
            );
            None
        }

        Message::AutocompleteResultsContact(contacts) => {
            model.edit_state.contact_autocomplete.update_items(contacts);
            model.log_debug(
                t!(
                    "updated_contact_suggestions",
                    count = model.edit_state.contact_autocomplete.items.len()
                )
                .to_string(),
            );
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

        Message::UserSelectNext => {
            if !model.users.is_empty() {
                let current_index = model.user_selection_state.selected().unwrap_or(0);
                let next_index = if current_index >= model.users.len() - 1 {
                    0
                } else {
                    current_index + 1
                };
                model.user_selection_state.select(Some(next_index));
            }
            None
        }

        Message::UserSelectPrevious => {
            if !model.users.is_empty() {
                let current_index = model.user_selection_state.selected().unwrap_or(0);
                let prev_index = if current_index == 0 {
                    model.users.len() - 1
                } else {
                    current_index - 1
                };
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

        Message::TimeEntrySelectRow(index) => {
            if index < model.time_entries_for_table.len() {
                model.time_entry_table_state.select(Some(index));
            }
            None
        }

        Message::EditTimeEntryFieldClick(field) => {
            // Get the right edit state based on whether we're in import mode or not
            let edit_state = if model.import_state.active {
                &mut model.import_state.edit_state
            } else if model.edit_state.active {
                &mut model.edit_state
            } else {
                return None; // Not in edit mode, ignore
            };

            update_edit_field_from_editor(edit_state);
            edit_state.selected_field = field;
            initialize_editor_or_autocomplete(edit_state);
            model.log_debug(t!(
                "update_debug_click_field",
                field = format!("{:?}", field)
            ));
            None
        }

        Message::EditTimeEntryKeyPress(key) => {
            // Get the right edit state based on whether we're in import mode or not
            let edit_state = if model.import_state.active {
                &mut model.import_state.edit_state
            } else if model.edit_state.active {
                &mut model.edit_state
            } else {
                return None; // Not in edit mode, ignore
            };

            edit_state.editor.input(key);
            update_edit_field_from_editor(edit_state);
            None
        }

        // Plugin View Messages
        Message::PluginViewShow => {
            model.plugin_view_state.active = true;
            // Select the first plugin if available
            if let Some(plugin_manager) = &model.plugin_manager {
                let plugins = plugin_manager.list_plugins();
                if !plugins.is_empty() {
                    model.plugin_view_state.selected_index = Some(0);
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
                if !plugins.is_empty() {
                    if let Some(index) = model.plugin_view_state.selected_index {
                        // Increment the index, wrapping around if necessary
                        model.plugin_view_state.selected_index = Some((index + 1) % plugins.len());
                    } else {
                        // If no plugin is selected, select the first one
                        model.plugin_view_state.selected_index = Some(0);
                    }
                }
            }
            None
        }
        Message::PluginViewSelectPrevious => {
            if let Some(plugin_manager) = &model.plugin_manager {
                let plugins = plugin_manager.list_plugins();
                if !plugins.is_empty() {
                    if let Some(index) = model.plugin_view_state.selected_index {
                        // Decrement the index, wrapping around if necessary
                        model.plugin_view_state.selected_index = Some(
                            if index == 0 {
                                plugins.len() - 1
                            } else {
                                index - 1
                            }
                        );
                    } else {
                        // If no plugin is selected, select the first one
                        model.plugin_view_state.selected_index = Some(0);
                    }
                }
            }
            None
        }

        Message::ImportTimeEntry => {
            if !model.edit_state.active && !is_import_active(model) {
                initialize_time_entry_import(model)
            } else {
                None
            }
        }

        Message::EditSave => {
            if model.import_state.active {
                return handle_import_save(model).await;
            }

            if model.edit_state.active {
                update_edit_field_from_editor(&mut model.edit_state);

                let admin_timezone_str = model
                    .administration
                    .time_zone
                    .clone()
                    .unwrap_or_else(|| "UTC".to_string());
                let time_entry_data = model.edit_state.try_into_time_entry(&admin_timezone_str);

                let is_creating = model.edit_state.time_entry_id.is_none();
                let admin_id = model.administration.id.clone().unwrap_or_default();
                let client = model.client.clone();
                let user_id = model.config.user_id.clone().unwrap_or_default();

                // Make sure description is not empty
                if model.edit_state.description.is_empty() {
                    ui::show_error(model, t!("update_description_required"));
                    return None;
                }

                if is_creating {
                    model.log_notice(format!(
                        "Creating new time entry: {}",
                        model.edit_state.description
                    ));
                    let endpoint = "time_entries.json";
                    crate::api::log_debug_curl(model, endpoint, "POST");

                    match crate::api::create_time_entry(
                        &client,
                        &admin_id,
                        &user_id,
                        time_entry_data,
                    )
                    .await
                    {
                        Ok(created_entry) => {
                            model.log_success(
                                t!(
                                    "time_entry_created_successfully",
                                    id = created_entry.id.clone().unwrap_or_default()
                                )
                                .to_string(),
                            );
                            ui::show_info(
                                model,
                                "create_success",
                                t!("success").to_string(),
                                t!("time_entry_was_created_successfully").to_string(),
                            );
                            get_time_entries(model).await;
                        }
                        Err(err) => {
                            let error_msg =
                                t!("update_failed_create_time_entry", error = err.to_string())
                                    .to_string();
                            model.log_error(error_msg.clone());
                            crate::ui::show_error(model, error_msg);
                        }
                    }
                } else if let Some(entry_id) = &model.edit_state.time_entry_id {
                    let entry_id = entry_id.clone();
                    model.log_notice(t!(
                        "updating_time_entry_notice",
                        entry_id = entry_id.clone(),
                        description = model.edit_state.description.clone()
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
                        Ok(updated_entry) => {
                            if let Some(index) = model
                                .time_entries
                                .iter()
                                .position(|entry| entry.id == Some(entry_id.clone()))
                            {
                                if let Some(table_index) = model
                                    .time_entries_for_table
                                    .iter()
                                    .position(|t_entry| t_entry.id == entry_id)
                                {
                                    model.time_entries[index] = updated_entry.clone();
                                    model.time_entries_for_table[table_index] = TimeEntryForTable {
                                        id: updated_entry.id.clone().unwrap_or_default(),
                                        customer: updated_entry
                                            .contact
                                            .clone()
                                            .unwrap_or_default()
                                            .company_name
                                            .clone()
                                            .unwrap_or_default(),
                                        project: updated_entry
                                            .project
                                            .clone()
                                            .unwrap_or_default()
                                            .name
                                            .clone()
                                            .unwrap_or_default(),
                                        description: updated_entry
                                            .description
                                            .clone()
                                            .unwrap_or_default(),
                                        started_at: updated_entry
                                            .started_at
                                            .clone()
                                            .unwrap_or_default(),
                                        ended_at: updated_entry
                                            .ended_at
                                            .clone()
                                            .unwrap_or_default(),
                                        billable: updated_entry.billable.unwrap_or_default(),
                                        source: "moneybird".to_string(),
                                        icon: None,
                                    };
                                    if let Some(backup_index) = model
                                        .time_entries_for_table_backup
                                        .iter()
                                        .position(|t_entry| t_entry.id == entry_id)
                                    {
                                        model.time_entries_for_table_backup[backup_index] =
                                            model.time_entries_for_table[table_index].clone();
                                    }
                                }
                            }
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
                        }
                        Err(err) => {
                            let error_msg =
                                t!("update_failed_update_time_entry", error = err.to_string())
                                    .to_string();
                            model.log_error(error_msg.clone());
                            crate::ui::show_error(model, error_msg);
                        }
                    }
                }
                model.edit_state = EditState::default();
            }
            None
        }

        Message::EditCancel => {
            if model.import_state.active {
                model.import_state.active = false;
                model.import_state.edit_state = EditState::default();
                model.import_state.original_entry = None;
                return None;
            }

            if model.edit_state.active {
                model.edit_state.active = false;
                model.log_notice(t!("canceled_editing").to_string());
            }
            None
        }

    }
}
