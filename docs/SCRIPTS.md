# Scripts Documentation

This document provides an overview of the custom scripts located in the `bin/` directory. These scripts automate tasks, manage the environment, and provide functionality for UI elements.

## Theme & Appearance

* `hypr-theme-set <theme-name>`: The core script for switching themes. It invokes the theme templating engine to regenerate all application configurations based on the selected theme, cycles the wallpaper, and triggers application reloads.
* `hypr-theme-get <key> [--default <value>]`: Retrieves a nested configuration value from the currently active `theme.yaml` using dot-notation (e.g., `hypr-theme-get gtk.icon_theme`).
* `hypr-state <set|clear> <flag_name>`: Manages persistent system state flags stored in `~/.local/state/hypr/`. The `clear` command supports glob patterns and always returns 0 even if no matching files are found, preventing crashes in `set -e` scripts.
* `hypr-theme-templater`: A wrapper script that invokes the core theme engine. It calls `hypr_config_manager` to read theme data, merge it with user overrides, process all Jinja2 templates, and write the final configuration files to `~/.config/`.
* `hypr-theme-validate`: Validates a `theme.yaml` file against the JSON schema.
* `hypr-theme-list`: Lists available themes.
* `hypr-theme-bg-next`: Cycles to the next wallpaper available in the current theme's `backgrounds/` directory.
* `hypr-theme-current`: Outputs the name of the current theme.
* `hypr-theme-next`: Switches to the next available theme.
* `hypr-theme-install <path>`: Installs a theme from a directory.
* `hypr-theme-remove <theme-name>`: Removes a user theme.
* `hypr-theme-update`: Updates the current theme settings.
* `hypr-theme-refresh`: Refreshes the current theme and regenerates configurations.
* `hypr-wallpaper-set <path>`: Sets a specific image as the wallpaper.
* `hypr-font-set <font-name>`: Updates the system font configuration.
* `hypr-font-current`: Outputs the current system font.
* `hypr-font-list`: Lists available monospace fonts.
* `hypr-plugin <command> [args]`: Lifecycle management utility for Hyprland plugins. Supports `list`, `install`, `enable`, `disable`, and `set` subcommands to interact with `hyprpm` and update `settings.yaml`.
* `hypr-theme-set-browser`: Sets the theme for the web browser.
* `hypr-theme-set-keyboard`: Sets the keyboard backlight theme.
* `hypr-theme-set-keyboard-asus-rog`: Sets keyboard backlight for Asus ROG laptops.
* `hypr-theme-set-keyboard-f16`: Sets keyboard backlight for Framework 16.
* `hypr-theme-set-obsidian`: Sets the theme for Obsidian.
* `hypr-theme-set-vscode`: Sets the theme for VS Code.

## System & Maintenance

