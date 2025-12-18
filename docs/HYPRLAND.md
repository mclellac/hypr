# Hyprland Configuration Documentation

This document describes the Hyprland configuration structure used in this environment.

## Overview

The Hyprland configuration is modular, sourcing multiple files to manage specific aspects of the desktop environment such as keybindings, autostart rules, monitors, and look-and-feel.

The main entry point is `~/.config/hypr/hyprland.conf`.

## File Structure

The configuration files are located in `~/.config/hypr/` (installed from `config/hypr/` in the repository).

*   `hyprland.conf`: The main configuration file. It acts as a coordinator, sourcing other files.
*   `autostart.conf`: Commands executed on Hyprland startup (e.g., launching Waybar, notification daemons, wallpaper scripts).
*   `bindings.conf`: Keybindings for window management, application launching, and system controls.
*   `input.conf`: Keyboard and mouse input settings (layout, sensitivity, touchpad gestures).
*   `monitors.conf`: Monitor configuration (resolution, refresh rate, positioning).
*   `envs.conf`: Environment variable definitions.
*   `hypridle.conf`: Idle configuration (managed by `hypridle`).
*   `hyprlock.conf`: Lock screen configuration (managed by `hyprlock`).
*   `hyprsunset.conf`: Blue light filter configuration.
*   `current/theme/hyprland.conf`: A symlink to the active theme's specific Hyprland settings (e.g., border colors, decoration rules).

## Default vs. User Configuration

The `hyprland.conf` file attempts to source "default" configurations from `~/.local/share/hypr/default/hypr/` before sourcing the user configurations in `~/.config/hypr/`. This structure implies a layered approach, although the current repository installation process primarily populates `~/.config/hypr`.

## Keybindings (`bindings.conf`)

Keybindings generally follow standard Hyprland conventions but may be customized. Refer to `config/hypr/bindings.conf` for the exact list.

Common patterns include:
*   `SUPER + Q`: Launch Terminal (Alacritty/Kitty).
*   `SUPER + C`: Close active window.
*   `SUPER + M`: Exit Hyprland.
*   `SUPER + E`: File Manager.
*   `SUPER + V`: Toggle floating.
*   `SUPER + R`: Application Launcher (Walker).

## Theming

Theme-specific Hyprland settings are applied via the `current/theme/hyprland.conf` symlink. This file typically defines:
*   Border colors (`col.active_border`, `col.inactive_border`).
*   Shadows and window decorations.
*   Specific window rules related to the theme.

## Window Rules

Window rules (both static and dynamic) are defined to control the behavior of specific applications (e.g., forcing a window to float or open in a specific workspace). These are likely distributed between `hyprland.conf` (or its sourced files) and the theme-specific config.
