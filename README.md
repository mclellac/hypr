# hypr

This repository was a fork of [Omarchy](https://github.com/omarchy/omarchy).

## Installation

The recommended way to install is by cloning the repository and executing `install.sh`.

```bash
git clone https://github.com/mclellac/hypr.git
cd hypr
./install.sh
```

The installer will synchronize the repository files to `~/.local/share/hypr`, which serves as the application's home directory. It will then run a series of setup scripts for packages, fonts, and configurations.

### Updates

To update your installation, simply pull the latest changes from the git repository and re-run the script:

```bash
cd /path/to/your/hypr/repo
git pull
./install.sh
```

The installer synchronizes the repository's files to `~/.local/share/hypr`. The templates and system themes in `~/.local/share/hypr` are updated, and the configuration generator ensures your personal customizations in your theme are preserved and reapplied.

## Features

- **High-Performance Preferences App:** Optimized for speed with a tiered caching system and batch-fetching backend. UI refreshes take under 5ms, and startup is near-instant.
- **Enhanced Appearance Customization:** Use the modern Libadwaita Preferences App to customize border colors, including gradient active borders with a custom angle.
- **Rock-Solid Stability:** Fully resolved all GTK/Adwaita parenting errors through a zero-tolerance unparenting architecture.
- **Wallpaper Management:** Browse and apply wallpapers from the current theme or your personal collection via the Preferences App.
- **Interactive Keybindings Editor:** Easily view, edit, and capture keybindings graphically with built-in conflict detection and support for the new Lua-based configuration system.
- **Hyprland 0.55+ Ready:** Fully optimized for the latest Hyprland releases, utilizing the new Lua-based configuration system for maximum performance and flexibility.
- **Hyprland Plugin Ecosystem Support:** Native `hyprpm` plugin compilation and management integrated into the Preferences App and CLI (`hypr-plugin`), featuring full configuration for `hyprglass` dynamic glassmorphism.
- **Standardized Shortcut Ecosystem:** Strictly mnemonic and consistent keybindings across all 50+ themes. No more `CTRL` for app launches; just clean `mainMod + <letter>` shortcuts.
- **Update Notifications:** Get notified of official Arch Linux and AUR package updates directly in Waybar.

## Directory Structure

This project follows the XDG Base Directory Specification and uses a template-based architecture.

- **`~/code/hypr` (Example)**: This is the location where you clone the git repository. You run `install.sh` from here to install or update.

- **`~/.local/share/hypr`**: This is the **installation directory**.
  - **`templates/`**: Contains the canonical configuration templates (Jinja2). These are the source of truth for all generated configs.
  - **`bin/`**: Helper scripts and core utilities.
  - **`hypr_config_manager/`**: Rust crate and binary for rendering templates and managing configuration state.
  - **`hypr_apps/`**: Rust-based GTK4/Libadwaita applications (Preferences, Keybindings Editor, etc.).
  - **`themes/`**: Default "system" themes.
  - **Do not edit files in this directory directly**, as they are overwritten on update.

- **`~/.config/`**: This is your **user configuration directory**.
  - **Generated Files**: The installer and theme switcher generate the final configuration files into their standard locations (e.g., `~/.config/hypr/hyprland.lua`, `~/.config/waybar/style.css`). It is not recommended to edit these directly, as changes will be overwritten.

## Keybindings

Keybindings are managed interactively through the **Keybindings Editor App**. All themes now follow a standardized, mnemonic schema:

| Keybinding              | Description               |
| ----------------------- | ------------------------- |
| `mainMod + Return`      | Open terminal             |
| `mainMod + SHIFT + Return`| Open Alacritty          |
| `mainMod + E`           | Open file manager         |
| `mainMod + B`           | Open browser              |
| `mainMod + N`           | Open Neovim               |
| `mainMod + G`           | Open Gmail                |
| `mainMod + SHIFT + G`   | Open Maps                 |
| `mainMod + M`           | Open Main Menu            |
| `mainMod + SHIFT + M`   | Open Theme Picker         |
| `mainMod + O`           | Open Obsidian             |
| `mainMod + SHIFT + O`   | Open Calendar             |
| `mainMod + SHIFT + P`   | Pop window out            |
| `mainMod + R`           | Toggle Dictation          |
| `mainMod + SHIFT + E`   | Open Emoji Picker         |
| `mainMod + SPACE`       | Application Launcher      |
| `mainMod + SHIFT + V`   | Clipboard History         |
| `mainMod + K`           | Show key bindings         |
| `mainMod + Q`           | Close active window       |
| `mainMod + Arrow keys`  | Move focus                |
| `mainMod + SHIFT + Arrows`| Swap window position    |
| `mainMod + [0-9]`       | Switch to workspace       |
| `mainMod + SHIFT + [0-9]` | Move window to workspace|
| `mainMod + COMMA`       | Dismiss last notification |
| `mainMod + SHIFT + COMMA` | Dismiss all notifications|
| `mainMod + CTRL + COMMA` | Silence notifications     |

## Theming

The theming system is powered by a Rust-based generator that creates all configuration files from a set of master templates. This ensures a consistent and robust experience.

To change the theme, you can use the `hypr-theme-set` script or the `hypr-preferences-app`.

```bash
# List available themes
hypr-theme-list

# Set a new theme
hypr-theme-set tokyo-night
```

Running this command triggers the theme engine, which re-renders all necessary configurations based on the selected theme's properties.

### Customization

- **For simple tweaks** (like changing a font or color), modify your active theme's `theme.yaml`.
- **For major changes**, you can create a new user theme by copying a system theme from ``$XDG_DATA_HOME/hypr/themes`` to ``$XDG_CONFIG_HOME/hypr/themes``.

For a complete guide on customization and theme creation, please see **[docs/THEMING.md](docs/THEMING.md)** and **[docs/CREATE_THEME.md](docs/CREATE_THEME.md)**.

### Neovim Theming

Each theme provides a Neovim theme file (`neovim.theme.lua`) that integrates with the existing LazyVim setup. When you switch themes, the appropriate theme file is rendered to `~/.config/nvim/lua/plugins/theme.lua`, and LazyVim handles the rest.

This approach ensures that your personal Neovim configuration in `~/.config/nvim/lua/plugins/` is never overwritten.

## Neovim Configuration

Your personal Neovim configuration is located in `~/.config/nvim/lua/plugins/`. You
can add your own plugins and settings here. The `hypr` setup uses LazyVim, so
you should follow the LazyVim conventions for customization.

The base `hypr` Neovim configuration is generated from templates.

## License

hypr configs are released under the
[MIT License](https://opensource.org/licenses/MIT).
