# Developer Guide

This document explains the architecture of the Hyprland configuration distribution for contributors and power users.

## Directory Structure

The project has two key locations to be aware of: the source repository and the installation directory.

### Source Repository (This Git Repo)

This is the repository containing the source for all scripts, applications, and default configurations.

- `bin/`: Executable helper scripts.
- `templates/`: **Crucial directory**. Contains all Jinja2 master templates for application configurations (e.g., `templates/hypr/hyprland.lua.j2`). These are the blueprint for the user's `~/.config` files.
- `themes/`: The default "system" themes provided by the project.
- `install.sh`: The main installation script.
- `hypr_config_manager/`: The core Rust crate for theme management and configuration generation.
- `hypr_apps/`: Contains Rust applications that integrate with the config manager. (e.g. `hypr_apps/hypr-preferences-app`, `hypr_apps/keybindings_editor`).

### Installation & User Directories

The project follows the XDG Base Directory Specification.

- **`~/.local/share/hypr`**: The primary **installation directory**. `install.sh` copies the repository contents here. This location holds the system themes, templates, and application code. **Files here should be considered read-only from a user's perspective**, as they are managed by the installer.

- **`~/.config/`**: The **user configuration directory**. All final configuration files are **generated** into this directory (e.g., `~/.config/hypr`, `~/.config/waybar`). User-specific overrides are also stored here.

## Core Architecture: Template-Only Generation

The project uses a **"Template-Only"** architecture. Symlinks are no longer used for theming.

**Data Source**: Configuration is built from a merge of two data sources (in order of precedence): System Defaults -> `theme.yaml`.
**Templates**: The `templates/` directory contains Jinja2 templates for all configuration files.
**Compiler**: The `hypr_config_manager` Rust binary (`hypr-theme-templater-bin`) acts as a "compiler". It takes the merged data and the templates to generate the final, complete configuration files directly into the user's `~/.config/` directory.
**Smart Reloading**: The generator tracks which files were changed and intelligently reloads only the affected applications and terminals.

Refer to **`docs/THEMING.md`** for a detailed user-facing explanation of this system.

### Template Dependencies & Walkers

The Rust configuration manager walks the `templates/` directory dynamically. It compares existing files, validates YAML config values, renders them via minijinja, and re-renders them. It uses a smart reload logic that only triggers reload signals (such as SIGUSR/SIGUSR1/SIGUSR2 or script commands) for modified apps and terminal devices.

## Tooling & Validation

To ensure the quality and correctness of themes and templates, the project provides several developer tools located in `bin/`.

### Theme Validation (`hypr-theme-validate`)

This tool validates a `theme.yaml` file against the JSON schema defined in `hypr_config_manager/theme.schema.json`.

```bash
# Validate a specific theme file
bin/hypr-theme-validate path/to/theme.yaml
```

The schema enforces data types and structure, preventing runtime errors during configuration generation.


## Preferences App (`hypr_apps/hypr-preferences-app`)

The `hypr_apps/hypr-preferences-app` is a GTK4/Libadwaita application written in Rust.

- **Backend Logic**:
  - **`store.rs`**: Exports the `ThemeStore` struct which manages theme states, stages edits, and coordinates saves.
  - **`pages/`**: Contains the visual tabs (Layout, Appearance, Input, etc.). The fields bind to the `ThemeStore` to retrieve values and commit updates.
- **Saving Process**: When a user clicks "Save & Apply":
        The UI commits changes to the `ThemeStore`, which writes to `theme.yaml`.
        It then runs the dynamic template rendering engine `generate_config`.
        Finally, it triggers `reload_applications` with the modified file list to reload waybar, hyprland, swayosd, terminals, etc.
- **Color Scheme Synchronization**: To ensure perfect visual alignment with user themes, the application utilizes `AdwStyleManager` to dynamically synchronize its dark/light mode preference:
  - On startup and whenever a `StoreEvent::ThemeChanged` event is fired, the application queries `ThemeStore::global().get_theme_data(&theme)`.
  - It maps `config.gtk.color_scheme` to the appropriate `adw::ColorScheme` (e.g., `adw::ColorScheme::PreferLight` or `adw::ColorScheme::PreferDark`) and sets it on the default style manager. This completely avoids `GtkSettings` deprecations and provides real-time theme updates within the running UI.

## Keybindings Editor App (`hypr_apps/keybindings_editor`)

The `hypr_apps/keybindings_editor` is another GTK4/Libadwaita Rust application designed to provide a graphical interface for managing Hyprland keybindings.

- **Data-Driven Architecture**: The entire keybinding ecosystem has been refactored to be heavily data-driven. The legacy system of parsing raw Hyprland `*.conf` files and storing arbitrary unbind overrides in `user_bindings.conf` was removed.
  - The application queries the `keybindings` JSON array object defined via `get_theme_setting` (which merges `defaults.yaml` and `theme.yaml`).
  - Save and delete operations manipulate this source list directly (adding or filtering key/modifier structures) preventing syntax errors and providing a singular source of truth.
  - The `bindings.lua` Jinja template interprets the `theme.keybindings` configuration array on-the-fly and generates a fully formatted, deduplicated Hyprland output configuration, complete with `# Category` groupings.
- **Programmatic UI**: The main window dialog UI for the Keybindings Editor is constructed **programmatically** in Rust using GTK4/Libadwaita widgets. This ensures greater control over dynamic elements.
- **Key Capture Mechanism**: The capture dialog listens for keyboard events:
  - It utilizes `Gtk.EventControllerKey` to capture keyboard inputs.
  - When capturing inputs, it handles Hyprland modifiers (`Super`, `Shift`, `Ctrl`, `Alt`) safely, entering an empty submap to bypass global desktop hotkeys, and restoring them only when all modifiers are released.
- **Conflict Detection**:
  - Performs checks against existing active keybindings mapped from active system binds.
  - Informs the user immediately with a toast notification if the combination is in use.
- **Floating Window Behavior**: The application is configured to launch as a floating window in Hyprland, as defined by a window rule in `templates/hypr/windows.lua.j2` matching its application ID.

## Script Architecture

Scripts in `bin/` should follow these guidelines:

**Safety**: Always use `set -euo pipefail`.
**Paths**: Use `${HYPR_PATH}` (default `~/.local/share/hypr`) to resolve static resources from the installation directory.
**Dependencies**: Check for dependencies using `command -v`.
**Linting**: Use `shellcheck` to validate scripts before committing.
