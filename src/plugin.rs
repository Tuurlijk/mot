//! Plugin system for mot
//! 
//! This module handles discovering, loading, and communicating with plugins that
//! provide time entries from external sources.

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use chrono::{DateTime, Utc};
use color_eyre::eyre::{self, eyre};
use rust_i18n::t;
use serde::{Deserialize, Serialize};

use crate::model::TimeEntryForTable;

/// Time entry model shared between host and plugins
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginTimeEntry {
    pub id: String,
    pub description: String,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub customer_id: Option<String>,
    pub customer_name: Option<String>,
    pub started_at: String, // RFC3339 format
    pub ended_at: String,   // RFC3339 format
    pub tags: Vec<String>,
    pub source: String,
    pub source_url: Option<String>,
}

impl From<PluginTimeEntry> for TimeEntryForTable {
    fn from(entry: PluginTimeEntry) -> Self {
        TimeEntryForTable {
            id: entry.id,
            description: entry.description,
            started_at: entry.started_at,
            ended_at: entry.ended_at,
            project: entry.project_name.unwrap_or_else(|| "Unknown".to_string()),
            customer: entry.customer_name.unwrap_or_else(|| "Unknown".to_string()),
            billable: true,
            icon: None,
            source: entry.source.clone(),
        }
    }
}

/// JSON-RPC request structure
#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: u64,
}

/// JSON-RPC response structure
#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<serde_json::Value>,
    error: Option<JsonRpcError>,
    id: u64,
}

/// JSON-RPC error structure
#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcError {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

/// Plugin manifest structure
#[derive(Debug, Deserialize)]
struct Manifest {
    plugin: PluginManifestInfo,
    executable: ExecutableInfo,
}

/// Plugin information from manifest
#[derive(Debug, Deserialize, Clone)]
struct PluginManifestInfo {
    name: String,
    version: String,
    description: Option<String>,
    icon: Option<String>,
}

/// Executable information from manifest
#[derive(Debug, Deserialize)]
struct ExecutableInfo {
    default: String,
    windows: Option<String>,
}

/// Request parameters for getting time entries
#[derive(Serialize, Deserialize)]
struct GetTimeEntriesParams {
    start_date: String, // RFC3339 format
    end_date: String,   // RFC3339 format
}

/// Request parameters for initializing a plugin
#[derive(Serialize, Deserialize)]
struct InitializeParams {
    config_path: String,
}

/// Information about a loaded plugin
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub initialized: bool,
    pub icon: Option<String>,
}

/// Loaded plugin instance
struct Plugin {
    info: PluginInfo,
    process: Child,
    request_id: u64,
    request_timeout: Duration,
}

impl Plugin {
    /// Send a JSON-RPC request to the plugin and read the response
    fn send_request(&mut self, method: &str, params: serde_json::Value) -> eyre::Result<JsonRpcResponse> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: self.request_id,
        };
        self.request_id += 1;

        let request_json = serde_json::to_string(&request)?;
        
        // Get stdin/stdout handles
        let stdin = self.process.stdin.as_mut().ok_or_else(|| {
            eyre!("Failed to open stdin for plugin {}", self.info.name)
        })?;
        
        let stdout = self.process.stdout.as_mut().ok_or_else(|| {
            eyre!("Failed to open stdout for plugin {}", self.info.name)
        })?;
        
        // Write request to plugin's stdin
        stdin.write_all(request_json.as_bytes())?;
        stdin.write_all(b"\n")?;
        stdin.flush()?;
        
        // Read response from plugin's stdout
        let mut reader = BufReader::new(stdout);
        let mut response_str = String::new();
        reader.read_line(&mut response_str)?;
        
        // Parse the response
        let response: JsonRpcResponse = serde_json::from_str(&response_str)?;
        Ok(response)
    }
    
    /// Clean up the plugin process
    fn shutdown(&mut self) -> eyre::Result<()> {
        // Try graceful shutdown first
        let shutdown_result = self.send_request("shutdown", serde_json::Value::Null);
        if shutdown_result.is_err() {
            // Force kill if graceful shutdown fails
            self.process.kill()?;
        }
        
        Ok(())
    }
}

