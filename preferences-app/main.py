"""
Main entry point for the Hyprland Preferences Application.
"""

import sys
import gi

gi.require_version('Gtk', '4.0')
gi.require_version('Adw', '1')

# pylint: disable=wrong-import-position
from gi.repository import Adw, Gio, GLib
from window import MainWindow
from pages.about import show_about_dialog


class PreferencesApp(Adw.Application):
    """The main application class."""

    def __init__(self):
        super().__init__(application_id='com.example.HyprlandPreferences', flags=0)

    def do_startup(self):
        """Initializes the application actions and menu."""
        super().do_startup()

        # Define actions
        about_action = Gio.SimpleAction.new("about", None)
        about_action.connect("activate", self.on_about)
        self.add_action(about_action)

        help_action = Gio.SimpleAction.new("help", None)
        help_action.connect("activate", self.on_help)
        self.add_action(help_action)

        quit_action = Gio.SimpleAction.new("quit", None)
        quit_action.connect("activate", self.on_quit)
        self.add_action(quit_action)

        # Set up menu
        menu = Gio.Menu()
        menu.append("Help", "app.help")
        menu.append("About Hypr Preferences", "app.about")

        # We need to expose this menu so the window can use it.
        # Since Adw.PreferencesWindow doesn't pick it up automatically from self.set_menubar() usually
        # (unless on macOS or GNOME Shell top bar), we will store it or set it as menubar anyway.
        self.set_menubar(menu)

    def do_activate(self):
        """Activates the application window."""
        win = self.props.active_window
        if not win:
            win = MainWindow(application=self)
        win.present()

    def on_about(self, _action, _param):
        """Shows the about dialog."""
        win = self.props.active_window
        if win:
            show_about_dialog(win)

    def on_help(self, _action, _param):
        """Opens the help URL."""
        url = "https://github.com/mclellac/hypr"
        try:
            Gio.AppInfo.launch_default_for_uri(url, None)
        except GLib.Error as e:
            print(f"Failed to open help: {e}")
            win = self.props.active_window
            if win:
                win.add_toast(Adw.Toast.new(f"Failed to open help: {e}"))

    def on_quit(self, _action, _param):
        """Quits the application."""
        self.quit()


def main():
    """Runs the application."""
    app = PreferencesApp()
    return app.run(sys.argv)


if __name__ == '__main__':
    sys.exit(main())
