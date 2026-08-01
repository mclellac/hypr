# Solaar Udev Rules

For users with Logitech mice and keyboards, `solaar` is an essential tool for managing unified receivers, battery levels, and custom buttons.

To grant user-level access to Logitech receivers without running as root, install the following udev rules:

Create a new file `/etc/udev/rules.d/42-logitech-unify-compat.rules`:

   ```bash
   sudo touch /etc/udev/rules.d/42-logitech-unify-compat.rules
   ```

Add the following rules:

   ```udev
   # Logitech Unifying receivers
   ACTION=="add", SUBSYSTEM=="hidraw", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="c52b", MODE="0660", GROUP="uucp"
   ACTION=="add", SUBSYSTEM=="hidraw", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="c532", MODE="0660", GROUP="uucp"
   ACTION=="add", SUBSYSTEM=="hidraw", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="c52f", MODE="0660", GROUP="uucp"
   ```

Reload the udev rules:

   ```bash
   sudo udevadm control --reload-rules && sudo udevadm trigger
   ```

Make sure your user is in the `uucp` group (the standard Arch Linux group for serial/HID access):

   ```bash
   sudo usermod -aG uucp $USER
   ```

## Known Warnings on Wayland

When running `solaar --window=hide` from a terminal or as an autostart command, you may observe the following warnings in your logs:

```text
rules cannot access modifier keys in Wayland, accessing process only works on GNOME with Solaar Gnome extension installed
(solaar:138954): libayatana-appindicator-WARNING **: 12:09:16.997: libayatana-appindicator is deprecated. Please use libayatana-appindicator-glib in newly written code.
```

These are **non-fatal warnings** and completely normal under a Wayland compositor like Hyprland.

- The **Wayland modifier warning** simply means Solaar cannot globally intercept keystrokes for its advanced "Rules" engine (a security feature of Wayland). Basic functionality like battery reporting and configuration still works perfectly.
- The **AppIndicator warning** is an upstream deprecation notice from the underlying library Solaar uses to draw its tray icon. It does not affect usability.
