"""
Keybindings settings page for the Hyprland Preferences Application.
"""

import os
import sys

from gi.repository import Adw, GLib, Gtk

# Adjust path to find utils
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
import utils


# Mapping Hyprland key names to GTK key names
KEY_MAP = {
    "SPACE": "space",
    "RETURN": "Return",
    "ENTER": "Return",
    "ESCAPE": "Escape",
    "ESC": "Escape",
    "TAB": "Tab",
    "BACKSPACE": "BackSpace",
    "DELETE": "Delete",
    "DEL": "Delete",
    "HOME": "Home",
    "END": "End",
    "PAGE_UP": "Page_Up",
    "PAGE_DOWN": "Page_Down",
    "LEFT": "Left",
    "RIGHT": "Right",
    "UP": "Up",
    "DOWN": "Down",
    "COMMA": "comma",
    "PERIOD": "period",
    "DOT": "period",
    "SLASH": "slash",
    "MINUS": "minus",
    "EQUAL": "equal",
    "BRACKETLEFT": "bracketleft",
    "BRACKETRIGHT": "bracketright",
    "BACKSLASH": "backslash",
    "GRAVE": "grave",
    "PRINT": "Print",
    "PAUSE": "Pause",
    "INSERT": "Insert",
    "MENU": "Menu",
    "NUM_LOCK": "Num_Lock",
    "CAPS_LOCK": "Caps_Lock",
    "SCROLL_LOCK": "Scroll_Lock",
}

# Common XF86 keys
KNOWN_XF86 = {
    "XF86AUDIORAISEVOLUME": "XF86AudioRaiseVolume",
    "XF86AUDIOLOWERVOLUME": "XF86AudioLowerVolume",
    "XF86AUDIOMUTE": "XF86AudioMute",
    "XF86AUDIOMICMUTE": "XF86AudioMicMute",
    "XF86MONBRIGHTNESSUP": "XF86MonBrightnessUp",
    "XF86MONBRIGHTNESSDOWN": "XF86MonBrightnessDown",
    "XF86AUDIONEXT": "XF86AudioNext",
    "XF86AUDIOPREV": "XF86AudioPrev",
    "XF86AUDIOPLAY": "XF86AudioPlay",
    "XF86AUDIOPAUSE": "XF86AudioPause",
    "XF86AUDIOSTOP": "XF86AudioStop",
    "XF86POWEROFF": "XF86PowerOff",
    "XF86CALCULATOR": "XF86Calculator",
    "XF86MAIL": "XF86Mail",
    "XF86HOMEPAGE": "XF86HomePage",
    "XF86SEARCH": "XF86Search",
    "XF86FAVORITES": "XF86Favorites",
}


def get_gtk_accelerator(mods, key):
    """
    Converts Hyprland modifier and key to a GTK accelerator string.
    Returns (accelerator_string, display_label).
    If accelerator_string is None, it means it cannot be a Gtk.ShortcutLabel accelerator.
    In that case, display_label should be used for a Gtk.Label.
    """

    # Clean up modifiers
    # Hyprland mods are like "SUPER SHIFT" or "SUPER, SHIFT"
    # GTK mods: <Super>, <Alt>, <Ctrl>, <Shift>

    accel = ""
    mod_list = mods.upper().replace(',', ' ').split()

    if "SUPER" in mod_list:
        accel += "<Super>"
    if "ALT" in mod_list:
        accel += "<Alt>"
    if "CTRL" in mod_list:
        accel += "<Ctrl>"
    if "CONTROL" in mod_list:
        accel += "<Ctrl>"
    if "SHIFT" in mod_list:
        accel += "<Shift>"

    k = key
    key_lower = k.lower()

    # Handle special keys that are not valid accelerators
    if key_lower.startswith(("code:", "mouse:", "mouse_")):
        # Not a valid accelerator
        return None, key

    key_upper = k.upper()

    if key_upper in KEY_MAP:
        accel += KEY_MAP[key_upper]
    elif key_upper.startswith("XF86"):
        if key_upper in KNOWN_XF86:
            accel += KNOWN_XF86[key_upper]
        else:
            accel += k
    elif len(k) == 1:
        if k.isalpha():
            accel += k.lower()
        else:
            accel += k
    else:
        accel += k

    return accel, None


