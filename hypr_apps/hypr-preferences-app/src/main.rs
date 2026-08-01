mod color_utils;
mod pages;
mod window;

use adw::gio;
use adw::prelude::*;
use libadwaita as adw;
use std::env;
use window::MainWindow;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_debug = args.contains(&"--debug".to_string());
    let filtered_args: Vec<String> = args.into_iter().filter(|a| a != "--debug").collect();

    let app = adw::Application::builder()
        .application_id("com.github.mclellac.hypr-preferences-app")
        .build();

    app.connect_startup(move |app| {
        // Use AdwStyleManager for dark/light theme - this is the modern, non-deprecated way
        let manager = adw::StyleManager::default();
        let store = hypr_config_manager::ThemeStore::global();
        let theme = store.current_theme();
        if let Ok(config) = store.get_theme_data(&theme) {
            let scheme = match config.gtk.color_scheme.as_str() {
                "prefer-light" => adw::ColorScheme::PreferLight,
                "prefer-dark" => adw::ColorScheme::PreferDark,
                _ => adw::ColorScheme::Default,
            };
            manager.set_color_scheme(scheme);
        } else {
            manager.set_color_scheme(adw::ColorScheme::PreferDark);
        }

        // Define Actions
        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(glib::clone!(
            #[weak]
            app,
            move |_, _| {
                if let Some(win) = app.active_window() {
                    let dialog = adw::AboutDialog::builder()
                        .application_name("Hypr Preferences App")
                        .developer_name("xorcsm")
                        .version("2.0.0")
                        .website("https://github.com/mclellac/hypr")
                        .issue_url("https://github.com/mclellac/hypr/issues")
                        .copyright("MIT")
                        .license_type(adw::gtk::License::MitX11)
                        .build();
                    dialog.present(Some(&win));
                }
            }
        ));
        app.add_action(&about_action);

        let help_action = gio::SimpleAction::new("help", None);
        help_action.connect_activate(|_, _| {
            let _ = std::process::Command::new("xdg-open")
                .arg("https://github.com/mclellac/hypr")
                .spawn();
        });
        app.add_action(&help_action);

        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(glib::clone!(
            #[weak]
            app,
            move |_, _| {
                app.quit();
            }
        ));
        app.add_action(&quit_action);

        // Set up Menu
        let menu = gio::Menu::new();
        menu.append(Some("Help"), Some("app.help"));
        menu.append(Some("About Hypr Preferences"), Some("app.about"));
        menu.append(Some("Quit"), Some("app.quit"));
        app.set_menubar(Some(&menu));

        if is_debug {
            println!("[Main] Startup complete. Debug mode active.");
        }
    });

    app.connect_activate(move |app| {
        let window = MainWindow::new(app);
        window.setup_ui();
        window.present();
    });

    app.run_with_args(&filtered_args);
}
