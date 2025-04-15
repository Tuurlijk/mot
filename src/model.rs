use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::Tz;
use ratatui::{
    style::{Color, Style},
    widgets::TableState,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
};
use supports_color::ColorLevel;
use tui_textarea::TextArea;

use crate::{
    config::Configuration,
    moneybird::{
        self,
        types::{Administration, TimeEntry, User},
    },
    ui,
};

// Add the import for AutocompleteState
pub use crate::ui::autocomplete::AutocompleteState;

// Re-export Moneybird types needed publicly
pub use crate::moneybird::types::{Contact, Project};

// Define the LogEntry struct
#[derive(Clone, Debug)]
pub(crate) struct LogEntry {
    pub(crate) timestamp: DateTime<Local>,
    pub(crate) severity: LogSeverity,
    pub(crate) message: String,
}

// Define log severity levels
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LogSeverity {
    Debug,
    Notice,
    Success,
    Warning,
    Error,
}

impl LogSeverity {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            LogSeverity::Debug => "DEBUG",
            LogSeverity::Notice => "NOTICE",
            LogSeverity::Success => "INFO",
            LogSeverity::Warning => "WARNING",
            LogSeverity::Error => "ERROR",
        }
    }
}

#[derive(Clone, Default)]
pub(crate) struct LogPanelState {
    pub(crate) visible: bool,
}

#[derive(Clone, Default)]
pub(crate) struct SearchState {
    pub(crate) active: bool,
    pub(crate) text_input: TextArea<'static>,
}

#[derive(Clone, Default)]
pub(crate) struct ModalStack {
    pub(crate) modals: Vec<ui::ModalData>,
}

impl ModalStack {
    /// Push a new modal onto the stack
    pub(crate) fn push(&mut self, modal: ui::ModalData) {
        self.modals.push(modal);
    }

    /// Pop a modal from the stack
    pub(crate) fn pop(&mut self) -> Option<ui::ModalData> {
        self.modals.pop()
    }

    /// Get the top modal without removing it
    pub(crate) fn top(&self) -> Option<&ui::ModalData> {
        self.modals.last()
    }

    /// Check if the stack is empty
    pub(crate) fn is_empty(&self) -> bool {
        self.modals.is_empty()
    }

    /// Get the number of modals in the stack
    pub(crate) fn len(&self) -> usize {
        self.modals.len()
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub(crate) enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Clone)]
pub(crate) struct Appearance {
    pub(crate) color_support: Option<ColorLevel>,
    pub(crate) color_mode: Mode,
    pub(crate) default_foreground_color: (u8, u8, u8),
    pub(crate) default_foreground_color_indexed: Color,
    pub(crate) default_foreground_color_dimmed: (u8, u8, u8),
    pub(crate) default_foreground_color_dimmed_indexed: Color,
    pub(crate) default_style: Style,
}

impl Default for Appearance {
    fn default() -> Self {
        let default_fg = (0, 0, 0); // Define the default color first
        Self {
            color_support: None,
            color_mode: Mode::Unspecified,
            default_foreground_color: default_fg, // Use the defined value
            default_foreground_color_indexed: Color::Indexed(ui::rgb_to_indexed(
                default_fg, // Use the defined value here too
            )),
            default_foreground_color_dimmed: (0, 0, 0),
            default_foreground_color_dimmed_indexed: Color::Indexed(ui::rgb_to_indexed(
                default_fg, // Use the defined value here too
            )),
            default_style: Style::default(),
        }
    }
}

// Track when the last modal interaction occurred
#[derive(Clone, Copy, Default)]
pub(crate) struct ModalInteraction {
    pub(crate) last_interaction: Option<std::time::Instant>,
    pub(crate) cooldown: std::time::Duration,
}

impl ModalInteraction {
    pub fn new(cooldown_ms: u64) -> Self {
        Self {
            last_interaction: None,
            cooldown: std::time::Duration::from_millis(cooldown_ms),
        }
    }

    pub fn record_interaction(&mut self) {
        self.last_interaction = Some(std::time::Instant::now());
    }

    pub fn is_in_cooldown(&self) -> bool {
        if let Some(last_time) = self.last_interaction {
            std::time::Instant::now().duration_since(last_time) < self.cooldown
        } else {
            false
        }
    }
}

// Track when the last key press occurred for debouncing
#[derive(Clone, Copy, Default)]
pub(crate) struct KeyDebounce {
    pub(crate) last_key_time: Option<std::time::Instant>,
    pub(crate) cooldown: std::time::Duration,
}

