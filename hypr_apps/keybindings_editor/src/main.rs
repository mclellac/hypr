mod edit_dialog;
mod utils;
mod window;

use adw::prelude::*;
use libadwaita as adw;
use window::KeybindingsEditorWindow;

fn main() {
    let app = adw::Application::builder()
        .application_id("com.github.hypr_apps.keybindings_editor")
        .build();

    app.connect_activate(|app| {
        // Ensure we use AdwStyleManager and not GtkSettings
        let style_manager = adw::StyleManager::default();
        let store = hypr_config_manager::ThemeStore::global();
        let theme = store.current_theme();
        if let Ok(config) = store.get_theme_data(&theme) {
            let scheme = match config.gtk.color_scheme.as_str() {
                "prefer-light" => adw::ColorScheme::PreferLight,
                "prefer-dark" => adw::ColorScheme::PreferDark,
                _ => adw::ColorScheme::Default,
            };
            style_manager.set_color_scheme(scheme);
        } else {
            style_manager.set_color_scheme(adw::ColorScheme::Default);
        }

        let window = KeybindingsEditorWindow::new(app);
        window.setup_ui();
        window.present();
    });

    app.run();
}
