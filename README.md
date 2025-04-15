# 📊 MOT - MoneyBird Time Tracker

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A beautiful terminal-based interface for managing your MoneyBird time entries with style.

![MOT Screenshot][mot.png]

## ✨ Features

- 🖥️ Terminal-based UI built with Ratatui 
- 🚀 Blazingly fast, written in Rust
- 📆 Browse and navigate time entries by week
- 🔄 Automatically refreshes data
- 🧩 Connects directly to MoneyBird's API
- 🌙 Proper error handling with helpful messages
- 🔐 Secure configuration for your API credentials
- 🌓 Automatic detection of system theme (light/dark mode)

## 🚀 Installation

### Using Cargo

```bash
cargo install mot
```

### From Source

```bash
git clone https://github.com/Tuurlijk/mot.git
cd mot
cargo build --release
```

The binary will be available in `target/release/mot`.

## 🔧 Configuration

On first run, MOT will create a default configuration file at:
- Linux/macOS: `~/.config/mot/config.toml`
- Windows: `%APPDATA%\mot\config.toml`

You'll need to edit this file to add your MoneyBird API token:

```toml
access_token = "your_moneybird_api_token"
api_url = "https://moneybird.com/api/v2"
administration_id = "your_administration_id" # Optional
week_starts_on = "monday" # Options: monday, tuesday, wednesday, thursday, friday, saturday, sunday
```

## 📖 Usage

Simply run `mot` to start the application. Use the following keyboard shortcuts:

### General Navigation & Actions

-   `◀` / `h`: Previous week
-   `▶` / `l`: Next week
-   `t`: Go to current week
-   `r`: Refresh time entries
-   `▲` / `k`: Move selection up
-   `▼` / `j`: Move selection down
-   `q`: Quit the application
-   `F12`: Toggle log panel visibility

### Time Entry Management (Main View)

-   `c`: Create a new time entry
-   `e` / `Enter` / `Space`: Edit selected time entry
-   `d` / `Delete`: Delete selected time entry (with confirmation)
-   `x`: Export current view to CSV (with confirmation)

### Search Mode (Filter)

-   `f` / `/`: Enter search/filter mode
-   *Type to filter entries*
-   `Esc`: Exit search mode
-   `Ctrl+U`: Clear search input

### Edit Mode

-   `Tab`: Move to next field
-   `Shift+Tab`: Move to previous field
-   `Ctrl+S`: Save changes (create or update entry)
-   `Esc`: Cancel editing / Hide autocomplete dropdown
-   **(Project/Contact Fields)**
    -   *Type to search*
    -   `Enter`: Select highlighted autocomplete item / Move to next field if no dropdown
    -   `↑` / `↓`: Navigate autocomplete dropdown
    -   `Ctrl+U`: Clear autocomplete input
-   **(Description Field)**
    -   `Enter`: Move to next field
    -   `Shift+Enter`: Insert newline
-   **(Date/Time Fields)**
    -   `Enter`: Move to next field

### User Selection (Initial Setup)

-   `↑` / `k`: Select previous user
-   `↓` / `j`: Select next user
-   `Enter`: Confirm selected user and save to config

### Modals (Pop-ups)

-   `Enter` / `y`: Confirm action / Dismiss info/error modal
-   `Esc` / `n`: Cancel action / Dismiss modal
-   *Any other key*: Dismiss info/error modal

## 🛣️ Roadmap

- [x] Add screenshot before release
- [ ] Add vhs gif recording with dummy data
- [x] Filter view by client and project
- [x] Export filtered CSV for a given week number
- [x] Internal logging + logging pane
- [x] CRUD operations for time entries
- [ ] Pull time logs from gitlab using dialogue
- [ ] Contact browser
- [ ] CRUD operations for contacts
- [ ] Project browser
- [ ] CRUD operations for projects
- [ ] Add ci workflows

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 💖 Acknowledgements

- [MoneyBird](https://www.moneybird.com/) for providing the API
- [Ratatui](https://github.com/ratatui-org/ratatui) for the amazing TUI framework
- [Dennis Ameling](https://github.com/dennisameling/moneybird-openapi) for the original MoneyBird OpenAPI specification, which was trimmed using [apisnip](https://crates.io/crates/apisnip) for this project.
- All contributors who have helped shape this project

[mot.png]: https://github.com/Tuurlijk/mot/blob/images/images/mot.png?raw=true