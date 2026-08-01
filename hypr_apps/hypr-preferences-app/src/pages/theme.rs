use crate::pages::PageSettings;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::ThemeStore;
use libadwaita as adw;
use std::cell::RefCell;

glib::wrapper! {
    pub struct ThemePage(ObjectSubclass<imp::ThemePage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ThemePage {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn debug_log(&self, msg: &str) {
        if std::env::args().any(|x| x == "--debug") {
            println!("[ThemePage] {}", msg);
        }
    }
}

impl PageSettings for ThemePage {
    fn setup_ui(&self) {
        let imp = self.imp();

        self.debug_log("Setting up theme page signals");

        // Font signals
        if let Some(font_btn) = imp.builder.object::<gtk::FontDialogButton>("font_button") {
            font_btn.connect_font_desc_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |btn| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    if let Some(desc) = btn.font_desc()
                        && let Some(family) = desc.family()
                    {
                        use yaml_serde;
                        ThemeStore::global().stage_update(
                            "fonts.font_family",
                            yaml_serde::Value::String(family.to_string()),
                        );
                    }
                }
            ));
        }

        // Theme list signals
        if let Some(flowbox) = imp.builder.object::<gtk::FlowBox>("flowbox") {
            flowbox.connect_child_activated(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |_, child| {
                    println!("[ThemePage] child_activated triggered! child: {:?}", child);
                    if let Some(card) = child.child() {
                        println!("[ThemePage] card widget: {:?}", card);
                        if let Some(card_box) = card.downcast_ref::<gtk::Box>() {
                            println!("[ThemePage] card_box is GtkBox");
                            if let Some(label) = card_box.last_child() {
                                println!("[ThemePage] label widget: {:?}", label);
                                if let Some(label_widget) = label.downcast_ref::<gtk::Label>() {
                                    let theme_name = label_widget.label().to_string();
                                    println!("[ThemePage] theme_name: {}", theme_name);
                                    obj.debug_log(&format!("Activating theme: {}", theme_name));

                                    // Spawn the theme changer script in a thread
                                    let theme_set_bin =
                                        hypr_config_manager::templater::get_hypr_path()
                                            .join("bin/hypr-theme-set");
                                    std::thread::spawn(move || {
                                        let status = std::process::Command::new(theme_set_bin)
                                            .arg(&theme_name)
                                            .status();

                                        if let Ok(s) = status {
                                            if s.success() {
                                                let main_context = glib::MainContext::default();
                                                main_context.invoke(move || {
                                                    ThemeStore::global().force_theme_changed();
                                                });
                                            } else {
                                                eprintln!(
                                                    "[ThemePage] hypr-theme-set status error"
                                                );
                                            }
                                        } else {
                                            eprintln!("[ThemePage] failed to spawn hypr-theme-set");
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            ));
        }

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

        if let Some(font_btn) = imp.builder.object::<gtk::FontDialogButton>("font_button") {
            let font_desc = gtk::pango::FontDescription::from_string(&data.fonts.font_family);
            font_btn.set_font_desc(&font_desc);
        }

        self.refresh_themes_list();

        *imp.is_refreshing.borrow_mut() = false;
    }
}

impl ThemePage {
    fn refresh_themes_list(&self) {
        let imp = self.imp();
        if let Some(flowbox) = imp.builder.object::<gtk::FlowBox>("flowbox") {
            flowbox.remove_all();

            let mut themes = std::collections::HashSet::new();
            let home = std::env::var("HOME").unwrap_or_default();
            let paths = [
                std::path::PathBuf::from(&home).join(".config/hypr/themes"),
                std::path::PathBuf::from("/usr/local/share/hypr/themes"),
                std::path::PathBuf::from(&home).join(".local/share/hypr/themes"),
            ];

            for path in paths {
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir()
                            && let Some(name) = entry.file_name().to_str()
                        {
                            themes.insert(name.to_string());
                        }
                    }
                }
            }

            let current_theme = ThemeStore::global().current_theme();
            let mut sorted_themes: Vec<_> = themes.into_iter().collect();
            sorted_themes.sort();

            for theme_name in sorted_themes {
                let card = self.create_theme_card(&theme_name);
                let child = gtk::FlowBoxChild::new();
                child.set_child(Some(&card));
                flowbox.append(&child);

                if theme_name == current_theme {
                    flowbox.select_child(&child);
                }
            }
        }
    }

    fn create_theme_card(&self, theme_name: &str) -> gtk::Box {
        let card = gtk::Box::new(gtk::Orientation::Vertical, 4);
        card.add_css_class("card");
        card.set_hexpand(true);
        card.set_halign(gtk::Align::Fill);

        let overlay = gtk::Overlay::new();
        let img = gtk::Image::builder()
            .icon_name("preferences-desktop-theme-symbolic")
            .pixel_size(64)
            .build();
        img.add_css_class("osd");
        overlay.set_child(Some(&img));

        let label = gtk::Label::builder()
            .label(theme_name)
            .wrap(true)
            .justify(gtk::Justification::Center)
            .build();
        label.add_css_class("heading");

        card.append(&overlay);
        card.append(&label);
        card
    }
}

pub mod imp {
    use super::*;

    pub struct ThemePage {
        pub builder: gtk::Builder,
        pub is_refreshing: RefCell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ThemePage {
        const NAME: &'static str = "ThemePage";
        type Type = super::ThemePage;
        type ParentType = adw::PreferencesPage;

        fn new() -> Self {
            let builder = gtk::Builder::from_string(include_str!("../../resources/pages/theme.ui"));
            Self {
                builder,
                is_refreshing: RefCell::new(false),
            }
        }
    }

    impl Default for ThemePage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ObjectImpl for ThemePage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.debug_log("ThemePage constructed - loading groups");

            if let Some(group) = self.builder.object::<adw::PreferencesGroup>("font_group") {
                obj.add(&group);
            }
            if let Some(group) = self.builder.object::<adw::PreferencesGroup>("themes_group") {
                obj.add(&group);
            }

            obj.debug_log("Widget setup complete");
        }
    }

    impl WidgetImpl for ThemePage {}
    impl PreferencesPageImpl for ThemePage {}
}
