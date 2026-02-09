import sys
import gi

gi.require_version('Gtk', '4.0')
gi.require_version('Adw', '1')

from gi.repository import Gtk, Adw
from window import MainWindow

class PreferencesApp(Adw.Application):
    def __init__(self):
        super().__init__(application_id='com.example.HyprlandPreferences',
                         flags=0)

    def do_activate(self):
        win = self.props.active_window
        if not win:
            win = MainWindow(application=self)
        win.present()

def main():
    app = PreferencesApp()
    return app.run(sys.argv)

if __name__ == '__main__':
    main()
