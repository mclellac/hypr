# `theme.yaml` Comprehensive Reference

This document provides a detailed, field-by-field breakdown of everything that can be configured in a `theme.yaml` file. It describes each field's purpose, the applications it affects, and how the values are applied within those applications.

---

## Metadata

 | Field | Description |
 | :--- | :--- |
 | `name` | The human-readable name of the theme. |
 | `description` | A brief summary of the theme's design or intent. |

---

## Colors (`colors:`)

Colors define the core aesthetic. Most are converted to specific formats (like `rgba()`) using filters.

 | Field | Description | Affected Applications & Usage |
 | :--- | :--- | :--- |
 | `background` | Primary background color. | **Kitty/Alacritty**: Window background. **Waybar**: Main bar background. **Mako**: Notification background. **Walker**: Menu background. |
 | `foreground` | Primary text/icon color. | **Kitty/Alacritty**: Default text color. **Waybar**: Main text/module color. **Mako**: Notification body text color. **Walker**: Default text color. |
 | `primary` | Main accent color. | **Kitty**: Default cursor and selection background. **Waybar**: `@accent_color`, used for active workspace, clock background, etc. **Mako**: Progress bar color. **Walker**: Selected item text color. **Starship**: `foam` palette color. **Fastfetch**: Hardware icons color. |
 | `secondary` | Secondary accent. | **Waybar**: `@secondary_accent_color`, used for weather background. **Starship**: `pine` palette color. **Fastfetch**: Software icons color. |
 | `alert` | Error/Critical color. | **Waybar**: `@error_color`, used for critical battery/temp and urgent workspaces. **Mako**: Border color for critical urgency notifications. **Starship**: `love` palette color (error symbol). |
 | `warning` | Warning/Pending color. | **Waybar**: `@warning_color`, used for updates and warning battery/temp levels. **Starship**: `gold` palette color. |
 | `success` | Success/Positive color. | **Starship**: Success symbol color. |
 | `cursor` | Terminal cursor color. | **Kitty/Alacritty**: The color of the typing cursor. |
 | `cursor_text` | Text on cursor. | **Kitty/Alacritty**: Color of the character under the cursor. |
 | `selection_bg` | Selection background. | **Kitty/Alacritty**: Highlight color when selecting text. |
 | `selection_fg` | Selection text. | **Kitty/Alacritty**: Color of text within a selection. |
 | `color0` - `color15` | ANSI 16-color palette. | **Kitty/Alacritty**: Standard terminal colors used by CLI apps (e.g., `ls`, `grep`). |

---

## Fonts (`fonts:`)

Specific font settings can be overridden in application-specific sections.

 | Field | Description | Affected Applications & Usage |
 | :--- | :--- | :--- |
 | `font_family` | Main UI and terminal font. | **Kitty/Alacritty**: Terminal font. **Waybar**: Default font for all modules. **Walker**: Main menu font. **Neovim**: GUI font (Neovide). **Hyprlock**: Clock/Label font. **SwayOSD**: OSD labels. **Swappy**: Annotation text. |
 | `font_style` | Font weight/style. | **Kitty/Ghostty**: Sets font style (e.g., `Medium`, `Regular`, `Bold`). |
 | `size` | Base font size. | **Kitty/Alacritty**: Default terminal text size. **Waybar**: Base module text size. **Mako**: Notification text size. **Neovim**: Neovide font size. |

---

## Layout & Decoration (`layout:`)

