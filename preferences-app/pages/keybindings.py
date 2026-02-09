from gi.repository import Adw, Gtk, GObject, GLib
import sys
import os

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
import utils

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

    if "SUPER" in mod_list: accel += "<Super>"
    if "ALT" in mod_list: accel += "<Alt>"
    if "CTRL" in mod_list: accel += "<Ctrl>"
    if "CONTROL" in mod_list: accel += "<Ctrl>"
    if "SHIFT" in mod_list: accel += "<Shift>"

    # Handle special keys that are not valid accelerators
    if key.lower().startswith("code:") or key.lower().startswith("mouse:") or key.lower().startswith("mouse_"):
        # Not a valid accelerator
        return None, key

    # Mapping Hyprland key names to GTK key names
    # Hyprland often uses UPPERCASE or specific names.
    key_map = {
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
        # F-keys are usually F1, F2... which are valid
    }

    k = key

    # Check map first
    if k.upper() in key_map:
        accel += key_map[k.upper()]
    elif k.upper().startswith("XF86"):
        # Hyprland XF86 keys might be mixed case or upper case.
        # GTK usually accepts the standard X11 names which are typically PascalCase-ish.
        # e.g. XF86AudioRaiseVolume.
        # If we receive XF86AUDIORAISEVOLUME, we might need to fix it.
        # Since there are many, let's try to map known ones or just pass it through if it looks okay.

        known_xf86 = {
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

        if k.upper() in known_xf86:
            accel += known_xf86[k.upper()]
        else:
            # Pass as is, hope for best
            accel += k

    elif len(k) == 1:
        # Single char.
        # If we have modifiers like Ctrl/Alt, usually lowercase 'a' is expected for <Ctrl>a.
        # But if Shift is present, it might be implicit?
        # <Shift>a is Shift+A. <Shift>A is also Shift+A.
        # <Ctrl>A is Ctrl+Shift+A usually in GTK parsing logic?
        # Let's stick to lowercase for single letters to be safe for accelerators.
        if k.isalpha():
             accel += k.lower()
        else:
             accel += k
    else:
        # Unknown key, maybe Title case it?
        # e.g. "Return" -> "Return".
        # If "RETURN" came in and wasn't in map (it is), we'd be here.
        # Try passing as is.
        accel += k

    return accel, None

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
            # Escape title for markup safety
            raw_title = b['desc'] if b['desc'] else b['dispatcher']
            title = GLib.markup_escape_text(raw_title)

            row = Adw.ActionRow(title=title)

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
        heading_text = f"Edit binding for {binding['desc'] or binding['dispatcher']}"
        super().__init__(heading=heading_text, body="")

        # In Adw 1.5+, use set_transient_for if available, or just pass parent to init?
        # Adw.MessageDialog usually takes no parent in init but has set_transient_for.
        if parent:
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
