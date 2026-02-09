"""
Main window implementation for the Hyprland Preferences Application.
"""

from gi.repository import Adw
from pages.general import GeneralPage
from pages.appearance import AppearancePage
from pages.keybindings import KeybindingsPage
from pages.about import AboutPage

class MainWindow(Adw.PreferencesWindow):
    """The main window containing preference pages."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)

        self.set_title("Hyprland Preferences")
        self.set_default_size(800, 600)

        self.add(GeneralPage())
        self.add(AppearancePage())
        self.add(KeybindingsPage())
        self.add(AboutPage())
