"""
Theme and Font settings page for the Hyprland Preferences Application.
"""

import sys
import os

# pylint: disable=wrong-import-position
from gi.repository import Adw, Gtk

# Adjust path to find utils
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
import utils

class ThemePage(Adw.PreferencesPage):
    """Page for configuring Theme and Font settings."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.set_title("Theme & Font")
        self.set_icon_name("preferences-desktop-font-symbolic")

        self.add(self._init_theme_group())
        self.add(self._init_font_group())

    def _init_theme_group(self):
        """Initializes the Theme settings group."""
        group = Adw.PreferencesGroup(title="Theme")

        self.theme_row = Adw.ComboRow(title="Theme")
        self.theme_row.set_subtitle("Select the active system theme")

        themes = utils.get_theme_list()
        if themes:
            self.theme_row.set_model(Gtk.StringList.new(themes))

            current = utils.get_current_theme()
            # Try to find current in list
            for i, theme in enumerate(themes):
                if theme.lower() == current.lower():
                    self.theme_row.set_selected(i)
                    break
        else:
            self.theme_row.set_subtitle("No themes found")
            self.theme_row.set_sensitive(False)

        self.theme_row.connect("notify::selected-item", self.on_theme_changed)
        group.add(self.theme_row)
        return group

    def _init_font_group(self):
        """Initializes the Font settings group."""
        group = Adw.PreferencesGroup(title="Font")

        self.font_row = Adw.ComboRow(title="Monospace Font")
        self.font_row.set_subtitle("Select the system monospace font")

        fonts = utils.list_fonts()
        if fonts:
            self.font_row.set_model(Gtk.StringList.new(fonts))

            current = utils.get_current_font()
            # Try to find current in list
            # Current font might differ slightly in name, try exact then fuzzy
            found = False
            for i, font in enumerate(fonts):
                if font == current:
                    self.font_row.set_selected(i)
                    found = True
                    break

            if not found:
                # Try simple fuzzy match
                for i, font in enumerate(fonts):
                    if current in font or font in current:
                        self.font_row.set_selected(i)
                        found = True
                        break
        else:
            self.font_row.set_subtitle("No fonts found")
            self.font_row.set_sensitive(False)

        self.font_row.connect("notify::selected-item", self.on_font_changed)
        group.add(self.font_row)
        return group

    def on_theme_changed(self, row, _):
        """Callback for theme changes."""
        item = row.get_selected_item()
        if item:
            selected = item.get_string()

            # Show toast immediately as this might take a second
            self._show_toast(f"Applying theme {selected}...")

            # Use GLib.idle_add or similar if we wanted to not block UI,
            # but for now we block simply.
            success = utils.set_theme(selected)
            if success:
                self._show_toast(f"Theme set to {selected}")
            else:
                self._show_toast(f"Failed to set theme {selected}")

    def on_font_changed(self, row, _):
        """Callback for font changes."""
        item = row.get_selected_item()
        if item:
            selected = item.get_string()
            self._show_toast(f"Applying font {selected}...")

            success = utils.set_font(selected)
            if success:
                self._show_toast(f"Font set to {selected}")
            else:
                self._show_toast(f"Failed to set font {selected}")

    def _show_toast(self, message):
        """Helper to show a toast message."""
        win = self.get_native()
        if win:
            win.add_toast(Adw.Toast.new(message))
