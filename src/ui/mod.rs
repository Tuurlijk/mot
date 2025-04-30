pub(crate) mod autocomplete;
pub(crate) mod color;
pub(crate) mod modal;
mod render;
pub(crate) mod shortcuts;

pub use autocomplete::Autocomplete;
pub use color::*;
pub use modal::*;
pub use render::*;
use rust_i18n::t;
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
        t!("ui_unnamed_contact").to_string()
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
            .unwrap_or_else(|| t!("ui_undefined").to_string()),
        None => t!("ui_undefined").to_string(),
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
            .unwrap_or_else(|| t!("ui_undefined").to_string()),
        None => t!("ui_undefined").to_string(),
    }
}

/// Generate a default icon based on a name, ensuring consistency across the application
pub fn get_default_icon(name: &str) -> String {
    // Available default icons (colored circles)
    let plugin_icons = ["🔴", "🟣", "🟡", "🟠", "🟢", "⚫", "⚪", "🟤"];

    // Normalize the string more aggressively to handle similar names
    // 1. Convert to lowercase
    // 2. Remove spaces, hyphens, and underscores
    // 3. Keep only alphanumeric characters
    let normalized_name = name
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    if normalized_name.is_empty() {
        return plugin_icons[0].to_string(); // Default for empty names
    }

    // Use a better hashing approach that produces more distinct results
    // We'll use a simple but effective hash algorithm (djb2)
    let mut hash: u32 = 5381;
    for c in normalized_name.chars() {
        // hash * 33 + c
        hash = hash.wrapping_mul(33).wrapping_add(c as u32);
    }

    plugin_icons[(hash % plugin_icons.len() as u32) as usize].to_string()
}
