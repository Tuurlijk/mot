use clap::Parser;
use rust_i18n::t;

#[derive(Parser)]
#[clap(version, about = about_str())]
pub(crate) struct Args {
    /// Export time entries to a csv file
    #[clap(short = 'e', long, action = clap::ArgAction::SetTrue, help = t!("cmd_export_help").to_string())]
    pub(crate) export: bool,

    /// Week number to export
    #[clap(short = 'w', long, default_value = "current week", help = t!("cmd_week_help").to_string())]
    pub(crate) week: String,

    /// Filter query
    #[clap(short = 'q', long, default_value = "", help = t!("cmd_query_help").to_string())]
    pub(crate) query: String,

    /// Set the display language (e.g., en, nl)
    #[clap(short = 'l', long, help = t!("cmd_language_help").to_string())]
    pub(crate) language: Option<String>,
    
    /// Debug a plugin initialization issues
    #[clap(long = "plugin-debug", value_name = "PLUGIN_NAME", help = t!("cmd_plugin_debug_help").to_string())]
    pub(crate) plugin_debug: Option<String>,
}

fn about_str() -> &'static str {
    // Fetch value from the environment variable
    let dynamic_value = env!("GIT_INFO").to_string();

    // Build the about string with the dynamic value
    let about_str = format!(
        r"
      __  __                   _    _        _ 
     |  \/  |___ _ _  ___ _  _| |__(_)_ _ __| |
     | |\/| / _ \ ' \/ -_) || | '_ \ | '_/ _` |
     |_|  |_\___/_||_\___|\_, |_.__/_|_| \__,_|
                         |__/   
     {}
     {}
     {}
 ",
        t!("cmd_app_title").to_string(),
        t!("cmd_app_credits").to_string(),
        dynamic_value
    );

    // Leak the dynamic string to get a static reference
    Box::leak(about_str.into_boxed_str())
}
