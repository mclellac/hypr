# Scripts Documentation

This document provides an overview of the custom scripts located in the `bin/` directory. These scripts automate tasks, manage the environment, and provide functionality for UI elements.

## Theme Management

*   `hypr-theme-set <theme-name>`: The core script for switching themes. It updates symlinks, reloads applications, and triggers wallpaper changes.
*   `hypr-theme-list`: Lists available themes.
*   `hypr-theme-bg-next`: Cycles to the next wallpaper in the current theme's `backgrounds/` directory.
*   `hypr-theme-current`: Outputs the name of the current theme.

## System & Maintenance

*   `arch-update`: Checks for available package updates (both official and AUR). Used by Waybar.
*   `hypr-update`: A wrapper for system updates.
*   `hypr-pkg-install / hypr-pkg-remove`: Helpers for package management.

## UI & Menus

*   `hypr-menu`: A general-purpose menu wrapper using `walker`.
*   `hypr-power-menu`: Launches a power menu (Shutdown, Reboot, etc.) using `walker`.
*   `hypr-launch-browser`: Intelligently launches the web browser, handling `.desktop` file parsing.
*   `hypr-screenshot` / `hypr-cmd-screenshot`: Tools for taking screenshots.
*   `hypr-weather`: Fetches weather data for the Waybar tooltip.

## Hardware & Toggles

*   `hypr-toggle-nightlight`: Toggles the blue light filter (likely `hyprsunset`).
*   `hypr-cmd-audio-switch`: Switches audio output devices.
*   `hypr-font-set`: Updates the font configuration for Waybar.

## Development & Misc

*   `audit_palettes_v2.py`: A Python script to audit and generate `palette.md` files, ensuring theme consistency.
*   `docs-menu`: A menu for accessing documentation.

## Notes

Many scripts rely on the `HYPR_PATH` environment variable or assume a specific directory structure. They often use `set -euo pipefail` for robustness.
