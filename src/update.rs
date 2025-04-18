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
};

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
async fn handle_autocomplete_refresh_project(model: &mut AppModel) {
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
}

// Helper function to refresh contact autocomplete suggestions (API call)
async fn handle_autocomplete_refresh_contact(model: &mut AppModel) {
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
                model.log_error(error_msg);
                model.edit_state.contact_autocomplete.update_items(vec![]);
            }
        }
        model.edit_state.contact_autocomplete.is_loading = false;
    }
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
                            model.edit_state.entry_id = selected_entry.id.clone();

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
                            model.edit_state.project_name =
                                orig_entry.project.clone().unwrap_or_default().name.clone();
                            model.edit_state.contact_id = orig_entry.contact_id.clone();
                            model.edit_state.contact_name = orig_entry
                                .contact
                                .clone()
                                .unwrap_or_default()
                                .company_name
                                .clone();

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

                let is_creating = model.edit_state.entry_id.is_empty();
                let admin_id = model.administration.id.clone().unwrap_or_default();
                let client = model.client.clone();
                let user_id = model.config.user_id.clone().unwrap_or_default();

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
                } else {
                    let entry_id = model.edit_state.entry_id.clone();
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
            if model.edit_state.active {
                const FIELD_ORDER: &[crate::model::EditField] = &[
                    crate::model::EditField::Description,
                    crate::model::EditField::Contact,
                    crate::model::EditField::Project,
                    crate::model::EditField::StartTime,
                    crate::model::EditField::EndTime,
                    crate::model::EditField::StartDate,
                    crate::model::EditField::EndDate,
                ];
                update_edit_field_from_editor(&mut model.edit_state);
                if let Some(current_index) = FIELD_ORDER
                    .iter()
                    .position(|&field| field == model.edit_state.selected_field)
                {
                    let next_index = (current_index + 1) % FIELD_ORDER.len();
                    model.edit_state.selected_field = FIELD_ORDER[next_index];
                    initialize_editor_or_autocomplete(&mut model.edit_state);
                }
            }
            None
        }

        Message::EditTimeEntryPreviousField => {
            if model.edit_state.active {
                const FIELD_ORDER: &[crate::model::EditField] = &[
                    crate::model::EditField::Description,
                    crate::model::EditField::Contact,
                    crate::model::EditField::Project,
                    crate::model::EditField::StartTime,
                    crate::model::EditField::EndTime,
                    crate::model::EditField::StartDate,
                    crate::model::EditField::EndDate,
                ];
                update_edit_field_from_editor(&mut model.edit_state);
                if let Some(current_index) = FIELD_ORDER
                    .iter()
                    .position(|&field| field == model.edit_state.selected_field)
                {
                    let prev_index = if current_index == 0 {
                        FIELD_ORDER.len() - 1
                    } else {
                        current_index - 1
                    };
                    model.edit_state.selected_field = FIELD_ORDER[prev_index];
                    initialize_editor_or_autocomplete(&mut model.edit_state);
                }
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
            if !model.edit_state.active {
                return None;
            }
            match model.edit_state.selected_field {
                crate::model::EditField::Project => handle_autocomplete_keypress(
                    &mut model.edit_state.project_autocomplete,
                    key.code,
                ),
                crate::model::EditField::Contact => handle_autocomplete_keypress(
                    &mut model.edit_state.contact_autocomplete,
                    key.code,
                ),
                _ => return None,
            }
            Some(Message::AutocompleteRefresh)
        }

        Message::AutocompleteSelect => {
            if !model.edit_state.active {
                return None;
            }
            match model.edit_state.selected_field {
                crate::model::EditField::Project => handle_autocomplete_select_project(model),
                crate::model::EditField::Contact => handle_autocomplete_select_contact(model),
                _ => {}
            }
            None
        }

        Message::AutocompleteRefresh => {
            if !model.edit_state.active {
                return None;
            }
            match model.edit_state.selected_field {
                crate::model::EditField::Project => {
                    handle_autocomplete_refresh_project(model).await
                }
                crate::model::EditField::Contact => {
                    handle_autocomplete_refresh_contact(model).await
                }
                _ => {}
            }
            None
        }

        Message::AutocompleteNextItem => {
            if !model.edit_state.active {
                return None;
            }
            match model.edit_state.selected_field {
                crate::model::EditField::Project => {
                    handle_autocomplete_navigation(&mut model.edit_state.project_autocomplete, true)
                }
                crate::model::EditField::Contact => {
                    handle_autocomplete_navigation(&mut model.edit_state.contact_autocomplete, true)
                }
                _ => {}
            }
            None
        }

        Message::AutocompletePreviousItem => {
            if !model.edit_state.active {
                return None;
            }
            match model.edit_state.selected_field {
                crate::model::EditField::Project => handle_autocomplete_navigation(
                    &mut model.edit_state.project_autocomplete,
                    false,
                ),
                crate::model::EditField::Contact => handle_autocomplete_navigation(
                    &mut model.edit_state.contact_autocomplete,
                    false,
                ),
                _ => {}
            }
            None
        }

        Message::AutocompleteClearInput => {
            if !model.edit_state.active {
                return None;
            }
            match model.edit_state.selected_field {
                crate::model::EditField::Project => {
                    handle_autocomplete_clear(&mut model.edit_state.project_autocomplete)
                }
                crate::model::EditField::Contact => {
                    handle_autocomplete_clear(&mut model.edit_state.contact_autocomplete)
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
            model.edit_state.entry_id = String::new();
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
            if model.edit_state.active {
                update_edit_field_from_editor(&mut model.edit_state);
                model.edit_state.selected_field = field;
                initialize_editor_or_autocomplete(&mut model.edit_state);
                model.log_debug(t!(
                    "update_debug_click_field",
                    field = format!("{:?}", field)
                ));
            }
            None
        }

        Message::EditTimeEntryKeyPress(key) => {
            if model.edit_state.active {
                model.edit_state.editor.input(key);
                update_edit_field_from_editor(&mut model.edit_state);
            }
            None
        }
    }
}