Primarily controls **Hyprland** behavior and visuals.

 | Field | Description | Affected Applications & Usage |
 | :--- | :--- | :--- |
 | `name` | Layout engine. | **Hyprland**: Sets `general:layout` (e.g., `dwindle`, `master`). |
 | `gaps_in` | Inner gaps. | **Hyprland**: `general:gaps_in`. Space between adjacent windows. |
 | `gaps_out` | Outer gaps. | **Hyprland**: `general:gaps_out`. Space between windows and screen edges. |
 | `border_size` | Border thickness. | **Hyprland**: `general:border_size`. Thickness of window borders. **Mako**: `border-size`. Thickness of notification borders. **Walker**: `.box-wrapper` border width. |
 | `active_border` | Focused border. | **Hyprland**: `general:col.active_border`. Color(s) for focused window (supports gradients). **Mako**: Border color for normal notifications. |
 | `inactive_border` | Unfocused border. | **Hyprland**: `general:col.inactive_border`. Color for unfocused windows. |
 | `rounding` | Corner radius. | **Hyprland**: `decoration:rounding`. Window corner roundness. **Waybar**: Module and tooltip corner roundness. **Mako**: Notification corner roundness. **Walker**: Overall menu corner roundness. |
 | `shadow_enabled` | Toggle shadows. | **Hyprland**: `decoration:shadow:enabled`. |
 | `shadow_range` | Shadow size. | **Hyprland**: `decoration:shadow:range`. **Waybar**: Tooltip shadow blur radius. |
 | `shadow_power` | Shadow falloff. | **Hyprland**: `decoration:shadow:render_power` (1-4). |
 | `shadow_color` | Shadow color. | **Hyprland**: `decoration:shadow:color`. **Waybar**: Tooltip shadow color. **Walker**: Menu shadow color. |
 | `shadow_offset` | Shadow position. | **Hyprland**: `decoration:shadow:offset` (e.g., "0 5"). **Waybar**: Tooltip shadow offset. |
 | `blur_enabled` | Toggle blur. | **Hyprland**: `decoration:blur:enabled`. |
 | `blur_size` | Blur radius. | **Hyprland**: `decoration:blur:size`. |
 | `blur_passes` | Blur iterations. | **Hyprland**: `decoration:blur:passes`. |
 | `blur_vibrancy` | Blur saturation. | **Hyprland**: `decoration:blur:vibrancy`. |
 | `glow_enabled` | Enable border glow. | **Hyprland**: `decoration:glow:enabled`. |
 | `glow_range` | Glow size. | **Hyprland**: `decoration:glow:range`. |
 | `glow_color` | Glow color. | **Hyprland**: `decoration:glow:color`. |
 | `animations_enabled` | Toggle animations. | **Hyprland**: `animations:enabled`. |
 | `animation_style` | Animation preset. | **Hyprland**: Selects a preset group of curves and speeds from `partials/animations.j2` (e.g., `fast`, `relaxed`, `bouncy`). |
 | `animation_windows_curve` | Custom curve. | **Hyprland**: Used as the `bezier` for windows if `animation_style` is `custom`. |
 | `animation_windows_speed` | Window speed. | **Hyprland**: Speed value for window animations. |
 | `animation_fade_speed` | Fade speed. | **Hyprland**: Speed value for fade animations. |
 | `animation_workspaces_speed` | Workspace speed. | **Hyprland**: Speed value for workspace switching animations. |
 | `groupbar_enabled` | Enable groupbars. | **Hyprland**: `group:groupbar:enabled`. |
 | `groupbar_font_size` | Groupbar font size. | **Hyprland**: `group:groupbar:font_size`. |
 | `groupbar_height` | Groupbar height. | **Hyprland**: `group:groupbar:height`. |
 | `groupbar_text_color` | Groupbar text color. | **Hyprland**: `group:groupbar:text_color`. |
 | `groupbar_col_active` | Active groupbar color. | **Hyprland**: `group:groupbar:col.active`. |
 | `groupbar_col_inactive` | Inactive groupbar color. | **Hyprland**: `group:groupbar:col.inactive`. |
 | `master_mfact` | Master factor. | **Hyprland**: `master:mfact`. |
 | `master_orientation` | Master orientation. | **Hyprland**: `master:orientation`. |
 | `master_new_status` | New window status. | **Hyprland**: `master:new_status`. |
 | `master_new_on_top` | New window on top. | **Hyprland**: `master:new_on_top`. |
 | `scrolling_column_width` | Column width. | **Hyprland**: `scrolling:column_width`. |
 | `scrolling_direction` | Scrolling direction. | **Hyprland**: `scrolling:direction`. |
 | `scrolling_focus_fit_method` | Focus fit method. | **Hyprland**: `scrolling:focus_fit_method`. |
 | `scrolling_follow_focus` | Follow focus. | **Hyprland**: `scrolling:follow_focus`. |
 | `scrolling_follow_min_visible` | Follow min visible. | **Hyprland**: `scrolling:follow_min_visible`. |
 | `scrolling_fullscreen_on_one_column` | Fullscreen on one column. | **Hyprland**: `scrolling:fullscreen_on_one_column`. |
 | `workspace_layouts` | Per-workspace layout. | **Hyprland**: Map of workspace IDs to layout names (e.g., `{"1": "master"}`). |

---

