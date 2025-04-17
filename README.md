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
- 🌐 Multilingual support with easy language switching

## 🚀 Installation

### Using Cargo

```bash
cargo install mot
```

### Pre-built Binaries

Pre-built binaries and packages are available on the [GitHub releases](https://github.com/Tuurlijk/mot/releases) page for multiple platforms and architectures:

#### Binary Archives
- **Linux**: x86_64, ARM (32/64-bit), RISC-V, PowerPC, s390x, and MUSL variants
- **Windows**: 32-bit and 64-bit zip archives
- **macOS**: Intel and Apple Silicon (ARM64) builds
- **FreeBSD**: x86_64 builds

#### Package Formats
- **Debian/Ubuntu**: Native `.deb` packages
- **Red Hat/Fedora/SUSE**: RPM packages
- **Arch Linux**: AUR package
- **macOS**: Homebrew formula and DMG disk image
- **Nix**: Package for NixOS and Nix package manager

Each release includes SHA256 checksums for verifying file integrity.

#### Quick Installation

```bash
# Linux x86_64 example
curl -L https://github.com/Tuurlijk/mot/releases/download/[version]/mot-linux-x86_64.tar.gz | tar xz
./mot

# Or install using your system's package manager
# Debian/Ubuntu
sudo dpkg -i mot_[version]_amd64.deb

# Homebrew (macOS)
brew install mot
```

Replace `[version]` with the desired release version (e.g., `v1.4.56`).

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
language = "en" # Options: en, nl (Optional, defaults to system language)
```

## 🌐 Localization

MOT provides full internationalization support for all user-facing text:

- **Supported Languages**:
  - English (en)
  - Dutch (nl)

- **Setting the Language**:
  - Via command line: `mot -l nl` or `mot --language nl`
  - Via configuration file: Add `language = "nl"` to your config.toml

You can also override the language for a single session by using the command line flag, which takes precedence over the configuration file setting.

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
- [x] Add ci workflows
- [x] Localization (English and Dutch)
- [ ] Additional language support (contributions welcome!)

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