* `arch-update`: Checks for available package updates (both official and AUR). Used by Waybar.
* `hypr-update`: A wrapper for system updates (updates packages, git repositories, etc.).
* `hypr-update-available`: Checks if system updates are available.
* `hypr-update-git`: Updates the project's git repository.
* `hypr-update-restart`: Restarts the system after a successful update.
* `hypr-update-system-pkgs`: Updates official system packages using pacman.
* `hypr-update-keyring`: Updates the Arch Linux keyring to prevent signature issues.
* `hypr-update-orphan-pkgs`: Identifies and removes orphan packages.
* `hypr-update-time`: Synchronizes the system time.
* `hypr-clean`: Cleans unused packages, caches, and temporary files.
* `hypr-check-deps`: Checks for missing system dependencies.
* `hypr-doctor`: Diagnoses common system issues and configuration errors.
* `hypr-drive-info`: Displays information and status for connected storage drives.
* `hypr-drive-select`: Interactive tool to select a specific drive for management.
* `hypr-drive-set-password`: Manages passwords and encryption for storage drives.
* `hypr-gamemode`: Toggles Hyprland's gamemode (disables animations and effects for performance).
* `hypr-install-dev-env`: Installs development tools and environments.
* `hypr-install-docker-dbs`: Sets up Docker-based database containers.
* `hypr-install-dropbox`: Installs and configures Dropbox.
* `hypr-install-steam`: Installs Steam and necessary dependencies.
* `hypr-install-tailscale`: Installs and configures Tailscale VPN.
* `hypr-setup-dns`: Configures system DNS settings (e.g., Quad9, Cloudflare).
* `hypr-setup-fido2`: Sets up FIDO2 security keys for authentication.
* `hypr-setup-fingerprint`: Sets up fingerprint authentication.
* `hypr-state`: Manages system state variables and persistence.
* `hypr-reset-sudo`: Resets the sudo timestamp.
* `hypr-sudo-keepalive`: Periodically updates the sudo timestamp to keep it active.
* `hypr-system-logout`: Logs out of the current Hyprland session.
* `hypr-system-reboot`: Reboots the system.
* `hypr-system-shutdown`: Shuts down the system.
* `hypr-refresh-applications`: Refreshes the desktop application database.
* `hypr-refresh-config`: Reloads all project-specific configuration files.
* `hypr-refresh-fastfetch`: Updates the fastfetch configuration.
* `hypr-refresh-hypridle`: Reloads the `hypridle` configuration.
* `hypr-refresh-hyprland`: Reloads the `hyprland` configuration.
* `hypr-refresh-hyprlock`: Reloads the `hyprlock` configuration.
* `hypr-refresh-hyprsunset`: Reloads the `hyprsunset` configuration.
* `hypr-refresh-pacman-mirrorlist`: Updates the pacman mirrorlist for faster downloads.
* `hypr-refresh-swayosd`: Reloads the `swayosd` configuration.
* `hypr-refresh-walker`: Reloads the `walker` application runner configuration.
* `hypr-refresh-waybar`: Reloads the Waybar configuration and CSS.
* `hypr-restart-app <app_name>`: Restarts a specific application.
* `hypr-restart-bluetooth`: Restarts the Bluetooth service.
* `hypr-restart-hyprctl`: Restarts the `hyprctl` daemon.
* `hypr-restart-hypridle`: Restarts the `hypridle` service.
* `hypr-restart-hyprsunset`: Restarts the `hyprsunset` service.
* `hypr-restart-mako`: Restarts the Mako notification daemon.
* `hypr-restart-pipewire`: Restarts PipeWire audio services.
* `hypr-restart-swayosd`: Restarts the SwayOSD service.
* `hypr-restart-terminal`: Restarts the terminal emulator.
* `hypr-restart-walker`: Restarts the `walker` daemon.
* `hypr-restart-waybar`: Restarts the Waybar process.
* `hypr-restart-wifi`: Restarts the NetworkManager service.
* `hypr-restart-xcompose`: Restarts XCompose configuration.
* `hypr-upload-log`: Uploads system or installation logs to 0x0.st for troubleshooting.

## Hardware & Hardware Fixes

