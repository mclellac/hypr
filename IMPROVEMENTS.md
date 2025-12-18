# Improvements and Observations

This document lists potential improvements, issues, and observations regarding the current Hyprland configuration and codebase.

## Critical / Bugs

1.  **Missing Default Configurations**:
    *   The `config/hypr/hyprland.conf` file attempts to source files from `~/.local/share/hypr/default/hypr/` (e.g., `autostart.conf`, `bindings/media.conf`).
    *   The installation scripts (`install.sh`, `install/config/config.sh`) do **not** appear to populate this directory.
    *   **Impact**: Hyprland may fail to start or will lack essential default keybindings and settings unless these files exist from a previous install or a different package.
    *   **Recommendation**: Either bundle these defaults in the repo and install them, or remove the dependency on `~/.local/share/hypr/default` and make `config/hypr/` self-contained.

2.  **Walker Theme Dependency**:
    *   `bin/hypr-theme-set` references `~/.local/share/hypr/default/walker/themes/hypr-default/style.css` when building the Walker theme.
    *   Like the Hyprland defaults, this file does not seem to be created by the installer.
    *   **Impact**: Walker theming might fail or look incorrect.

3.  **Hardcoded Paths**:
    *   Scripts often rely on `~/.config/hypr/` or `~/.local/share/`. While standard, using environment variables consistently (like `$XDG_CONFIG_HOME`) would be more robust.

## VM Optimizations

Since this configuration runs inside a VM:

1.  **Animation Overhead**:
    *   Hyprland animations (blur, drop shadows, window movement) consume significant GPU resources, which might be limited in a VM.
    *   **Recommendation**: Create a "VM Mode" or a specific config snippet that disables `decoration:blur`, `decoration:drop_shadow`, and simplifies animations.

2.  **Cursor Issues**:
    *   VMs often have issues with hardware cursors (e.g., cursor disappearing or lagging).
    *   **Recommendation**: Ensure `cursor:no_hardware_cursors = true` is set in the Hyprland config.

3.  **Refresh Rate**:
    *   `monitors.conf` should be checked to ensure it doesn't force a refresh rate unsupported by the virtual display driver.

## Code Quality & Maintainability

1.  **Script Redundancy**:
    *   There are many scripts in `bin/` (e.g., `hypr-cmd-*`, `hypr-refresh-*`). Some might be obsolete or redundant. A cleanup pass is recommended.

2.  **Error Handling in Scripts**:
    *   While many scripts use `set -euo pipefail`, some might lack specific checks for dependencies (like `walker` or `jq`) before running.

3.  **Documentation Sync**:
    *   The reliance on `palette.md` is excellent, but manual updates are error-prone. The `audit_palettes_v2.py` script helps, but CI checks could ensure `palette.md` always matches the config files.

## Feature Enhancements

1.  **Dynamic scaling**:
    *   For HiDPI VM displays, easy scaling toggles in the menu would be beneficial.

2.  **Clipboard Manager Integration**:
    *   Ensure `cliphist` or the clipboard provider in `walker` is correctly persisted and doesn't crash the VM (as noted in memory, there were conflicts with `elephant` service).
