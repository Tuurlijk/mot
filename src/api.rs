use crate::config::Configuration;
use crate::moneybird::types::{Contact, Project, TimeEntry, User};
use crate::moneybird::{self, types::Administration};
use crate::{datetime, AppModel, TimeEntryForTable};
use color_eyre::eyre::Result;
use reqwest::Response;
use serde_json::Value;

pub(crate) fn create_moneybird_client(client_config: &Configuration) -> moneybird::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", &client_config.access_token)
            .parse()
            .unwrap(),
    );

    moneybird::Client::new_with_client(
        &client_config.api_url,
        reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap(),
    )
}

/// Handles API response errors and returns a formatted error message
pub(crate) async fn handle_api_error(response: Response, context: &str) -> Result<String> {
    let status = response.status();
    let url = response.url().to_string();

    let error_text = match response.text().await {
        Ok(text) => {
            // Try to parse as JSON for better error messages
            match serde_json::from_str::<Value>(&text) {
                Ok(json) => {
                    if let Some(errors) = json.get("errors") {
                        format!("{}: {}", status, serde_json::to_string_pretty(errors)?)
                    } else if let Some(error) = json.get("error") {
                        // Moneybird returns validation errors in format {"error": {"field": ["error message"]}}
                        if error.is_object() {
                            let mut error_messages = Vec::new();
                            for (field, msgs) in error.as_object().unwrap() {
                                if msgs.is_array() {
                                    for msg in msgs.as_array().unwrap() {
                                        if msg.is_string() {
                                            error_messages.push(format!(
                                                "{}: {}",
                                                field,
                                                msg.as_str().unwrap()
                                            ));
                                        }
                                    }
                                } else if msgs.is_string() {
                                    error_messages.push(format!(
                                        "{}: {}",
                                        field,
                                        msgs.as_str().unwrap()
                                    ));
                                }
                            }
                            if !error_messages.is_empty() {
                                format!("{}: {}", status, error_messages.join(", "))
                            } else {
                                format!("{}: {}", status, serde_json::to_string_pretty(error)?)
                            }
                        } else {
                            format!("{}: {}", status, serde_json::to_string_pretty(error)?)
                        }
                    } else {
                        format!("{}: {}", status, text)
                    }
                }
                Err(_) => format!("{}: {}", status, text),
            }
        }
        Err(_) => format!("{}", status),
    };

    // Add information about the URL that was called
    let error_with_url = format!("{}\nAPI URL: {}", error_text, url);

    Err(color_eyre::eyre::eyre!("{}: {}", context, error_with_url))
}

pub(crate) async fn get_first_administration(client: &moneybird::Client) -> Result<Administration> {
    match client.get_administrations().send().await {
        Ok(response) => {
            let administrations = response.into_inner();
            administrations
                .first()
                .cloned()
                .ok_or_else(|| color_eyre::eyre::eyre!("No administrations found"))
        }
        Err(err) => {
            // Since we can't directly access the response, we'll use a generic error message
            Err(color_eyre::eyre::eyre!(
                "Failed to get administrations: {}",
                err
            ))
        }
    }
}

pub(crate) async fn get_administration_by_id(
    client: &moneybird::Client,
    administration_id: &str,
) -> Result<Administration> {
    match client.get_administrations().send().await {
        Ok(response) => {
            let administrations = response.into_inner();
            administrations
                .into_iter()
                .find(|administration| {
                    administration.id.clone().unwrap_or_default() == administration_id
                })
                .ok_or_else(|| {
                    color_eyre::eyre::eyre!(
                        "Administration with ID {} not found",
                        administration_id
                    )
                })
        }
        Err(err) => Err(color_eyre::eyre::eyre!(
            "Failed to get administrations: {}",
            err
        )),
    }
}

