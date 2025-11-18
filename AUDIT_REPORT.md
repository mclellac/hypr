# Neovim Theming Issue: Comprehensive Audit Report

This report documents a thorough investigation into the root cause of the Neovim theming failure, where the theme remains stuck on `tokyonight` despite the execution of the `hypr-theme-set` script.

## 1. Initial State Analysis: Installation Scripts

This section will analyze the state of the system immediately after installation, focusing on the `install/config/theme.sh` and `install/packaging/lazyvim.sh` scripts.

*   **`install/config/theme.sh` Analysis:**
    *   This script sets the initial system-wide theme to `material`.
    *   It does **not** create any symlinks or configuration files for Neovim directly.
    *   It calls `bin/hypr-theme-set material` at the end, delegating the initial theme setup for all applications, including Neovim, to the theme-switching script.

*   **`install/packaging/lazyvim.sh` Analysis:**
    *   This script clones the official `LazyVim/starter` repository to set up the base Neovim configuration.
    *   It then copies the custom configuration files from this repository's `config/nvim/` directory into the new LazyVim setup.
    *   Crucially, it executes `rm -f "${NVIM_CONFIG_DIR}/lua/plugins/theme.lua"`. This is a deliberate action to ensure that no static theme file from the starter configuration is present. It confirms that the theme is intended to be managed dynamically.

## 2. Runtime Analysis: `hypr-theme-set` Script

This section will detail the file operations performed by the `bin/hypr-theme-set` script when a user attempts to switch themes.

*   **`update_neovim_theme` Function Analysis:**
    *   The script's logic is confirmed to be correct.
    *   It checks for `neovim.plugin` in the selected theme's directory.
    *   If found, it creates a symbolic link: `ln -sf /path/to/theme/neovim.plugin ~/.config/nvim/lua/plugins/theme.lua`.
    *   This operation successfully places the correct theme configuration into the Neovim plugin directory.
    *   The script is functioning as intended. The failure is not in the script itself.

## 3. Neovim Configuration Analysis: `lazy.lua`

This section will perform a deep dive into the LazyVim configuration to identify the precise mechanism that is overriding the dynamic theme selection.

*   **Plugin Load Order:** The configuration loads plugins in two stages: first, the base `"LazyVim/LazyVim"` plugin set, and second, the custom plugins from the `lua/plugins/` directory (which includes our `theme.lua`).
*   **Configuration Overrides:** The critical issue is that the base `"LazyVim/LazyVim"` plugin has its own default configuration, which includes setting the `tokyonight` colorscheme. This default is loaded *before* our custom `theme.lua` plugin.

## 4. Conclusion: Root Cause Identification

This section will synthesize the findings from the previous sections to provide a definitive conclusion on the root cause of the theming failure.

*   **Definitive Root Cause:** The Neovim theme is stuck on `tokyonight` because it is the default theme for the base LazyVim configuration. The custom theme plugin, loaded via `theme.lua`, is being loaded *after* the default has already been applied, and it is not configured in a way that overrides the base setting. The dynamic theme-switching mechanism is working correctly on the file system level, but it is being defeated by the foundational load order and default settings of the LazyVim framework.
