"""
Main entry point for the Hyprland Preferences Application.
"""

import sys
import gi

gi.require_version('Gtk', '4.0')
gi.require_version('Adw', '1')

from gi.repository import Adw
from window import MainWindow

class PreferencesApp(Adw.Application):
    """The main application class."""

    def __init__(self):
        super().__init__(application_id='com.example.HyprlandPreferences', flags=0)

    def do_activate(self):
        """Activates the application window."""
        win = self.props.active_window
        if not win:
            win = MainWindow(application=self)
        win.present()

def main():
    """Runs the application."""
    app = PreferencesApp()
    return app.run(sys.argv)

if __name__ == '__main__':
    sys.exit(main())
