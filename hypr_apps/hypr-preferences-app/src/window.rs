use crate::pages::PageSettings;
use adw::prelude::AdwApplicationWindowExt;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio;
use gtk4 as gtk;
use hypr_config_manager::{StoreEvent, ThemeStore};
use libadwaita as adw;
use std::cell::RefCell;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        let window: Self = glib::Object::builder().property("application", app).build();

        window.connect_close_request(glib::clone!(
            #[weak]
            app,
            #[upgrade_or]
            glib::Propagation::Proceed,
            move |_| {
                app.quit();
                glib::Propagation::Proceed
            }
        ));

        window
    }

    pub fn setup_ui(&self) {
        let imp = self.imp();

        // Header bar buttons
        let save_btn = gtk::Button::builder()
            .label("Save & Apply")
            .sensitive(false)
            .build();
        imp.header_bar.pack_end(&save_btn);

        save_btn.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_| {
                let is_debug = std::env::args().any(|x| x == "--debug");
                if is_debug {
                    println!("[MainWindow] Save & Apply clicked.");
                    let staged = ThemeStore::global().get_staged_updates();
                    println!("[MainWindow] Staged updates to commit: {:?}", staged);
                }
                match ThemeStore::global().commit_theme_updates() {
                    Ok((success, msg)) => {
                        if is_debug {
                            println!(
                                "[MainWindow] Commit updates result: success={}, msg={}",
                                success, msg
                            );
                        }
                        if success {
                            obj.add_toast(&msg);
                        }
                    }
                    Err(e) => {
                        eprintln!("[MainWindow] Error committing updates: {}", e);
                    }
                }
            }
        ));

        // Menu button
        let menu_btn = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .build();
        if let Some(app) = self.application() {
            menu_btn.set_menu_model(app.menubar().as_ref());
        }
        imp.header_bar.pack_end(&menu_btn);

        // ThemeStore listener
        let (sender, receiver) = std::sync::mpsc::channel::<StoreEvent>();
        let save_btn_weak = save_btn.downgrade();
        let obj_weak = self.downgrade();

        glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            let obj = match obj_weak.upgrade() {
                Some(o) => o,
                None => return glib::ControlFlow::Break,
            };
            while let Ok(event) = receiver.try_recv() {
                if let Some(save_btn) = save_btn_weak.upgrade() {
                    let has_changes = ThemeStore::global().has_staged_updates();
                    save_btn.set_sensitive(has_changes);
                    if has_changes {
                        if !save_btn.has_css_class("suggested-action") {
                            save_btn.add_css_class("suggested-action");
                        }
                    } else {
                        save_btn.remove_css_class("suggested-action");
                    }
                }

                if event == StoreEvent::ThemeChanged {
                    // Use targeted refresh when possible, but for ThemeChanged a full refresh is currently expected.
                    obj.refresh_all_pages();

                    let manager = adw::StyleManager::default();
                    let store = ThemeStore::global();
                    let theme = store.current_theme();
                    if let Ok(config) = store.get_theme_data(&theme) {
                        let scheme = match config.gtk.color_scheme.as_str() {
                            "prefer-light" => adw::ColorScheme::PreferLight,
                            "prefer-dark" => adw::ColorScheme::PreferDark,
                            _ => adw::ColorScheme::Default,
                        };
                        manager.set_color_scheme(scheme);
                    }
                }
            }
            glib::ControlFlow::Continue
        });

        ThemeStore::global().add_listener(move |event| {
            let _ = sender.send(event);
        });

        // Sidebar selection logic
        imp.sidebar_list.connect_row_selected(glib::clone!(
            #[weak]
            imp,
            move |_, row| {
                if let Some(row) = row {
                    let page_id = row.widget_name();
                    imp.stack.set_visible_child_name(&page_id);

                    if let Some(action_row) = row.downcast_ref::<adw::ActionRow>() {
                        imp.window_title.set_title(&glib::markup_escape_text(&action_row.title()));
                    }
                }
            }
        ));

        // Populate Pages
        self.add_page(
            "theme",
            "Theme",
            "preferences-desktop-theme-symbolic",
            crate::pages::theme::ThemePage::new().upcast(),
        );
        self.add_page(
            "appearance",
            "Appearance",
            "preferences-desktop-wallpaper-symbolic",
            crate::pages::appearance::AppearancePage::new().upcast(),
        );
        self.add_page(
            "wallpapers",
            "Wallpapers",
            "preferences-desktop-wallpaper-symbolic",
            crate::pages::wallpapers::WallpapersPage::new().upcast(),
        );
        self.add_page(
            "displays",
            "Displays",
            "preferences-desktop-display-symbolic",
            crate::pages::displays::DisplaysPage::new().upcast(),
        );
        self.add_page(
            "keybindings",
            "Keybindings",
            "preferences-desktop-keyboard-shortcuts-symbolic",
            crate::pages::keybindings::KeybindingsPage::new().upcast(),
        );
        self.add_page(
            "layout",
            "Layout",
            "view-grid-symbolic",
            crate::pages::layout::LayoutPage::new().upcast(),
        );
        self.add_page(
            "input",
            "Input",
            "input-mouse-symbolic",
            crate::pages::input::InputPage::new().upcast(),
        );
        self.add_page(
            "windows",
            "Windows",
            "window-new-symbolic",
            crate::pages::windows::WindowsPage::new().upcast(),
        );
        self.add_page(
            "system",
            "System",
            "utilities-system-monitor-symbolic",
            crate::pages::system::SystemPage::new().upcast(),
        );
        self.add_page(
            "hardware",
            "Hardware",
            "input-gaming-symbolic",
            crate::pages::hardware::HardwarePage::new().upcast(),
        );
        self.add_page(
            "plugins",
            "Plugins",
            "system-run-symbolic",
            crate::pages::plugins::PluginsPage::new().upcast(),
        );
        self.add_page(
            "backup",
            "Backup &amp; Restore",
            "document-save-symbolic",
            crate::pages::backup::BackupPage::new().upcast(),
        );
        self.add_page(
            "applications",
            "Applications",
            "applications-system-symbolic",
            crate::pages::applications::ApplicationsPage::new().upcast(),
        );

        // Select first row
        if let Some(row) = imp.sidebar_list.row_at_index(0) {
            imp.sidebar_list.select_row(Some(&row));
        }
    }

    fn add_page(&self, id: &str, title: &str, icon_name: &str, page: gtk::Widget) {
        let imp = self.imp();

        // Use trait for setup
        if let Some(p) = page.downcast_ref::<crate::pages::theme::ThemePage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::appearance::AppearancePage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::wallpapers::WallpapersPage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::displays::DisplaysPage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::keybindings::KeybindingsPage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::layout::LayoutPage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::input::InputPage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::windows::WindowsPage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::system::SystemPage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::hardware::HardwarePage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::plugins::PluginsPage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::backup::BackupPage>() {
            p.setup_ui();
        } else if let Some(p) = page.downcast_ref::<crate::pages::applications::ApplicationsPage>()
        {
            p.setup_ui();
        }

        let row = adw::ActionRow::builder()
            .title(title)
            .name(id)
            .use_markup(true)
            .build();
        let icon = gtk::Image::from_icon_name(icon_name);
        row.add_prefix(&icon);

        imp.sidebar_list.insert(&row, -1);

        imp.stack.add_named(&page, Some(id));
        imp.pages.borrow_mut().push(page);
    }

    pub fn refresh_all_pages(&self) {
        let imp = self.imp();
        if std::env::args().any(|x| x == "--debug") {
            println!("[MainWindow] Refreshing all pages...");
        }
        for page in imp.pages.borrow().iter() {
            if let Some(p) = page.downcast_ref::<crate::pages::theme::ThemePage>() {
                p.refresh_settings();
            } else if let Some(p) = page.downcast_ref::<crate::pages::appearance::AppearancePage>()
            {
                p.refresh_settings();
            } else if let Some(p) = page.downcast_ref::<crate::pages::wallpapers::WallpapersPage>()
            {
                p.refresh_settings();
            } else if let Some(p) = page.downcast_ref::<crate::pages::displays::DisplaysPage>() {
                p.refresh_settings();
            } else if let Some(p) =
                page.downcast_ref::<crate::pages::keybindings::KeybindingsPage>()
            {
                p.refresh_settings();
            } else if let Some(p) = page.downcast_ref::<crate::pages::layout::LayoutPage>() {
                p.refresh_settings();
            } else if let Some(p) = page.downcast_ref::<crate::pages::input::InputPage>() {
                p.refresh_settings();
            } else if let Some(p) = page.downcast_ref::<crate::pages::windows::WindowsPage>() {
                p.refresh_settings();
            } else if let Some(p) = page.downcast_ref::<crate::pages::system::SystemPage>() {
                p.refresh_settings();
            } else if let Some(p) = page.downcast_ref::<crate::pages::hardware::HardwarePage>() {
                p.refresh_settings();
            } else if let Some(p) = page.downcast_ref::<crate::pages::plugins::PluginsPage>() {
                p.refresh_settings();
            } else if let Some(p) = page.downcast_ref::<crate::pages::backup::BackupPage>() {
                p.refresh_settings();
            } else if let Some(p) =
                page.downcast_ref::<crate::pages::applications::ApplicationsPage>()
            {
                p.refresh_settings();
            }
        }
    }

    pub fn add_toast(&self, message: &str) {
        let imp = self.imp();
        let toast = adw::Toast::new(message);
        imp.toast_overlay.add_toast(toast);
    }
}

