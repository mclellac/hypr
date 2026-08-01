# Hyprland Preferences Application

The **Hyprland Preferences Application** (`hypr-preferences-app`) is a native GTK4/Libadwaita desktop application, written in Rust, designed to configure Hyprland and its ecosystem. It provides a user-friendly graphical interface for modifying themes, keybindings, window layouts, inputs, and various other system behaviors.

## Features

- **Theming & Appearance**: Switch and manage themes seamlessly, configuring gaps, borders, rounding, and GTK settings.
- **Displays & Input**: Manage night light, display hotplugging, keyboard layouts, and mouse/touchpad behavior.
- **Keybindings**: Edit desktop keyboard shortcuts natively within the application (via the Keybindings Editor module).
- **Layout & Windows**: Configure window layouts (master, dwindle), groupbars, and application specific window rules.
- **Applications**: Configure terminal emulators (Kitty, Alacritty, Ghostty), waybar, rofi/walker, and notification daemons from a centralized location.

## Build and Run

To build the application in release mode:

```bash
cargo build --release
```

To run the application for development and testing:

```bash
cargo run
```

## Install

To install the application onto your system via `cargo`:

```bash
cargo install --path .
```
