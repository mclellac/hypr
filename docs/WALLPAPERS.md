# Wallpaper Management

This document describes the wallpaper management system for the Hyprland environment.

## Overview

Wallpaper management is handled by a simple, persistent system. It consists of:

**Persistence Mechanism**: A symlink at `~/.config/hypr/current_background` that points to the active wallpaper image file.
**CLI Tool**: `hypr-wallpaper-set` updates the symlink and reloads the background process (`swaybg`).
**GUI Application**: The **Preferences App** (`hypr-preferences-app`) provides a visual interface to browse and select wallpapers from your theme's backgrounds or your personal collection.

## Components

### `hypr-preferences-app` (Graphical Interface)

The primary way for most users to manage wallpapers. Launch the app and go to the **Wallpaper** tab to see a grid of available backgrounds. Clicking one applies it instantly.

### `hypr-wallpaper-set` (CLI Tool)

The underlying script used to set the wallpaper.

**Usage:**

```bash
hypr-wallpaper-set /path/to/image.jpg
```

This script:

Validates the image file.
Updates the `~/.config/hypr/current_background` symlink to point to the new image.
Restarts the `swaybg` process to display the new wallpaper.

### `hypr-theme-bg-next`

A script that cycles to the next wallpaper available in the current theme's `backgrounds/` directory. It uses `hypr-wallpaper-set` internally to apply the change.

## Persistence

By using `hypr-wallpaper-set` (or a script that calls it), the `~/.config/hypr/current_background` symlink is updated. The generated `autostart.conf` is configured to load this symlink on startup:

```ini
# In autostart.conf (generated)
exec-once = swaybg -i ~/.config/hypr/current_background -m fill
```

This ensures the selected wallpaper persists across reboots.
