# Applications and Tools Documentation

This document outlines the key applications and tools integrated into this Hyprland environment.

## Core Applications

### Waybar (`config/waybar/`)
The status bar. It displays information like workspaces, date/time, hardware status (CPU, RAM), and media controls.
*   **Config**: `config.jsonc` defines the modules and their layout.
*   **Style**: `style.css` handles the visual appearance. It imports `~/.config/hypr/current/theme/waybar.css` to apply the active theme's colors.
*   **Custom Modules**: Uses scripts like `bin/arch-update` and `bin/hypr-weather` to provide dynamic data.

### Walker (`config/walker/`)
The application launcher and clipboard manager.
*   **Config**: `config.toml` contains general settings.
*   **Elephant**: `elephant.toml` configures the service component, likely for clipboard history or other background tasks.
*   **Theming**: Dynamic theming is handled by merging the theme's `walker.css` into the active configuration.

### Terminal Emulators
*   **Kitty** (`config/kitty/`): The primary terminal. Its configuration is split between a base `kitty.conf` and a theme-specific `theme.conf` (symlinked).
*   **Alacritty** (`config/alacritty/`): An alternative terminal. Theming is applied by updating `alacritty.toml`.

### Neovim (`config/nvim/`)
The text editor.
*   **LazyVim**: The configuration is based on LazyVim.
*   **Theme Integration**: A `theme.lua` file in `lua/plugins/` is symlinked to the active theme's `neovim.plugin` file, ensuring the editor matches the system theme.

### Hyprlock (`config/hypr/hyprlock.conf`)
The screen locker.
*   It uses variables like `$background_color` and `$accent_color` which are likely populated from the theme configuration or environment variables.

### Hypridle (`config/hypr/hypridle.conf`)
Idle management daemon.
*   Manages screen dimming and locking after periods of inactivity.

## Other Tools

*   **Mako**: Notification daemon. Styled via `mako.ini` in the theme directory.
*   **Starship**: Shell prompt, configured in `config/starship.toml`.
*   **Btop**: System monitor, themed via `btop.theme`.
*   **Fastfetch**: System information fetcher.
    *   **Config**: `config/fastfetch/config.jsonc` defines the structure and modules displayed (OS, kernel, uptime, packages, etc.).
