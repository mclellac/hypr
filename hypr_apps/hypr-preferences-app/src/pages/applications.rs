use crate::pages::PageSettings;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::ThemeStore;
use libadwaita as adw;
use std::cell::RefCell;

glib::wrapper! {
    pub struct ApplicationsPage(ObjectSubclass<imp::ApplicationsPage>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl PageSettings for ApplicationsPage {
    fn setup_ui(&self) {
        // Initial population and signal connection
        self.setup_all_apps();
        self.setup_browsers();
        self.refresh_settings();
    }

    fn refresh_settings(&self) {
        let imp = self.imp();
        *imp.is_refreshing.borrow_mut() = true;

        let store = ThemeStore::global();
        let theme = store.current_theme();
        let data = store
            .get_theme_data(&theme)
            .unwrap_or_else(|_| std::sync::Arc::new(hypr_config_manager::ThemeConfig::default()));

        // Refresh all bound rows
        let rows = imp.rows.borrow();
        for (key, widget) in rows.iter() {
            if let Some(row) = widget.downcast_ref::<adw::SpinRow>() {
                row.set_value(data.get_raw_f64(key, 0.0));
            } else if let Some(row) = widget.downcast_ref::<adw::SwitchRow>() {
                row.set_active(data.get_raw_bool(key, false));
            } else if let Some(row) = widget.downcast_ref::<adw::EntryRow>() {
                row.set_text(&data.get_raw_str(key, ""));
            } else if let Some(row) = widget.downcast_ref::<adw::ComboRow>() {
                let val = data.get_raw_str(key, "");
                if let Some(model) = row.model()
                    && let Some(string_list) = model.downcast_ref::<gtk::StringList>()
                {
                    for i in 0..string_list.n_items() {
                        if string_list.string(i).unwrap() == val {
                            row.set_selected(i);
                            break;
                        }
                    }
                }
            }
        }

        self.load_browsers_values();
        self.load_webapps_list();
        *imp.is_refreshing.borrow_mut() = false;
    }
}

impl ApplicationsPage {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn setup_browsers(&self) {
        let imp = self.imp();
        if let Some(combo) = imp.browser_combo.borrow().as_ref() {
            combo.connect_selected_item_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |combo| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let idx = combo.selected();
                    let desktops = obj.imp().browser_desktops.borrow();
                    if let Some(desktop) = desktops.get(idx as usize) {
                        let _ = std::process::Command::new("xdg-settings")
                            .env_remove("KDE_SESSION_VERSION")
                            .env_remove("KDE_FULL_SESSION")
                            .arg("set")
                            .arg("default-web-browser")
                            .arg(desktop)
                            .spawn();
                    }
                }
            ));
        }

        if let Some(btn) = imp.install_webapp_btn.borrow().as_ref() {
            btn.connect_clicked(|_| {
                let _ = std::process::Command::new("hypr-launch-webapp").spawn();
            });
        }
    }

    fn load_browsers_values(&self) {
        let imp = self.imp();

        // Find all available browsers
        let mut browsers = Vec::new();
        let app_dirs = ["/usr/share/applications", "/usr/local/share/applications"];

        for dir in app_dirs {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|ext| ext == "desktop")
                        && let Ok(content) = std::fs::read_to_string(&path)
                        && content.contains("Categories=")
                        && content.contains("WebBrowser")
                    {
                        let name = content
                            .lines()
                            .find(|l| l.starts_with("Name="))
                            .map(|l| l[5..].to_string())
                            .unwrap_or_else(|| entry.file_name().to_string_lossy().to_string());
                        browsers.push((name, entry.file_name().to_string_lossy().to_string()));
                    }
                }
            }
        }

        browsers.sort_by(|a, b| a.0.cmp(&b.0));

        let mut browser_names = Vec::new();
        let mut browser_desktops = Vec::new();
        for (name, desktop) in browsers {
            browser_names.push(name);
            browser_desktops.push(desktop);
        }

        let current_browser = std::process::Command::new("xdg-settings")
            .env_remove("KDE_SESSION_VERSION")
            .env_remove("KDE_FULL_SESSION")
            .arg("get")
            .arg("default-web-browser")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();

        if let Some(combo) = imp.browser_combo.borrow().as_ref() {
            let model =
                gtk::StringList::new(&browser_names.iter().map(|s| s.as_str()).collect::<Vec<_>>());
            combo.set_model(Some(&model));

            if let Some(idx) = browser_desktops.iter().position(|d| d == &current_browser) {
                combo.set_selected(idx as u32);
            }
        }

        *imp.browser_desktops.borrow_mut() = browser_desktops;
    }

    fn setup_all_apps(&self) {
        // Waybar
        let view = self.load_view(
            "waybar",
            include_str!("../../resources/pages/applications_waybar.ui"),
        );
        self.bind_row(&view, "waybar_position", "modules.waybar.position", "combo");
        self.bind_row(&view, "waybar_height", "modules.waybar.height", "spin");
        self.bind_row(
            &view,
            "waybar_margin_top",
            "modules.waybar.margin_top",
            "spin",
        );
        self.bind_row(
            &view,
            "waybar_margin_bottom",
            "modules.waybar.margin_bottom",
            "spin",
        );
        self.bind_row(
            &view,
            "waybar_margin_left",
            "modules.waybar.margin_left",
            "spin",
        );
        self.bind_row(
            &view,
            "waybar_margin_right",
            "modules.waybar.margin_right",
            "spin",
        );
        self.bind_row(&view, "waybar_spacing", "modules.waybar.spacing", "spin");
        self.bind_row(&view, "waybar_font_size", "waybar.style.size", "spin");

        let home = std::env::var("HOME").unwrap_or_default();
        self.setup_editor(
            &view,
            "waybar_editor",
            std::path::PathBuf::from(&home).join(".config/waybar/user.css"),
        );

        // Mako
        let view = self.load_view(
            "mako",
            include_str!("../../resources/pages/applications_mako.ui"),
        );
        self.bind_row(&view, "mako_font_size", "mako.font_size", "spin");

        // SwayOSD
        let view = self.load_view(
            "swayosd",
            include_str!("../../resources/pages/applications_swayosd.ui"),
        );
        self.bind_row(&view, "swayosd_font_size", "swayosd.font_size", "spin");

        // Walker
        let view = self.load_view(
            "walker",
            include_str!("../../resources/pages/applications_walker.ui"),
        );
        self.bind_row(&view, "walker_font_size", "walker.font_size", "spin");
        self.bind_row(&view, "walker_opacity", "walker.opacity", "spin");
        self.bind_row(
            &view,
            "walker_mod_clipboard",
            "walker.mod_clipboard",
            "switch",
        );
        self.bind_row(
            &view,
            "walker_mod_snippets",
            "walker.mod_snippets",
            "switch",
        );
        self.bind_row(&view, "walker_mod_symbols", "walker.mod_symbols", "switch");
        self.bind_row(&view, "walker_width", "walker.width", "spin");
        self.bind_row(&view, "walker_max_height", "walker.max_height", "spin");

        // Kitty
        let view = self.load_view(
            "kitty",
            include_str!("../../resources/pages/applications_kitty.ui"),
        );
        self.bind_row(&view, "kitty_font_spin", "kitty.font_size", "spin");
        self.bind_row(&view, "kitty_opacity_spin", "kitty.opacity", "spin");
        self.bind_row(&view, "kitty_padding_spin", "kitty.padding", "spin");
        self.bind_row(&view, "kitty_scrollback_spin", "kitty.scrollback", "spin");
        self.bind_row(&view, "kitty_cursor_shape", "kitty.cursor_shape", "combo");
        self.bind_row(
            &view,
            "kitty_cursor_trail_spin",
            "kitty.cursor_trail",
            "spin",
        );
        self.bind_row(
            &view,
            "kitty_cursor_trail_decay_entry",
            "kitty.cursor_trail_decay",
            "entry",
        );
        self.bind_row(
            &view,
            "kitty_cursor_blink_interval_spin",
            "kitty.cursor_blink_interval",
            "spin",
        );
        self.bind_row(
            &view,
            "kitty_confirm_close_switch",
            "kitty.confirm_close",
            "switch",
        );

        // Alacritty
        let view = self.load_view(
            "alacritty",
            include_str!("../../resources/pages/applications_alacritty.ui"),
        );
        self.bind_row(&view, "alacritty_font_size", "alacritty.font_size", "spin");
        self.bind_row(&view, "alacritty_opacity", "alacritty.opacity", "spin");
        self.bind_row(&view, "alacritty_padding_x", "alacritty.padding_x", "spin");
        self.bind_row(&view, "alacritty_padding_y", "alacritty.padding_y", "spin");
        self.bind_row(
            &view,
            "alacritty_cursor_shape",
            "alacritty.cursor_shape",
            "combo",
        );
        self.setup_editor(
            &view,
            "alacritty_editor",
            std::path::PathBuf::from(&home).join(".config/alacritty/user.toml"),
        );

        // Ghostty
        let view = self.load_view(
            "ghostty",
            include_str!("../../resources/pages/applications_ghostty.ui"),
        );
        self.bind_row(&view, "ghostty_font_size", "ghostty.font_size", "spin");
        self.bind_row(&view, "ghostty_opacity", "ghostty.opacity", "spin");
        self.bind_row(&view, "ghostty_padding", "ghostty.padding", "spin");
        self.bind_row(
            &view,
            "ghostty_cursor_shape",
            "ghostty.cursor_shape",
            "combo",
        );
        self.bind_row(
            &view,
            "ghostty_copy_on_select",
            "ghostty.copy_on_select",
            "combo",
        );

        // Voxtype
        let view = self.load_view(
            "voxtype",
            include_str!("../../resources/pages/applications_voxtype.ui"),
        );
        self.bind_row(&view, "voxtype_enabled", "voxtype.enabled", "switch");
        self.bind_row(&view, "voxtype_model", "voxtype.model", "combo");
        self.bind_row(&view, "voxtype_language", "voxtype.language", "entry");
        self.bind_row(&view, "voxtype_mode", "voxtype.mode", "combo");
        self.bind_row(
            &view,
            "voxtype_notify_start",
            "voxtype.notifications.recording_start",
            "switch",
        );
        self.bind_row(
            &view,
            "voxtype_notify_stop",
            "voxtype.notifications.recording_stop",
            "switch",
        );
        self.bind_row(
            &view,
            "voxtype_notify_text",
            "voxtype.notifications.transcription",
            "switch",
        );

        // Browsers
        let view = self.load_view(
            "browsers",
            include_str!("../../resources/pages/applications_browsers.ui"),
        );
        self.setup_editor(
            &view,
            "chromium_flags_editor",
            std::path::PathBuf::from(&home).join(".config/chromium-flags.conf"),
        );
        self.setup_editor(
            &view,
            "brave_flags_editor",
            std::path::PathBuf::from(&home).join(".config/brave-flags.conf"),
        );
    }

    fn load_view(&self, id: &str, ui_string: &str) -> gtk::Builder {
        let imp = self.imp();
        let builder = gtk::Builder::from_string(ui_string);
        if let Some(content) = builder.object::<gtk::Widget>("page_content") {
            content.unparent();

            let scrolled = gtk::ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::Never)
                .vexpand(true)
                .hexpand(true)
                .build();

            let clamp = adw::Clamp::builder()
                .maximum_size(680)
                .hexpand(true)
                .halign(gtk::Align::Fill)
                .margin_top(24)
                .margin_bottom(24)
                .margin_start(24)
                .margin_end(24)
                .build();

            clamp.set_child(Some(&content));
            scrolled.set_child(Some(&clamp));

            imp.content_stack.add_named(&scrolled, Some(id));
        }

        // Capture specific widgets for browsers page
        if id == "browsers" {
            if let Some(combo) = builder.object::<adw::ComboRow>("default_browser_combo") {
                *imp.browser_combo.borrow_mut() = Some(combo);
            }
            if let Some(btn) = builder.object::<gtk::Button>("install_webapp_btn") {
                *imp.install_webapp_btn.borrow_mut() = Some(btn);
            }
            if let Some(group) = builder.object::<adw::PreferencesGroup>("webapps_list_group") {
                *imp.webapps_group.borrow_mut() = Some(group);
            }
        }

        builder
    }

    fn bind_row(&self, builder: &gtk::Builder, id: &str, theme_key: &'static str, type_name: &str) {
        let imp = self.imp();
        match type_name {
            "spin" => {
                if let Some(row) = builder.object::<adw::SpinRow>(id) {
                    row.connect_value_notify(glib::clone!(
                        #[weak(rename_to = obj)]
                        self,
                        move |row| {
                            if *obj.imp().is_refreshing.borrow() {
                                return;
                            }
                            let val = row.value();
                            let val_int = val as i32;
                            if val == val_int as f64 {
                                ThemeStore::global().stage_update(
                                    theme_key,
                                    yaml_serde::Value::Number(val_int.into()),
                                );
                            } else {
                                ThemeStore::global()
                                    .stage_update(theme_key, yaml_serde::Value::from(val));
                            }
                        }
                    ));
                    imp.rows.borrow_mut().insert(theme_key, row.upcast());
                }
            }
            "switch" => {
                if let Some(row) = builder.object::<adw::SwitchRow>(id) {
                    row.connect_active_notify(glib::clone!(
                        #[weak(rename_to = obj)]
                        self,
                        move |row| {
                            if *obj.imp().is_refreshing.borrow() {
                                return;
                            }
                            ThemeStore::global()
                                .stage_update(theme_key, yaml_serde::Value::Bool(row.is_active()));
                        }
                    ));
                    imp.rows.borrow_mut().insert(theme_key, row.upcast());
                }
            }
            "combo" => {
                if let Some(row) = builder.object::<adw::ComboRow>(id) {
                    row.connect_selected_item_notify(glib::clone!(
                        #[weak(rename_to = obj)]
                        self,
                        move |row| {
                            if *obj.imp().is_refreshing.borrow() {
                                return;
                            }
                            if let Some(item) = row.selected_item()
                                && let Some(string_obj) = item.downcast_ref::<gtk::StringObject>()
                            {
                                ThemeStore::global().stage_update(
                                    theme_key,
                                    yaml_serde::Value::String(string_obj.string().to_string()),
                                );
                            }
                        }
                    ));
                    imp.rows.borrow_mut().insert(theme_key, row.upcast());
                }
            }
            "entry" => {
                if let Some(row) = builder.object::<adw::EntryRow>(id) {
                    row.connect_text_notify(glib::clone!(
                        #[weak(rename_to = obj)]
                        self,
                        move |row| {
                            if *obj.imp().is_refreshing.borrow() {
                                return;
                            }
                            ThemeStore::global().stage_update(
                                theme_key,
                                yaml_serde::Value::String(row.text().to_string()),
                            );
                        }
                    ));
                    imp.rows.borrow_mut().insert(theme_key, row.upcast());
                }
            }
            _ => {}
        }
    }

    fn load_webapps_list(&self) {
        let imp = self.imp();
        if let Some(group) = imp.webapps_group.borrow().as_ref() {
            // Clear existing webapp rows safely
            for row in imp.webapp_rows.borrow_mut().drain(..) {
                group.remove(&row);
            }

            let home = std::env::var("HOME").unwrap_or_default();
            let desktop_dir = std::path::PathBuf::from(home).join(".local/share/applications");
            if let Ok(entries) = std::fs::read_dir(desktop_dir) {
                let mut webapps = Vec::new();
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|ext| ext == "desktop")
                        && let Ok(content) = std::fs::read_to_string(&path)
                        && content.contains("hypr-launch-webapp")
                    {
                        // Extract name
                        let name = content
                            .lines()
                            .find(|l| l.starts_with("Name="))
                            .map(|l| l[5..].to_string())
                            .unwrap_or_else(|| entry.file_name().to_string_lossy().to_string());

                        // Extract url from Exec=hypr-launch-webapp <url>
                        let url = content
                            .lines()
                            .find(|l| l.starts_with("Exec="))
                            .map(|l| {
                                let exec_line = l[5..].trim();
                                if let Some(pos) = exec_line.find("hypr-launch-webapp") {
                                    exec_line[pos + 18..].trim().to_string()
                                } else {
                                    exec_line.to_string()
                                }
                            })
                            .unwrap_or_default();

                        let filename = entry.file_name().to_string_lossy().to_string();
                        webapps.push((name, url, filename));
                    }
                }

                webapps.sort_by(|a, b| a.0.cmp(&b.0));

                for (name, url, filename) in webapps {
                    let row = adw::ActionRow::builder()
                        .title(&name)
                        .subtitle(&url)
                        .build();

                    let delete_btn = gtk::Button::builder()
                        .icon_name("user-trash-symbolic")
                        .valign(gtk::Align::Center)
                        .css_classes(vec!["flat".to_string()])
                        .tooltip_text("Delete web app")
                        .build();

                    let filename_clone = filename.clone();
                    delete_btn.connect_clicked(glib::clone!(
                        #[weak(rename_to = obj)]
                        self,
                        move |_| {
                            let home = std::env::var("HOME").unwrap_or_default();
                            let desktop_file = std::path::PathBuf::from(&home)
                                .join(".local/share/applications")
                                .join(&filename_clone);
                            let icon_file = std::path::PathBuf::from(&home)
                                .join(".local/share/applications/icons")
                                .join(filename_clone.replace(".desktop", ".png"));
                            let _ = std::fs::remove_file(desktop_file);
                            let _ = std::fs::remove_file(icon_file);
                            obj.load_webapps_list();
                        }
                    ));

                    row.add_suffix(&delete_btn);
                    group.add(&row);
                    imp.webapp_rows.borrow_mut().push(row.upcast());
                }
            }
        }
    }

    fn setup_editor(&self, builder: &gtk::Builder, id: &str, file_path: std::path::PathBuf) {
        if let Some(editor) = builder.object::<gtk::TextView>(id) {
            let buffer = editor.buffer();
            if file_path.exists()
                && let Ok(content) = std::fs::read_to_string(&file_path)
            {
                buffer.set_text(&content);
            }

            buffer.connect_changed(|_| {
                ThemeStore::global()
                    .stage_update("__apps_changed__", yaml_serde::Value::Bool(true));
            });

            let buffer_weak = buffer.downgrade();
            let (sender, receiver) = std::sync::mpsc::channel::<hypr_config_manager::StoreEvent>();
            let obj_weak = self.downgrade();
            glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
                let _obj = match obj_weak.upgrade() {
                    Some(o) => o,
                    None => return glib::ControlFlow::Break,
                };
                while let Ok(event) = receiver.try_recv() {
                    if event == hypr_config_manager::StoreEvent::CommitRequested
                        && let Some(buf) = buffer_weak.upgrade()
                    {
                        let start = buf.start_iter();
                        let end = buf.end_iter();
                        let text = buf.text(&start, &end, false);
                        let _ = std::fs::write(&file_path, text);
                    }
                }
                glib::ControlFlow::Continue
            });
            ThemeStore::global().add_listener(move |event| {
                let _ = sender.send(event);
            });
        }
    }
}

