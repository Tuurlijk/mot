use config::{Config, File};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use locale_config::Locale;

pub fn detect_system_language() -> Option<String> {
    let locale = Locale::user_default();
    let locale_str = locale.to_string();
    
    let lang_code = locale_str
        .split(|c| c == '-' || c == '_' || c == '.')
        .next()?
        .to_lowercase();
    
    if ["en", "nl"].contains(&lang_code.as_str()) {
        Some(lang_code)
    } else {
        None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub access_token: String,
    pub api_url: String,
    pub administration_id: Option<String>,
    pub user_id: Option<String>,
    #[serde(default = "default_week_start")]
    pub week_starts_on: String,
    pub language: Option<String>,
}

fn default_week_start() -> String {
    "monday".to_string()
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            access_token: "your_access_token_here".to_string(),
            api_url: "https://moneybird.com/api/v2".to_string(),
            administration_id: None,
            user_id: None,
            week_starts_on: default_week_start(),
            language: None,
        }
    }
}

impl Configuration {
    pub fn get_user_id(&self) -> String {
        self.user_id.clone().unwrap_or_default()
    }

    pub fn get_administration_id(&self) -> String {
        self.administration_id.clone().unwrap_or_default()
    }
}

pub fn get_config() -> Config {
    let config_path = get_config_path();

    // Create config directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|_| {
            println!("{}", t!("config_create_dir_error"));
            std::process::exit(1);
        });
    }

    // Check if the config file exists, if not create it from template
    if !config_path.exists() {
        create_default_config(&config_path);
    }

    Config::builder()
        .add_source(File::from(config_path).required(true))
        .build()
        .unwrap_or_else(|_| {
            println!("{}", t!("config_load_error"));
            std::process::exit(1);
        })
}

pub fn get_configuration() -> Configuration {
    let config = get_config();
    config
        .try_deserialize::<Configuration>()
        .unwrap_or_else(|e| {
            println!("{}: {}", t!("config_deserialize_error"), e);
            std::process::exit(1);
        })
}

pub fn save_configuration(config: &Configuration) -> Result<(), color_eyre::eyre::Error> {
    let config_path = get_config_path();

    let toml_string = toml::to_string_pretty(config)?;

    let mut file = fs::File::create(&config_path).map_err(|e| {
        color_eyre::eyre::eyre!(t!(
            "config_create_file_error",
            path = format!("{:?}", config_path),
            error = e
        ))
    })?;

    file.write_all(toml_string.as_bytes()).map_err(|e| {
        color_eyre::eyre::eyre!(t!(
            "config_write_file_error",
            path = format!("{:?}", config_path),
            error = e
        ))
    })?;

    Ok(())
}

fn create_default_config(config_path: &PathBuf) {
    let default_config = Configuration::default();
    let toml = toml::to_string_pretty(&default_config).unwrap_or_else(|_| {
        println!("{}", t!("config_serialize_error"));
        std::process::exit(1);
    });

    let mut file = fs::File::create(config_path).unwrap_or_else(|_| {
        println!(
            "{}",
            t!(
                "config_create_file_error_path",
                path = format!("{:?}", config_path)
            )
        );
        std::process::exit(1);
    });

    file.write_all(toml.as_bytes()).unwrap_or_else(|_| {
        println!("{}", t!("config_write_error"));
        std::process::exit(1);
    });

    println!(
        "{}",
        t!(
            "config_created_default",
            path = format!("{:?}", config_path)
        )
    );
}

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .ok_or_else(|| color_eyre::eyre::eyre!(t!("config_determine_dir_error")))
        .unwrap_or_else(|_| {
            println!("{}", t!("config_determine_dir_error"));
            std::process::exit(1);
        })
        .join(get_program_name())
        .join("config.toml")
}

fn get_program_name() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|exe_path| {
            exe_path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| {
            println!("{}", t!("config_executable_path_error"));
            "unknown".to_string()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_system_language() {
        // This test doesn't check for a specific value since system locales vary
        // It just ensures the function returns a valid value without panicking
        let language = detect_system_language();
        if let Some(lang) = language {
            // If a language was detected, it should be one we support
            assert!(["en", "nl"].contains(&lang.as_str()), 
                "Detected language '{}' should be one of our supported languages", lang);
        }
        // If None is returned, that's also valid if no supported language was found
    }
}
