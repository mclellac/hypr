"""
Main window implementation for the Hyprland Preferences Application.
"""

# pylint: disable=import-error
from gi.repository import Adw, Gtk
from pages.general import GeneralPage
from pages.appearance import AppearancePage
from pages.keybindings import KeybindingsPage
from pages.input import InputPage


class MainWindow(Adw.PreferencesWindow):
    """The main window containing preference pages."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)

        self.set_title("Hyprland Preferences")
        self.set_default_size(800, 600)

        self.add(GeneralPage())
        self.add(AppearancePage())
        self.add(InputPage())
        self.add(KeybindingsPage())

        # Attempt to inject the primary menu button into the header bar
        self._setup_menu_button()

    def _setup_menu_button(self):
        """Finds the AdwHeaderBar and adds a primary menu button."""
        header_bar = self._find_header_bar(self)
        if header_bar:
            app = self.get_application()
            if app:
                menu_model = app.get_menubar()
                if menu_model:
                    btn = Gtk.MenuButton()
                    btn.set_icon_name("open-menu-symbolic")
                    btn.set_menu_model(menu_model)
                    btn.set_tooltip_text("Main Menu")
                    header_bar.pack_end(btn)

    def _find_header_bar(self, widget):
        """Recursively finds an AdwHeaderBar in the widget hierarchy."""
        if isinstance(widget, Adw.HeaderBar):
            return widget

        child = widget.get_first_child()
        while child:
            res = self._find_header_bar(child)
            if res:
                return res
            child = child.get_next_sibling()
        return None
