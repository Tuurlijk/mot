use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub access_token: String,
    pub api_url: String,
    pub administration_id: Option<String>,
    pub user_id: Option<String>,
    #[serde(default = "default_week_start")]
    pub week_starts_on: String,
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
            println!("Could not create config directory");
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
            println!("Could not load configuration file");
            std::process::exit(1);
        })
}

pub fn get_configuration() -> Configuration {
    let config = get_config();
    config
        .try_deserialize::<Configuration>()
        .unwrap_or_else(|e| {
            println!("Could not deserialize configuration: {}", e);
            std::process::exit(1);
        })
}

pub fn save_configuration(config: &Configuration) -> Result<(), color_eyre::eyre::Error> {
    let config_path = get_config_path();

    let toml_string = toml::to_string_pretty(config)?;

    let mut file = fs::File::create(&config_path).map_err(|e| {
        color_eyre::eyre::eyre!(
            "Could not create/open config file for writing at {:?}: {}",
            config_path,
            e
        )
    })?;

    file.write_all(toml_string.as_bytes()).map_err(|e| {
        color_eyre::eyre::eyre!("Could not write to config file at {:?}: {}", config_path, e)
    })?;

    Ok(())
}

fn create_default_config(config_path: &PathBuf) {
    let default_config = Configuration::default();
    let toml = toml::to_string_pretty(&default_config).unwrap_or_else(|_| {
        println!("Could not serialize default configuration");
        std::process::exit(1);
    });

    let mut file = fs::File::create(config_path).unwrap_or_else(|_| {
        println!("Could not create config file at {:?}", config_path);
        std::process::exit(1);
    });

    file.write_all(toml.as_bytes()).unwrap_or_else(|_| {
        println!("Could not write to config file");
        std::process::exit(1);
    });

    println!("Created default configuration at {:?}", config_path);
}

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .ok_or_else(|| color_eyre::eyre::eyre!("Could not determine config directory"))
        .unwrap_or_else(|_| {
            println!("Could not determine config directory");
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
            println!("Failed to get executable path");
            "unknown".to_string()
        })
}
