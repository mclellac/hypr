"""
Input settings page for the Hyprland Preferences Application.
"""

import sys
import os

# pylint: disable=wrong-import-position
from gi.repository import Adw, Gtk

# Adjust path to find utils
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
import utils

class InputPage(Adw.PreferencesPage):
    """Page for configuring input settings like mouse and touchpad."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.set_title("Input")
        self.set_icon_name("input-mouse-symbolic")

        self.add(self._init_mouse_group())
        self.add(self._init_touchpad_group())

    def _init_mouse_group(self):
        """Initializes the Mouse settings group."""
        group = Adw.PreferencesGroup(title="Mouse")

        # Sensitivity
        self.sensitivity = Adw.SpinRow(title="Sensitivity")
        self.sensitivity.set_subtitle("Mouse cursor sensitivity")
        self.sensitivity.set_digits(2)
        self.sensitivity.set_adjustment(
            Gtk.Adjustment(value=0.0, lower=-1.0, upper=1.0, step_increment=0.1)
        )
        val = utils.get_input_value(["input", "sensitivity"])
        if val:
            try:
                self.sensitivity.set_value(float(val))
            except ValueError:
                pass
        self.sensitivity.connect("notify::value", self.on_sensitivity_changed)
        group.add(self.sensitivity)

        # Follow Mouse
        self.follow_mouse = Adw.ComboRow(title="Follow Mouse")
        self.follow_mouse.set_subtitle("Window focus behavior")
        self.follow_mouse.set_model(Gtk.StringList.new([
            "0 (None)", "1 (Strict)", "2 (Loose)", "3 (Loose w/ Click)"
        ]))
        val = utils.get_input_value(["input", "follow_mouse"])
        if val:
            try:
                idx = int(val)
                if 0 <= idx <= 3:
                    self.follow_mouse.set_selected(idx)
            except ValueError:
                pass
        self.follow_mouse.connect("notify::selected-item", self.on_follow_mouse_changed)
        group.add(self.follow_mouse)

        return group

    def _init_touchpad_group(self):
        """Initializes the Touchpad settings group."""
        group = Adw.PreferencesGroup(title="Touchpad")

        # Natural Scroll
        self.natural_scroll = Adw.SwitchRow(title="Natural Scroll")
        self.natural_scroll.set_subtitle("Invert scroll direction")
        val = utils.get_input_value(["input", "touchpad", "natural_scroll"])
        if val:
            self.natural_scroll.set_active(val.lower() == "true")
        self.natural_scroll.connect("notify::active", self.on_natural_scroll_changed)
        group.add(self.natural_scroll)

        return group

    def on_sensitivity_changed(self, row, _):
        """Callback for sensitivity changes."""
        val = f"{row.get_value():.2f}"
        utils.set_input_value(["input", "sensitivity"], val)
        self._show_toast(f"Sensitivity set to {val}")

    def on_follow_mouse_changed(self, row, _):
        """Callback for follow_mouse changes."""
        selected = row.get_selected()
        utils.set_input_value(["input", "follow_mouse"], str(selected))
        self._show_toast(f"Follow Mouse set to {selected}")

    def on_natural_scroll_changed(self, row, _):
        """Callback for natural_scroll toggle."""
        val = str(row.get_active()).lower()
        utils.set_input_value(["input", "touchpad", "natural_scroll"], val)
        status = "enabled" if row.get_active() else "disabled"
        self._show_toast(f"Natural Scroll {status}")

    def _show_toast(self, message):
        """Helper to show a toast message."""
        win = self.get_native()
        if win:
            win.add_toast(Adw.Toast.new(message))
