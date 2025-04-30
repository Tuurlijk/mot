# Mot Plugin System

Mot includes a plugin system that allows developers to integrate time entries from external sources. Plugins use a simple JSON-RPC protocol over stdin/stdout, making them easy to implement in any language.

## Table of Contents
- [Overview](#overview)
- [Plugin Setup](#plugin-setup)
  - [Location](#location)
  - [Required Files](#required-files)
- [Communication Protocol](#communication-protocol)
  - [JSON-RPC Format](#json-rpc-format)
  - [Required Methods](#required-methods)
- [Data Structures](#data-structures)
- [Example Plugins](#example-plugins)
- [Debugging Guide](#debugging-plugins)
- [Security and Limitations](#security-and-limitations)

## Overview

The plugin system allows mot to:
- Discover plugins in the user's configuration directory
- Initialize plugins with appropriate configuration
- Fetch time entries from external sources
- Display these entries in the main time entry table

## Plugin Setup

### Location

Plugins are discovered in the following location:
- **Linux/macOS**: `~/.config/mot/plugins/`
- **Windows**: `%APPDATA%\mot\plugins\`

Each plugin should be placed in its own subdirectory within the plugins folder. The directory name does not need to match the plugin's name in the manifest.toml file. MOT will use the name specified in the manifest.toml file for internal identification and display purposes.

### Required Files

A plugin directory must contain:

1. **manifest.toml** - Plugin metadata
2. **config.toml** - Plugin configuration
3. **Executable** - The plugin binary or script

#### manifest.toml

This file defines the plugin metadata and executable information:

```toml
[plugin]
name = "example-plugin"
version = "1.0.0"
description = "A plugin to fetch time entries from an example service"
icon = "🔧" # Optional emoji icon to display in the time entry table

[executable]
default = "plugin-executable"      # For Linux/macOS
windows = "plugin-executable.exe"  # For Windows (optional)
```

> **Note:** The plugin name in the manifest can be any string (e.g., "Hello?" or "my-company/gitlab-plugin") and doesn't need to match the directory name. MOT will use this name to identify the plugin internally and for displaying icons.

#### config.toml

This file contains configuration parameters for the plugin:

```toml
# Example configuration for a plugin
enabled = true
api_token = "your-api-token"
base_url = "https://api.example.com"
```

The plugin will be provided with the path to this file during initialization.

#### Executable

This is the actual plugin code (binary or script) that implements the JSON-RPC interface. The executable must:

- Accept input on stdin (JSON-RPC requests)
- Produce output on stdout (JSON-RPC responses)
- Be executable (file permissions `+x` on Unix-like systems)
- Run continuously (don't exit after handling a single request)

## Communication Protocol

Plugins communicate with mot using the JSON-RPC 2.0 protocol over stdin/stdout.

### JSON-RPC Format

> **⚠️ IMPORTANT:** All communication must follow the JSON-RPC 2.0 specification. Each request and response must include the `jsonrpc`, `id`, and either `method`+`params` (for requests) or `result`/`error` (for responses). Responses must always wrap your data in this format - returning raw data arrays without the JSON-RPC envelope will cause errors.

#### Request Format

```json
{
  "jsonrpc": "2.0",
  "method": "method_name",
  "params": {
    "param1": "value1",
    "param2": "value2"
  },
  "id": 1
}
```

#### Response Format

```json
{
  "jsonrpc": "2.0",
  "result": {
    "key": "value"
  },
  "id": 1
}
```

#### Error Response Format

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32000,
    "message": "Error message"
  },
  "id": 1
}
```

### Required Methods

Your plugin must implement the following methods:

#### 1. `initialize`

Called when mot starts or discovers the plugin.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "initialize",
  "params": {
    "config_path": "/path/to/config.toml"
  },
  "id": 1
}
```

**Expected Response:**
```json
{
  "jsonrpc": "2.0",
  "result": true,
  "id": 1
}
```

#### 2. `get_time_entries`

Called to retrieve time entries for a specific date range.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "get_time_entries",
  "params": {
    "start_date": "2023-01-01T00:00:00Z",
    "end_date": "2023-01-07T23:59:59Z"
  },
  "id": 2
}
```

> **Note:** All date/time values in the plugin protocol use UTC timezone (indicated by the 'Z' suffix in RFC3339 format). Your plugin should expect and handle UTC dates consistently.

**Expected Response:**
```json
{
  "jsonrpc": "2.0",
  "result": [
    {
      "id": "unique-id-1",
      "description": "Task description",
      "project_name": "Project Name",
      "customer_name": "Customer Name",
      "started_at": "2023-01-01T09:00:00Z",
      "ended_at": "2023-01-01T10:30:00Z",
      "tags": ["tag1", "tag2"],
      "source": "Source System Name",
      "source_url": "https://example.com/entry/1",
      "billable": true
    }
  ],
  "id": 2
}
```

#### 3. `shutdown`

Called when mot is closing or unloading the plugin.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "shutdown",
  "params": null,
  "id": 3
}
```

**Expected Response:**
```json
{
  "jsonrpc": "2.0",
  "result": true,
  "id": 3
}
```

## Data Structures

### Time Entry Object

| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique identifier for the time entry |
| description | string | Description of the time entry |
| project_name | string (optional) | Name of the associated project |
| customer_name | string (optional) | Name of the associated customer/client |
| started_at | string | Start time (RFC3339 format) |
| ended_at | string | End time (RFC3339 format) |
| tags | array of strings | Tags associated with the entry |
| source | string | Display name for the source system |
| source_url | string (optional) | URL to the entry in the source system |
| billable | boolean | Whether the time entry is billable |

> **Note:** The `plugin_name` field will be automatically populated by MOT with the plugin's manifest name - you don't need to include this field in your returned time entries.

## Example Plugins

### Bash Example

A minimal example of a bash plugin that returns a static time entry:

#### manifest.toml
```toml
[plugin]
name = "hello"
version = "0.1.0"
description = "A simple example plugin that returns a static time entry"

[executable]
default = "hello.sh"
windows = "hello.sh"  # Same for Windows, but would need modifications to work
```

#### config.toml
```toml
# Hello Plugin Configuration
enabled = true
```

#### hello.sh
```bash
#!/bin/bash

# Function to print JSON-RPC error response
send_error() {
  local id=$1
  local code=$2
  local message=$3
  echo "{\"jsonrpc\":\"2.0\",\"error\":{\"code\":$code,\"message\":\"$message\"},\"id\":$id}"
}

# Function to print JSON-RPC success response
send_success() {
  local id=$1
  local result=$2
  echo "{\"jsonrpc\":\"2.0\",\"result\":$result,\"id\":$id}"
}

# Read requests line by line from stdin
while read -r request; do
  # Extract method and id from the request using grep
  method=$(echo "$request" | grep -o '"method":"[^"]*"' | cut -d':' -f2 | tr -d '"')
  id=$(echo "$request" | grep -o '"id":[0-9]*' | cut -d':' -f2)
  
  # Handle different method calls
  case "$method" in
    "initialize")
      # Just return success for initialization
      send_success "$id" "true"
      ;;
      
    "get_time_entries")
      # Get timestamps in a portable way
      now=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
      
      # Simple way to add hours that works on all systems
      hour=$(date -u +"%H")
      hour=$((hour + 2))
      if [ $hour -ge 24 ]; then
        hour=$((hour - 24))
      fi
      hour=$(printf "%02d" $hour)
      
      later=$(date -u +"%Y-%m-%dT${hour}:%M:%SZ")
      
      # Create a JSON array with a single time entry - note the escaped quotes for valid JSON
      time_entries="[{\"id\":\"hello-1\",\"description\":\"Hello from bash plugin\",\"project_name\":\"Example Project\",\"customer_name\":\"Example Customer\",\"started_at\":\"$now\",\"ended_at\":\"$later\",\"tags\":[\"example\",\"bash\"],\"source\":\"Hello Plugin\",\"source_url\":null,\"billable\":true}]"
      
      send_success "$id" "$time_entries"
      ;;
      
    "shutdown")
      # Just send success and exit
      send_success "$id" "true"
      exit 0
      ;;
      
    *)
      # Unknown method
      send_error "$id" -32601 "Method not found: $method"
      ;;
  esac
done
```

Make sure to make the script executable:
```bash
chmod +x hello.sh
```

### Python Example

```python
#!/usr/bin/env python3
import json
import sys
import datetime

def handle_initialize(params, request_id):
    # You can access the config path from params
    config_path = params.get("config_path")
    # You could load the config here if needed
    # config = toml.load(config_path) if os.path.exists(config_path) else {}
    
    return {"jsonrpc": "2.0", "result": True, "id": request_id}

def handle_get_time_entries(params, request_id):
    # Get date range from params
    start_date_str = params.get("start_date", "")
    end_date_str = params.get("end_date", "")
    
    # Create sample entries (in real plugin, fetch from your data source)
    now = datetime.datetime.utcnow()
    time_entries = []
    
    # Create a sample entry
    entry_start = now
    entry_end = now + datetime.timedelta(hours=1)
    
    time_entries.append({
        "id": "python-1",
        "description": "Example from Python plugin",
        "project_name": "Python Project",
        "customer_name": "Python Customer",
        "started_at": entry_start.strftime("%Y-%m-%dT%H:%M:%SZ"),
        "ended_at": entry_end.strftime("%Y-%m-%dT%H:%M:%SZ"),
        "tags": ["python", "example"],
        "source": "Python Plugin",
        "source_url": None,
        "billable": True
    })
    
    return {"jsonrpc": "2.0", "result": time_entries, "id": request_id}

def handle_shutdown(params, request_id):
    # Return success and exit
    response = {"jsonrpc": "2.0", "result": True, "id": request_id}
    print(json.dumps(response))
    sys.stdout.flush()
    sys.exit(0)

# Main request handling loop
for line in sys.stdin:
    try:
        request = json.loads(line)
        method = request.get("method")
        params = request.get("params", {})
        request_id = request.get("id")
        
        if method == "initialize":
            response = handle_initialize(params, request_id)
        elif method == "get_time_entries":
            response = handle_get_time_entries(params, request_id)
        elif method == "shutdown":
            handle_shutdown(params, request_id)
            # handle_shutdown will exit, so we never reach here
        else:
            response = {
                "jsonrpc": "2.0",
                "error": {"code": -32601, "message": f"Method not found: {method}"},
                "id": request_id
            }
            
        print(json.dumps(response))
        sys.stdout.flush()  # Ensure output is sent immediately
        
    except json.JSONDecodeError:
        print(json.dumps({
            "jsonrpc": "2.0",
            "error": {"code": -32700, "message": "Parse error"},
            "id": None
        }))
        sys.stdout.flush()
    except Exception as e:
        print(json.dumps({
            "jsonrpc": "2.0",
            "error": {"code": -32603, "message": f"Internal error: {str(e)}"},
            "id": request_id if 'request_id' in locals() else None
        }))
        sys.stdout.flush()
```

### Node.js Example

```javascript
#!/usr/bin/env node
const readline = require('readline');

// Create interface to read from stdin
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

// Helper function to send success response
function sendSuccess(id, result) {
  console.log(JSON.stringify({
    jsonrpc: "2.0",
    result: result,
    id: id
  }));
}

// Helper function to send error response
function sendError(id, code, message) {
  console.log(JSON.stringify({
    jsonrpc: "2.0",
    error: {
      code: code,
      message: message
    },
    id: id
  }));
}

// Handle initialize method
function handleInitialize(params, id) {
  // You can access config path via params.config_path
  sendSuccess(id, true);
}

// Handle get_time_entries method
function handleGetTimeEntries(params, id) {
  const now = new Date();
  const later = new Date(now.getTime() + 60 * 60 * 1000); // 1 hour later
  
  const timeEntries = [{
    id: "node-1",
    description: "Example from Node.js plugin",
    project_name: "Node.js Project",
    customer_name: "Node.js Customer",
    started_at: now.toISOString(),
    ended_at: later.toISOString(),
    tags: ["node", "javascript", "example"],
    source: "Node.js Plugin",
    source_url: null,
    billable: true
  }];
  
  sendSuccess(id, timeEntries);
}

// Handle shutdown method
function handleShutdown(params, id) {
  sendSuccess(id, true);
  process.exit(0);
}

// Process each line of input
rl.on('line', (line) => {
  try {
    const request = JSON.parse(line);
    const method = request.method;
    const params = request.params || {};
    const id = request.id;
    
    switch (method) {
      case "initialize":
        handleInitialize(params, id);
        break;
      case "get_time_entries":
        handleGetTimeEntries(params, id);
        break;
      case "shutdown":
        handleShutdown(params, id);
        break;
      default:
        sendError(id, -32601, `Method not found: ${method}`);
    }
  } catch (error) {
    if (error instanceof SyntaxError) {
      sendError(null, -32700, "Parse error");
    } else {
      const id = error.request_id ? error.request_id : null;
      sendError(id, -32603, `Internal error: ${error.message}`);
    }
  }
});
```

### External Examples

Beyond the simple examples included in this repository, you can find real-world plugins developed by the community:

- **[mot-plugin-gitlab](https://crates.io/crates/mot-plugin-gitlab)**: An example of a plugin written in Rust and published to crates.io that integrates with the GitLab API to fetch time tracking data. Check its source code for a more complex implementation example.

## Debugging Plugins

If your plugin isn't working as expected:

### Common Issues and Solutions

1. **JSON-RPC format issues**: Always return properly formatted JSON-RPC responses
2. **Plugin exiting early**: Ensure your plugin runs in a continuous loop
3. **Permissions problems**: Make sure your plugin is executable
4. **Date/time handling**: Handle UTC dates correctly
5. **Missing output flushing**: Always flush stdout after writing

### Built-in Debug Mode

MOT includes a built-in debug mode to help diagnose plugin issues:

1. Press 'p' to open the plugin view
2. Select the plugin you want to debug
3. Press Ctrl+D to run the plugin in debug mode
4. MOT will send a test request to the plugin and display the raw response

### Debug Logging

For troubleshooting, add logging to your plugin:

```bash
# Bash example
exec 2> "/path/to/debug.log"
log_debug() { echo "[$(date +"%Y-%m-%dT%H:%M:%S")] $1" >&2; }
log_debug "Plugin started"
```

```python
# Python example
import logging
logging.basicConfig(filename='/path/to/debug.log', level=logging.DEBUG)
logging.debug("Plugin started")
```

### Common Mistakes

1. **Missing JSON-RPC wrapper**: The most common error is returning just the time entries array without the JSON-RPC wrapper. Always format your response as:
   ```json
   {
     "jsonrpc": "2.0",
     "result": [ your_time_entries_array ],
     "id": request_id_from_incoming_request
   }
   ```

2. **Not reading the request ID**: Each response must include the same ID that was in the request.

3. **Forgetting to flush output**: Some languages buffer stdout. Always flush after writing (e.g., `sys.stdout.flush()` in Python).

4. **JSON escaping issues**: When constructing JSON manually (like in bash), ensure all quotes are properly escaped.

5. **Exiting after handling a request**: Your plugin should continue running in a loop to handle multiple requests.

### Error Codes

| Code | Description |
|------|-------------|
| -32700 | Parse error - invalid JSON |
| -32600 | Invalid request - malformed JSON-RPC |
| -32601 | Method not found |
| -32602 | Invalid params |
| -32603 | Internal error |
| -32000 to -32099 | Server error (implementation specific) |

## Security and Limitations

### Security Considerations

- Plugins have access to the system they run on. Be careful with third-party plugins.
- Sensitive information (like API tokens) should be stored securely in the plugin's config file.
- Validate and sanitize all data between mot and plugins.

### Limitations

- Plugins can only add time entries, not modify existing Moneybird entries.
- Plugin time entries are read-only within mot.
- Plugins must provide complete time entry information as mot does not supplement missing data. 