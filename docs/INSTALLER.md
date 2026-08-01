# Installer Guide

The `install.sh` script is the main entry point for installing, updating, and re-installing the Hyprland environment. It is designed to be intelligent, non-destructive when updating, and thorough when forcing a fresh install.

## Usage

```bash
./install.sh [OPTIONS]
```

### Options

* `--update`: Explicitly update the code and templates *only*. This preserves your user configurations, your active theme, and your currently selected wallpaper. It will not overwrite your `~/.config/hypr` directory.
* `--force`: Force a full re-installation. This completely overwrites all configurations and effectively acts as a fresh install, deleting your current `~/.config/hypr` and recreating it from defaults.
* `--theme <name>`: Set a specific theme after the installation is complete. If used with `--update`, it will switch to this theme while preserving other settings.
* `--packages`: Only check and install missing system packages without reapplying configurations or resetting themes.
* `--help`: Show the help message.

### Default Behavior (No Arguments)

If `install.sh` is run without any arguments, it will check for an existing installation (checking if the installation folder `~/.local/share/hypr` and the configuration folder `~/.config/hypr` both exist):

**If an existing installation is detected**, it defaults to **Update Install** (`UPDATE` mode) to safely apply updates without destroying your custom layouts, themes, or wallpapers.
**If no existing installation is found**, it defaults to a **Fresh Install** (`FRESH` mode) to set up the default layout and theme config.

## How It Works

### Installation Modes

The installer operates in two primary modes: `FRESH` and `UPDATE`.

* **FRESH Mode (`--force` or fresh install)**:
  * Wipes the existing configuration (`~/.config/hypr`).
  * Wipes the existing installed files (`~/.local/share/hypr`).
  * Re-creates everything from the repository source and recompiles Rust binaries.

* **UPDATE Mode (`--update` or update install)**:
  * Retains the existing `~/.config/hypr` directory.
  * Preserves user-specific template settings (specifically backing up and restoring `templates/hypr/settings.yaml`).
  * Copies new code changes into `~/.local/share/hypr` without aggressively wiping user configuration data.
  * Preserves the currently active theme and wallpaper.

### Synchronization

The installer synchronizes the contents of the repository to your user's local share directory (`~/.local/share/hypr`).
If a fresh install is triggered, the directory is aggressively cleaned out before copying, ensuring files that were removed from the repository are also correctly removed from your system.

### Automatic Reloading

When the installation completes, `install.sh` will automatically reload Hyprland and associated applications. It does this by:

* Calling `hyprctl reload`.
* Invoking the Rust reloader binary (`hypr-theme-templater`) to trigger soft reloads for services like Waybar, Mako, and SwayOSD.

You do not need to manually log out or reboot to see changes.
