# Wallpaper Management Plan

This document outlines the plan for a new wallpaper management application for the Hyprland environment. The goal is to provide a user-friendly interface for selecting and managing wallpapers, moving away from file-system-based theme dependency.

## Current State

Currently, wallpapers are:
- Stored within theme directories (`themes/<name>/backgrounds/`).
- Cycled using `hypr-theme-bg-next`.
- Persisted via a symlink at `~/.config/hypr/current/background`.

## Proposed Solution: `hypr-wallpaper-manager`

A dedicated application to manage wallpapers.

### Features

1.  **Graphical User Interface (GUI)**
    - A simple grid view of available wallpapers.
    - Thumbnails for quick preview.
    - "Set Wallpaper" button.
    - "Add Wallpaper" button to import external images.

2.  **Wallpaper Sources**
    - **Theme Wallpapers:** Automatically detect wallpapers from the current theme (and optionally other themes).
    - **User Library:** A dedicated directory for user-added wallpapers (e.g., `~/Pictures/Wallpapers` or `~/.local/share/hypr/wallpapers`).

3.  **Persistence**
    - The application must use the `hypr-wallpaper-set` script (or equivalent logic) to ensure the `~/.config/hypr/current/background` symlink is updated.
    - This ensures `autostart.conf` restores the correct wallpaper on reboot.

4.  **Integration**
    - The application should be launchable via the application launcher (Walker) or a keybinding.
    - It should replace or augment the `hypr-theme-bg-next` functionality.

### Technical Implementation

- **Language/Framework:** Python (GTK/Tkinter) or a lightweight native tool.
- **Backend:**
  - Scan directories for images.
  - Call `hypr-wallpaper-set <path>` when an image is selected.
- **Preview:**
  - Generate thumbnails to avoid loading large images.

### Benefits

- **Decoupling:** Users can set wallpapers independent of the active theme.
- **Persistence:** By updating the central symlink, the selection persists across sessions.
- **User Experience:** Visual selection is superior to cycling blindly.

## Interim Solution

Until the application is built, users should use the `hypr-wallpaper-set` script to set wallpapers persistently:

```bash
hypr-wallpaper-set /path/to/image.jpg
```
