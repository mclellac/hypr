# hypr

Turn a fresh Arch installation into a fully-configured, beautiful, and modern web development system based on Hyprland by running a single command. That's the one-line pitch for hypr. No need to write bespoke configs for every essential tool just to get started or to be up on all the latest command-line tools. hypr is an opinionated take on what Linux can be at its best.

This repository is a fork of [Omarchy](https://github.com/omarchy/omarchy).

## Installation

To install hypr, run the following command in your terminal:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/mclellac/hypr/main/install.sh)"
```

The installation script will:

1.  Install a comprehensive list of packages using `yay`.
2.  Copy the configuration files to your `~/.config` directory.
3.  Replace your `~/.bashrc` with the one provided by hypr.
4.  Install fonts, TUIs, and web apps.
5.  Set up the default theme.

## Features

*   **Update Notifications:** Get notified of official Arch Linux and AUR package updates directly in Waybar.
*   **One-Click Updates:** Click the update icon to open a terminal and update all your packages.
*   **Comprehensive Theming:** Easily switch between themes for all your applications.
*   **Pre-configured Applications:** A wide range of applications are pre-configured for a seamless experience.

## Directory Structure

The installation script copies the default configuration files directly into your `~/.config` directory, which is the standard location for application configurations in Linux. All the scripts for managing the environment are installed to `~/.local/bin`, and the themes are located in `~/.config/hypr/themes`.

Here's a brief overview of the key directories:

*   `~/.config`: Contains the configuration files for all the integrated applications.
*   `~/.local/bin`: Contains all the `hypr-*` scripts for managing the environment.
*   `~/.config/hypr/themes`: Contains the different themes for the desktop environment.

## Keybindings

The default keybindings are defined within the repository. You can customize them by editing the following file after installation:

*   `~/.config/hypr/bindings.conf`: User-defined keybindings.

You can customize the keybindings by editing `~/.config/hypr/bindings.conf`.

Here are some of the most important default keybindings:

| Keybinding              | Description                   |
| ----------------------- | ----------------------------- |
| `SUPER + Return`        | Open terminal                 |
| `SUPER + E`             | Open file manager             |
| `SUPER + B`             | Open browser                  |
| `SUPER + Q`             | Close active window           |
| `SUPER + J`             | Toggle split                  |
| `SUPER + V`             | Toggle floating               |
| `SUPER + F`             | Force full screen             |
| `SUPER + Arrow keys`    | Move focus                    |
| `SUPER + [0-9]`         | Switch to workspace           |
| `SUPER + SHIFT + [0-9]` | Move window to workspace      |
| `SUPER + SPACE`         | Launch apps                   |
| `SUPER + K`             | Show key bindings             |
| `SUPER + COMMA`         | Dismiss last notification     |
| `SUPER + SHIFT + COMMA` | Dismiss all notifications     |
| `, PRINT`               | Screenshot of region          |
| `SHIFT, PRINT`          | Screenshot of window          |
| `CTRL, PRINT`           | Screenshot of display         |

## Theming

The theming system is located in `~/.config/hypr/themes`. Each theme is a directory containing configuration files for various applications.

To change the theme, you can use the `hypr-theme-set` script:

```bash
hypr-theme-set <theme-name>
```

For example, to set the theme to `tokyo-night`, you would run:

```bash
hypr-theme-set tokyo-night
```

The `hypr-theme-set` script will update the symlinks for the current theme and restart the necessary components to apply the new theme.

### Neovim Theming

The `hypr` theming system supports two methods for theming Neovim, ensuring both simplicity and flexibility.

1.  **Plugin-Based Theming (Preferred Method):**
    For complex themes that are distributed as full Neovim plugins (e.g., `folke/tokyonight.nvim`), a theme can include a `neovim.plugin` file. This file defines the theme as a LazyVim plugin. The `hypr-theme-set` script will copy this file to `~/.config/nvim/lua/plugins/theme.lua`, allowing `lazy.nvim` to manage it. This is the most robust method.

2.  **Simple Colorscheme Theming (Legacy):**
    For simpler themes that only require setting a colorscheme name, a theme can include a `neovim.colorscheme` file. This file contains only the name of the colorscheme (e.g., `catppuccin-latte`). The script will then generate a basic plugin file to set the colorscheme for LazyVim.

This dual approach ensures that your personal Neovim configuration in `~/.config/nvim/lua/user/` is never overwritten when you switch themes.

## Neovim Configuration

Your personal Neovim configuration is located in `~/.config/nvim/lua/user/`. You can add your own plugins and settings here. The `hypr` setup uses LazyVim, so you should follow the LazyVim conventions for customization.

The base `hypr` Neovim configuration is defined in the repository. During the initial installation, these files are copied to `~/.config/nvim/`. After that, your local configuration will not be overwritten by `hypr` updates unless you manually re-run the installation script.

## License

hypr is released under the [MIT License](https://opensource.org/licenses/MIT).
