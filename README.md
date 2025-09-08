# hypr (a fork of Omarchy)

Turn a fresh Arch installation into a fully-configured, beautiful, and modern web development system based on Hyprland by running a single command. That's the one-line pitch for hypr. No need to write bespoke configs for every essential tool just to get started or to be up on all the latest command-line tools. hypr is an opinionated take on what Linux can be at its best.

This repository is a fork of [Omarchy](https://github.com/omarchy/omarchy).

## Installation

To install hypr, run the following command in your terminal:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/mclellac/hypr/main/install.sh)"
```

**Note:** You will need to replace `your-username` with the actual username for the repository.

The installation script will:

1.  Install a comprehensive list of packages using `yay`.
2.  Copy the configuration files to your `~/.config` directory.
3.  Replace your `~/.bashrc` with the one provided by hypr.

## Directory Structure

After installation, the hypr configuration files will be located in `~/.local/share/hypr`. The script also copies the configuration files to the `~/.config` directory, which is the standard location for application configurations in Linux.

Here's a brief overview of the key directories:

*   `~/.local/share/hypr`: The main directory for the hypr installation.
*   `~/.config`: Contains the configuration files for various applications, copied from `~/.local/share/hypr/config`.
*   `~/.local/share/hypr/bin`: Contains various scripts for managing the environment.
*   `~/.local/share/hypr/themes`: Contains different themes for the desktop environment.

## Configuration

The configuration for hypr is managed through a set of files located in the `config/` directory of this repository. When you run the installation script, these files are copied to your `~/.config` directory.

The main configuration files are:

*   `~/.config/hypr/hyprland.conf`: The main configuration file for the Hyprland window manager.
*   `~/.config/waybar/config.jsonc`: The configuration file for the Waybar status bar.
*   `~/.config/alacritty/alacritty.toml`: The configuration file for the Alacritty terminal emulator.

You can customize your environment by editing these files.

## Suggestions for Improvement

Here are some suggestions for improving the configuration:

*   **Use Environment Variables:** Instead of hardcoding paths and other values in the configuration files, consider using environment variables. This would make the configuration more portable and easier to manage.
*   **Modularize the Configuration:** The `hyprland.conf` file is quite large. It could be split into smaller, more manageable files that are then included in the main configuration file.
*   **Add a Configuration Management Tool:** For more complex configurations, a tool like `stow` or `chezmoi` could be used to manage the dotfiles. This would make it easier to track changes and keep the configuration in sync across multiple machines.
*   **Provide More Themes:** The repository already includes a good selection of themes, but more could be added. It would also be beneficial to provide instructions on how to create and install new themes.

## License

hypr is released under the [MIT License](https://opensource.org/licenses/MIT).
