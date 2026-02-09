"""
About page logic for the Hyprland Preferences Application.
"""

from gi.repository import Adw, Gtk


def show_about_dialog(parent):
    """Opens the About dialog."""
    dialog = Adw.AboutDialog(
        application_name="Hypr Preferences",
        developer_name="xorcsm",
        version="1.0.0",
        copyright="© 2026 xorcsm",
        license_type=Gtk.License.MIT_X11,
        website="https://github.com/mclellac/hyprl",
        issue_url="https://github.com/mclellac/hypr/issues",
    )
    dialog.present(parent)