* `hypr-hw-match`: Utility to match hardware against DMI product names or families.
* `hypr-hw-apple`: General hardware profile for Apple machines.
* `hypr-hw-asus`: General hardware profile for ASUS machines.
* `hypr-hw-asus-expertbook-b9406`: Specific hardware profile for Asus ExpertBook B9406.
* `hypr-hw-asus-rog`: Specific hardware profile for Asus ROG laptops.
* `hypr-hw-bluetooth`: Bluetooth hardware initialization and configuration.
* `hypr-hw-dell-xps-oled`: Specific hardware profile for Dell XPS with OLED displays.
* `hypr-hw-external-monitors`: Manages configuration for external monitor setups.
* `hypr-hw-framework`: General hardware profile for Framework machines.
* `hypr-hw-framework16`: Specific hardware profile for Framework 16.
* `hypr-hw-hybrid-gpu`: Manages configuration for hybrid GPU (Intel/NVIDIA) systems.
* `hypr-hw-intel`: General hardware profile for Intel-based systems.
* `hypr-hw-intel-ptl`: Hardware profile for Intel Panther Lake systems.
* `hypr-hw-network`: Network hardware initialization and configuration.
* `hypr-hw-nvidia`: NVIDIA hardware configuration, driver installation, and environment setup.
* `hypr-hw-printer`: Printer hardware and service configuration.
* `hypr-hw-recover-internal-monitor`: Attempts to recover the internal monitor after a disconnection event.
* `hypr-hw-set-wireless-regdom`: Sets the wireless regulatory domain based on location.
* `hypr-hw-surface`: Specific hardware profile for Microsoft Surface devices.
* `hypr-hw-touchpad`: Detects and prints the name of the first touchpad/trackpad device found on the system.
* `hypr-hw-usb-autosuspend`: Configures USB autosuspend settings.
* `hypr-hw-vulkan`: Detection and status reporting for Vulkan hardware support.
* `hypr-hw-vulkan-install`: Installs the appropriate Vulkan drivers matching the detected GPU.
* `hypr-hw-fix-apple-spi-keyboard`: Fix for Apple SPI keyboards on newer MacBooks.
* `hypr-hw-fix-apple-suspend-nvme`: Fix for NVMe suspend issues on Apple hardware.
* `hypr-hw-fix-apple-t2`: Comprehensive support for Apple T2 chip MacBooks (audio, fan, WiFi).
* `hypr-hw-fix-asus-audio-mixer`: Fix for Asus ROG audio volume and mixer issues.
* `hypr-hw-fix-asus-mic`: Fix for Asus microphone input issues.
* `hypr-hw-fix-asus-ptl-b9406-display`: Specific display fix for Asus ExpertBook B9406.
* `hypr-hw-fix-asus-ptl-b9406-touchpad`: Specific touchpad fix for Asus ExpertBook B9406.
* `hypr-hw-fix-asus-z13-touchpad`: Specific touchpad fix for Asus ROG Flow Z13.
* `hypr-hw-fix-bcm43xx`: Fix for Broadcom BCM43xx WiFi chipsets.
* `hypr-hw-fix-dell-xps-haptic-touchpad`: Fix for haptic touchpads on Dell XPS models.
* `hypr-hw-fix-fkeys`: Fix for function keys behavior (media vs function).
* `hypr-hw-fix-framework-f13-amd-audio-input`: Fix for audio input on Framework 13 AMD models.
* `hypr-hw-fix-framework-qmk-hid`: Fix for Framework QMK HID devices.
* `hypr-hw-fix-intel-ipu7-camera`: Fix for Intel IPU7 MIPI cameras.
* `hypr-hw-fix-intel-lpmd`: Fix for Intel Low Power Mode Daemon.
* `hypr-hw-fix-intel-ptl-kernel`: Kernel-level fixes for Intel Panther Lake hardware.
* `hypr-hw-fix-intel-thermald`: Fix for Intel thermald configuration.
* `hypr-hw-fix-intel-video-acceleration`: Fix for Intel hardware video acceleration.
* `hypr-hw-fix-intel-wifi7-eht`: Fix for Intel WiFi 7 EHT (Extremely High Throughput).
* `hypr-hw-fix-surface-keyboard`: Fix for Microsoft Surface keyboards.
* `hypr-hw-fix-synaptic-touchpad`: Fix for Synaptic touchpad devices.
* `hypr-hw-fix-tuxedo-backlight`: Fix for keyboard and screen backlight on Tuxedo laptops.
* `hypr-hw-fix-yt6801-ethernet`: Fix for Motorcomm YT6801 Ethernet controllers.
* `hypr-hw-ignore-power-button`: Configures the system to ignore physical power button presses.
* `hypr-toggle-hybrid-gpu`: Toggles between integrated and discrete GPUs.
* `hypr-toggle-touchpad`: Toggles the touchpad on and off.
* `hypr-swayosd-brightness`: Displays a brightness OSD using SwayOSD.
* `hypr-swayosd-client`: A wrapper for the SwayOSD client for custom notifications.
* `hypr-cmd-apple-display-brightness`: Controls brightness specifically for Apple displays.
* `hypr-cmd-audio-switch`: Switches between available audio output devices.
* `hypr-ac-present`: Returns success if the system is connected to AC power.
* `hypr-battery-capacity`: Returns the battery's full design capacity in Wh.
* `hypr-battery-monitor`: Monitors battery levels and sends notifications for low battery.
* `hypr-battery-remaining-time`: Returns the estimated time remaining until the battery is empty or full.
* `hypr-audio-status`: Provides formatted audio status for Waybar.
* `hypr-network-status`: Provides formatted network status for Waybar.
* `hypr-pubip`: Fetches and displays the public IP address.
* `hypr-powerprofiles-init`: Initializes the power profile daemon.
* `hypr-powerprofiles-list`: Lists available power profiles.
* `hypr-powerprofiles-set`: Sets the active power profile (e.g., performance, balanced, power-saver).
* `hypr-waybar-cpu`: CPU usage monitoring module for Waybar.
* `hypr-waybar-disk`: Disk usage monitoring module for Waybar.
* `hypr-waybar-memory`: Memory usage monitoring module for Waybar.
* `hypr-waybar-temperature`: Temperature monitoring module for Waybar.
* `hypr-weather`: Fetches and formats weather data for Waybar.

