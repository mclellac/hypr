# Waybar Improvements

This document outlines potential improvements for the Waybar configuration and theming, specifically tailored for an Arch Linux environment (potentially running in a VM).

## Module Improvements

*   **Hyprland Submap:** Add `hyprland/submap` to visualize current keybinding modes (e.g., resize mode).
*   **Window List/Taskbar:** Implement `wlr/taskbar` or `hyprland/window` to show active applications.
*   **Privacy Indicator:** Add a privacy module to indicate when the microphone or camera is in use.
*   **Gamemode:** Add a gamemode indicator to show when optimizations are active.
*   **Detailed Updates:** Improve the `custom/update` module to separate and display counts for Arch official repository updates vs. AUR updates in the tooltip.
*   **Weather:** Enhance the weather module with more detailed forecasting and better icons.
*   **Media Control (MPRIS):** Add an MPRIS module to control media players (Spotify, Firefox, etc.) directly from the bar, including track info.
*   **Idle Inhibitor:** Add an `idle_inhibitor` module to easily toggle screen dimming/locking prevention.
*   **Power Menu:** Create a custom power menu module that launches a logout/reboot/shutdown menu (using `wlogout` or `walker`).
*   **Network Speed:** Add upload/download speeds to the network module configuration.
*   **Battery:** Improve battery module to show time remaining and power draw in the tooltip.
*   **Disk Usage:** Add a disk usage module to monitor root or home partition space.

## Theming & Visuals

*   **Theme Variables:** Refactor CSS to use standard `@define-color` variables for all colors, making it easier to switch themes globally without rewriting `style.css`.
*   **Icon Consistency:** Ensure all modules use a consistent icon font (e.g., Nerd Fonts) and style.
*   **Tooltips:** Standardize tooltip styling (borders, padding, colors) across all modules.
*   **Grouped Modules:** Utilize Waybar's grouping feature to visually group related modules (e.g., hardware stats, system tray).

## General Configuration

*   **Click Actions:** ensure all `on-click` actions use Hyprland-compatible dispatchers (`hyprctl dispatch`) or wrappers like `hypr-launch-floating-terminal-with-presentation`.
*   **Signal Handling:** Use `pkill -SIGRTMIN+N waybar` for updating modules instead of relying on external scripts like `waybar-signal` if they are not present.
*   **VM Optimizations:** For VM users, ensure modules like `battery` or `backlight` hide gracefully if the hardware is not present.
