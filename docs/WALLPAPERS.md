# Wallpaper Management

This document describes the wallpaper management system for the Hyprland environment.

## Overview

Wallpaper management has been decoupled from themes to allow for user persistence and greater flexibility. The system consists of:

1.  **Persistence Mechanism**: A symlink at `~/.config/hypr/current/background` that points to the active wallpaper.
2.  **CLI Tool**: `hypr-wallpaper-set` updates the symlink and reloads `swaybg`.
3.  **GUI Application**: `hypr-wallpaper-manager` allows users to browse and select wallpapers visually.

## Components

### `hypr-wallpaper-manager` (GUI)

A GTK4 application for managing wallpapers.

**Features:**
- **Grid View**: Displays thumbnails of available wallpapers.
- **Sources**:
    - **Theme Wallpapers**: Automatically detects wallpapers from the current theme (`~/.config/hypr/current/theme/backgrounds`).
    - **User Library**: Manages user-added wallpapers in `~/.local/share/hypr/wallpapers`.
- **Add Wallpaper**: Allows importing external images into the user library.
- **Set Wallpaper**: Updates the desktop background immediately and persistently.

**Usage:**
Launch "Wallpaper Manager" from the application launcher (Walker), or run:
```bash
hypr-wallpaper-manager
```

### `hypr-wallpaper-set` (CLI)

The underlying script used to set the wallpaper.

**Usage:**
```bash
hypr-wallpaper-set /path/to/image.jpg
```

This script:
1.  Validates the image file.
2.  Updates `~/.config/hypr/current/background` symlink.
3.  Restarts `swaybg` via `uwsm`.

### `hypr-theme-bg-next`

Legacy script to cycle wallpapers in the current theme. It has been updated to use `hypr-wallpaper-set` internally.

## Persistence

By using `hypr-wallpaper-set` (directly or via the Manager), the `~/.config/hypr/current/background` symlink is updated. The `autostart.conf` script loads this symlink on startup:

```ini
exec-once = uwsm app -- swaybg -i ~/.config/hypr/current/background -m fill
```

This ensures the selected wallpaper persists across reboots.
