use clap::Parser;

#[derive(Parser)]
#[clap(version, about = about_str())]
pub(crate) struct Args {
    /// Export time entries to a csv file
    #[clap(short = 'e', long, action = clap::ArgAction::SetTrue)]
    pub(crate) export: bool,

    /// Week number to export
    #[clap(short = 'w', long, default_value = "current week")]
    pub(crate) week: String,

    /// Filter query
    #[clap(short = 'q', long, default_value = "")]
    pub(crate) query: String,
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
    Moneybird Terminal User Interface
    Coded with ♥️ by Michiel Roos
    {}
",
        dynamic_value
    );

    // Leak the dynamic string to get a static reference
    Box::leak(about_str.into_boxed_str())
}
