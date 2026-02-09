from gi.repository import Adw, Gtk, GObject
import sys
import os

# Adjust path to find utils
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
import utils

class GeneralPage(Adw.PreferencesPage):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.set_title("General")
        self.set_icon_name("preferences-system-symbolic")

        group = Adw.PreferencesGroup(title="System")
        self.add(group)

        # Mod Key
        self.mod_row = Adw.ComboRow(title="Mod Key")
        self.mod_row.set_model(Gtk.StringList.new(["SUPER", "ALT"]))

        current_mod = utils.get_main_mod()
        if current_mod == "ALT":
            self.mod_row.set_selected(1)
        else:
            self.mod_row.set_selected(0)

        self.mod_row.connect("notify::selected-item", self.on_mod_changed)
        group.add(self.mod_row)

        # Font
        self.font_row = Adw.ComboRow(title="System Font (Monospace)")
        fonts = utils.list_fonts()
        if not fonts:
            fonts = ["No fonts found"]

        self.font_list = Gtk.StringList.new(fonts)
        self.font_row.set_model(self.font_list)

        # We don't easily know current font without parsing, so default to first in list or index 0
        # If user changes it, it updates.

        self.font_row.connect("notify::selected-item", self.on_font_changed)
        group.add(self.font_row)

    def on_mod_changed(self, row, param):
        item = row.get_selected_item()
        if item:
            selected = item.get_string()
            utils.set_main_mod(selected)
            print(f"Set Mod Key to {selected}")

    def on_font_changed(self, row, param):
        item = row.get_selected_item()
        if item:
            selected = item.get_string()
            if selected != "No fonts found":
                # This might freeze UI slightly as it runs subprocess.
                # Ideally run in thread, but for simplicity we run blocking.
                success = utils.set_font(selected)
                if success:
                    print(f"Set Font to {selected}")
                else:
                    print(f"Failed to set font to {selected}")
