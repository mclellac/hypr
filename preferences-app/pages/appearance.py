"""
Appearance settings page for the Hyprland Preferences Application.
"""

import sys
import os

# pylint: disable=wrong-import-position
from gi.repository import Adw, Gtk

# Adjust path to find utils
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
import utils

class AppearancePage(Adw.PreferencesPage):
    """Page for configuring appearance settings like gaps, borders, and shadows."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.set_title("Appearance")
        self.set_icon_name("preferences-desktop-theme-symbolic")

        self.add(self._init_gaps_group())
        self.add(self._init_shadows_group())
        self.add(self._init_blur_group())

    def _init_gaps_group(self):
        """Initializes the Gaps & Borders settings group."""
        gaps_group = Adw.PreferencesGroup(title="Gaps &amp; Borders")

        # Gaps In
        self.gaps_in = Adw.SpinRow(title="Gaps In")
        self.gaps_in.set_subtitle("Space between windows")
        self.gaps_in.set_tooltip_text("Sets the inner gap between windows in pixels.")
        self.gaps_in.set_adjustment(
            Gtk.Adjustment(value=0, lower=0, upper=100, step_increment=1)
        )
        val = utils.get_looknfeel_value(["general", "gaps_in"])
        if val:
            try:
                self.gaps_in.set_value(float(val))
            except ValueError:
                pass
        self.gaps_in.connect("notify::value", self.on_gaps_in_changed)
        gaps_group.add(self.gaps_in)

        # Gaps Out
        self.gaps_out = Adw.SpinRow(title="Gaps Out")
        self.gaps_out.set_subtitle("Space between windows and screen edge")
        self.gaps_out.set_tooltip_text(
            "Sets the outer gap between windows and the monitor edge in pixels."
        )
        self.gaps_out.set_adjustment(
            Gtk.Adjustment(value=0, lower=0, upper=100, step_increment=1)
        )
        val = utils.get_looknfeel_value(["general", "gaps_out"])
        if val:
            try:
                self.gaps_out.set_value(float(val))
            except ValueError:
                pass
        self.gaps_out.connect("notify::value", self.on_gaps_out_changed)
        gaps_group.add(self.gaps_out)

        # Rounding
        self.rounding = Adw.SpinRow(title="Rounding")
        self.rounding.set_subtitle("Corner radius")
        self.rounding.set_tooltip_text("Sets the window corner rounding radius in pixels.")
        self.rounding.set_adjustment(
            Gtk.Adjustment(value=0, lower=0, upper=50, step_increment=1)
        )
        val = utils.get_looknfeel_value(["decoration", "rounding"])
        if val:
            try:
                self.rounding.set_value(float(val))
            except ValueError:
                pass
        self.rounding.connect("notify::value", self.on_rounding_changed)
        gaps_group.add(self.rounding)

        return gaps_group

    def _init_shadows_group(self):
        """Initializes the Shadows settings group."""
        shadow_group = Adw.PreferencesGroup(title="Shadows")

        # Enabled
        self.shadow_enabled = Adw.SwitchRow(title="Enabled")
        self.shadow_enabled.set_subtitle("Enable window shadows")
        self.shadow_enabled.set_tooltip_text("Toggle window shadows on or off.")
        val = utils.get_looknfeel_value(["decoration", "shadow", "enabled"])
        if val:
            self.shadow_enabled.set_active(val.lower() == "true")
        self.shadow_enabled.connect("notify::active", self.on_shadow_enabled_changed)
        shadow_group.add(self.shadow_enabled)

        # Range
        self.shadow_range = Adw.SpinRow(title="Range")
        self.shadow_range.set_subtitle("Shadow size")
        self.shadow_range.set_tooltip_text("Sets the size (range) of the shadow in pixels.")
        self.shadow_range.set_adjustment(
            Gtk.Adjustment(value=0, lower=0, upper=100, step_increment=1)
        )
        val = utils.get_looknfeel_value(["decoration", "shadow", "range"])
        if val:
            try:
                self.shadow_range.set_value(float(val))
            except ValueError:
                pass
        self.shadow_range.connect("notify::value", self.on_shadow_range_changed)
        shadow_group.add(self.shadow_range)

        # Render Power
        self.shadow_power = Adw.SpinRow(title="Render Power")
        self.shadow_power.set_subtitle("Shadow sharpness (1-4)")
        self.shadow_power.set_tooltip_text(
            "Sets the render power (falloff) of the shadow. Higher values mean sharper shadows."
        )
        self.shadow_power.set_adjustment(
            Gtk.Adjustment(value=0, lower=0, upper=10, step_increment=1)
        )
        val = utils.get_looknfeel_value(["decoration", "shadow", "render_power"])
        if val:
            try:
                self.shadow_power.set_value(float(val))
            except ValueError:
                pass
        self.shadow_power.connect("notify::value", self.on_shadow_power_changed)
        shadow_group.add(self.shadow_power)

        # Color
        self.shadow_color = Adw.EntryRow(title="Color")
        self.shadow_color.set_title("Shadow Color")
        self.shadow_color.set_tooltip_text("Sets the shadow color in hex format (e.g., #00000066).")
        val = utils.get_looknfeel_value(["decoration", "shadow", "color"])
        if val:
            self.shadow_color.set_text(val)
        self.shadow_color.connect("apply", self.on_shadow_color_changed)
        self.shadow_color.connect("entry-activated", self.on_shadow_color_changed)

        shadow_group.add(self.shadow_color)

        return shadow_group

    def _init_blur_group(self):
        """Initializes the Blur settings group."""
        blur_group = Adw.PreferencesGroup(title="Blur")

        # Enabled
        self.blur_enabled = Adw.SwitchRow(title="Enabled")
        self.blur_enabled.set_subtitle("Enable background blur")
        val = utils.get_looknfeel_value(["decoration", "blur", "enabled"])
        if val:
            self.blur_enabled.set_active(val.lower() == "true")
        self.blur_enabled.connect("notify::active", self.on_blur_enabled_changed)
        blur_group.add(self.blur_enabled)

        # Size
        self.blur_size = Adw.SpinRow(title="Size")
        self.blur_size.set_subtitle("Blur size")
        self.blur_size.set_adjustment(
            Gtk.Adjustment(value=0, lower=0, upper=20, step_increment=1)
        )
        val = utils.get_looknfeel_value(["decoration", "blur", "size"])
        if val:
            try:
                self.blur_size.set_value(float(val))
            except ValueError:
                pass
        self.blur_size.connect("notify::value", self.on_blur_size_changed)
        blur_group.add(self.blur_size)

        # Passes
        self.blur_passes = Adw.SpinRow(title="Passes")
        self.blur_passes.set_subtitle("Number of passes")
        self.blur_passes.set_adjustment(
            Gtk.Adjustment(value=0, lower=0, upper=10, step_increment=1)
        )
        val = utils.get_looknfeel_value(["decoration", "blur", "passes"])
        if val:
            try:
                self.blur_passes.set_value(float(val))
            except ValueError:
                pass
        self.blur_passes.connect("notify::value", self.on_blur_passes_changed)
        blur_group.add(self.blur_passes)

        # Vibrancy
        self.blur_vibrancy = Adw.SpinRow(title="Vibrancy")
        self.blur_vibrancy.set_subtitle("Vibrancy of the blur")
        self.blur_vibrancy.set_digits(4)
        self.blur_vibrancy.set_adjustment(
            Gtk.Adjustment(value=0.0, lower=0.0, upper=1.0, step_increment=0.01)
        )
        val = utils.get_looknfeel_value(["decoration", "blur", "vibrancy"])
        if val:
            try:
                self.blur_vibrancy.set_value(float(val))
            except ValueError:
                pass
        self.blur_vibrancy.connect("notify::value", self.on_blur_vibrancy_changed)
        blur_group.add(self.blur_vibrancy)

        return blur_group

    def _show_toast(self, message):
        """Helper to show a toast message."""
        win = self.get_native()
        if win:
            win.add_toast(Adw.Toast.new(message))

    def on_gaps_in_changed(self, row, _):
        """Callback for gaps_in changes."""
        val = str(int(row.get_value()))
        utils.set_looknfeel_value(["general", "gaps_in"], val)
        self._show_toast(f"Gaps In set to {val}")

    def on_gaps_out_changed(self, row, _):
        """Callback for gaps_out changes."""
        val = str(int(row.get_value()))
        utils.set_looknfeel_value(["general", "gaps_out"], val)
        self._show_toast(f"Gaps Out set to {val}")

    def on_rounding_changed(self, row, _):
        """Callback for rounding changes."""
        val = str(int(row.get_value()))
        utils.set_looknfeel_value(["decoration", "rounding"], val)
        self._show_toast(f"Rounding set to {val}")

    def on_shadow_enabled_changed(self, row, _):
        """Callback for shadow enabled toggle."""
        val = str(row.get_active()).lower()
        utils.set_looknfeel_value(["decoration", "shadow", "enabled"], val)
        status = "enabled" if row.get_active() else "disabled"
        self._show_toast(f"Shadows {status}")

    def on_shadow_range_changed(self, row, _):
        """Callback for shadow range changes."""
        val = str(int(row.get_value()))
        utils.set_looknfeel_value(["decoration", "shadow", "range"], val)
        self._show_toast(f"Shadow range set to {val}")

    def on_shadow_power_changed(self, row, _):
        """Callback for shadow render power changes."""
        val = str(int(row.get_value()))
        utils.set_looknfeel_value(["decoration", "shadow", "render_power"], val)
        self._show_toast(f"Shadow power set to {val}")

    def on_shadow_color_changed(self, row):
        """Callback for shadow color changes."""
        val = row.get_text()
        utils.set_looknfeel_value(["decoration", "shadow", "color"], val)
        self._show_toast(f"Shadow color set to {val}")

    def on_blur_enabled_changed(self, row, _):
        """Callback for blur enabled toggle."""
        val = str(row.get_active()).lower()
        utils.set_looknfeel_value(["decoration", "blur", "enabled"], val)
        status = "enabled" if row.get_active() else "disabled"
        self._show_toast(f"Blur {status}")

    def on_blur_size_changed(self, row, _):
        """Callback for blur size changes."""
        val = str(int(row.get_value()))
        utils.set_looknfeel_value(["decoration", "blur", "size"], val)
        self._show_toast(f"Blur size set to {val}")

    def on_blur_passes_changed(self, row, _):
        """Callback for blur passes changes."""
        val = str(int(row.get_value()))
        utils.set_looknfeel_value(["decoration", "blur", "passes"], val)
        self._show_toast(f"Blur passes set to {val}")

    def on_blur_vibrancy_changed(self, row, _):
        """Callback for blur vibrancy changes."""
        val = f"{row.get_value():.4f}"
        utils.set_looknfeel_value(["decoration", "blur", "vibrancy"], val)
        self._show_toast(f"Blur vibrancy set to {val}")
