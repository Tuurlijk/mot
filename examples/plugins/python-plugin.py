#!/usr/bin/env python3
"""
Example Python plugin for mot

Installation:
1. Create the plugin directory: ~/.config/mot/plugins/python-example/
2. Copy this file to that directory and make it executable:
   chmod +x python-plugin.py
3. Create manifest.toml and config.toml (see below)
"""

import json
import sys
import datetime
import os.path

# Example manifest.toml:
"""
[plugin]
name = "python-example"
version = "0.1.0"
description = "Example Python plugin for mot"

[executable]
default = "python-plugin.py"
windows = "python-plugin.py"
"""

# Example config.toml:
"""
# Python Plugin Configuration
enabled = true
num_entries = 3  # Number of example entries to generate
"""

# Function to handle 'initialize' method
def handle_initialize(params, request_id):
    config_path = params.get("config_path")
    # You could load the config file here
    # config = toml.load(config_path) if os.path.exists(config_path) else {}
    
    # For this example, we just log the path and return success
    log_debug(f"Initializing with config path: {config_path}")
    return {"jsonrpc": "2.0", "result": True, "id": request_id}

# Function to handle 'get_time_entries' method
def handle_get_time_entries(params, request_id):
    # Get date range from params
    start_date_str = params.get("start_date", "")
    end_date_str = params.get("end_date", "")
    
    log_debug(f"Fetching time entries from {start_date_str} to {end_date_str}")
    
    # Create sample entries (in real plugin, fetch from your data source)
    now = datetime.datetime.utcnow()
    time_entries = []
    
    # Create 3 sample entries
    for i in range(3):
        entry_start = now + datetime.timedelta(hours=i)
        entry_end = entry_start + datetime.timedelta(hours=1)
        
        time_entries.append({
            "id": f"python-{i+1}",
            "description": f"Python task #{i+1}",
            "project_id": "py-proj-1",
            "project_name": "Python Project",
            "customer_id": "py-cust-1",
            "customer_name": "Python Customer",
            "started_at": entry_start.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "ended_at": entry_end.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "tags": ["python", "example", f"task-{i+1}"],
            "source": "Python Example Plugin",
            "source_url": None,
            "billable": True
        })
    
    return {"jsonrpc": "2.0", "result": time_entries, "id": request_id}

# Function to handle 'shutdown' method
def handle_shutdown(params, request_id):
    log_debug("Shutting down plugin")
    # Return success and exit
    response = {"jsonrpc": "2.0", "result": True, "id": request_id}
    print(json.dumps(response))
    sys.stdout.flush()
    sys.exit(0)

# Helper function to log debug messages
def log_debug(message):
    # For logging you could write to a file in the plugin directory
    # with open("/tmp/mot-python-plugin.log", "a") as f:
    #     f.write(f"{datetime.datetime.now().isoformat()}: {message}\n")
    pass

# Main request handling loop
def main():
    log_debug("Plugin started")
    
    for line in sys.stdin:
        log_debug(f"Received request: {line.strip()}")
        
        try:
            request = json.loads(line)
            method = request.get("method")
            params = request.get("params", {})
            request_id = request.get("id")
            
            log_debug(f"Processing method: {method}")
            
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
                
            log_debug(f"Sending response: {json.dumps(response)}")
            print(json.dumps(response))
            sys.stdout.flush()  # Ensure output is sent immediately
            
        except json.JSONDecodeError:
            log_debug("Error: Invalid JSON")
            error_response = {
                "jsonrpc": "2.0",
                "error": {"code": -32700, "message": "Parse error"},
                "id": None
            }
            print(json.dumps(error_response))
            sys.stdout.flush()
        except Exception as e:
            log_debug(f"Error: {str(e)}")
            error_response = {
                "jsonrpc": "2.0",
                "error": {"code": -32603, "message": f"Internal error: {str(e)}"},
                "id": request_id if 'request_id' in locals() else None
            }
            print(json.dumps(error_response))
            sys.stdout.flush()

if __name__ == "__main__":
    main() 