impl KeyDebounce {
    pub fn new(cooldown_ms: u64) -> Self {
        Self {
            last_key_time: None,
            cooldown: std::time::Duration::from_millis(cooldown_ms),
        }
    }

    pub fn record_keypress(&mut self) {
        self.last_key_time = Some(std::time::Instant::now());
    }

    pub fn is_in_cooldown(&self) -> bool {
        if let Some(last_time) = self.last_key_time {
            std::time::Instant::now().duration_since(last_time) < self.cooldown
        } else {
            false
        }
    }
}

#[derive(Clone, Default)]
pub(crate) struct TimeEntryForTable {
    pub id: String,
    pub customer: String,
    pub project: String,
    pub description: String,
    pub started_at: String,
    pub ended_at: String,
    pub billable: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct EditState {
    pub(crate) active: bool,
    pub(crate) entry_id: String,    // ID of the time entry being edited
    pub(crate) description: String, // Description text
    pub(crate) project_id: Option<String>, // Selected project ID
    pub(crate) project_name: Option<String>, // Selected project name
    pub(crate) contact_id: Option<String>, // Selected contact ID
    pub(crate) contact_name: Option<String>, // Selected contact name
    pub(crate) start_date: String,  // Start date value
    pub(crate) start_time: String,  // Start time value
    pub(crate) end_date: String,    // End date value
    pub(crate) end_time: String,    // End time value
    pub(crate) editor: TextArea<'static>, // Shared editor for all fields
    pub(crate) selected_field: EditField, // Currently selected field

    // Autocomplete state for project selection
    pub(crate) project_autocomplete: AutocompleteState<Project>,

    // Autocomplete state for contact selection
    pub(crate) contact_autocomplete: AutocompleteState<Contact>,
}

impl EditState {
    /// Convert the EditState to a TimeEntry for updating, interpreting input times in the provided timezone
    pub fn try_into_time_entry(&self, timezone: &str) -> crate::moneybird::types::TimeEntry {
        // Get the timezone object, fallback to UTC
        let tz: Tz = timezone.parse().unwrap_or(chrono_tz::UTC);

        // Parse the local date and time strings to create NaiveDateTime objects
        let start_date =
            NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").unwrap_or_else(|_| {
                // Fallback to today if parsing fails (Consider a better fallback?)
                Utc::now().with_timezone(&tz).date_naive()
            });

        let start_time =
            NaiveTime::parse_from_str(&self.start_time, "%H:%M").unwrap_or_else(|_| {
                // Fallback to midnight if parsing fails
                NaiveTime::from_hms_opt(0, 0, 0).unwrap()
            });

        let end_date = NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d").unwrap_or_else(|_| {
            // Fallback to today if parsing fails (Consider a better fallback?)
            Utc::now().with_timezone(&tz).date_naive()
        });

        let end_time = NaiveTime::parse_from_str(&self.end_time, "%H:%M").unwrap_or_else(|_| {
            // Fallback to midnight if parsing fails
            NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        });

        // Combine date and time into NaiveDateTime
        let start_naive = NaiveDateTime::new(start_date, start_time);
        let end_naive = NaiveDateTime::new(end_date, end_time);

        // Interpret the NaiveDateTime using the administration's timezone
        let start_in_admin_tz = tz
            .from_local_datetime(&start_naive)
            .single()
            .unwrap_or_else(|| {
                // Handle ambiguity or non-existent times if necessary, e.g., during DST changes
                // For simplicity, fallback to UTC interpretation if ambiguity occurs
                tz.from_utc_datetime(&start_naive)
            });

        let end_in_admin_tz = tz
            .from_local_datetime(&end_naive)
            .single()
            .unwrap_or_else(|| {
                // Handle ambiguity or non-existent times if necessary
                tz.from_utc_datetime(&end_naive)
            });

        // Convert to UTC and format as RFC3339
        let started_at = start_in_admin_tz.with_timezone(&Utc).to_rfc3339();
        let ended_at = end_in_admin_tz.with_timezone(&Utc).to_rfc3339();

        crate::moneybird::types::TimeEntry {
            id: Some(self.entry_id.clone()),
            description: Some(self.description.clone()),
            project_id: self.project_id.clone(),
            project: Some(Project {
                id: self.project_id.clone(),
                name: None,
                budget: None,
                state: None,
            }),
            contact_id: self.contact_id.clone(),
            contact: Some(Contact {
                id: self.contact_id.clone(),
                company_name: None,
                firstname: None,
                lastname: None,
                phone: None,
                address1: None,
                address2: None,
                zipcode: None,
                city: None,
                country: None,
                customer_id: None,
                delivery_method: None,
                direct_debit: None,
                email_ubl: None,
                estimate_workflow_id: None,
                invoice_workflow_id: None,
                sepa_active: None,
                sepa_bic: None,
                sepa_iban: None,
                sepa_iban_account_name: None,
                sepa_mandate_date: None,
                sepa_mandate_id: None,
                sepa_sequence_type: None,
                si_identifier: None,
                si_identifier_type: None,
                tax_number: None,
                bank_account: None,
                chamber_of_commerce: None,
                send_estimates_to_attention: None,
                send_estimates_to_email: None,
                send_invoices_to_attention: None,
                send_invoices_to_email: None,
            }),
            started_at: Some(started_at),
            ended_at: Some(ended_at),
            // Keep other fields as None/default
            administration_id: None,
            billable: None,
            created_at: None,
            events: Vec::new(),
            notes: Vec::new(),
            paused_duration: None,
            updated_at: None,
            user_id: None,
        }
    }
}

