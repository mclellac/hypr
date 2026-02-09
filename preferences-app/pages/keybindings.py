from gi.repository import Adw, Gtk, GObject, GLib
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
            # Escape text for XML markup safety (title/subtitle use Pango markup)
            raw_title = b['desc'] if b['desc'] else b['dispatcher']
            safe_title = GLib.markup_escape_text(raw_title)

            raw_subtitle = f"{b['dispatcher']} {b['arg']}"
            safe_subtitle = GLib.markup_escape_text(raw_subtitle)

            row = Adw.ActionRow(title=safe_title)
            row.set_subtitle(safe_subtitle)

            # Form accelerator
            mods = b['mods'].replace("$mainMod", self.main_mod)
            key = b['key']

            accel = self.format_accelerator(mods, key)

            if accel:
                shortcut = Gtk.ShortcutLabel(accelerator=accel)
                row.add_suffix(shortcut)
            else:
                # Fallback: Just display as text if parsing fails or is complex
                fallback_text = f"{mods} + {key}"
                label = Gtk.Label(label=fallback_text)
                label.add_css_class("dim-label")
                row.add_suffix(label)

            row.set_activatable(True)
            # Connect using closure to capture binding
            row.connect("activated", self.on_row_activated, b)

            self.group.add(row)

    def format_accelerator(self, mods, key):
        """Formats Hyprland mod+key string into Gtk Accelerator string."""
        accel = ""

        # Gtk accelerators use <Control>, <Shift>, <Alt>, <Super>
        # Check mods string - Hyprland uses "SUPER", "ALT", "CTRL", "SHIFT"
        # We need to map them.

        # Avoid double adding if mods are repeated or combined strangely
        has_super = "SUPER" in mods or "$mainMod" in mods # mainMod is usually SUPER or ALT, handled by caller replacement?
        # Caller replaced $mainMod with actual value (e.g. SUPER or ALT) before calling this function.

        # Ensure order is consistent and handle potential overlaps if mods string is messy
        # But simple `in` checks are fine if mods string is canonical.

        # NOTE: Hyprland config uses "SUPER", "ALT", "CTRL", "SHIFT".

        if "SUPER" in mods: accel += "<Super>"
        if "ALT" in mods: accel += "<Alt>"
        if "CTRL" in mods: accel += "<Control>"
        if "SHIFT" in mods: accel += "<Shift>"

        k = key.strip()

        # Special keys handling
        if k.startswith("code:") or k.startswith("mouse:"):
            return None

        # Common replacements for GTK accelerator compatibility
        key_map = {
            "COMMA": "comma",
            "PERIOD": "period",
            "SLASH": "slash",
            "BACKSLASH": "backslash",
            "MINUS": "minus",
            "EQUAL": "equal",
            "SPACE": "space",
            "TAB": "Tab",
            "RETURN": "Return",
            "ENTER": "Return",
            "ESCAPE": "Escape",
            "BACKSPACE": "BackSpace",
            "DELETE": "Delete",
            "HOME": "Home",
            "END": "End",
            "PAGE_UP": "Page_Up",
            "PAGE_DOWN": "Page_Down",
            "LEFT": "Left",
            "RIGHT": "Right",
            "UP": "Up",
            "DOWN": "Down",
            "PRINT": "Print",
            "PAUSE": "Pause",
            "INSERT": "Insert",
        }

        # Check XF86 keys - GTK might support some if mapped correctly, but often safer to fallback
        if k.startswith("XF86"):
             # Try mapping common media keys if possible, or return None to use text label
             # GDK_KEY_AudioRaiseVolume exist, but string representation might be specific
             return None

        if k in key_map:
            k = key_map[k]
        elif len(k) == 1:
            k = k.lower()

        accel += k
        return accel

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
        # Escape body text too
        desc = binding['desc'] or binding['dispatcher']
        safe_desc = GLib.markup_escape_text(desc)

        super().__init__(heading="Edit Keybinding", body=f"Edit binding for {safe_desc}")

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
