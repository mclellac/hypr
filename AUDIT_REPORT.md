# Audit Report

**Date:** February 4, 2026
**Scope:** Full repository audit (Hyprland, Waybar, Scripts, Theming)
**Goal:** Improve configuration quality, stability, and workflow adherence to modern standards.

## Executive Summary

The repository is in excellent condition, showing a strong adherence to modern Hyprland standards (v0.53+) and robust scripting practices. The theming engine is flexible and functional. However, there are opportunities to standardize application launching via `uwsm`, clean up CSS artifacts, and harden some helper scripts against edge cases.

## 1. Hyprland Configuration & Compatibility

### Findings
*   **Version Compliance (v0.53+):** The configuration correctly uses the new `shadow { ... }` block syntax in `default/hypr/looknfeel.conf`.
*   **Window Rules:** The deprecated `windowrulev2` syntax is correctly avoided in active configurations. It appears only in commented-out lines in `config/hypr/windows.conf`.
*   **Environment:** The usage of `default/hypr/envs.conf` ensures a clean separation of environment variables.

### Recommendations
*   **Standardize Launching:** While most keybindings in `config/hypr/bindings.conf` correctly use `uwsm app --` (e.g., Nautilus, Obsidian), some entries launch applications directly (e.g., `kitty`, `hypr-launch-browser`).
    *   *Action:* Update `bindings.conf` to consistently use `uwsm app --` for all application launches to ensure proper systemd scope integration.
    *   *Example:* `bindd = $mainMod, T, Kitty, exec, uwsm app -- env LIBGL_ALWAYS_SOFTWARE="1" kitty`

## 2. Waybar Configuration

### Findings
*   **JSON Validity:** `config.jsonc` is valid and well-structured.
*   **Module Robustness:** Custom modules (`weather`, `update`) correctly use the `json` return type and are backed by robust scripts.
*   **CSS Cleanliness:** `config/waybar/style.css` contains selectors for IDs that do not appear in the configuration, such as `#window`.
    *   *Note:* While harmless, these clutter the codebase and can lead to confusion during theming.
*   **Variable Inheritance:** The configuration relies on a comprehensive set of variables (e.g., `@cpu_color`, `@memory_color`). The audit confirms these are expected to be present in theme files.

### Recommendations
*   **CSS Cleanup:** Remove unused selectors from `config/waybar/style.css` to strictly match the modules defined in `config.jsonc`.

## 3. Scripting & Code Quality

### Findings
*   **`bin/hypr-weather`:** excellent implementation with timeouts, retries, and cache fallbacks.
*   **`bin/hypr-exec`:** This script uses `bash -c "hypr-show-logo; ${cmd}; hypr-show-done"`.
    *   *Risk:* This method of string interpolation is vulnerable if `${cmd}` contains quotes or special characters.
    *   *Improvement:* Use proper argument passing or an array to safely execute the command.
*   **`bin/arch-update`:** Solid Python implementation handling JSON output correctly.

### Recommendations
*   **Harden `hypr-exec`:** Refactor the script to handle complex command strings more safely, ensuring that arguments passed to it (like `htop` or `btop`) don't break the interpolation.

## 4. Theming Engine

### Findings
*   **Neovim Integration:** The `neovim.plugin` file in themes (e.g., `amateur`) effectively acts as a bridge, using `dofile` to load the local colorscheme.
    *   *Observation:* The README describes a "Plugin-Based" vs "Simple" method, but the implementation is a hybrid. It works well but the documentation distinction is slightly blurry.
*   **Walker CSS:** The `walker.css` file in `themes/amateur` redefines `@window_bg_color` twice.
    *   *Action:* Remove the duplicate definition to ensure deterministic behavior.
*   **Palette Consistency:** The `audit_palettes.py` tool is available to enforce `palette.md` structure, which is a great asset for maintaining documentation.

### Recommendations
*   **Deduplicate CSS:** Scan all theme `walker.css` files for duplicate variable declarations.
*   **Clarify Docs:** Update `THEMING.md` or `README.md` to explicitly describe the current "hybrid" Neovim loading strategy if it's the intended standard.

## 5. Workflow & UX Enhancements

### Findings
*   **Clipboard Manager:** The clipboard workflow relies on a pipeline: `cliphist list | walker --dmenu | cliphist decode | wl-copy`. This is a robust and user-friendly integration of Walker.
*   **Keybinding Descriptions:** The widespread use of `bindd` ensures that the help menu (Super+K) is populated with meaningful descriptions.

### Recommendations
*   **Universal UWSM:** As mentioned in Section 1, moving strictly to `uwsm` for all spawned processes will improve session management and potentially fix edge cases with environment variable inheritance.
*   **Browser Launching:** The `$browser` variable invokes `hypr-launch-browser`. Check if this script internally uses `uwsm`. If not, wrap it.

## Conclusion

The repository is mature and follows best practices. The primary audit findings are minor refinements rather than critical fixes. Focusing on consistent `uwsm` usage and hardening the `hypr-exec` script will yield the highest return on stability.
