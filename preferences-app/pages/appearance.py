from gi.repository import Adw, Gtk, GObject
import sys
import os

# Adjust path to find utils
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
import utils

class AppearancePage(Adw.PreferencesPage):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.set_title("Appearance")
        self.set_icon_name("preferences-desktop-theme-symbolic")

        # Gaps & Borders -> Gaps and Borders (avoid markup issue)
        gaps_group = Adw.PreferencesGroup(title="Gaps and Borders")
        self.add(gaps_group)

        self.gaps_in = Adw.SpinRow(title="Gaps In")
        self.gaps_in.set_adjustment(Gtk.Adjustment(value=0, lower=0, upper=100, step_increment=1))
        val = utils.get_looknfeel_value(["general", "gaps_in"])
        if val:
            try:
                self.gaps_in.set_value(float(val))
            except ValueError:
                pass
        self.gaps_in.connect("notify::value", self.on_gaps_in_changed)
        gaps_group.add(self.gaps_in)

        self.gaps_out = Adw.SpinRow(title="Gaps Out")
        self.gaps_out.set_adjustment(Gtk.Adjustment(value=0, lower=0, upper=100, step_increment=1))
        val = utils.get_looknfeel_value(["general", "gaps_out"])
        if val:
            try:
                self.gaps_out.set_value(float(val))
            except ValueError:
                pass
        self.gaps_out.connect("notify::value", self.on_gaps_out_changed)
        gaps_group.add(self.gaps_out)

        self.rounding = Adw.SpinRow(title="Rounding")
        self.rounding.set_adjustment(Gtk.Adjustment(value=0, lower=0, upper=50, step_increment=1))
        val = utils.get_looknfeel_value(["decoration", "rounding"])
        if val:
            try:
                self.rounding.set_value(float(val))
            except ValueError:
                pass
        self.rounding.connect("notify::value", self.on_rounding_changed)
        gaps_group.add(self.rounding)

        # Shadows
        shadow_group = Adw.PreferencesGroup(title="Shadows")
        self.add(shadow_group)

        self.shadow_enabled = Adw.SwitchRow(title="Enabled")
        val = utils.get_looknfeel_value(["decoration", "shadow", "enabled"])
        if val: self.shadow_enabled.set_active(val.lower() == "true")
        self.shadow_enabled.connect("notify::active", self.on_shadow_enabled_changed)
        shadow_group.add(self.shadow_enabled)

        self.shadow_range = Adw.SpinRow(title="Range")
        self.shadow_range.set_adjustment(Gtk.Adjustment(value=0, lower=0, upper=100, step_increment=1))
        val = utils.get_looknfeel_value(["decoration", "shadow", "range"])
        if val:
            try:
                self.shadow_range.set_value(float(val))
            except ValueError:
                pass
        self.shadow_range.connect("notify::value", self.on_shadow_range_changed)
        shadow_group.add(self.shadow_range)

        self.shadow_power = Adw.SpinRow(title="Render Power")
        self.shadow_power.set_adjustment(Gtk.Adjustment(value=0, lower=0, upper=10, step_increment=1))
        val = utils.get_looknfeel_value(["decoration", "shadow", "render_power"])
        if val:
            try:
                self.shadow_power.set_value(float(val))
            except ValueError:
                pass
        self.shadow_power.connect("notify::value", self.on_shadow_power_changed)
        shadow_group.add(self.shadow_power)

        self.shadow_color = Adw.EntryRow(title="Color")
        val = utils.get_looknfeel_value(["decoration", "shadow", "color"])
        if val: self.shadow_color.set_text(val)
        self.shadow_color.connect("apply", self.on_shadow_color_changed)
        self.shadow_color.connect("entry-activated", self.on_shadow_color_changed)

        shadow_group.add(self.shadow_color)

    def on_gaps_in_changed(self, row, param):
        utils.set_looknfeel_value(["general", "gaps_in"], str(int(row.get_value())))

    def on_gaps_out_changed(self, row, param):
        utils.set_looknfeel_value(["general", "gaps_out"], str(int(row.get_value())))

    def on_rounding_changed(self, row, param):
        utils.set_looknfeel_value(["decoration", "rounding"], str(int(row.get_value())))

    def on_shadow_enabled_changed(self, row, param):
        utils.set_looknfeel_value(["decoration", "shadow", "enabled"], str(row.get_active()).lower())

    def on_shadow_range_changed(self, row, param):
        utils.set_looknfeel_value(["decoration", "shadow", "range"], str(int(row.get_value())))

    def on_shadow_power_changed(self, row, param):
        utils.set_looknfeel_value(["decoration", "shadow", "render_power"], str(int(row.get_value())))

    def on_shadow_color_changed(self, row):
        utils.set_looknfeel_value(["decoration", "shadow", "color"], row.get_text())