## UI, Menus & Launchers

* `hypr-menu`: A general-purpose menu wrapper using `walker`.
* `hypr-menu-keybindings`: Launches a searchable keybindings menu using `walker`.
* `hypr-menu-md`: Launches a menu for browsing and opening Markdown files.
* `hypr-power-menu`: Launches a power management menu (Shutdown, Reboot, Logout, etc.).
* `hypr-work-menu`: A menu for workspace-specific tasks and applications.
* `hypr-socials-menu`: A menu for social, chat, communication web applications, and desktop applications.
* `hypr-docs-menu`: A menu for quick access to project documentation.
* `hypr-launch-about`: Launches the "About" information window.
* `hypr-launch-audio`: Launches the system audio mixer/controller.
* `hypr-launch-bluetooth`: Launches the Bluetooth management interface.
* `hypr-launch-browser`: Intelligently launches the default web browser with appropriate flags.
* `hypr-launch-editor`: Launches the preferred text editor.
* `hypr-launch-floating-terminal-with-presentation`: Launches a floating terminal in presentation mode.
* `hypr-launch-or-focus`: Launches an application or focuses its window if already running.
* `hypr-launch-or-focus-tui`: Launches or focuses a TUI application in a terminal.
* `hypr-launch-or-focus-webapp`: Launches or focuses a web application.
* `hypr-launch-webapp`: Launches a web application in its own standalone window.
* `hypr-launch-screensaver`: Manually triggers the screensaver.
* `hypr-launch-tui`: Launches a specific TUI application in a new terminal window.
* `hypr-launch-walker`: Launches the Walker application runner/search tool.
* `hypr-launch-wifi`: Launches the WiFi network selection interface.
* `hypr-lock-screen`: Immediately locks the screen using `hyprlock`.
* `hypr-cmd-screenshot`: Tool for taking screenshots (region, window, or full screen).
* `hypr-cmd-screenrecord`: Tool for recording the screen to a video file.
* `hypr-cmd-screenrecord-stop`: Stops the current screen recording process.
* `hypr-cmd-screensaver`: Controls screensaver state and settings.
* `hypr-cmd-terminal-cwd`: Opens a new terminal in the current working directory of the active window.
* `hypr-cmd-close-all-windows`: Closes all open windows on the current workspace.
* `hypr-exec <command>`: Launches a command in a dedicated floating terminal window.
* `hypr-show-done`: Displays a "Task Completed" notification.
* `hypr-show-logo`: Displays the project logo in the terminal.
* `hypr-active-window-transparency-toggle`: Toggles transparency for the currently active window.
* `hypr-monitor-focused`: Returns information about the currently focused monitor.
* `hypr-monitor-scaling-cycle`: Cycles through different monitor scaling factors.
* `hypr-monitor-watch`: Monitors for changes in monitor configuration and reacts.
* `hypr-window-gaps-toggle`: Toggles the display of gaps between windows.
* `hypr-window-pop`: "Pops" the active window into a floating state with specific dimensions.
* `hypr-workspace-layout-toggle`: Toggles between different workspace layouts (e.g., tiling vs master).
* `hypr-toggle`: General-purpose script for toggling various system features.
* `hypr-toggle-enabled`: Helper to check if a specific feature toggle is currently enabled.
* `hypr-toggle-idle`: Toggles idle management (hypridle) on and off.
* `hypr-toggle-nightlight`: Toggles the blue light filter (hyprsunset).
* `hypr-toggle-screensaver`: Toggles the automatic screensaver activation.
* `hypr-toggle-suspend`: Toggles the system's ability to suspend.
* `hypr-toggle-waybar`: Toggles the visibility of the Waybar.
* `hypr-zenmode`: Toggles "Zen Mode" (hides bars, gaps, and distractions).

