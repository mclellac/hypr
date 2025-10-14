# AGENTS.md: A Deep Dive into the Hyprland Environment

This document provides an exhaustive, file-by-file audit of this Hyprland configuration repository. It is designed to be a definitive reference for understanding how the environment is installed, configured, and themed, with a focus on tracking every file copy, symlink, and modification.

## Part 1: Initial Installation - File and Symlink Mapping

The installation is initiated by `install.sh`, which sources several scripts. The most critical for file system state are in `install/config/`. The variable `$HYPR_PATH` refers to the root of this repository.

### 1.1. `install/config/config.sh`

This script lays the primary foundation for the user's configuration.

**Operations:**

1.  **Creates `~/.config`:**
    *   `mkdir -p ~/.config`
    *   Ensures the user's primary configuration directory exists.

2.  **Copies Repository `config/` to User's `~/.config`:**
    *   `cp -R $HYPR_PATH/config/* ~/.config/`
    *   This is a bulk copy operation. Every file and directory from the repository's `config/` directory is copied into `~/.config`. This includes, but is not limited to:
        *   `config/Typora/` -> `~/.config/Typora/`
        *   `config/alacritty/` -> `~/.config/alacritty/`
        *   `config/btop/` -> `~/.config/btop/`
        *   `config/environment.d/` -> `~/.config/environment.d/`
        *   `config/fastfetch/` -> `~/.config/fastfetch/`
        *   `config/fcitx5/` -> `~/.config/fcitx5/`
        *   `config/fontconfig/` -> `~/.config/fontconfig/`
        *   `config/hypr/` -> `~/.config/hypr/`
        *   `config/lazygit/` -> `~/.config/lazygit/`
        *   `config/nvim/` -> `~/.config/nvim/`
        *   `config/swayosd/` -> `~/.config/swayosd/`
        *   `config/systemd/` -> `~/.config/systemd/`
        *   `config/uwsm/` -> `~/.config/uwsm/`
        *   `config/walker/` -> `~/.config/walker/`
        *   `config/waybar/` -> `~/.config/waybar/`
        *   `config/xournalpp/` -> `~/.config/xournalpp/`
        *   `config/brave-flags.conf` -> `~/.config/brave-flags.conf`
        *   `config/chromium-flags.conf` -> `~/.config/chromium-flags.conf`
        *   `config/hypr.ttf` -> `~/.config/hypr.ttf`
        *   `config/starship.toml` -> `~/.config/starship.toml`

3.  **Copies Default `.bashrc`:**
    *   `cp $HYPR_PATH/default/bashrc ~/.bashrc`
    *   The user's `~/.bashrc` is overwritten with the one from the repository.

### 1.2. `install/config/theme.sh`

This script sets up the entire theming infrastructure.

**Operations:**

1.  **Creates Theme Directories:**
    *   `mkdir -p ~/.config/hypr/themes`
    *   `mkdir -p ~/.config/hypr/current`

2.  **Symlinks All Available Themes:**
    *   `for f in $HYPR_PATH/themes/*; do ln -nfs "$f" ~/.config/hypr/themes/; done`
    *   Each theme directory inside the repository's `themes/` directory is symlinked into `~/.config/hypr/themes/`. For example:
        *   `$HYPR_PATH/themes/tokyo-night` -> `~/.config/hypr/themes/tokyo-night`
        *   `$HYPR_PATH/themes/gruvbox` -> `~/.config/hypr/themes/gruvbox`
        *   ...and so on for all other themes.

3.  **Sets the Default Theme:**
    *   `ln -snf ~/.config/hypr/themes/tokyo-night ~/.config/hypr/current/theme`
    *   This is a critical step. A symlink named `theme` is created inside `~/.config/hypr/current`. It points to the default theme, `tokyo-night`. This `theme` symlink is the central pivot for the entire theming system.

4.  **Sets the Default Background:**
    *   `ln -snf ~/.config/hypr/current/theme/backgrounds/1-scenery... .png ~/.config/hypr/current/background`
    *   A symlink named `background` is created, pointing to a specific wallpaper within the default theme.

5.  **Creates Initial Application Theme Symlinks:**
    *   These symlinks connect application configurations to files *inside the current theme directory*. This is how they become theme-aware.
    *   **Neovim:** `ln -snf ~/.config/hypr/current/theme/neovim.lua ~/.config/nvim/lua/plugins/theme.lua`
    *   **btop:** `mkdir -p ~/.config/btop/themes` then `ln -snf ~/.config/hypr/current/theme/btop.theme ~/.config/btop/themes/current.theme`
    *   **mako:** `mkdir -p ~/.config/mako` then `ln -snf ~/.config/hypr/current/theme/mako.ini ~/.config/mako/config`

### 1.3. `install/config/branding.sh`

This script sets up user-configurable branding assets.

**Operations:**

