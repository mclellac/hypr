from gi.repository import Adw, Gtk, GObject
import sys
import os

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
import utils

class KeybindingsPage(Adw.PreferencesPage):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.set_title("Keybindings")
        self.set_icon_name("input-keyboard-symbolic")

        self.main_mod = utils.get_main_mod()

        # Initialize group
        self.group = Adw.PreferencesGroup(title="Defined Keybindings")
        self.add(self.group)

        self.refresh_bindings()

    def refresh_bindings(self):
        self.remove(self.group)
        self.group = Adw.PreferencesGroup(title="Defined Keybindings")
        self.add(self.group)

        bindings = utils.get_keybindings()

        for b in bindings:
            title = b['desc'] if b['desc'] else b['dispatcher']
            row = Adw.ActionRow(title=title)
            subtitle = f"{b['dispatcher']} {b['arg']}"
            row.set_subtitle(subtitle)

            # Form accelerator
            mods = b['mods'].replace("$mainMod", self.main_mod)
            key = b['key']

            accel = ""
            if "SUPER" in mods: accel += "<Super>"
            if "ALT" in mods: accel += "<Alt>"
            if "CTRL" in mods: accel += "<Ctrl>"
            if "SHIFT" in mods: accel += "<Shift>"

            if "code:" in key:
                accel += key
            else:
                accel += key.upper()

            shortcut = Gtk.ShortcutLabel(accelerator=accel)
            row.add_suffix(shortcut)

            row.set_activatable(True)
            # Connect using closure to capture binding
            row.connect("activated", self.on_row_activated, b)

            self.group.add(row)

    def on_row_activated(self, row, binding):
        dialog = EditBindingDialog(self.get_root(), binding, self.main_mod)
        dialog.connect("response", self.on_dialog_response)
        dialog.present()

    def on_dialog_response(self, dialog, response):
        if response == "save":
            new_mods = dialog.mods_entry.get_text()
            new_key = dialog.key_entry.get_text()

            success = utils.update_keybinding(dialog.binding['file'], dialog.binding['line'], new_mods, new_key)
            if success:
                # Update local binding object so we don't need full re-read?
                # Better to re-read to confirm file state.
                self.refresh_bindings()
            else:
                print("Failed to update binding")
        dialog.close()

class EditBindingDialog(Adw.MessageDialog):
    def __init__(self, parent, binding, main_mod):
        super().__init__(heading="Edit Keybinding", body=f"Edit binding for {binding['desc'] or binding['dispatcher']}")

        # In Adw 1.5+, use set_transient_for if available, or just pass parent to init?
        # Adw.MessageDialog usually takes no parent in init but has set_transient_for.
        self.set_transient_for(parent)

        self.add_response("cancel", "Cancel")
        self.add_response("save", "Save")
        self.set_response_appearance("save", Adw.ResponseAppearance.SUGGESTED)

        self.binding = binding

        content = Adw.PreferencesGroup()

        self.mods_entry = Adw.EntryRow(title="Modifiers")
        self.mods_entry.set_text(binding['mods'])
        content.add(self.mods_entry)

        self.key_entry = Adw.EntryRow(title="Key")
        self.key_entry.set_text(binding['key'])
        content.add(self.key_entry)

        self.set_extra_child(content)