## Clipboard & Notifications

* `hypr-clipboard-clear`: Clears the clipboard history and current content.
* `hypr-clipboard-status`: Provides clipboard status and history preview for Waybar.
* `hypr-notification-clear`: Clears all currently active notifications.
* `hypr-notification-history`: Displays a menu of recent notification history.
* `hypr-notification-logger`: Logs incoming notifications to a file for auditing.
* `hypr-notification-status`: Provides notification status (e.g., DND mode) for Waybar.
* `hypr-toggle-notification-silencing`: Toggles "Do Not Disturb" mode for notifications.
* `hypr-audit-mako`: Audits and validates the Mako notification daemon configuration.

## Package Management

* `hypr-pkg-add`: Adds a package to the list of tracked/installed packages.
* `hypr-pkg-drop`: Removes a package from the tracked list without necessarily uninstalling it.
* `hypr-pkg-install`: Installs a package and adds it to the tracked list.
* `hypr-pkg-remove`: Uninstalls a package and removes it from the tracked list.
* `hypr-pkg-missing`: Lists tracked packages that are not currently installed on the system.
* `hypr-pkg-present`: Checks if a specific package is installed.
* `hypr-pkg-ignored`: Lists packages that are explicitly ignored by the update system.
* `hypr-pkg-pinned`: Lists packages that are pinned to a specific version.
* `hypr-pkg-aur-accessible`: Checks if the AUR is accessible.
* `hypr-pkg-aur-install`: Specialized script for installing packages from the AUR.
* `hypr-tui-install`: A TUI (Text User Interface) for searching and installing packages.
* `hypr-tui-remove`: A TUI for selecting and removing installed packages.
* `hypr-tui-remove-all`: A TUI for bulk removal of packages.

## Development & Utilities

* `hypr-preferences-app`: Launches the main project preferences application.
* `hypr-keybindings-editor`: Launches a GUI/TUI for editing keybindings.
* `hypr-xcompose-editor`: Launches an editor for personal XCompose sequences.
* `hypr-utils`: A collection of shared shell utility functions used by other scripts.
* `hypr-version`: Displays the current version of the project.
* `hypr-version-branch`: Displays the current git branch of the project.
* `hypr-vim-nav`: Enables Vim-like navigation in various UI components.
* `hypr-cmd-first-run`: Script executed during the first run of the project after installation.
* `hypr-cmd-missing`: Helper to identify missing command dependencies.
* `hypr-cmd-present`: Helper to check if a command is available in the PATH.
* `hypr-cmd-tzupdate`: Updates the system timezone based on current location.

## Web & TUI Applications

* `hypr-webapp-install`: Installs a web application as a standalone desktop entry (PWA).
* `hypr-webapp-remove`: Removes an installed web application.
* `hypr-webapp-remove-all`: Removes all installed web applications.
* `hypr-webapp-handler-hey`: Custom URL handler for the HEY email web application.
* `hypr-webapp-handler-zoom`: Custom URL handler for Zoom meeting links.
* `hypr-voxtype-config`: Configures the Voxtype voice-to-text system.
* `hypr-voxtype-install`: Installs Voxtype and its dependencies.
* `hypr-voxtype-model`: Manages AI models used by Voxtype.
* `hypr-voxtype-remove`: Uninstalls Voxtype.
* `hypr-voxtype-status`: Provides Voxtype's current status for Waybar.

## Notes

Many scripts rely on the `HYPR_PATH` environment variable or assume a specific directory structure. They often use `set -euo pipefail` for robustness and incorporate error handling for missing dependencies.

## Script Safety & Formatting
All Bash and Shell scripts in the `bin/` directory adhere to modern shell scripting guidelines, and strictly enforce the following rules, verified via `shellcheck`:
- Proper and consistent escaping and quoting to prevent globbing and word-splitting.
- Use of safe arrays and MapFile for output splitting.
- Proper evaluation and command-substitution syntax.
- All Python scripts in the `bin/` directory follow strict linting checks via `pylint` and `flake8`, ensuring full `snake_case` naming consistency and broad-exception compliance.
