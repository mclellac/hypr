# Theming System Documentation

This document describes the dynamic theming system used to manage the appearance of Hyprland and related applications.

## Overview

The theming system is built around a central symlink `~/.config/hypr/current/theme` which points to the active theme directory within `~/.config/hypr/themes/`. Applications are configured to look for their theme-specific settings relative to this path or are updated dynamically when the theme changes.

## Theme Directory Structure

All themes are stored in `~/.config/hypr/themes/` (installed from `themes/` in the repository). Each theme directory (e.g., `themes/cyberpunk/`) is a self-contained unit containing configuration snippets for various applications.

Typical contents of a theme directory:

*   `palette.md`: The definitive source of truth for the theme's color palette.
*   `hyprland.conf`: Hyprland settings (border colors, decoration).
*   `hyprlock.conf`: Lock screen appearance.
*   `waybar.css`: CSS variables for Waybar.
*   `kitty.conf`: Color definitions for the Kitty terminal.
*   `alacritty.toml`: Color definitions for Alacritty.
*   `walker.css`: Styling for the Walker application launcher.
*   `mako.ini`: Notification daemon styling.
*   `btop.theme`: Color scheme for btop system monitor.
*   `neovim.plugin`: A Lua file (lazy.nvim spec) to apply the corresponding Neovim theme.
*   `chromium.theme`: RGB values for Chromium browser theming.
*   `icons.theme`: Name of the icon theme to use.
*   `backgrounds/`: A directory containing wallpaper images for the theme.

## Theme Switching Mechanism

Themes are switched using the `hypr-theme-set` script (located in `bin/`). This script performs the following actions:

1.  **Symlink Update**: Updates `~/.config/hypr/current/theme` to point to the new theme directory.
2.  **Application Updates**:
    *   **Walker**: Combines the theme's `walker.css` with a base CSS file to generate the active configuration.
    *   **Neovim**: Symlinks `neovim.plugin` to `~/.config/nvim/lua/plugins/theme.lua`.
    *   **GTK/GNOME**: Updates GTK theme, icon theme, and color scheme (light/dark) using `gsettings`.
    *   **Chromium**: Updates Chromium's theme color and mode.
3.  **Reloading**: Sends signals (`SIGUSR1`, `SIGUSR2`) or reloads commands (`hyprctl reload`, `makoctl reload`) to refresh running applications immediately.
4.  **Wallpaper**: cycles to a new wallpaper from the new theme's `backgrounds/` folder using `hypr-theme-bg-next`.

## Palette Definition (`palette.md`)

Each theme includes a `palette.md` file. This file documents the hex codes for every color used in the theme, mapped to semantic names (e.g., `accent_color`, `background_color`, `term_color0`). Scripts may use this file to audit or generate configurations, ensuring consistency across all applications.

## Creating a New Theme

To create a new theme:
1.  Copy an existing theme directory (e.g., `themes/material`).
2.  Rename the directory.
3.  Edit `palette.md` to define the new colors.
4.  Update all configuration files (`waybar.css`, `kitty.conf`, etc.) to match the new palette.
5.  Add wallpapers to the `backgrounds/` subdirectory.
