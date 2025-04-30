//! Plugin system for mot
//! 
//! This module handles discovering, loading, and communicating with plugins that
//! provide time entries from external sources.

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use color_eyre::eyre::{self, eyre};
 // Import log crate for macros
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::process::{Child, Command};
use tokio::sync::{mpsc, oneshot};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader as TokioBufReader};

use crate::model::TimeEntryForTable;

/// Time entry model shared between host and plugins
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginTimeEntry {
    pub id: String,
    pub description: String,
    pub project_name: Option<String>,
    pub customer_name: Option<String>,
    pub started_at: String, // RFC3339 format
    pub ended_at: String,   // RFC3339 format
    pub tags: Vec<String>,
    pub source_url: Option<String>,
    pub source: String,     // Display name for the source system
    pub plugin_name: Option<String>, // Required - should match manifest name
    pub billable: bool,
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
            billable: entry.billable,
            icon: None, // Icons will be applied by the app based on plugin_name
            source: entry.source,
            plugin_name: entry.plugin_name, // Pass through plugin_name directly
        }
    }
}

// Define necessary RPC methods directly, no need for a full trait/macro here for client-side
const INITIALIZE_METHOD: &str = "initialize";
const GET_TIME_ENTRIES_METHOD: &str = "get_time_entries";
const SHUTDOWN_METHOD: &str = "shutdown";

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

/// Information about a loaded plugin
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub initialized: bool,
    pub icon: Option<String>,
}

/// Represents the communication channel with a plugin process.
/// This uses raw JSON-RPC requests over stdin/stdout.
struct PluginRpcChannel {
    tx: mpsc::Sender<JsonRpcRequestWithCallback>,
    process: Child, // Keep handle to manage process lifetime
}

struct JsonRpcRequestWithCallback {
    method: String,
    params: Value, // Use serde_json::Value for flexibility
    callback: oneshot::Sender<eyre::Result<Value>>, // Send back Result<Value, Error>
}

