# Improvements and Optimizations

This document outlines potential structural and functional improvements for the repository.

## Architecture & Maintenance

### 1. Unified Package Management
**Current State:** Packages are installed via various scripts (e.g., `webapps.sh`), mixing logic with data.
**Improvement:** Extract package lists into plain text files (e.g., `packages.txt`, `webapps.csv`). The install scripts should read from these files. This separation allows users to easily view, add, or remove dependencies without modifying the script logic.

### 2. Interactive Installer
**Current State:** The `install.sh` script runs a predefined sequence of installation steps.
**Improvement:** Implement an interactive mode or command-line flags (e.g., `--minimal`, `--no-gaming`, `--server`). This is particularly relevant for VM environments where users might want a lighter footprint (skipping Steam, Docker, or specific web apps).

### 3. Systemd User Services
**Current State:** Background tasks are likely managed via `autostart.conf` or cron-like behavior.
**Improvement:** Migrate persistent user services (like battery monitoring, weather fetching, or custom Waybar updaters) to **Systemd User Units**. This provides robust process management, automatic restarts on failure, and centralized logging via `journalctl --user`.

### 4. Code Quality & Linting
**Current State:** Scripts use `set -euo pipefail` which is good, but static analysis is manual.
**Improvement:** Integrate `ShellCheck` into the development workflow (e.g., as a pre-commit hook or CI step). This will automatically catch common bash pitfalls, quoting issues, and portability problems across the many scripts in `bin/`.

## Usability Enhancements

### 5. Theme Previews
**Current State:** Themes are selected by name without visual feedback.
**Improvement:** Implement a mechanism to generate or store screenshot previews for each theme. A selection tool (like a custom `rofi`/`walker` mode) could display these previews, making it easier to choose a theme.

### 6. Secrets & API Key Management
**Current State:** If any scripts (like weather or AI tools) require API keys, they might be hardcoded or require manual config editing.
**Improvement:** Standardize a secrets management approach, such as reading from a specific secure file (e.g., `~/.config/hypr/secrets.env`) that is ignored by git, or using a password manager CLI integration.

### 7. Presentation Mode / Idle Inhibition
**Current State:** `hypridle` is configured for automatic locking.
**Improvement:** Add a "Presentation Mode" toggle in the UI (Waybar/Menu). This would temporarily inhibit `hypridle` (e.g., `hyprctl dispatch dpms on` persistence or stopping the idle service) to prevent screen locking during video playback or VM usage, which is a common annoyance.

### 8. XDG Base Directory Compliance
**Current State:** Some scripts may rely on fixed paths like `~/.config` or `~/.local`.
**Improvement:** rigoroulsy audit all scripts to use `$XDG_CONFIG_HOME`, `$XDG_DATA_HOME`, and `$XDG_CACHE_HOME` environment variables. This ensures the setup respects user preferences for directory structure and makes the system more portable.
