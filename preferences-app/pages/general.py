"""
General settings page for the Hyprland Preferences Application.
"""

import sys
import os

# pylint: disable=wrong-import-position
from gi.repository import Adw, Gtk

# Adjust path to find utils
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
import utils

class GeneralPage(Adw.PreferencesPage):
    """Page for configuring general system settings like Mod key."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.set_title("General")
        self.set_icon_name("preferences-system-symbolic")

        group = Adw.PreferencesGroup(title="System")
        self.add(group)

        # Mod Key
        self.mod_row = Adw.ComboRow(title="Mod Key")
        self.mod_row.set_tooltip_text("Select the main modifier key for Hyprland bindings.")
        self.mod_row.set_model(Gtk.StringList.new(["SUPER", "ALT"]))

        current_mod = utils.get_main_mod()
        if current_mod == "ALT":
            self.mod_row.set_selected(1)
        else:
            self.mod_row.set_selected(0)

        self.mod_row.connect("notify::selected-item", self.on_mod_changed)
        group.add(self.mod_row)

        # Layout Group
        layout_group = Adw.PreferencesGroup(title="Layout")
        self.add(layout_group)

        # Pseudotile
        self.pseudotile = Adw.SwitchRow(title="Pseudotile")
        self.pseudotile.set_subtitle("Enable pseudotiling for dwindle layout")
        val = utils.get_looknfeel_value(["dwindle", "pseudotile"])
        if val:
            self.pseudotile.set_active(val.lower() == "true")
        self.pseudotile.connect("notify::active", self.on_pseudotile_changed)
        layout_group.add(self.pseudotile)

        # Preserve Split
        self.preserve_split = Adw.SwitchRow(title="Preserve Split")
        self.preserve_split.set_subtitle("Preserve split direction in dwindle layout")
        val = utils.get_looknfeel_value(["dwindle", "preserve_split"])
        if val:
            self.preserve_split.set_active(val.lower() == "true")
        self.preserve_split.connect("notify::active", self.on_preserve_split_changed)
        layout_group.add(self.preserve_split)

    def on_mod_changed(self, row, _):
        """Callback for Mod key changes."""
        item = row.get_selected_item()
        if item:
            selected = item.get_string()
            utils.set_main_mod(selected)

            win = self.get_native()
            if win:
                win.add_toast(Adw.Toast.new(f"Set Mod Key to {selected}"))

    def on_pseudotile_changed(self, row, _):
        """Callback for pseudotile toggle."""
        val = str(row.get_active()).lower()
        utils.set_looknfeel_value(["dwindle", "pseudotile"], val)
        status = "enabled" if row.get_active() else "disabled"
        win = self.get_native()
        if win:
            win.add_toast(Adw.Toast.new(f"Pseudotile {status}"))

    def on_preserve_split_changed(self, row, _):
        """Callback for preserve_split toggle."""
        val = str(row.get_active()).lower()
        utils.set_looknfeel_value(["dwindle", "preserve_split"], val)
        status = "enabled" if row.get_active() else "disabled"
        win = self.get_native()
        if win:
            win.add_toast(Adw.Toast.new(f"Preserve Split {status}"))