pub(crate) async fn get_all_contacts(
    client: &moneybird::Client,
    administration_id: &str,
) -> Result<Vec<Contact>> {
    match client
        .get_contacts()
        .administration_id(administration_id)
        .send()
        .await
    {
        Ok(response) => Ok(response
            .into_inner()
            .into_iter()
            .map(|contact| contact.into())
            .collect()),
        Err(err) => {
            let endpoint = "contacts.json";
            let context = format!(
                "Failed to get contacts for administration {}",
                administration_id
            );
            handle_moneybird_error(err, &context, endpoint, "GET", administration_id).await?;
            unreachable!();
        }
    }
}

pub(crate) async fn get_contact_by_id(
    client: &moneybird::Client,
    administration_id: &str,
    contact_id: &str,
) -> Result<Contact> {
    match client
        .get_contact()
        .administration_id(administration_id)
        .contact_id(contact_id)
        .send()
        .await
    {
        Ok(response) => Ok(response.into_inner().into()),
        Err(err) => {
            let endpoint = format!("contacts/{}.json", contact_id);
            let context = format!("Failed to get contact {}", contact_id);
            handle_moneybird_error(err, &context, &endpoint, "GET", administration_id).await?;
            unreachable!();
        }
    }
}

pub(crate) async fn get_contacts_by_query(
    client: &moneybird::Client,
    administration_id: &str,
    query: &str,
) -> Result<Vec<Contact>> {
    match client
        .get_contacts()
        .administration_id(administration_id)
        .query(query)
        .send()
        .await
    {
        Ok(response) => Ok(response
            .into_inner()
            .into_iter()
            .map(|contact| contact.into())
            .collect()),
        Err(err) => {
            let endpoint = format!("contacts.json?query={}", query);
            let context = format!("Failed to get contacts by query {}", query);
            handle_moneybird_error(err, &context, &endpoint, "GET", administration_id).await?;
            unreachable!();
        }
    }
}

pub(crate) async fn get_all_projects(
    client: &moneybird::Client,
    administration_id: &str,
) -> Result<Vec<Project>> {
    match client
        .get_projects()
        .administration_id(administration_id)
        .send()
        .await
    {
        Ok(response) => Ok(response
            .into_inner()
            .into_iter()
            .map(|project| project.into())
            .collect()),
        Err(err) => {
            let endpoint = "projects.json";
            let context = format!(
                "Failed to get projects for administration {}",
                administration_id
            );
            handle_moneybird_error(err, &context, endpoint, "GET", administration_id).await?;
            unreachable!();
        }
    }
}

pub(crate) async fn get_project_by_id(
    client: &moneybird::Client,
    administration_id: &str,
    project_id: &str,
) -> Result<Project> {
    match client
        .get_project()
        .administration_id(administration_id)
        .project_id(project_id)
        .send()
        .await
    {
        Ok(response) => Ok(response.into_inner().into()),
        Err(err) => {
            let endpoint = format!("projects/{}.json", project_id);
            let context = format!("Failed to get project {}", project_id);
            handle_moneybird_error(err, &context, &endpoint, "GET", administration_id).await?;
            unreachable!();
        }
    }
}