pub mod imp {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct ApplicationsPage {
        pub split_view: adw::NavigationSplitView,
        pub sidebar_list: gtk::ListBox,
        pub content_stack: adw::ViewStack,
        pub is_refreshing: RefCell<bool>,
        pub rows: RefCell<HashMap<&'static str, gtk::Widget>>,
        pub browser_combo: RefCell<Option<adw::ComboRow>>,
        pub browser_desktops: RefCell<Vec<String>>,
        pub install_webapp_btn: RefCell<Option<gtk::Button>>,
        pub webapps_group: RefCell<Option<adw::PreferencesGroup>>,
        pub webapp_rows: RefCell<Vec<gtk::Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ApplicationsPage {
        const NAME: &'static str = "ApplicationsPage";
        type Type = super::ApplicationsPage;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for ApplicationsPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.set_orientation(gtk::Orientation::Vertical);
            obj.set_vexpand(true);
            obj.set_hexpand(true);

            self.split_view.set_vexpand(true);
            self.split_view.set_hexpand(true);

            // Sidebar Page
            self.sidebar_list
                .set_selection_mode(gtk::SelectionMode::Single);
            self.sidebar_list.add_css_class("navigation-sidebar");

            let stack_weak = self.content_stack.downgrade();
            self.sidebar_list.connect_row_selected(move |_, row| {
                if let Some(row) = row
                    && let Some(stack) = stack_weak.upgrade()
                {
                    let name = row.widget_name();
                    stack.set_visible_child_name(&name);
                }
            });

            let apps = [
                ("waybar", "Waybar"),
                ("browsers", "Browsers"),
                ("mako", "Mako"),
                ("swayosd", "SwayOSD"),
                ("walker", "Walker"),
                ("kitty", "Kitty"),
                ("alacritty", "Alacritty"),
                ("ghostty", "Ghostty"),
                ("voxtype", "Voxtype"),
            ];

            for (id, title) in apps.iter() {
                let row = adw::ActionRow::builder().title(*title).name(*id).build();
                self.sidebar_list.append(&row);
            }

            let sidebar_toolbar_view = adw::ToolbarView::new();
            let sidebar_header = adw::HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("Apps", ""))
                .build();
            sidebar_toolbar_view.add_top_bar(&sidebar_header);

            let sidebar_scroll = gtk::ScrolledWindow::builder()
                .child(&self.sidebar_list)
                .hscrollbar_policy(gtk::PolicyType::Never)
                .propagate_natural_width(true)
                .build();
            sidebar_toolbar_view.set_content(Some(&sidebar_scroll));

            let sidebar_page = adw::NavigationPage::builder()
                .title("Apps")
                .child(&sidebar_toolbar_view)
                .build();

            self.split_view.set_sidebar(Some(&sidebar_page));

            // Content Page
            let content_toolbar_view = adw::ToolbarView::new();
            // We don't need a header bar in the content page since it's inside the main window
            content_toolbar_view.set_content(Some(&self.content_stack));

            let content_page = adw::NavigationPage::builder()
                .title("Settings")
                .child(&content_toolbar_view)
                .build();

            self.split_view.set_content(Some(&content_page));

            obj.append(&self.split_view);
        }
    }

    impl WidgetImpl for ApplicationsPage {}
    impl BoxImpl for ApplicationsPage {}
}
