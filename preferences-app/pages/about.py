"""
About page for the Hyprland Preferences Application.
"""

from gi.repository import Adw, Gtk, Gio, GLib

class AboutPage(Adw.PreferencesPage):
    """Page for displaying application information and help."""

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.set_title("About")
        self.set_icon_name("help-about-symbolic")

        self.add(self._init_info_group())

    def _init_info_group(self):
        """Initializes the Information group."""
        group = Adw.PreferencesGroup(title="Information")

        # Help Row
        help_row = Adw.ActionRow(title="Help")
        help_row.set_subtitle("Visit the project documentation")
        help_row.set_icon_name("help-browser-symbolic")
        help_row.set_activatable(True)
        help_row.connect("activated", self.on_help_activated)
        group.add(help_row)

        # About Row
        about_row = Adw.ActionRow(title="About Hyprland Preferences")
        about_row.set_subtitle("View version and license information")
        about_row.set_icon_name("help-about-symbolic")
        about_row.set_activatable(True)
        about_row.connect("activated", self.on_about_activated)
        group.add(about_row)

        return group

    def on_help_activated(self, _row):
        """Opens the help documentation."""
        # Open URL using Gtk.show_uri or simpler method if available
        # Gtk.show_uri(parent, uri, timestamp)
        # Using a dummy URL for now, or maybe the repo URL if known
        url = "https://github.com/hyprland/hyprland" # Placeholder
        try:
            # Gio.AppInfo.launch_default_for_uri(url, None) is standard
            Gio.AppInfo.launch_default_for_uri(url, None)
        except GLib.Error as e:
            print(f"Failed to open help: {e}")
            win = self.get_native()
            if win:
                win.add_toast(Adw.Toast.new(f"Failed to open help: {e}"))

    def on_about_activated(self, _row):
        """Opens the About dialog."""
        dialog = Adw.AboutDialog(
            application_name="Hyprland Preferences",
            developer_name="Jules",
            version="1.0.0",
            copyright="© 2024 Jules",
            license_type=Gtk.License.MIT_X11,
            website="https://github.com/hyprland/hyprland",
            issue_url="https://github.com/hyprland/hyprland/issues"
        )
        dialog.present(self.get_native())
