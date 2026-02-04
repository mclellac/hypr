# Hyprland Configuration Audit Report

## Overview
This report details the findings from an audit of the Hyprland configuration files, focusing on compatibility with Hyprland 0.53+ and adherence to modern syntax requirements (e.g., removal of `windowrulev2`, transition to `shadow` blocks).

## Critical Issues

### 1. `windowrulev2` Usage Detected
The `windowrulev2` keyword is explicitly deprecated/forbidden per the requirements, but it was found in the following active configuration:

*   **File**: `themes/kanagawa/hyprland.conf`
*   **Line**: `windowrulev2 = opacity 0.98 0.95, class:Alacritty`
*   **Action Required**: Convert this to the new `windowrule` syntax.
    ```ini
    # Suggested Fix
    windowrule {
        match:class = Alacritty
        opacity = 0.98 0.95
    }
    ```

### 2. Commented-out `windowrulev2`
While not active, commented-out legacy syntax can lead to confusion.

*   **File**: `config/hypr/windows.conf`
*   **Line**: `#windowrulev2 = tag +floating-window, class:^(hypr-exec)$`
*   **Action Required**: Update to the new syntax or remove if no longer needed.

## Potential Issues & Verification

### 1. `hypr-toggle-nightlight` Script Logic
The script `bin/hypr-toggle-nightlight` attempts to control `hyprsunset` via `hyprctl`.
*   **Code**: `hyprctl hyprsunset temperature ...`
*   **Issue**: `hyprsunset` is typically a standalone application. Unless a specific Hyprland plugin is installed that exposes this `hyprctl` command, this will fail.
*   **Action Required**: Verify if a `hyprsunset` plugin is in use. If using the standalone binary, the script must be updated to kill/restart the process with new arguments or use its specific IPC if available.

### 2. Redundant Environment Variables
`config/hypr/envs.conf` and `default/hypr/envs.conf` define variables like `GDK_BACKEND`, `QT_QPA_PLATFORM`, etc.
*   **Context**: When using `uwsm` (Universal Wayland Session Manager), many of these environment variables are handled automatically or should be set in the UWSM environment config rather than Hyprland config.
*   **Action Required**: Review `envs.conf` and consider delegating environment setup to UWSM or removing defaults that Hyprland/UWSM sets automatically to keep the config clean.

## Improvements & Cleanup

### 1. `col.shadow` Deprecation
The deprecated `col.shadow` variable has been correctly replaced by the `shadow { ... }` block in `default/hypr/looknfeel.conf`. However, legacy comments remain in themes:
*   **Files**: `themes/material/hyprland.conf`, `themes/material-bright/hyprland.conf`
*   **Action**: Remove these commented-out lines to clean up the configuration.

### 2. `windowrule` Consistency
The configuration uses a mix of one-line `windowrule = match:..., ...` and block `windowrule { ... }` syntax.
*   **Recommendation**: Standardize on the block syntax (`windowrule { ... }`) for complex rules (like those in `apps/browser.conf`) for better readability and maintainability.

### 3. `layout` Configuration
`default/hypr/looknfeel.conf` sets `layout = dwindle`. Ensure that the `dwindle` section does not contain deprecated variables like `no_gaps_when_only` (none were found, but good to keep in mind).

### 4. `input` Configuration
Ensure `follow_mouse` and `sensitivity` values in `default/hypr/input.conf` are within the expected ranges for Hyprland 0.53+. `follow_mouse = 0` (manual focus) is a significant deviation from default (1); verify this is the intended behavior.

## Verified Features

### 1. `bindd` Syntax
The configuration uses `bindd` extensively.
*   **Status**: Verified Correct.
*   **Reason**: `bindd` is required for the keybinding description functionality used by the `mainMod+k` help menu. It should not be replaced.