pub(crate) async fn get_time_entries_by_date_range(
    client: &moneybird::Client,
    administration_id: &str,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<TimeEntry>> {
    // Create a date range filter for the MoneyBird API
    let filter = match create_date_range_filter(start_date, end_date) {
        Ok(filter) => filter,
        Err(err) => return Err(err),
    };

    match client
        .get_time_entries()
        .administration_id(administration_id)
        .filter(&filter)
        .send()
        .await
    {
        Ok(response) => {
            let time_entries: Vec<TimeEntry> = response
                .into_inner()
                .into_iter()
                .map(|time_entry| time_entry.into())
                .collect();

            Ok(time_entries)
        }
        Err(err) => {
            let endpoint = format!("time_entries.json?filter={}", filter);
            let context = format!(
                "Failed to get time entries for date range {}-{}",
                start_date, end_date
            );
            handle_moneybird_error(err, &context, &endpoint, "GET", administration_id).await?;
            unreachable!();
        }
    }
}

pub(crate) async fn check_connectivity(client: &moneybird::Client) -> Result<(), String> {
    match client.get_administrations().send().await {
        Ok(_) => Ok(()),
        Err(err) => {
            // Try to get the underlying error details from the error message
            let err_msg = err.to_string();

            // Check for common connection issues based on the error message
            if err_msg.contains("connection refused")
                || err_msg.contains("dns error")
                || err_msg.contains("network")
                || err_msg.contains("tcp connect error")
            {
                return Err(
                    "Network connection error. Please check your internet connection.".to_string(),
                );
            }

            if err_msg.contains("timeout") {
                return Err(
                    "Connection timed out. The server took too long to respond.".to_string()
                );
            }

            if err_msg.contains("401") || err_msg.contains("403") {
                return Err("Authentication error. Please check your API token.".to_string());
            }

            if err_msg.contains("404") {
                return Err("Resource not found. Please check your configuration.".to_string());
            }

            if err_msg.contains("5") && (err_msg.contains("status") || err_msg.contains("code")) {
                return Err("MoneyBird server error. Please try again later.".to_string());
            }

            // Generic error if we can't determine the specific cause
            Err(format!("API connection error: {}", err))
        }
    }
}

/// Helper function to log a debug curl command to the application logs
pub(crate) fn log_debug_curl(model: &mut AppModel, endpoint: &str, method: &str) {
    let admin_id = model.administration.id.clone().unwrap_or_default();
    let token = &model.config.access_token;

    // Generate the curl command
    let curl_cmd = generate_debug_curl(endpoint, method, &admin_id, token);

    // Log to the application logs
    model.log_debug(format!("Debug API call with: {}", curl_cmd));
}

/// Helper function to fetch time entries for the current date range
pub(crate) async fn get_time_entries(model: &mut AppModel) {
    let admin_id = model.administration.id.clone().unwrap_or_default();

    // Get administration timezone, default to UTC if not set
    let admin_timezone_str = model
        .administration
        .time_zone
        .clone()
        .unwrap_or_else(|| "UTC".to_string());

    // Calculate week range using the week_offset and configured week start day
    let week_range = datetime::get_week_range_strings(
        model.week_offset,
        &admin_timezone_str,
        &model.config.week_starts_on,
    );

    // Create a date range filter for the MoneyBird API
    let filter = match create_date_range_filter(&week_range.0, &week_range.1) {
        Ok(filter) => filter,
        Err(err) => {
            // If we couldn't create a filter, show an error
            crate::ui::show_error(model, format!("Failed to create date filter: {}", err));
            model.log_error(format!("Failed to create date filter: {}", err));
            return;
        }
    };

    // Prepare debug information in case of failure
    let endpoint = format!("time_entries.json?filter={}", filter);
    log_debug_curl(model, &endpoint, "GET");

    // Get time entries for the date range
    match crate::api::get_time_entries_by_date_range(
        &model.client,
        &admin_id,
        &week_range.0,
        &week_range.1,
    )
    .await
    {
        Ok(entries) => {
            model.time_entries = entries.clone();

            // Fetch missing contacts and projects
            fetch_missing_contacts_and_projects(model, &admin_id).await;

            // Now populate the time_entries_for_table
            model.time_entries_for_table = entries
                .iter()
                .map(|entry| TimeEntryForTable {
                    id: entry.id.clone().unwrap_or_default(),
                    customer: entry
                        .contact
                        .clone()
                        .unwrap_or_default()
                        .company_name
                        .clone()
                        .unwrap_or_default(),
                    project: entry
                        .project
                        .clone()
                        .unwrap_or_default()
                        .name
                        .clone()
                        .unwrap_or_default(),
                    description: entry.description.clone().unwrap_or_default(),
                    started_at: entry.started_at.clone().unwrap_or_default(),
                    ended_at: entry.ended_at.clone().unwrap_or_default(),
                    billable: entry.billable.unwrap_or_default(),
                })
                .collect();

            model.time_entries_for_table_backup = model.time_entries_for_table.clone();

            if model.search_state.active {
                model.filter_items();
            }

            // New entries are loaded, so select the first entry
            if !model.time_entries_for_table.is_empty() {
                model.time_entry_table_state.select(Some(0));
            } else {
                model.time_entry_table_state.select(None);
            }

            model.log_success(format!(
                "Fetched {} time entries for the selected week.",
                entries.len()
            ));
            for entry in &entries {
                model.log_success(format!(
                    "Entry: {:?} {:?}",
                    entry.id.clone().unwrap_or_default(),
                    entry.started_at.clone().unwrap_or_default()
                ));
            }
        }
        Err(err) => {
            // If we couldn't fetch entries, clear the list and show error
            crate::ui::show_error(model, format!("Failed to fetch time entries: {}", err));
            model.log_error(format!("Failed to fetch time entries: {}", err));
            model.time_entries = Vec::new();
            model.time_entries_for_table = Vec::new();
            model.time_entries_for_table_backup = Vec::new();
            model.time_entry_table_state.select(None);
        }
    }
}

/// Helper function to fetch any missing contacts and projects for time entries
pub(crate) async fn fetch_missing_contacts_and_projects(model: &mut AppModel, admin_id: &str) {
    // Track errors to display them after all fetching is done
    let mut errors = Vec::new();

    // The get_all_contacts call only returns the first 100 contacts, so we need to fetch the contacts we need for the time entries if they are not in the first 100
    // The get_all_projects call only returns the first 100 projects, so we need to fetch the projects we need for the time entries if they are not in the first 100
    for time_entry in &model.time_entries.clone() {
        if let Some(contact_id) = &time_entry.contact_id {
            if !model
                .contacts
                .iter()
                .any(|c| c.id.clone().unwrap_or_default() == *contact_id)
            {
                match crate::api::get_contact_by_id(
                    &model.client,
                    admin_id,
                    &contact_id.to_string(),
                )
                .await
                {
                    Ok(contact) => {
                        model.log_success(format!(
                            "Fetched contact with ID: {}; name: {}",
                            contact_id,
                            contact.company_name.clone().unwrap_or_default()
                        ));
                        model.contacts.push(contact);
                    }
                    Err(err) => {
                        errors.push(format!(
                            "Failed to fetch contact with ID {}: {}",
                            contact_id, err
                        ));
                    }
                }
            }
        }
        if let Some(project_id) = &time_entry.project_id {
            if !model
                .projects
                .iter()
                .any(|c| c.id.clone().unwrap_or_default() == *project_id)
            {
                match crate::api::get_project_by_id(
                    &model.client,
                    admin_id,
                    &project_id.to_string(),
                )
                .await
                {
                    Ok(project) => {
                        model.log_success(format!(
                            "Fetched project with ID: {}; name: {}",
                            project_id,
                            project.name.clone().unwrap_or_default()
                        ));
                        model.projects.push(project);
                    }
                    Err(err) => {
                        errors.push(format!(
                            "Failed to fetch project with ID {}: {}",
                            project_id, err
                        ));
                    }
                }
            }
        }
    }

    // Display errors if any
    if !errors.is_empty() {
        // Just show the first error to avoid overwhelming the user
        if errors.len() == 1 {
            model.log_error(errors[0].clone());
            crate::ui::show_error(model, errors[0].clone());
        } else {
            model.log_error(format!(
                "Failed to fetch {} items. First error: {}",
                errors.len(),
                errors[0]
            ));
            crate::ui::show_error(
                model,
                format!(
                    "Failed to fetch {} items. First error: {}",
                    errors.len(),
                    errors[0]
                ),
            );
        }
    }
}

/// Generate a curl command that can be used to debug API calls
pub(crate) fn generate_debug_curl(
    endpoint: &str,
    method: &str,
    administration_id: &str,
    token: &str,
) -> String {
    // Mask the token for security
    let visible_token_part = if token.len() > 8 {
        format!("{}...{}", &token[0..4], &token[token.len() - 4..])
    } else {
        "****".to_string()
    };

    format!(
        "curl -s -H \"Content-Type: application/json\" -H \"Authorization: Bearer {}\" \\\n  -X{} \\\n  https://moneybird.com/api/v2/{}/{}",
        visible_token_part,
        method,
        administration_id,
        endpoint
    )
}

pub(crate) async fn delete_time_entry_by_id(
    client: &moneybird::Client,
    administration_id: &str,
    time_entry_id: &str,
) -> Result<()> {
    match client
        .delete_time_entry()
        .administration_id(administration_id)
        .time_entry_id(time_entry_id)
        .send()
        .await
    {
        Ok(response) => {
            response.into_inner();
            Ok(())
        }
        Err(err) => {
            let endpoint = format!("time_entries/{}.json", time_entry_id);
            let context = format!("Failed to delete time entry {}", time_entry_id);
            handle_moneybird_error(err, &context, &endpoint, "DELETE", administration_id).await?;
            unreachable!();
        }
    }
}

pub(crate) async fn create_time_entry(
    client: &moneybird::Client,
    administration_id: &str,
    user_id: &str,
    time_entry: TimeEntry,
) -> Result<TimeEntry> {
    match client
        .create_time_entry()
        .administration_id(administration_id)
        .body(crate::moneybird::types::TimeEntryCreate {
            time_entry: crate::moneybird::types::TimeEntryCreateTimeEntry {
                billable: time_entry.billable,
                contact_id: time_entry.contact_id,
                started_at: time_entry.started_at.unwrap_or_default(),
                description: time_entry.description.unwrap_or_default(),
                detail_id: None,
                ended_at: time_entry.ended_at.unwrap_or_default(),
                paused_duration: time_entry.paused_duration,
                project_id: time_entry.project_id,
                user_id: user_id.to_string(),
            },
        })
        .send()
        .await
    {
        Ok(response) => Ok(response.into_inner().into()),
        Err(err) => {
            let endpoint = "time_entries.json".to_string();
            let context = "Failed to create time entry".to_string();
            handle_moneybird_error(err, &context, &endpoint, "POST", administration_id).await?;
            unreachable!();
        }
    }
}

pub(crate) async fn get_time_entry_by_id(
    client: &moneybird::Client,
    administration_id: &str,
    time_entry_id: &str,
) -> Result<TimeEntry> {
    match client
        .get_time_entry()
        .administration_id(administration_id)
        .time_entry_id(time_entry_id)
        .send()
        .await
    {
        Ok(response) => Ok(response.into_inner().into()),
        Err(err) => {
            let endpoint = format!("time_entries/{}.json", time_entry_id);
            let context = format!("Failed to get time entry {}", time_entry_id);
            handle_moneybird_error(err, &context, &endpoint, "GET", administration_id).await?;
            unreachable!();
        }
    }
}

pub(crate) async fn update_time_entry_by_id(
    client: &moneybird::Client,
    administration_id: &str,
    time_entry_id: &str,
    time_entry: TimeEntry,
) -> Result<TimeEntry> {
    match client
        .update_time_entry()
        .administration_id(administration_id)
        .time_entry_id(time_entry_id)
        .body(crate::moneybird::types::TimeEntryUpdate {
            time_entry: crate::moneybird::types::TimeEntryUpdateTimeEntry {
                administration_id: time_entry.administration_id,
                billable: time_entry.billable,
                contact_id: time_entry.contact_id,
                contact: time_entry.contact,
                created_at: time_entry.created_at,
                description: time_entry.description,
                detail_id: None,
                ended_at: time_entry.ended_at.unwrap_or_default(),
                events: time_entry.events,
                id: time_entry.id,
                notes: time_entry.notes,
                paused_duration: time_entry.paused_duration,
                project_id: time_entry.project_id,
                project: time_entry.project,
                started_at: time_entry.started_at,
                updated_at: time_entry.updated_at,
                user_id: time_entry.user_id,
            },
        })
        .send()
        .await
    {
        Ok(response) => Ok(response.into_inner().into()),
        Err(err) => {
            let endpoint = format!("time_entries/{}.json", time_entry_id);
            let context = format!("Failed to update time entry {}", time_entry_id);

            // For PATCH requests, we want to provide a more helpful error message with
            // sample payload data structure
            if let moneybird::Error::UnexpectedResponse(response) = err {
                handle_api_error(response, &context).await?;
                unreachable!();
            } else {
                // Use the common error handler for other types of errors
                handle_moneybird_error(err, &context, &endpoint, "PATCH", administration_id)
                    .await?;
                unreachable!();
            }
        }
    }
}

/// Common error handling for API requests
/// This abstracts away the boilerplate code for handling errors from the Moneybird API
pub(crate) async fn handle_moneybird_error<T: std::fmt::Debug>(
    err: moneybird::Error<T>,
    context: &str,
    endpoint: &str,
    method: &str,
    administration_id: &str,
) -> Result<()> {
    // Generate debug curl command with a placeholder token
    let token = "YOUR-TOKEN-HERE";
    let curl_cmd = generate_debug_curl(endpoint, method, administration_id, token);

    // Log the curl command for debugging
    eprintln!("Debug with: {}", curl_cmd);

    // Extract the response body if it's an UnexpectedResponse error
    if let moneybird::Error::UnexpectedResponse(response) = err {
        let context = format!("{}. Debug with:\n{}", context, curl_cmd);
        handle_api_error(response, &context).await?;
        unreachable!();
    } else if let moneybird::Error::ErrorResponse(response_value) = err {
        let status = response_value.status();
        let error_message = format!("API error: {}", status);
        return Err(color_eyre::eyre::eyre!(
            "{}: {}. Debug with:\n{}",
            context,
            error_message,
            curl_cmd
        ));
    } else {
        return Err(color_eyre::eyre::eyre!(
            "{}: {}. Debug with:\n{}",
            context,
            err,
            curl_cmd
        ));
    }
}

/// Format a date string for use with the MoneyBird API
/// Converts ISO dates like "2023-01-31T12:00:00Z" to the format MoneyBird expects: "20230131"
pub(crate) fn format_date_for_moneybird(date_str: &str) -> Result<String> {
    let formatted = date_str.replace(['-', ':', '.', 'T', 'Z'], "");
    let formatted = match formatted.split('+').next() {
        Some(s) => s.to_string(),
        None => date_str.to_string(),
    };

    // Check if we have enough characters to slice (at least YYYYMMDD)
    if formatted.len() < 8 {
        return Err(color_eyre::eyre::eyre!("Invalid date format: {}", date_str));
    }

    // Take only the date part (first 8 chars of formatted string - YYYYMMDD)
    Ok(formatted[..8].to_string())
}

/// Create a date range filter for the MoneyBird API
pub(crate) fn create_date_range_filter(start_date: &str, end_date: &str) -> Result<String> {
    let start_date_formatted = format_date_for_moneybird(start_date)?;
    let end_date_formatted = format_date_for_moneybird(end_date)?;

    // Final format: "period:YYYYMMDD..YYYYMMDD"
    Ok(format!(
        "period:{}..{}",
        start_date_formatted, end_date_formatted
    ))
}

pub(crate) async fn get_all_users(
    client: &moneybird::Client,
    administration_id: &str,
) -> Result<Vec<User>> {
    match client
        .get_users()
        .administration_id(administration_id)
        .send()
        .await
    {
        Ok(response) => Ok(response.into_inner().into_iter().collect()),
        Err(err) => {
            let endpoint = "users.json";
            let context = format!(
                "Failed to get users for administration {}",
                administration_id
            );
            handle_moneybird_error(err, &context, endpoint, "GET", administration_id).await?;
            unreachable!();
        }
    }
}
