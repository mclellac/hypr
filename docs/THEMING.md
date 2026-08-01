# Theming System Documentation

This document describes the "template-only" theming system used to manage the appearance of Hyprland and related applications.

## Overview: A "Template-Only" Architecture

The theming system is built on a powerful principle: all application configurations are **generated** from a set of master templates and a defined color palette. This approach ensures consistency, stability, and easy customization without relying on fragile symlinks.

When a theme is applied, a Rust-based generator (`hypr-theme-templater-bin`) reads data from multiple sources, combines it with Jinja2 templates via MiniJinja, and writes the final configuration files directly to their destination in `~/.config/` (e.g., `~/.config/waybar/style.css`, `~/.config/hypr/looknfeel.conf`).

## Configuration Data Flow

Your final configuration is a result of two layers of data merged together. Layers that are lower in this list have higher precedence and will override the layers above them.

**System Defaults**: A set of base values provided by `hypr_config_manager/src/defaults.yaml`. Any missing values in a theme's `theme.yaml` will fall back to these defaults, reducing boilerplate.
**Theme Definition (`theme.yaml`)**: The primary file for any theme, defining its colors, fonts, layout settings, and **keybindings**.

## Theme Directory Structure

System themes are located in `$XDG_DATA_HOME/hypr/themes/` (default: `~/.local/share/hypr/themes/`). User-created themes are located in `$XDG_CONFIG_HOME/hypr/themes/` (default: `~/.config/hypr/themes/`).

A typical theme directory contains:

- **`theme.yaml`**: **(Required)** The definitive source of truth for the theme's properties, including colors, fonts, layout variables, and application-specific settings.
- **`*.theme.*` files**: A collection of partial config files that contain theme-specific logic that can't be expressed in `theme.yaml`. These include:
  - `alacritty.theme.toml`
  - `btop.theme`
  - `chromium.theme`
  - `ghostty.theme.conf`
  - `hyprlock.theme.conf`
  - `kitty.theme.conf`
  - `mako.theme.conf`
  - `neovim.theme.lua`
  - `swayosd.theme.css`
  - `walker.theme.css`
  - `waybar.theme.css`
  - `swappy/config` (Templated)
- `backgrounds/`: A directory containing wallpaper images for the theme.

### UI/UX Unification

To maintain a cohesive look across the entire desktop environment, applications such as Waybar, Walker, SwayOSD, Swappy, and Hyprlock are styled dynamically using the global theme properties defined in `theme.yaml`. They do not use hardcoded fonts, border sizes, or border radii. Instead, they inherit `theme.fonts.font_family`, `theme.layout.border_size`, and `theme.layout.rounding` directly, ensuring 100% consistency with the Hyprland master configuration. Modifying these global variables will automatically regenerate their respective configuration templates.

Additionally, **GTK 3.0 and 4.0 accent colors and widgets** are managed via templated `gtk.css` files (`templates/gtk-3.0/gtk.css.j2` and `templates/gtk-4.0/gtk.css.j2`). These files not only update whenever the theme's accent color or primary color changes, but they also apply the global theme to many standard GTK widgets (such as buttons, switches, progress bars, toolbars, and titlebars) to ensure classical applications blend seamlessly with modern Wayland/Hyprland aesthetics.

### Font Unification & Overrides

A core principle of the theming system is **Font Unification**. By default, all applications use the global font family defined in `theme.fonts.font_family`. This ensures that typography remains consistent across the entire workspace.

However, recognizing that different screen elements may require different scales, several applications support independent **font size overrides**:

- **Waybar**: `theme.waybar.style.size`
- **Mako**: `theme.mako.font_size`
- **Walker**: `theme.walker.font_size`
- **SwayOSD**: `theme.swayosd.font_size`
- **Kitty/Alacritty/Ghostty**: `theme.kitty.font_size` / `theme.alacritty.font_size` / `theme.ghostty.font_size`
- **Voxtype**: `theme.voxtype.font_size` (if supported)

When these overrides are set to `0` or `null`, the system automatically falls back to the global `theme.fonts.size`. This design allows for fine-grained control over readability without sacrificing the unified aesthetic provided by a shared font family.

- `icons.theme`: (Optional) A text file containing the name of the GTK icon theme to apply.
- `palette.md`: (Optional) A human-readable Markdown file documenting the theme's colors.

## Theme Switching

You can switch themes in two ways:

**Command Line**: Use the `hypr-theme-set` command.

    ```bash
    # List available themes
    hypr-theme-list

    # Set a new theme
    hypr-theme-set "tokyo-night"
    ```

    This command invokes the configuration generator, which intelligently rebuilds only the necessary files and reloads the affected applications.

**Preferences Application**: Launch the `hypr-preferences-app` and navigate to the "Theme" page to visually select and apply themes.

## Customizing Your Theme

There are two primary ways to customize your setup.

### Method 1: Editing the Active Theme (Recommended)

For changing a few settings, like font size or an accent color, the best method is to edit the `theme.yaml` of your active theme.

Locate your active theme's `theme.yaml`. If you are using a user theme, it will be in `$XDG_CONFIG_HOME/hypr/themes/<theme-name>/theme.yaml`.
Add or modify the settings you want to change.

**Example `theme.yaml` snippet:**

```yaml
# Change the primary font and its size
fonts:
  font_family: "JetBrains Mono Nerd Font"
  size: 14

# Change the active border color for Hyprland
colors:
  hypr:
    active_border: "rgba(ffc837ee) rgba(f39c12ee) 45deg"
```

Re-apply your current theme for the changes to take effect: `hypr-theme-set "$(hypr-theme-current)"`.

### Method 2: Creating a New Theme (Advanced)

If you want to make significant changes or create a whole new theme, you should create a "user theme".



**Manual Method**:

**Copy a System Theme**: Find a system theme you like in `$XDG_DATA_HOME/hypr/themes/` (default: `~/.local/share/hypr/themes/`) and copy it to `$XDG_CONFIG_HOME/hypr/themes/` (default: `~/.config/hypr/themes/`).

    ```bash
    # Example: Create a custom theme based on 'material'
    cp -r $XDG_DATA_HOME/hypr/themes/material $XDG_CONFIG_HOME/hypr/themes/my-custom-material
    ```

**Edit Your User Theme**: You can now safely edit all the files inside `$XDG_CONFIG_HOME/hypr/themes/my-custom-material`, including `theme.yaml` and any of the `*.theme.*` files.

**Apply Your New Theme**: Use `hypr-theme-set` with the name of your new theme directory.

    ```bash
    hypr-theme-set "my-custom-material"
    ```

For a comprehensive walkthrough of all available options in `theme.yaml`, see the **[theme.yaml Reference Guide](THEME.YAML.md)** and **[docs/CREATE_THEME.md](CREATE_THEME.md)**.

## Built-in System Themes

By default, Hypr installs a variety of system themes. These are automatically detected from `$XDG_DATA_HOME/hypr/themes/`. Current built-in themes include:

- `adwaita-dark`
- `amateur`
- `catppuccin`
- `catppuccin-latte`
- `cyan`
- `cyberpunk`
- `deus-ex`
- `everforest`
- `green-monster`
- `gruvbox`
- `kanagawa`
- `material`
- `material-bright`
- `matte-black`
- `nord`
- `osaka-jade`
- `ristretto`
- `rose-pine`
- `tokyo-night`
- `vancouver`
