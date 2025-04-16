// Allow dead code for moneybird.rs, which is generated
#![allow(dead_code)]
#![allow(unused_imports)]

mod api;
mod cmd;
mod config;
mod datetime;
mod event;
mod file;
mod model;
mod moneybird;
mod moneybird_traits;
mod tui;
mod ui;
mod update;

// Import and initialize rust-i18n
use rust_i18n::t;

// Initialize i18n with locales directory and English as fallback
rust_i18n::i18n!("locales", fallback = "en");

use clap::Parser;
use color_eyre::eyre::{self};
use event::handle_event;
use model::{AppModel, RunningState, TimeEntryForTable};
use ratatui::layout::{Constraint, Layout};
use ratatui::Frame;

use ui::{render_search, render_time_entries_table, render_time_entry_detail};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args: cmd::Args = cmd::Args::parse();

    let mut model = AppModel {
        ..Default::default()
    };
    model.config = config::get_configuration();

    // Set locale from command line arguments first, then from config if available
    if let Some(language) = &args.language {
        rust_i18n::set_locale(language);
        model.log_notice(t!("notice_language_command_line", language = language));

        // Optionally update config with the selected language
        model.config.language = Some(language.clone());
    } else if let Some(language) = &model.config.language {
        rust_i18n::set_locale(language);
        model.log_notice(t!("notice_language_configured", language = language));
    }

    // Check connectivity to the MoneyBird API but don't exit on failure
    model.log_notice(t!("notice_checking_api"));
    if let Err(err) = api::check_connectivity(&model.client).await {
        model.log_error(t!("connection_error", error = err.to_string()));
        ui::show_connection_error(&mut model, t!("connection_error", error = err.to_string()));
        // Continue with the app - the error will be shown in the main loop
    } else {
        model.log_success(t!("success_connection"));
    }

    // Initialize the application colors
    ui::color::setup_colors(&mut model.appearance);
    model.log_success(t!("success_colors_initialized"));

    // Try to get administration information if we have connectivity
    if !model.has_blocking_error() {
        match model.config.administration_id.clone() {
            Some(administration_id) => {
                match api::get_administration_by_id(&model.client, &administration_id).await {
                    Ok(administration) => {
                        model.administration = administration;
                        model.log_notice(t!(
                            "notice_administration",
                            id = model.administration.id.clone().unwrap_or_default(),
                            name = model.administration.name.clone().unwrap_or_default()
                        ));
                    }
                    Err(err) => {
                        ui::show_error(
                            &mut model,
                            t!("error_administration", error = err.to_string()),
                        );
                    }
                }
            }
            None => match api::get_first_administration(&model.client).await {
                Ok(administration) => {
                    model.administration = administration;
                    model.log_notice(t!(
                        "notice_administration",
                        id = model.administration.id.clone().unwrap_or_default(),
                        name = model.administration.name.clone().unwrap_or_default()
                    ));
                }
                Err(err) => {
                    ui::show_error(
                        &mut model,
                        t!("error_get_administrations", error = err.to_string()),
                    );
                }
            },
        }
    }

    // Check for user_id in config, fetch users if necessary
    if !model.has_blocking_error() {
        if model.config.user_id.is_none() {
            model.log_notice(t!("notice_no_user_id"));
            let administration_id = model.administration.id.clone().unwrap_or_default();
            if !administration_id.is_empty() {
                match api::get_all_users(&model.client, &administration_id).await {
                    Ok(users) => {
                        if users.is_empty() {
                            let err_msg = t!("error_no_users");
                            model.log_error(err_msg.clone());
                            ui::show_error(&mut model, err_msg);
                        } else {
                            model.log_success(t!("success_fetched_users", count = users.len()));
                            model.users = users;
                            model.user_selection_active = true; // Activate user selection mode
                                                                // Select the first user by default
                            if !model.users.is_empty() {
                                model.user_selection_state.select(Some(0));
                            }
                        }
                    }
                    Err(err) => {
                        let err_msg = t!("error_fetch_users", error = err.to_string());
                        model.log_error(err_msg.clone());
                        ui::show_error(&mut model, err_msg);
                    }
                }
            } else {
                // This case should ideally not happen if administration was fetched successfully
                let err_msg = t!("error_missing_admin_id");
                model.log_error(err_msg.clone());
                ui::show_error(&mut model, err_msg);
            }
        } else {
            model.log_success(t!(
                "success_using_user_id",
                user_id = model.config.user_id.clone().unwrap_or_default()
            ));
        }
    }

    // Only try to fetch contacts, projects and time entries if we have a valid administration
    // AND if user selection is not currently required
    if !model.has_blocking_error()
        && !model.user_selection_active
        && !model
            .administration
            .id
            .clone()
            .unwrap_or_default()
            .is_empty()
    {
        let administration_id = model.administration.id.clone().unwrap_or_default();

        // Try to fetch projects
        match api::get_all_projects(&model.client, &administration_id).await {
            Ok(projects) => {
                model.projects = projects;
            }
            Err(err) => {
                ui::show_error(
                    &mut model,
                    t!("error_fetch_projects", error = err.to_string()),
                );
            }
        }

        // Get time entries for the current week
        api::get_time_entries(&mut model).await;
    }

    if args.export {
        if model.has_blocking_error() {
            // Display model.modal_stack.top() title using eyre
            let modal = model.modal_stack.top().unwrap();
            return Err(eyre::eyre!("{}\n{}", modal.title, modal.message));
        } else {
            // Export time entries to a csv file using command-line options
            return file::handle_export_command(&mut model, args.week.clone(), args.query.clone())
                .await;
        }
    }

    // Initialize TUI elements
    tui::install_panic_hook();

    let mut terminal = tui::init_terminal()?;

    // Main event loop
    while model.running_state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&mut model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update::update(&mut model, current_msg.unwrap()).await;
        }
    }

    // Clean up and exit
    tui::restore_terminal()?;
    Ok(())
}

fn view(model: &mut AppModel, frame: &mut Frame) {
    // First determine the main content area based on log panel visibility
    let main_area = if model.log_panel_state.visible {
        // Split the screen to include the log panel on the right (20% width)
        let [main_area, log_area] =
            Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
                .areas(frame.area());

        // Render the log panel in the right area
        ui::render_log_panel(model, log_area, frame);

        // Return main area for content
        main_area
    } else {
        // Use the full frame area when log panel is not visible
        frame.area()
    };

    // Render appropriate UI based on state
    if model.user_selection_active {
        // If user selection is active, render the user selection list
        ui::render_user_selection(model, main_area, frame);
    } else if model.edit_state.active {
        // When in edit mode, show the edit form
        ui::render_time_entry_edit(model, main_area, frame);
    } else {
        // Normal view with time entries
        if model.search_state.active {
            let [top, search, bottom] = Layout::vertical([
                Constraint::Percentage(58),
                Constraint::Length(2),
                Constraint::Percentage(40),
            ])
            .areas(main_area);
            render_time_entries_table(model, top, frame);
            render_search(model, search, frame);
            render_time_entry_detail(model, bottom, frame);
        } else {
            let [top, bottom] =
                Layout::vertical([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .areas(main_area);
            render_time_entries_table(model, top, frame);
            render_time_entry_detail(model, bottom, frame);
        }
    }

    // Draw modal if active (always on top)
    if !model.modal_stack.is_empty() {
        ui::render_modal(model, frame);
    }
}