## Input (`input:`)

 | Field | Description | Affected Applications & Usage |
 | :--- | :--- | :--- |
 | `main_mod` | Modifier key. | **Hyprland**: Defines `$mainMod` (e.g., `SUPER`, `ALT`) used in `bindings.conf`. |
 | `kb_layout` | Keymap layout. | **Hyprland**: `input:kb_layout`. |
 | `kb_options` | XKB options. | **Hyprland**: `input:kb_options` (e.g., `compose:caps`). |
 | `touchpad_natural_scroll` | Natural scroll. | **Hyprland**: `input:touchpad:natural_scroll`. |
 | `sensitivity` | Mouse speed. | **Hyprland**: `input:sensitivity`. |
 | `follow_mouse` | Focus behavior. | **Hyprland**: `input:follow_mouse`. |
 | `solaar_enabled` | Solaar toggle. | **Hyprland**: If true, starts `solaar` in `autostart.conf`. |
 | `touchpad_enabled` | Touchpad toggle. | **Hyprland**: Toggles physical laptop touchpad status dynamically. |

---

## App-Specific Overrides

### Kitty (`kitty:`)

 | Field | Usage in `kitty.conf` |
 | :--- | :--- |
 | `opacity` | Sets `background_opacity`. |
 | `padding` | Sets `window_padding_width`. |
 | `scrollback` | Sets `scrollback_lines`. |
 | `cursor_shape` | Sets `cursor_shape` (`block`, `beam`, `underline`). |
 | `cursor_trail` | Sets `cursor_trail` (number of steps). |
 | `cursor_trail_decay` | Sets `cursor_trail_decay`. |
 | `cursor_blink_interval` | Sets `cursor_blink_interval`. |
 | `confirm_os_window_close` | Sets `confirm_os_window_close`. |
 | `font_size` | Sets `font_size`. |

### Alacritty (`alacritty:`)

 | Field | Usage in `alacritty.toml` |
 | :--- | :--- |
 | `opacity` | Sets `window.opacity`. |
 | `padding_x` | Sets `window.padding.x`. |
 | `padding_y` | Sets `window.padding.y`. |
 | `cursor_shape` | Sets `cursor.style.shape`. |
 | `font_size` | Sets `font.size`. |

### Ghostty (`ghostty:`)

 | Field | Usage in `ghostty/config` |
 | :--- | :--- |
 | `opacity` | Sets `background-opacity`. |
 | `padding` | Sets `window-padding-x` and `window-padding-y`. |
 | `cursor_shape` | Sets `cursor-style` (`block`, `beam`, `underline`). |
 | `font_size` | Sets `font-size`. |
 | `copy_on_select` | Sets `copy-on-select` behavior. |

### Waybar (`waybar:`)

 | Field | Usage in `modules.css` / `config.jsonc` |
 | :--- | :--- |
 | `style.padding` | Global padding for all modules. |
 | `style.margin` | Horizontal margin for right-aligned modules. |
 | `style.size` | Font size for the whole bar. |
 | `colors.hardware_border` | Border color for the `#hardware` group (CPU, RAM, Disk). |

### Waybar Modules Layout (`modules: waybar:`)

 | Field | Usage in `config.jsonc` |
 | :--- | :--- |
 | `position` | Bar position (`top`, `bottom`, `left`, `right`). |
 | `height` | Fixed height of the bar. |
 | `margin_top` / `bottom` / `left` / `right` | Bar margins. |
 | `spacing` | Space between modules. |
 | `left` / `center` / `right` | Lists of module names to display in each section. |
 | `config` | A dictionary of custom module configurations (overrides default module JSON). |
 | `workspaces` | Configuration for the `hyprland/workspaces` module (icons, persistent, etc). |

#### Workspaces (`modules: waybar: workspaces:`)

 | Field | Description | Default |
 | :--- | :--- | :--- |
 | `active_only` | Only show active workspaces. | `false` |
 | `all_outputs` | Show workspaces from all outputs. | `true` |
 | `format` | The format of the workspace button. | `{icon}` |
 | `show_special` | Show special workspaces. | `false` |
 | `icons` | Map of workspace IDs/names to icons. | (Standard icons) |
 | `persistent` | Map of outputs to number of persistent workspaces. | `{"*": 6}` |

### Mako (`mako:`)

 | Field | Usage in `theme.config` |
 | :--- | :--- |
 | `timeout` | Default expiration time for notifications. |
 | `font_size` | Font size for notification text. |
 | `margin` | Distance from screen edge. |
 | `padding` | Internal notification padding. |