/// Plugin system manager
pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
    plugins_dir: PathBuf,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> eyre::Result<Self> {
        // Get the config directory
        let config_dir = dirs::config_dir()
            .ok_or_else(|| eyre!("Could not determine config directory"))?;
        
        let plugins_dir = config_dir.join("mot").join("plugins");

        Ok(Self {
            plugins: HashMap::new(),
            plugins_dir,
        })
    }
    
    /// Get the plugins directory path
    pub fn plugins_dir(&self) -> &Path {
        &self.plugins_dir
    }
    
    /// Scan the plugins directory and load available plugins
    pub fn discover_plugins(&mut self) -> eyre::Result<Vec<(String, Result<String, String>)>> {
        let mut results = Vec::new();
        
        if !self.plugins_dir.exists() {
            fs::create_dir_all(&self.plugins_dir)?;
            results.push((self.plugins_dir.to_string_lossy().to_string(), 
                          Ok(t!("plugin_created_plugins_dir", path = format!("{:?}", self.plugins_dir)).to_string())));
            return Ok(results);
        }
        
        for entry in fs::read_dir(&self.plugins_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                match self.load_plugin(&path) {
                    Ok(plugin_name) => {
                        results.push((path.to_string_lossy().to_string(), 
                                     Ok(t!("plugin_loaded", name = plugin_name).to_string())));
                    }
                    Err(err) => {
                        results.push((path.to_string_lossy().to_string(), 
                                     Err(t!("plugin_load_failed", path = format!("{:?}", path), error = err.to_string()).to_string())));
                    }
                }
            }
        }
        
        Ok(results)
    }
    
    /// Load a plugin from a directory
    fn load_plugin(&mut self, plugin_dir: &Path) -> eyre::Result<String> {
        let manifest_path = plugin_dir.join("manifest.toml");
        let config_path = plugin_dir.join("config.toml");
        
        // Check if manifest and config exist
        if !manifest_path.exists() {
            return Err(eyre!("manifest.toml not found in {:?}", plugin_dir));
        }
        
        if !config_path.exists() {
            return Err(eyre!("config.toml not found in {:?}", plugin_dir));
        }
        
        // Parse manifest.toml
        let manifest_content = fs::read_to_string(&manifest_path)?;
        let manifest: Manifest = toml::from_str(&manifest_content)?;
        
        // Verify the plugin name matches the directory name
        let dir_name = plugin_dir.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| eyre!("Invalid plugin directory name"))?;
            
        if manifest.plugin.name != dir_name {
            println!("{}", t!("plugin_name_mismatch", manifest_name = manifest.plugin.name, dir_name = dir_name));
        }
        
        // Get executable path based on platform
        let executable_name = if cfg!(windows) {
            manifest.executable.windows.as_ref().unwrap_or(&manifest.executable.default)
        } else {
            &manifest.executable.default
        };
        
        let executable_path = plugin_dir.join(executable_name);
        
        // Verify executable exists
        if !executable_path.exists() {
            return Err(eyre!("Executable not found: {:?}", executable_path));
        }
        
        #[cfg(unix)]
        {
            // Check if the file is executable on Unix
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&executable_path)?;
            let permissions = metadata.permissions();
            if permissions.mode() & 0o111 == 0 {
                return Err(eyre!("Plugin executable is not executable: {:?}", executable_path));
            }
        }
        
        // Start the plugin process
        let process = Command::new(&executable_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(plugin_dir) // Run in the plugin directory
            .spawn()?;
        
        // Create plugin info
        let plugin_info = PluginInfo {
            name: manifest.plugin.name.clone(),
            version: manifest.plugin.version.clone(),
            description: manifest.plugin.description.clone(),
            initialized: false,
            icon: manifest.plugin.icon.clone(),
        };
        
        // Create plugin instance
        let plugin = Plugin {
            info: plugin_info,
            process,
            request_id: 1,
            request_timeout: Duration::from_secs(10),
        };
        
        // Store the plugin
        self.plugins.insert(manifest.plugin.name.clone(), plugin);
        
        Ok(manifest.plugin.name)
    }
    
    /// Initialize all loaded plugins
    pub fn initialize_plugins(&mut self) -> eyre::Result<Vec<(String, Result<(), String>)>> {
        let plugins_to_init: Vec<String> = self.plugins.keys().cloned().collect();
        let mut results = Vec::new();
        
        for plugin_name in plugins_to_init {
            if let Some(plugin) = self.plugins.get_mut(&plugin_name) {
                let config_path = self.plugins_dir
                    .join(&plugin_name)
                    .join("config.toml")
                    .to_string_lossy()
                    .to_string();
                
                let params = serde_json::to_value(InitializeParams { 
                    config_path 
                })?;
                
                match plugin.send_request("initialize", params) {
                    Ok(response) => {
                        if let Some(error) = response.error {
                            results.push((plugin_name.clone(), 
                                         Err(t!("plugin_init_failed", name = plugin_name, error = error.message).to_string())));
                        } else {
                            plugin.info.initialized = true;
                            results.push((plugin_name.clone(), 
                                         Ok(())));
                        }
                    }
                    Err(err) => {
                        results.push((plugin_name.clone(), 
                                     Err(t!("plugin_init_error", name = plugin_name, error = err.to_string()).to_string())));
                    }
                }
            }
        }
        
        Ok(results)
    }
    
    /// Get time entries from a specific plugin
    pub fn get_time_entries(&mut self, plugin_name: &str, 
                        start_date: &DateTime<Utc>, 
                        end_date: &DateTime<Utc>) -> eyre::Result<Vec<PluginTimeEntry>> {
        let plugin = self.plugins.get_mut(plugin_name).ok_or_else(|| {
            eyre!("Plugin not found: {}", plugin_name)
        })?;
        
        if !plugin.info.initialized {
            return Err(eyre!("Plugin {} is not initialized", plugin_name));
        }
        
        let params = serde_json::to_value(GetTimeEntriesParams { 
            start_date: start_date.to_rfc3339(),
            end_date: end_date.to_rfc3339(),
        })?;
        
        let response = plugin.send_request("get_time_entries", params)?;
        
        if let Some(error) = response.error {
            return Err(eyre!("Plugin error: {}", error.message));
        }
        
        if let Some(result) = response.result {
            let entries: Vec<PluginTimeEntry> = serde_json::from_value(result)?;
            Ok(entries)
        } else {
            Ok(vec![])
        }
    }
    
    /// Get time entries from all plugins for a date range
    pub fn get_all_time_entries(&mut self, 
                              start_date: &DateTime<Utc>, 
                              end_date: &DateTime<Utc>) -> eyre::Result<(Vec<PluginTimeEntry>, Vec<(String, String)>)> {
        let mut all_entries = Vec::new();
        let mut errors = Vec::new();
        let plugin_names: Vec<String> = self.plugins.keys().cloned().collect();
        
        for plugin_name in plugin_names {
            match self.get_time_entries(&plugin_name, start_date, end_date) {
                Ok(entries) => {
                    all_entries.extend(entries);
                }
                Err(err) => {
                    errors.push((plugin_name.clone(), 
                                t!("plugin_get_entries_error", name = plugin_name, error = err.to_string()).to_string()));
                }
            }
        }
        
        Ok((all_entries, errors))
    }
    
    /// Shutdown all plugins
    pub fn shutdown(&mut self) -> eyre::Result<Vec<(String, String)>> {
        let mut errors = Vec::new();
        
        for (name, plugin) in self.plugins.iter_mut() {
            if let Err(e) = plugin.shutdown() {
                errors.push((name.clone(), 
                            t!("plugin_shutdown_error", name = name, error = e.to_string()).to_string()));
            }
        }
        
        self.plugins.clear();
        Ok(errors)
    }
    
    /// Get a list of loaded plugin information
    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins
            .values()
            .map(|plugin| plugin.info.clone())
            .collect()
    }
    
    /// Check if a plugin with the given name is loaded and initialized
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.get(name)
            .map(|p| p.info.initialized)
            .unwrap_or(false)
    }
} 