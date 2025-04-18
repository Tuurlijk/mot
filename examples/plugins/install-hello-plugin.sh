#!/bin/bash

# Script to install the "hello" example plugin

# Get the config directory
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/mot/plugins/hello"

# Create the plugin directory
mkdir -p "$CONFIG_DIR"

echo "Creating plugin in: $CONFIG_DIR"

# Create manifest.toml
cat > "$CONFIG_DIR/manifest.toml" << 'EOF'
[plugin]
name = "hello"
version = "0.1.0"
description = "A simple example plugin that returns a static time entry"

[executable]
default = "hello.sh"
windows = "hello.sh"  # Same for Windows, but would need modifications to work
EOF

# Create config.toml
cat > "$CONFIG_DIR/config.toml" << 'EOF'
# Hello Plugin Configuration
enabled = true
EOF

# Create hello.sh
cat > "$CONFIG_DIR/hello.sh" << 'EOF'
#!/bin/bash

# Enable debugging output to a log file
exec 2> "${XDG_CONFIG_HOME:-$HOME/.config}/mot/plugins/hello/debug.log"

# Debug function to log messages
log_debug() {
  echo "[$(date +"%Y-%m-%dT%H:%M:%S")] $1" >&2
}

log_debug "Plugin started"

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
  log_debug "Received request: $request"
  
  # Extract method and id from the request using grep
  method=$(echo "$request" | grep -o '"method":"[^"]*"' | cut -d':' -f2 | tr -d '"')
  id=$(echo "$request" | grep -o '"id":[0-9]*' | cut -d':' -f2)
  
  log_debug "Extracted method: $method, id: $id"
  
  # Handle different method calls
  case "$method" in
    "initialize")
      # Just return success for initialization
      log_debug "Handling initialize"
      send_success "$id" "true"
      ;;
      
    "get_time_entries")
      log_debug "Handling get_time_entries"
      
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
      
      log_debug "now: $now, later: $later"
      
      # Create a JSON array with a single time entry
      # Using compact JSON to reduce potential formatting errors
      time_entries="[{\"id\":\"hello-1\",\"description\":\"Hello from bash plugin\",\"project_id\":\"proj-1\",\"project_name\":\"Example Project\",\"customer_id\":\"cust-1\",\"customer_name\":\"Example Customer\",\"started_at\":\"$now\",\"ended_at\":\"$later\",\"tags\":[\"example\",\"bash\"],\"source\":\"Hello Plugin\",\"source_url\":null,\"billable\":true}]"
      
      log_debug "Sending response with time entries"
      send_success "$id" "$time_entries"
      ;;
      
    "shutdown")
      # Just send success and exit
      log_debug "Handling shutdown"
      send_success "$id" "true"
      log_debug "Plugin shutting down"
      exit 0
      ;;
      
    *)
      # Unknown method
      log_debug "Unknown method: $method"
      send_error "$id" -32601 "Method not found: $method"
      ;;
  esac
done

log_debug "Plugin exited main loop"
EOF

# Make the script executable
chmod +x "$CONFIG_DIR/hello.sh"

echo "Plugin installation complete!"
echo "To test the plugin, start mot and press 'p' to view plugins."
echo "If everything is working, you should see the 'hello' plugin listed."
echo "When you return to the main view, you should see an additional time entry from the plugin."
echo "Debug logs are available at: ${CONFIG_DIR}/debug.log" 