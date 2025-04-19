#!/bin/bash

# Script to install the Python example plugin

# Get the config directory
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/mot/plugins/python-example"

# Create the plugin directory
mkdir -p "$CONFIG_DIR"

echo "Creating plugin in: $CONFIG_DIR"

# Create manifest.toml
cat > "$CONFIG_DIR/manifest.toml" << 'EOF'
[plugin]
name = "python-example"
version = "0.1.0"
description = "Example Python plugin for mot"
icon = "🐍"  # Snake emoji will be displayed in the time entry table

[executable]
default = "python-plugin.py"
windows = "python-plugin.py"
EOF

# Create config.toml
cat > "$CONFIG_DIR/config.toml" << 'EOF'
# Python Plugin Configuration
enabled = true
num_entries = 3  # Number of example entries to generate
EOF

# Copy the python script
cp "$(dirname "$0")/python-plugin.py" "$CONFIG_DIR/"
chmod +x "$CONFIG_DIR/python-plugin.py"

echo "Plugin installation complete!"
echo "To test the plugin, start mot and press 'p' to view plugins."
echo "If everything is working, you should see the 'python-example' plugin listed."
echo "When you return to the main view, you should see additional time entries from the plugin." 