from gi.repository import Adw, Gtk
from pages.general import GeneralPage
from pages.appearance import AppearancePage
from pages.keybindings import KeybindingsPage

class MainWindow(Adw.PreferencesWindow):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)

        self.set_title("Hyprland Preferences")
        self.set_default_size(800, 600)

        self.add(GeneralPage())
        self.add(AppearancePage())
        self.add(KeybindingsPage())