### Neovim (`nvim:`)

 | Field | Usage in `init.lua` |
 | :--- | :--- |
 | `padding` | Sets `vim.g.neovide_padding_top/bottom/left/right` for Neovide. |

### SwayOSD (`swayosd:`)

 | Field | Usage in `style.css` |
 | :--- | :--- |
 | `font_size` | Font size for OSD labels. |

### Walker (`walker:`)

 | Field | Usage in `theme.css` / `config.toml` |
 | :--- | :--- |
 | `font_size` | Sets the base font size for the Walker menu. |
 | `width` | Forced width of the main Walker window. |
 | `max_height` | Maximum height of the Walker window. |

### Voxtype (`voxtype:`)

 | Field | Description |
 | :--- | :--- |
 | `enabled` | Toggle voxtype functionality. |
 | `language` | Language for transcription. |
 | `mode` | Operation mode (`type`, `clipboard`, etc). |
 | `model` | Whisper model to use. |
 | `notifications.recording_start` | Toggle notification on recording start. |
 | `notifications.recording_stop` | Toggle notification on recording stop. |
 | `notifications.transcription` | Toggle notification on transcription completion. |

### Hyprland Internal (`hypr:`)

 | Section | Field | Description |
 | :--- | :--- | :--- |
 | `idle` | `timeout` | Idle timeout in seconds. |
 | `idle` | `lock_timeout` | Timeout before locking screen. |
 | `idle` | `dpms_timeout` | Timeout before turning off display. |
 | `window` | `opacity.active` | Default active window opacity. |
 | `window` | `opacity.inactive` | Default inactive window opacity. |
 | `window` | `suppress_maximize` | Whether to suppress maximize requests. |
 | `window` | `workspace_assignment.*` | Toggle automatic workspace assignment for categories. |
 | `window` | `animation.*` | Set animation style for categories. |
 | `sunset` | `temperature` | Color temperature in Kelvin for night light (e.g. `6000`). |

---

## GTK & System Features (`gtk:`, `cursor:`)

These are typically accessed via `theme.get(...)` in templates.

 | Section | Field | Usage |
 | :--- | :--- | :--- |
 | `gtk` | `icon_theme` | Sets `gtk-icon-theme-name` in GTK settings. |
 | `gtk` | `color_scheme` | Sets `gtk-application-prefer-dark-theme` (1 if `prefer-dark`). |
 | `gtk` | `qt_theme` | Sets `QT_QPA_PLATFORMTHEME` environment variable. |
 | `cursor` | `theme` | Sets `gtk-cursor-theme-name`. |
 | `cursor` | `size` | Sets `gtk-cursor-theme-size`. |

---

## Keybindings (`keybindings:`)

A centralized list containing all keybindings, replacing traditional static `.conf` files.

 | Field | Description |
 | :--- | :--- |
 | `type` | The dispatcher type (`bindd`, `bindm`, `bindeld`, etc). |
 | `mods` | Modifier keys (e.g. `$mainMod SHIFT`). |
 | `key` | The key or mouse button (e.g. `RETURN`, `mouse:272`). |
 | `desc` | Human-readable description used by the editor UI and `bindd`. |
 | `dispatcher` | The hyprland command (e.g. `exec`, `workspace`, `killactive`). |
 | `arg` | The arguments passed to the dispatcher (e.g. `kitty`, `e+1`). |
 | `category` | The category used to group bindings in the generated `bindings.conf` and UI. |

---

## Plugins (`plugins:`)

Configures Hyprland plugins managed via `hyprpm` and rendered into `~/.config/hypr/plugins.lua`.

 | Section | Field | Description | Default |
 | :--- | :--- | :--- | :--- |
 | `hyprglass` | `enabled` | Toggle hyprglass plugin loading. | `false` |
 | `hyprglass` | `repository` | Git repository URL for `hyprpm add`. | `"https://github.com/hyprnux/hyprglass"` |
 | `hyprglass` | `blur_size` | Background blur radius. | `7` |
 | `hyprglass` | `blur_passes` | Number of blur passes. | `3` |
 | `hyprglass` | `opacity` | Surface glass opacity overlay. | `0.85` |
 | `hyprglass` | `glass_color` | Color tint for glass effect. | `"rgba(255, 255, 255, 0.1)"` |
 | `hyprglass` | `glass_borders` | Enable glass styling on borders. | `true` |

