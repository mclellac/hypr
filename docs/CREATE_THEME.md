# Creating a Custom Theme

This guide walks you through the process of creating a new theme for the Hyprland environment.

## Overview

A theme is a collection of configuration files and assets that define the look and feel of your desktop. Themes are stored in `$XDG_DATA_HOME/hypr/themes/` (default: `~/.local/share/hypr/themes/`) (system themes) or `$XDG_CONFIG_HOME/hypr/themes/` (default: `~/.config/hypr/themes/`) (user themes).

## Directory Structure

A valid theme directory must contain the following:

```text
my-theme/
├── theme.yaml              # (Required) Main configuration file
├── backgrounds/            # (Required) Directory with wallpaper images
├── waybar.theme.css        # (Optional) Waybar CSS styling
├── kitty.theme.conf        # (Optional) Kitty terminal settings
├── alacritty.theme.toml    # (Optional) Alacritty terminal settings
├── ghostty.theme.conf      # (Optional) Ghostty terminal settings
├── mako.theme.conf         # (Optional) Mako notification settings
├── swayosd.theme.css       # (Optional) SwayOSD CSS styling
├── neovim.theme.lua        # (Optional) Neovim theme settings
├── hyprlock.theme.conf     # (Optional) Hyprlock settings
├── btop.theme              # (Optional) Btop theme settings
├── chromium.theme          # (Optional) Chromium theme settings
├── ... (other partials)
└── icons.theme             # (Optional) GTK icon theme name
```

## Step-by-Step Guide

### Create a User Theme

The easiest way to start is to copy an existing system theme.

```bash
# Copy the 'material' theme to your user themes directory
cp -r $XDG_DATA_HOME/hypr/themes/material $XDG_CONFIG_HOME/hypr/themes/my-custom-theme
```

### Edit `theme.yaml`

The `theme.yaml` file is the heart of your theme. It defines colors, fonts, layout settings, and more.

#### Full Configuration Reference

For a complete list of all available options in `theme.yaml`, including detailed descriptions and application mappings, please refer to the **[theme.yaml Reference Guide](THEME.YAML.md)**.

### Customize Partials

Partials allow you to inject raw configuration code into the final generated files.

* **`waybar.theme.css`**: Add custom CSS to style Waybar beyond the variables.
* **`kitty.theme.conf`**: Add specific Kitty config lines.
* **`neovim.theme.lua`**: Add Lua code for Neovim theming.

### Apply Your Theme

Once you are done editing, apply the theme using the command line:

```bash
hypr-theme-set "my-custom-theme"
```

The system will regenerate all configurations and reload the applications.
