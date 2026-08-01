# Hyprland Configuration User Guide

Welcome to your new Hyprland environment! This guide covers installation, configuration, and daily usage.

## Installation & Updates

To install or update the configuration, clone the repository (if you haven't already) and run the installation script:

```bash
# If you don't have the repository yet:
git clone https://github.com/your-repo/hypr.git

cd hypr
./install.sh
```

The `install.sh` script is safe to re-run at any time to update your installation with the latest changes from the repository.

### How it Works

The installer synchronizes the repository's files with the installation directory at `~/.local/share/hypr`. Your personal themes and configurations in `~/.config/hypr/themes` are **never** touched by the installer.

## Configuration

This project uses a **template-based system** where your final configuration files (e.g., in `~/.config/hypr/`) are **generated** from master templates. This makes the system robust and easy to update.

- **`~/.local/share/hypr/`**: This directory contains the default templates and system themes. **You should not edit these files directly**, as they will be overwritten during updates.

- **`~/.config/hypr/` and other `~/.config/` locations**: This is where the generated configuration files are placed. You should not edit these files directly, as your changes will be lost the next time you switch themes or apply a new setting.

### How to Customize

The recommended way to customize your setup is by modifying the settings in your active theme's `theme.yaml` file.

This file allows you to change any setting for the theme. For example, to change the font size and touchpad scrolling direction, you would edit your theme's `theme.yaml` to look like this:

```yaml
fonts:
    size: 14

input:
    touchpad_natural_scroll: "true"
```

After saving the file, re-apply your current theme using the `hypr-theme-set` command or the Preferences App to see your changes.

For more advanced customization, see the full **[Theming Guide](THEMING.md)**.

### Themes

Themes define the overall look and feel of your desktop. You can switch themes using the `hypr-theme-set` command or the "Theme" page in the Preferences App. This triggers the generator to rebuild all your configurations with the new theme's settings.

For a detailed guide on how to customize themes, see **[docs/THEMING.md](THEMING.md)**.

## Preferences App

Launch the Preferences App from the application launcher (search for "Preferences") or by running `hypr-preferences-app`.

When you change settings in the app, they are staged internally. When you click "Save & Apply", the app writes your changes to your `theme.yaml`, and then regenerates all necessary configurations to apply the changes system-wide.

Features:

- **Theme**: Change the active theme and font.
- **Appearance**: Customize gaps, borders, shadows, and blur.
- **Keybindings**: View, edit, and interactively capture keybindings. The editor includes conflict detection for existing Hyprland bindings and common system shortcuts.
- **System**: Run maintenance tasks.

## Keybindings Editor App

Launch the Keybindings Editor App from the application launcher (search for "Keybindings") or by running `hypr-keybindings-editor`.

This application provides a dedicated interface for managing your Hyprland keybindings. You can interactively capture key combinations, and the editor will automatically detect potential conflicts with your existing configuration or common system shortcuts, providing immediate feedback. It also launches as a floating window for convenience.

## System Maintenance

This distribution includes several utility scripts:

- **`hypr-doctor`**: Diagnoses common issues with your setup.
  - Run: `hypr-doctor`
- **`hypr-update`**: Updates system packages (pacman/yay), git repositories, and snapshots.
  - Run: `hypr-update`
- **`hypr-clean`**: Cleans unused packages and caches to free up space.
  - Run: `hypr-clean`
- **`hypr-wallpaper-set`**: CLI tool to set a specific image as the wallpaper.
  - Run: `hypr-wallpaper-set /path/to/image.jpg`
- **Preferences App**: Visually manage wallpapers via the "Wallpaper" tab.
  - Run: `hypr-preferences-app`

## Troubleshooting

If you encounter issues:

Run `hypr-doctor` to check for missing dependencies or configuration errors.
Check the log file at `~/.local/state/hypr/hypr.log`.
Ensure your user config files are valid.