1.  **Creates Branding Directory:**
    *   `mkdir -p ~/.config/hypr/branding`

2.  **Copies Branding Assets:**
    *   `cp $HYPR_PATH/icon.txt ~/.config/hypr/branding/about.txt`
    *   `cp $HYPR_PATH/logo.txt ~/.config/hypr/branding/screensaver.txt`
    *   These are plain text files that can be edited by the user to change the appearance of the `fastfetch` "about" screen and the screensaver.

## Part 2: The Theming System in Action (`hypr-theme-set`)

When a user runs `hypr-theme-set <theme-name>`, the following sequence of file operations occurs.

### Step 1: The Atomic Symlink Switch

*   **Command:** `ln -nsf $THEME_PATH $CURRENT_THEME_DIR`
*   **Source (`$THEME_PATH`):** `~/.config/hypr/themes/<theme-name>`
*   **Destination (`$CURRENT_THEME_DIR`):** `~/.config/hypr/current/theme`
*   **Effect:** This single command re-points the central `theme` symlink to the new theme directory. All configurations that rely on this path are now effectively pointed at the new theme's assets.

### Step 2: Application-Specific File Modifications (Direct Overwrites)

These applications have their theme configuration files directly modified or created by the script.

*   **Walker (Application Launcher):**
    *   **Target File:** `~/.config/walker/themes/hypr-default/style.css`
    *   **Operation:** This file is completely **overwritten**.
    *   **Process:**
        1.  `mkdir -p ~/.config/walker/themes/hypr-default`
        2.  `cat $THEME_PATH/walker.css > .../style.css` (The new theme's CSS is written to the file).
        3.  `cat $WALKER_DEFAULT_THEME_CSS >> .../style.css` (A default base style is appended).

*   **Neovim:**
    *   **Target File:** `~/.config/nvim/lua/plugins/theme.lua`
    *   **Operation:** This file is deleted and then conditionally re-created.
    *   **Process:** The `hypr-theme-set` script implements a dual-method theming system:
        1.  **Plugin Method:** It first checks for the existence of a `neovim.plugin` file in the theme directory (e.g., `~/.config/hypr/themes/tokyo-night/neovim.plugin`). If found, this file, which is expected to be a valid LazyVim plugin definition, is copied directly to `~/.config/nvim/lua/plugins/theme.lua`.
        2.  **Legacy Colorscheme Method:** If `neovim.plugin` is not found, the script then checks for a `neovim.colorscheme` file. If this file exists, the script reads the colorscheme name from it and dynamically creates a new `theme.lua` file containing a basic LazyVim configuration block to set that colorscheme.
    *   **Effect:** This logic allows themes to be defined either as complete Neovim plugins or as simple colorscheme names, providing both power and backward compatibility.

*   **Alacritty:**
    *   **Target File:** `~/.config/alacritty/alacritty.toml`
    *   **Operation:** `touch`
    *   **Process:** The script does not modify the file's content. It updates its modification timestamp.
    *   **Mechanism:** The `alacritty.toml` file (copied during installation) contains the line `import = ["~/.config/hypr/current/theme/alacritty.toml"]`. By `touch`-ing the main config, Alacritty is forced to reload it, re-evaluate the `import` statement, and since `~/.config/hypr/current/theme` now points to the new theme, it loads the new theme's `alacritty.toml`.

### Step 3: Configurations Updated via Existing Symlinks (Indirect)

The following applications update their theme automatically because their configuration was already symlinked to a file within `~/.config/hypr/current/theme` during the initial installation. When the parent `theme` symlink changes (Step 1), these links instantly point to the new theme's files.

*   **btop:** `~/.config/btop/themes/current.theme` -> `~/.config/hypr/current/theme/btop.theme`
*   **mako:** `~/.config/mako/config` -> `~/.config/hypr/current/theme/mako.ini`
*   **Waybar:** The main `~/.config/waybar/style.css` contains an `@import url("~/.config/hypr/current/theme/waybar.css");` statement.
*   **Hyprland/Hyprlock/etc:** The main `~/.config/hypr/hyprland.conf` sources `~/.config/hypr/current/theme/hyprland.conf`.

### Step 4: Background Change

*   **Action:** The `hypr-theme-bg-next` script is called.
*   **Effect:** This script selects a random image from the new theme's `backgrounds/` directory and updates the `~/.config/hypr/current/background` symlink to point to it.

### Step 5: Process Reloads

To make the changes take effect, the script sends signals or restarts the relevant processes:

*   `pkill -SIGUSR2 btop`: Reloads btop.
*   `hypr-restart-waybar`: Restarts the waybar process.
*   `hypr-restart-swayosd`: Restarts the on-screen display daemon.
*   `hypr-restart-walker`: Restarts the walker (and elephant) service.
*   `makoctl reload`: Reloads the notification daemon.
*   `hyprctl reload`: Reloads the Hyprland compositor configuration.