impl From<EditState> for crate::moneybird::types::TimeEntry {
    fn from(_edit_state: EditState) -> Self {
        // This From impl might become problematic as it doesn't have timezone context.
        // Consider removing it or requiring timezone. For now, maybe default to UTC?
        // Or perhaps panic, as it shouldn't be called directly without context?
        // Let's make it require timezone context if possible, or remove it.
        // For now, let's remove it to avoid incorrect usage.
        panic!("Direct conversion from EditState to TimeEntry is not supported without timezone context. Use try_into_time_entry instead.");
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum EditField {
    Description,
    Project,
    Contact,
    StartDate,
    StartTime,
    EndDate,
    EndTime,
}

impl Default for EditField {
    fn default() -> Self {
        Self::Description
    }
}

pub(crate) struct AppModel {
    pub config: Configuration,
    pub running_state: RunningState,
    pub client: moneybird::Client,
    pub administration: Administration,
    pub time_entries: Vec<TimeEntry>,
    pub time_entries_for_table_backup: Vec<TimeEntryForTable>,
    pub time_entries_for_table: Vec<TimeEntryForTable>,
    pub time_entry_table_state: TableState,
    pub projects: Vec<Project>,
    pub contacts: Vec<Contact>,
    pub users: Vec<User>,
    pub user_selection_active: bool,
    pub user_selection_state: TableState,
    pub search_state: SearchState,
    pub appearance: Appearance,
    pub week_offset: i32, // How many weeks from current (0 = current, -1 = previous, 1 = next)
    pub modal_stack: ModalStack,
    pub log_panel_state: LogPanelState,
    pub log_entries: Vec<LogEntry>,
    pub edit_state: EditState,
    pub modal_interaction: ModalInteraction,
    pub key_debounce: KeyDebounce,
    pub table_area: Option<Rect>,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            config: Configuration::default(),
            running_state: RunningState::default(),
            client: crate::api::create_moneybird_client(&crate::config::get_configuration()),
            administration: Administration::default(),
            time_entries: Vec::new(),
            time_entries_for_table_backup: Vec::new(),
            time_entries_for_table: Vec::new(),
            time_entry_table_state: TableState::default(),
            projects: Vec::new(),
            contacts: Vec::new(),
            users: Vec::new(),
            user_selection_active: false,
            user_selection_state: TableState::default(),
            search_state: SearchState::default(),
            appearance: Appearance::default(),
            week_offset: 0,
            modal_stack: ModalStack::default(),
            log_panel_state: LogPanelState::default(),
            log_entries: Vec::new(),
            edit_state: EditState::default(),
            modal_interaction: ModalInteraction::new(300), // 300ms cooldown for modals
            key_debounce: KeyDebounce::new(200),           // 200ms cooldown for keypresses
            table_area: None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum Mode {
    /// Represents the dark mode option.
    Dark,
    /// Represents the light mode option.
    Light,
    /// Used when the system theme mode is unspecified.
    Unspecified,
}

impl AppModel {
    // Helper method to ensure selection is valid
    pub(crate) fn ensure_valid_selection(&mut self) {
        if let Some(selected_idx) = self.time_entry_table_state.selected() {
            if selected_idx < self.time_entries_for_table.len() {
                self.time_entry_table_state.select(Some(selected_idx));
            } else if let Some(selected) = self.time_entry_table_state.selected() {
                if selected >= self.time_entries.len() {
                    self.log_warning(format!("Selected index out of bounds: {}", selected));
                    self.time_entry_table_state.select(Some(0));
                }
            } else if !self.time_entries_for_table.is_empty() {
                // If current selection is out of bounds but we have items, select first item
                self.time_entry_table_state.select(Some(0));
                self.log_warning("Selected index out of bounds".to_string());
            } else {
                // No items left, clear selection
                self.time_entry_table_state.select(None);
                self.log_warning("No items left, clear selection".to_string());
            }
        }
    }

    // Check if there's a blocking error modal active
    pub(crate) fn has_blocking_error(&self) -> bool {
        if let Some(modal) = self.modal_stack.top() {
            // Check for specific blocking error IDs
            if let Some(id) = &modal.id {
                match id.as_str() {
                    "connection_error" => return true,
                    // Add other blocking error IDs here
                    _ => {}
                }
            }

            // All errors without specific IDs are also considered blocking
            if modal.modal_type == ui::ModalType::Error {
                return true;
            }
        }

        // No blocking error
        false
    }

    // Add a log entry to the log list
    pub(crate) fn log(&mut self, message: impl Into<String>, severity: Option<LogSeverity>) {
        let entry = LogEntry {
            timestamp: Local::now(),
            severity: severity.unwrap_or(LogSeverity::Notice),
            message: message.into(),
        };
        self.log_entries.push(entry);

        // Keep a maximum of 100 log entries (optional)
        if self.log_entries.len() > 100 {
            self.log_entries.remove(0);
        }
    }

    // Convenience methods for different log levels
    pub(crate) fn log_debug(&mut self, message: impl Into<String>) {
        // Only log if debug is enabled via RUST_LOG=debug or RUST_LOG=trace
        if self.is_debug_enabled() {
            self.log(message, Some(LogSeverity::Debug))
        }
    }

    pub(crate) fn log_notice(&mut self, message: impl Into<String>) {
        self.log(message, Some(LogSeverity::Notice))
    }

    pub(crate) fn log_success(&mut self, message: impl Into<String>) {
        self.log(message, Some(LogSeverity::Success))
    }

    pub(crate) fn log_warning(&mut self, message: impl Into<String>) {
        self.log(message, Some(LogSeverity::Warning))
    }

    pub(crate) fn log_error(&mut self, message: impl Into<String>) {
        self.log(message, Some(LogSeverity::Error))
    }

    // Check if debug logging is enabled based on RUST_LOG environment variable
    pub(crate) fn is_debug_enabled(&self) -> bool {
        if let Ok(log_level) = std::env::var("RUST_LOG") {
            return log_level.to_lowercase() == "debug" || log_level.to_lowercase() == "trace";
        }
        false
    }

    // Filter items based on query
    pub(crate) fn filter_items(&mut self) {
        // Get the current search text and filter items
        let query = self
            .search_state
            .text_input
            .lines()
            .first()
            .unwrap()
            .clone();

        // Reset to full list for each query. The user may have entered a non mat
        self.time_entries_for_table = self.time_entries_for_table_backup.clone();

        self.ensure_valid_selection();

        if query.is_empty() {
            return;
        }

        // Convert query to lowercase for case-insensitive matching
        let query_lower = query.to_lowercase();

        // Filter items with direct string matching:
        // - All matches must contain the query string (case insensitive)
        // - Score is calculated based on which fields match:
        //   - client: 4 points
        //   - project: 2 points
        //   - description: 1 point
        let scored_items: Vec<(TimeEntryForTable, i32)> = self
            .time_entries_for_table
            .iter()
            .filter_map(|item| {
                let customer_lower = item.customer.to_lowercase();
                let project_lower = item.project.to_lowercase();
                let description_lower = item.description.to_lowercase();

                // Simple contains matching
                let client_match = customer_lower.contains(&query_lower);
                let project_match = project_lower.contains(&query_lower);
                let desc_match = description_lower.contains(&query_lower);

                // Calculate score based on which fields match
                let mut score = 0;
                if client_match {
                    score += 4;
                }
                if project_match {
                    score += 2;
                }
                if desc_match {
                    score += 1;
                }

                // Only include items with at least one match
                if score > 0 {
                    Some((item.clone(), score))
                } else {
                    None
                }
            })
            .collect();

        if self.time_entries_for_table.len() > scored_items.len() {
            self.log_success(format!(
                "Filtered items from {} to {}",
                self.time_entries_for_table.len(),
                scored_items.len()
            ));
        }

        // Extract just the items
        self.time_entries_for_table = scored_items.into_iter().map(|(item, _)| item).collect();

        self.ensure_valid_selection();
    }
}