class KeybindingsPage(Adw.PreferencesPage):
    """Page for viewing and editing keybindings."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.set_title("Keybindings")
        self.set_icon_name("input-keyboard-symbolic")

        self.main_mod = utils.get_main_mod()
        self.group = None
        self.refresh_bindings()

    def refresh_bindings(self):
        """Refreshes the list of keybindings."""
        if self.group:
            self.remove(self.group)

        self.group = Adw.PreferencesGroup(title="Defined Keybindings")
        self.add(self.group)

        bindings = utils.get_keybindings()

        for b in bindings:
            row = self._create_binding_row(b)
            self.group.add(row)

    def _create_binding_row(self, b):
        # Escape title for markup safety
        raw_title = b['desc'] if b['desc'] else b['dispatcher']
        title = GLib.markup_escape_text(raw_title)

        row = Adw.ActionRow(title=title)
        row.set_tooltip_text("Click to edit this keybinding.")

        # Escape subtitle
        raw_subtitle = f"{b['dispatcher']} {b['arg']}"
        subtitle = GLib.markup_escape_text(raw_subtitle)
        row.set_subtitle(subtitle)

        # Form accelerator
        mods = b['mods'].replace("$mainMod", self.main_mod)
        key = b['key']

        accel_str, label_text = get_gtk_accelerator(mods, key)

        if accel_str:
            shortcut = Gtk.ShortcutLabel(accelerator=accel_str)
            row.add_suffix(shortcut)
        else:
            # Fallback for special keys
            lbl = Gtk.Label(label=label_text)
            lbl.add_css_class("dim-label")
            row.add_suffix(lbl)

        row.set_activatable(True)
        # Connect using closure to capture binding
        row.connect("activated", self.on_row_activated, b)
        return row

    def on_row_activated(self, _row, binding):
        """Callback for when a keybinding row is activated."""
        dialog = EditBindingDialog(self.get_root(), binding, self.main_mod)
        dialog.connect("response", self.on_dialog_response)
        dialog.present()

    def on_dialog_response(self, dialog, response):
        """Callback for dialog response."""
        win = self.get_native()
        if response == "save":
            new_mods = dialog.mods_entry.get_text()
            new_key = dialog.key_entry.get_text()

            success = utils.update_keybinding(
                dialog.binding['file'], dialog.binding['line'], new_mods, new_key
            )
            if success:
                self.refresh_bindings()
                if win:
                    win.add_toast(Adw.Toast.new("Keybinding updated successfully"))
            else:
                if win:
                    win.add_toast(Adw.Toast.new("Failed to update binding"))
        dialog.close()


class EditBindingDialog(Adw.MessageDialog):
    """Dialog for editing a keybinding."""

    def __init__(self, parent, binding, _main_mod):
        heading_text = f"Edit binding for {binding['desc'] or binding['dispatcher']}"
        super().__init__(heading=heading_text, body="")

        if parent:
            self.set_transient_for(parent)

        self.add_response("cancel", "Cancel")
        self.add_response("save", "Save")
        self.set_response_appearance("save", Adw.ResponseAppearance.SUGGESTED)

        self.binding = binding

        content = Adw.PreferencesGroup()

        self.mods_entry = Adw.EntryRow(title="Modifiers")
        self.mods_entry.set_text(binding['mods'])
        self.mods_entry.set_tooltip_text(
            "Modifiers (e.g. SUPER, SHIFT, CTRL, ALT). Separate with space or comma."
        )
        content.add(self.mods_entry)

        self.key_entry = Adw.EntryRow(title="Key")
        self.key_entry.set_text(binding['key'])
        self.key_entry.set_tooltip_text("Key (e.g. A, SPACE, RETURN, code:10).")
        content.add(self.key_entry)

        self.set_extra_child(content)
