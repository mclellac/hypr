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

## Directory Structure

After installation, the hypr configuration files will be located in `~/.local/share/hypr`. The script also copies the configuration files to the `~/.config` directory, which is the standard location for application configurations in Linux.

Here's a brief overview of the key directories:

*   `~/.local/share/hypr`: The main directory for the hypr installation.
*   `~/.config`: Contains the configuration files for various applications, copied from `~/.local/share/hypr/config`.
*   `~/.local/share/hypr/bin`: Contains various scripts for managing the environment.
*   `~/.local/share/hypr/themes`: Contains different themes for the desktop environment.

## Keybindings

The keybindings are defined in the following files:

*   `~/.config/hypr/bindings.conf`: User-defined keybindings.
*   `~/.local/share/hypr/default/hypr/bindings/media.conf`: Default media keybindings.
*   `~/.local/share/hypr/default/hypr/bindings/tiling.conf`: Default tiling keybindings.
*   `~/.local/share/hypr/default/hypr/bindings/utilities.conf`: Default utility keybindings.

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

The theming system is located in `~/.local/share/hypr/themes`. Each theme is a directory containing configuration files for various applications.

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

The Neovim colorscheme is now managed separately from the main Neovim configuration. When you set a theme, the `hypr-theme-set` script will automatically update the colorscheme for Neovim without overwriting your other settings.

Each theme directory contains a `neovim.colorscheme` file that specifies the colorscheme to be used. This file is read by the `hypr-theme-set` script, which then updates a dedicated file at `~/.config/nvim/lua/user/colorscheme.lua`.

This approach allows you to customize your Neovim configuration freely without worrying about it being overwritten when you change themes.

## Neovim Configuration

Your personal Neovim configuration is located in `~/.config/nvim/lua/user/`. You can add your own plugins and settings here. The `hypr` setup uses LazyVim, so you should follow the LazyVim conventions for customization.

The base `hypr` Neovim configuration is located in `~/.local/share/hypr/config/nvim/`. During the initial installation, these files are copied to `~/.config/nvim/`. After that, your local configuration will not be overwritten by `hypr` updates unless you manually re-run the installation script.

## License

hypr is released under the [MIT License](https://opensource.org/licenses/MIT).