impl PluginRpcChannel {
    async fn new(mut process: Child) -> eyre::Result<Self> {
        let stdin = process.stdin.take().ok_or_else(|| eyre!("Failed to take plugin stdin"))?;
        let stdout = process.stdout.take().ok_or_else(|| eyre!("Failed to take plugin stdout"))?;
        let stderr = process.stderr.take(); // Optionally capture stderr

        let (tx, mut rx) = mpsc::channel::<JsonRpcRequestWithCallback>(32);

        // Stdio communication task
        tokio::spawn(async move {
            let mut reader = TokioBufReader::new(stdout);
            let mut writer = stdin;
            let mut response_map: HashMap<u64, oneshot::Sender<eyre::Result<Value>>> = HashMap::new();
            let mut next_id: u64 = 0;
            let mut line_buffer = String::new();

            loop {
                tokio::select! {
                    // Read from stdout
                    read_result = reader.read_line(&mut line_buffer) => {
                        match read_result {
                            Ok(0) => {
                                log::warn!("Plugin stdout closed unexpectedly.");
                                break;
                            }
                            Ok(_) => {
                                log::trace!("Received from plugin: {}", line_buffer.trim());
                                match serde_json::from_str::<Value>(&line_buffer) {
                                    Ok(value) => {
                                        if let Some(response) = value.as_object() {
                                            if let Some(id_val) = response.get("id") {
                                                if let Some(id) = id_val.as_u64() {
                                                    if let Some(callback) = response_map.remove(&id) {
                                                        if let Some(error) = response.get("error") {
                                                            let _ = callback.send(Err(eyre!("Plugin RPC Error: {}", error)));
                                                        } else if let Some(result) = response.get("result") {
                                                            let _ = callback.send(Ok(result.clone()));
                                                        } else {
                                                            let _ = callback.send(Err(eyre!("Invalid RPC response: missing result or error")));
                                                        }
                                                    } else {
                                                        log::warn!("Received response for unknown request ID: {}", id);
                                                    }
                                                } else {
                                                     log::error!("Invalid response ID type: {:?}", id_val);
                                                }
                                            } else {
                                                 // Could be a notification from the plugin, log it?
                                                 log::debug!("Received notification or non-standard message from plugin: {}", line_buffer.trim());
                                            }
                                        } else {
                                            log::error!("Received non-object JSON from plugin: {}", line_buffer.trim());
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("Failed to parse JSON from plugin: {}. Content: {}", e, line_buffer.trim());
                                    }
                                }
                                line_buffer.clear(); // Important!
                            }
                            Err(e) => {
                                log::error!("Error reading from plugin stdout: {}", e);
                                break;
                            }
                        }
                    }

                    // Receive request to send
                    Some(req_with_cb) = rx.recv() => {
                        next_id += 1;
                        let request_id = next_id;
                        let request = serde_json::json!({
                            "jsonrpc": "2.0",
                            "method": req_with_cb.method,
                            "params": req_with_cb.params,
                            "id": request_id,
                        });

                        match serde_json::to_string(&request) {
                            Ok(mut request_str) => {
                                request_str.push('\n'); // Add newline delimiter
                                log::trace!("Sending to plugin: {}", request_str.trim());
                                if let Err(e) = writer.write_all(request_str.as_bytes()).await {
                                    log::error!("Error writing to plugin stdin: {}", e);
                                    let _ = req_with_cb.callback.send(Err(eyre!("Failed to send request to plugin: {}", e)));
                                    // Don't break here, maybe subsequent writes work?
                                } else {
                                     // Store callback for when response arrives
                                     response_map.insert(request_id, req_with_cb.callback);
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to serialize RPC request: {}", e);
                                let _ = req_with_cb.callback.send(Err(eyre!("Failed to serialize request: {}", e)));
                            }
                        }
                    }

                    else => {
                        // Channel closed or other select! branch triggered break
                        break;
                    }
                }
            }
            log::debug!("Plugin communication task finished.");
            // Notify any pending requests that the channel is closed
            for (_, callback) in response_map {
                let _ = callback.send(Err(eyre!("Plugin communication channel closed")));
            }
        });

        // Optional: Spawn a task to log stderr
        if let Some(mut stderr_reader) = stderr.map(TokioBufReader::new) {
            tokio::spawn(async move {
                let mut line = String::new();
                while let Ok(n) = stderr_reader.read_line(&mut line).await {
                    if n == 0 { break; }
                    log::warn!("[Plugin STDERR]: {}", line.trim_end());
                    line.clear();
                }
            });
        }

        Ok(Self { tx, process })
    }

    async fn request<P: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        params: P,
    ) -> eyre::Result<R> {
        let params_value = serde_json::to_value(params)
            .map_err(|e| eyre!("Failed to serialize params: {}", e))?;
        
        let (callback_tx, callback_rx) = oneshot::channel();

        let req = JsonRpcRequestWithCallback {
            method: method.to_string(),
            params: params_value,
            callback: callback_tx,
        };

        self.tx.send(req).await
            .map_err(|_| eyre!("Failed to send request to plugin communication task"))?;

        // Wait for the response from the communication task
        let result_value = callback_rx.await
            .map_err(|_| eyre!("Plugin response channel closed prematurely"))??; // Double ?? to propagate eyre::Result

        // Deserialize the result Value into the expected type R
        serde_json::from_value(result_value)
            .map_err(|e| eyre!("Failed to deserialize plugin response: {}", e))
    }
}

/// Loaded plugin instance
struct Plugin {
    info: PluginInfo,
    channel: PluginRpcChannel,
    directory: PathBuf, // Store the directory path where the plugin was loaded from
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
    pub async fn discover_plugins(&mut self) -> eyre::Result<Vec<(String, Result<String, String>)>> {
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
                match self.load_plugin(&path).await {
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
    async fn load_plugin(&mut self, plugin_dir: &Path) -> eyre::Result<String> {
        let manifest_path = plugin_dir.join("manifest.toml");
        let config_path = plugin_dir.join("config.toml"); // Keep config path for initialize call
        
        // Check if manifest and config exist
        if !manifest_path.exists() {
            return Err(eyre!("manifest.toml not found in {:?}", plugin_dir));
        }
        
        // Config existence check is done here, read content for 'enabled' flag
        let (enabled, config_read_error) = match fs::read_to_string(&config_path) {
            Ok(content) => {
                match content.parse::<toml::Value>() {
                    Ok(config_value) => {
                        // Default to true if key is missing or not a boolean
                        let enabled_flag = config_value
                            .get("enabled")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true);
                        (enabled_flag, None) // Return enabled flag and no error
                    }
                    Err(e) => {
                        // Config exists but couldn't parse, default to enabled=true, log error
                        (true, Some(eyre!("Failed to parse config.toml {:?}: {}, defaulting to enabled=true", config_path, e)))
                    }
                }
            }
            Err(_) => {
                // Config file doesn't exist or can't be read, default to enabled=true and report error
                 (true, Some(eyre!("config.toml not found or unreadable in {:?}, defaulting to enabled=true", plugin_dir)))
            }
        };
        
        // Log any error encountered while reading the config
        if let Some(err) = config_read_error {
            log::warn!("{}", err);
        }

        // Parse manifest.toml
        let manifest_content = fs::read_to_string(&manifest_path)?;
        let manifest: Manifest = toml::from_str(&manifest_content)?;
        
        // Verify the plugin name matches the directory name
        let dir_name = plugin_dir.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| eyre!("Invalid plugin directory name"))?;
            
        if manifest.plugin.name != dir_name {
             log::warn!("{}", t!("plugin_name_mismatch", manifest_name = manifest.plugin.name, dir_name = dir_name));
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
        
        // Start the plugin process using tokio::process::Command
        let process = Command::new(&executable_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()) // Keep stderr piped for potential logging/debugging
            .current_dir(plugin_dir) // Run in the plugin directory
            .kill_on_drop(true) // Ensure process is killed if Plugin struct is dropped
            .spawn()?;
        
        // Create plugin info, now including the read 'enabled' status
        let plugin_info = PluginInfo {
            name: manifest.plugin.name.clone(),
            version: manifest.plugin.version.clone(),
            description: manifest.plugin.description.clone(),
            enabled, // Use the value read from config.toml
            initialized: false, // Start as not initialized
            icon: manifest.plugin.icon.clone(),
        };

        // Create the RPC channel
        let channel = PluginRpcChannel::new(process).await?;
        
        // Create plugin instance
        let plugin = Plugin {
            info: plugin_info,
            channel,
            directory: plugin_dir.to_path_buf(), // Store the plugin directory path
        };
        
        // Store the plugin
        self.plugins.insert(manifest.plugin.name.clone(), plugin);
        
        Ok(manifest.plugin.name)
    }
    
    /// Initialize all loaded plugins
    pub async fn initialize_plugins(&mut self) -> eyre::Result<Vec<(String, Result<(), String>)>> {
        let plugins_to_init: Vec<String> = self.plugins.keys().cloned().collect();
        let mut results = Vec::new();
        
        for plugin_name in plugins_to_init {
            if let Some(plugin) = self.plugins.get_mut(&plugin_name) {
                // Skip initialization if the plugin is disabled
                if !plugin.info.enabled {
                    log::info!("Skipping initialization for disabled plugin: {}", plugin_name);
                    // Add a specific result indicating it was skipped due to being disabled
                    results.push((plugin_name.clone(), Err(t!("plugin_init_skipped_disabled", name = plugin_name).to_string())));
                    continue;
                }

                // Use the stored directory path instead of plugin name
                let config_path = plugin.directory.join("config.toml")
                    .to_string_lossy()
                    .to_string();
                
                // Check if config file exists and is readable
                if !Path::new(&config_path).exists() {
                    results.push((plugin_name.clone(), 
                                Err(t!("plugin_init_error_config_not_found", name = plugin_name, path = config_path).to_string())));
                    continue;
                }
                
                // Try to read the config file to verify it's accessible
                match fs::read_to_string(&config_path) {
                    Ok(_) => {
                        // Config file is readable, proceed with initialization
                        let params = serde_json::json!({ "config_path": config_path });
                        match plugin.channel.request::<Value, bool>(INITIALIZE_METHOD, params).await {
                            Ok(true) => {
                                plugin.info.initialized = true;
                                results.push((plugin_name.clone(), Ok(())));
                            }
                            Ok(false) => {
                                // Plugin returned false - indicates initialization failure in the plugin itself
                                results.push((plugin_name.clone(), 
                                            Err(t!("plugin_init_rejected", name = plugin_name, 
                                                error = "Plugin explicitly rejected initialization (returned false)").to_string())));
                            }
                            Err(err) => {
                                // Provide detailed error information from the RPC error
                                let error_msg = format!("RPC error: {}", err);
                                log::error!("Plugin '{}' initialization error: {}", plugin_name, error_msg);
                                
                                // Try to determine if it's a communication issue or a plugin internal error
                                let detailed_error = if error_msg.contains("Plugin RPC Error") {
                                    // This was an error returned by the plugin itself
                                    t!("plugin_init_internal_error", name = plugin_name, error = err.to_string())
                                } else if error_msg.contains("Plugin response channel closed") || 
                                          error_msg.contains("Failed to send request") {
                                    // Communication error - plugin may have crashed
                                    t!("plugin_init_communication_error", name = plugin_name, error = err.to_string())
                                } else {
                                    // Other error
                                    t!("plugin_init_error", name = plugin_name, error = err.to_string())
                                };
                                
                                results.push((plugin_name.clone(), Err(detailed_error.to_string())));
                            }
                        }
                    }
                    Err(file_err) => {
                        // Config file exists but can't be read
                        results.push((plugin_name.clone(), 
                                    Err(t!("plugin_init_error_config_not_readable", 
                                        name = plugin_name, path = config_path, error = file_err.to_string()).to_string())));
                    }
                }
            } else {
                // This shouldn't happen since we're iterating over plugin keys
                log::warn!("Attempted to initialize non-existent plugin: {}", plugin_name);
            }
        }
        
        Ok(results)
    }
    
    /// Get time entries from a specific plugin
    pub async fn get_time_entries(&mut self, plugin_name: &str, 
                            start_date: &DateTime<Utc>, 
                            end_date: &DateTime<Utc>) -> eyre::Result<Vec<PluginTimeEntry>> {
        let plugin = self.plugins.get_mut(plugin_name).ok_or_else(|| {
            eyre!("Plugin not found: {}", plugin_name)
        })?;
        
        // Check if plugin is enabled and initialized before proceeding
        if !plugin.info.enabled {
            return Err(eyre!("Plugin '{}' is disabled", plugin_name));
        }
        if !plugin.info.initialized {
            return Err(eyre!("Plugin '{}' is not initialized", plugin_name));
        }
        
        let params = serde_json::json!({
            "start_date": start_date.to_rfc3339(),
            "end_date": end_date.to_rfc3339()
        });
        
        match plugin.channel.request::<Value, Vec<PluginTimeEntry>>(GET_TIME_ENTRIES_METHOD, params).await {
            Ok(mut entries) => {
                // Ensure each entry has the correct plugin_name set
                for entry in &mut entries {
                    entry.plugin_name = Some(plugin_name.to_string());
                }
                Ok(entries)
            }
            Err(err) => {
                 Err(eyre!("Failed to get time entries from {}: {}", plugin_name, err))
            }
        }
    }
    
    /// Get time entries from all plugins for a date range
    pub async fn get_all_time_entries(&mut self, 
                                start_date: &DateTime<Utc>, 
                                end_date: &DateTime<Utc>) -> eyre::Result<(Vec<PluginTimeEntry>, Vec<(String, String)>)> {
        let mut all_entries = Vec::new();
        let mut errors = Vec::new();
        let plugin_names: Vec<String> = self.plugins.keys().cloned().collect();
        
        for plugin_name in plugin_names {
            match self.get_time_entries(&plugin_name, start_date, end_date).await {
                Ok(entries) => {
                    all_entries.extend(entries);
                }
                Err(err) => {
                    // Check if the error is specifically about being disabled or uninitialized
                    let err_string = err.to_string();
                    if err_string.contains("is disabled") || err_string.contains("is not initialized") {
                        // Log as info, don't add to user-facing errors
                        log::info!("Skipping entries for plugin '{}': {}", plugin_name, err_string);
                    } else {
                        // Actual error getting entries, report it
                        errors.push((plugin_name.clone(), 
                                    t!("plugin_get_entries_error", name = plugin_name, error = err_string).to_string()));
                    }
                }
            }
        }
        
        Ok((all_entries, errors))
    }
    
    /// Shutdown all plugins gracefully.
    /// Attempts to send a shutdown request and then ensures the process is terminated.
    pub async fn shutdown(&mut self) -> eyre::Result<Vec<(String, String)>> {
        let mut errors = Vec::new();
        let plugin_names: Vec<String> = self.plugins.keys().cloned().collect();

        for name in plugin_names {
            if let Some(plugin) = self.plugins.remove(&name) {
                // Attempt graceful shutdown via RPC
                match plugin.channel.request::<(), bool>(SHUTDOWN_METHOD, ()).await {
                    Ok(_) => {
                        log::debug!("Plugin {} acknowledged shutdown request.", name);
                    }
                    Err(e) => {
                        // Log shutdown call error but continue with process termination
                        log::warn!("Plugin {} shutdown RPC failed: {}. Proceeding with termination.", name, e);
                        errors.push((name.clone(), 
                                     t!("plugin_shutdown_error", name = name, error = e.to_string()).to_string()));
                    }
                }
                
                // Ensure process is terminated regardless of RPC result
                // The process handle is in PluginRpcChannel, which should be dropped
                // when the Plugin is dropped after being removed from the map.
                // The kill_on_drop(true) on the Command should handle termination.
                // We can add an explicit kill attempt if needed, but let's rely on drop first.
                // plugin.channel.process.kill().await; // Example if explicit kill is needed
            }
        }
        
        // Should be empty now, but clear just in case
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
    
    /// Check if a plugin with the given name is loaded, enabled, and initialized
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.get(name)
            .map(|p| p.info.enabled && p.info.initialized)
            .unwrap_or(false)
    }

    /// Debug a plugin by getting the result field from its response to a get_time_entries request
    /// Returns the pretty-printed JSON string of the result field
    pub async fn debug_plugin_response(&mut self, plugin_name: &str, 
                                start_date: &DateTime<Utc>, 
                                end_date: &DateTime<Utc>) -> eyre::Result<String> {
        let plugin = self.plugins.get_mut(plugin_name).ok_or_else(|| {
            eyre!("Plugin not found: {}", plugin_name)
        })?;
        
        if !plugin.info.initialized {
            return Err(eyre!("Plugin {} is not initialized", plugin_name));
        }
        
        // Create params for debug request
        let params = serde_json::json!({
            "start_date": start_date.to_rfc3339(),
            "end_date": end_date.to_rfc3339()
        });
        
        // Send the request and get the raw response as a String
        let (callback_tx, callback_rx) = oneshot::channel();
        
        let req = JsonRpcRequestWithCallback {
            method: GET_TIME_ENTRIES_METHOD.to_string(),
            params,
            callback: callback_tx,
        };
        
        plugin.channel.tx.send(req).await
            .map_err(|_| eyre!("Failed to send request to plugin communication task"))?;
            
        // Wait for the response from the communication task
        let result_value = callback_rx.await
            .map_err(|_| eyre!("Plugin response channel closed prematurely"))??;
            
        // Return the value as a serialized JSON string
        serde_json::to_string_pretty(&result_value)
            .map_err(|e| eyre!("Failed to serialize debug response: {}", e))
    }
    
    /// Debug plugin initialization specifically
    pub async fn debug_plugin_initialization(&mut self, plugin_name: &str) -> eyre::Result<String> {
        // Check if the plugin is already loaded
        let plugin_loaded_info = if self.plugins.contains_key(plugin_name) {
            let plugin = &self.plugins[plugin_name];
            (true, plugin.info.initialized, None::<String>)
        } else {
            (false, false, None::<String>)
        };

        // If not loaded, search for it in plugins directory based on manifest
        let plugin_dir = if !plugin_loaded_info.0 {
            // Search for the plugin in all subdirectories
            let mut found_dir = None;
            
            if let Ok(entries) = fs::read_dir(&self.plugins_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let manifest_path = path.join("manifest.toml");
                        if manifest_path.exists() {
                            if let Ok(content) = fs::read_to_string(&manifest_path) {
                                if let Ok(manifest) = toml::from_str::<Manifest>(&content) {
                                    if manifest.plugin.name == plugin_name {
                                        found_dir = Some(path);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // If we found a directory or if we should try the direct approach as fallback
            found_dir.unwrap_or_else(|| self.plugins_dir.join(plugin_name))
        } else {
            // Plugin is already loaded, use its stored directory path
            self.plugins[plugin_name].directory.clone()
        };
        
        // Check if the plugin directory exists
        let dir_exists = plugin_dir.exists();
        if !dir_exists {
            return Ok(format!("Plugin directory not found for plugin '{}'. Checked: {:?}", plugin_name, plugin_dir));
        }
        
        // Check for the manifest.toml file
        let manifest_path = plugin_dir.join("manifest.toml");
        if !manifest_path.exists() {
            return Ok(format!("Manifest file not found: {:?}", manifest_path));
        }
        
        // Check for the config.toml file
        let config_path = plugin_dir.join("config.toml");
        if !config_path.exists() {
            return Ok(format!("Config file not found: {:?}", config_path));
        }
        
        // Try to read and parse the manifest
        let manifest_result = match fs::read_to_string(&manifest_path) {
            Ok(content) => {
                match toml::from_str::<Manifest>(&content) {
                    Ok(manifest) => {
                        format!("✅ Manifest parsed successfully:\n- Name: {}\n- Version: {}\n- Description: {}\n- Executable: {}",
                            manifest.plugin.name,
                            manifest.plugin.version,
                            manifest.plugin.description.unwrap_or_else(|| "None".to_string()),
                            manifest.executable.default)
                    }
                    Err(e) => {
                        format!("❌ Failed to parse manifest.toml: {}", e)
                    }
                }
            }
            Err(e) => {
                format!("❌ Failed to read manifest.toml: {}", e)
            }
        };
        
        // Try to read the config file
        let config_result = match fs::read_to_string(&config_path) {
            Ok(content) => {
                format!("✅ Config file read successfully ({}B)", content.len())
            }
            Err(e) => {
                format!("❌ Failed to read config.toml: {}", e)
            }
        };
        
        // Check the executable
        let executable_path = if let Ok(manifest_content) = fs::read_to_string(&manifest_path) {
            if let Ok(manifest) = toml::from_str::<Manifest>(&manifest_content) {
                let executable_name = if cfg!(windows) {
                    manifest.executable.windows.as_ref().unwrap_or(&manifest.executable.default)
                } else {
                    &manifest.executable.default
                };
                plugin_dir.join(executable_name)
            } else {
                plugin_dir.join("unknown_executable")
            }
        } else {
            plugin_dir.join("unknown_executable")
        };
        
        let executable_result = if executable_path.exists() {
            let mut result = format!("✅ Executable found: {:?}", executable_path);
            
            #[cfg(unix)]
            {
                // Check if the file is executable on Unix
                use std::os::unix::fs::PermissionsExt;
                match fs::metadata(&executable_path) {
                    Ok(metadata) => {
                        let permissions = metadata.permissions();
                        if permissions.mode() & 0o111 == 0 {
                            result = format!("❌ Executable exists but is not executable: {:?}", executable_path);
                        } else {
                            result.push_str(" (executable bit is set)");
                        }
                    }
                    Err(e) => {
                        result = format!("❌ Failed to check executable permissions: {:?}: {}", executable_path, e);
                    }
                }
            }
            
            result
        } else {
            format!("❌ Executable not found: {:?}", executable_path)
        };
        
        // Get plugin loaded state
        let (is_loaded, is_initialized, _) = plugin_loaded_info;
        
        // Check if the plugin is already loaded
        let plugin_loaded = if is_loaded {
            format!("✅ Plugin is loaded. Initialized: {}", is_initialized)
        } else {
            // Extract the directory name for comparison with plugin name
            let dir_name = plugin_dir.file_name().and_then(|n| n.to_str()).unwrap_or_default();
            if plugin_name != dir_name {
                format!("❌ Plugin is not loaded (Note: plugin name '{}' doesn't match directory name '{}')", 
                    plugin_name, dir_name)
            } else {
                "❌ Plugin is not loaded".to_string()
            }
        };
        
        // If the plugin isn't loaded, try to load it
        let load_test = if !is_loaded {
            match self.load_plugin(&plugin_dir).await {
                Ok(loaded_name) => {
                    format!("✅ Plugin loaded successfully with name: {}", loaded_name)
                }
                Err(e) => {
                    format!("❌ Failed to load plugin: {}", e)
                }
            }
        } else {
            "⚠️ Plugin already loaded, skipping load test".to_string()
        };
        
        // If the plugin is loaded but not initialized, try to initialize it
        let init_test = if self.plugins.contains_key(plugin_name) && 
                        !self.plugins[plugin_name].info.initialized {
            if let Some(plugin) = self.plugins.get_mut(plugin_name) {
                let config_path = plugin_dir.join("config.toml")
                    .to_string_lossy()
                    .to_string();
                
                let params = serde_json::json!({ "config_path": config_path });
                match plugin.channel.request::<Value, bool>(INITIALIZE_METHOD, params).await {
                    Ok(true) => {
                        plugin.info.initialized = true;
                        "✅ Plugin initialized successfully".to_string()
                    }
                    Ok(false) => {
                        "❌ Plugin returned false during initialization".to_string()
                    }
                    Err(err) => {
                        format!("❌ Plugin initialization error: {}", err)
                    }
                }
            } else {
                "⚠️ Plugin unexpectedly disappeared from registry".to_string()
            }
        } else if self.plugins.contains_key(plugin_name) && 
                 self.plugins[plugin_name].info.initialized {
            "ℹ️ Plugin already initialized, skipping initialization test".to_string()
        } else {
            "⚠️ Plugin not loaded, can't run initialization test".to_string()
        };
        
        // Build and return the complete debug report
        let report = format!(
            "# Plugin Initialization Debug Report for '{}'\n\n\
            ## Directory structure\n\
            Plugin directory: {:?}\n\
            Manifest file: {}\n\
            Config file: {}\n\
            Executable: {}\n\n\
            ## Plugin state\n\
            {}\n\n\
            ## Load test\n\
            {}\n\n\
            ## Initialization test\n\
            {}\n",
            plugin_name,
            plugin_dir,
            manifest_result,
            config_result,
            executable_result,
            plugin_loaded,
            load_test,
            init_test
        );
        
        Ok(report)
    }
} 