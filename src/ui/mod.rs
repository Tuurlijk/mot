pub(crate) mod autocomplete;
pub(crate) mod color;
pub(crate) mod modal;
mod render;
pub(crate) mod shortcuts;

pub use autocomplete::Autocomplete;
pub use color::*;
pub use modal::*;
pub use render::*;
pub use shortcuts::*;

/// Format a contact name for display, prioritizing company name and falling back to person name if needed
pub(crate) fn format_contact_name(contact: &crate::moneybird::types::Contact) -> String {
    // First try company name
    if let Some(company) = &contact.company_name {
        if !company.is_empty() {
            return company.clone();
        }
    }

    // If no company name, try to build a person name from first and last name
    let first = contact.firstname.clone().unwrap_or_default();
    let last = contact.lastname.clone().unwrap_or_default();

    if !first.is_empty() || !last.is_empty() {
        if !first.is_empty() && !last.is_empty() {
            format!("{} {}", first, last)
        } else {
            format!("{}{}", first, last)
        }
    } else {
        // No name information available
        "Unnamed Contact".to_string()
    }
}

/// Get a project name from the list of projects by project ID, returning "Undefined" if not found
pub(crate) fn get_project_name(
    projects: &[crate::moneybird::types::Project],
    project_id: Option<&String>,
) -> String {
    match project_id {
        Some(id) => projects
            .iter()
            .find(|p| p.id.as_ref().unwrap_or(&"".to_string()) == id)
            .map(|p| p.name.clone().unwrap_or_default())
            .unwrap_or_else(|| "Undefined".to_string()),
        None => "Undefined".to_string(),
    }
}

/// Get a contact name from the list of contacts by contact ID, returning "Undefined" if not found
pub(crate) fn get_contact_name(
    contacts: &[crate::moneybird::types::Contact],
    contact_id: Option<&String>,
) -> String {
    match contact_id {
        Some(id) => contacts
            .iter()
            .find(|c| c.id.as_ref().unwrap_or(&"".to_string()) == id)
            .map(format_contact_name)
            .unwrap_or_else(|| "Undefined".to_string()),
        None => "Undefined".to_string(),
    }
}