pub mod imp {
    use super::*;

    #[derive(Default)]
    pub struct MainWindow {
        pub stack: adw::ViewStack,
        pub sidebar_list: gtk::ListBox,
        pub sidebar_search: gtk::SearchEntry,
        pub split_view: adw::NavigationSplitView,
        pub header_bar: adw::HeaderBar,
        pub toast_overlay: adw::ToastOverlay,
        pub window_title: adw::WindowTitle,
        pub pages: RefCell<Vec<gtk::Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for MainWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            self.split_view.set_min_sidebar_width(240.0);
            self.split_view.set_sidebar_width_fraction(0.25);

            // Sidebar
            let sidebar_toolbar_view = adw::ToolbarView::new();
            let sidebar_header = adw::HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("Settings", ""))
                .build();
            sidebar_toolbar_view.add_top_bar(&sidebar_header);

            let sidebar_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
            sidebar_box.set_margin_start(12);
            sidebar_box.set_margin_end(12);
            sidebar_box.set_margin_top(6);
            sidebar_box.set_margin_bottom(6);

            self.sidebar_search
                .set_placeholder_text(Some("Search pages..."));
            sidebar_box.append(&self.sidebar_search);

            let scrolled = gtk::ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::Never)
                .propagate_natural_height(true)
                .vexpand(true)
                .build();
            scrolled.set_child(Some(&self.sidebar_list));
            sidebar_box.append(&scrolled);

            sidebar_toolbar_view.set_content(Some(&sidebar_box));

            self.sidebar_search.connect_search_changed(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                move |_| {
                    imp.sidebar_list.invalidate_filter();
                }
            ));

            let obj_weak = self.obj().downgrade();
            self.sidebar_list.set_filter_func(move |row| {
                if let Some(obj) = obj_weak.upgrade() {
                    let imp = obj.imp();
                    let text = imp.sidebar_search.text().to_lowercase();
                    if text.is_empty() {
                        return true;
                    }
                    if let Some(action_row) = row.downcast_ref::<adw::ActionRow>() {
                        action_row.title().to_lowercase().contains(&text)
                    } else {
                        true
                    }
                } else {
                    false
                }
            });

            let sidebar_page = adw::NavigationPage::builder()
                .title("Settings")
                .child(&sidebar_toolbar_view)
                .build();
            self.split_view.set_sidebar(Some(&sidebar_page));

            // Content
            self.header_bar.set_title_widget(Some(&self.window_title));
            self.window_title.set_title("Theme");

            let content_toolbar_view = adw::ToolbarView::new();
            content_toolbar_view.add_top_bar(&self.header_bar);
            content_toolbar_view.set_content(Some(&self.stack));
            self.stack.set_vexpand(true);

            let content_page = adw::NavigationPage::builder()
                .title("Content")
                .child(&content_toolbar_view)
                .build();
            self.split_view.set_content(Some(&content_page));

            self.toast_overlay.set_child(Some(&self.split_view));
            obj.set_content(Some(&self.toast_overlay));
            obj.set_default_width(1100);
            obj.set_default_height(800);
        }
    }

    impl WidgetImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl ApplicationWindowImpl for MainWindow {}
    impl AdwApplicationWindowImpl for MainWindow {}
}
