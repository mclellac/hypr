use crate::pages::PageSettings;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gdk_pixbuf;
use gtk::glib;
use gtk4 as gtk;
use libadwaita as adw;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Default, Clone)]
struct WallpaperConfig {
    #[serde(default)]
    pub wallpaper_dirs: Vec<String>,
    pub auto_cycle: Option<bool>,
    pub cycle_interval: Option<i32>,
}

glib::wrapper! {
    pub struct WallpapersPage(ObjectSubclass<imp::WallpapersPage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl WallpapersPage {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl PageSettings for WallpapersPage {
    fn setup_ui(&self) {
        let imp = self.imp();

        if let Some(btn) = imp.builder.object::<gtk::Button>("add_source_btn") {
            btn.connect_clicked(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |_| {
                    obj.on_add_source();
                }
            ));
        }

        if let Some(btn) = imp.builder.object::<gtk::Button>("import_btn") {
            btn.connect_clicked(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |_| {
                    obj.on_import();
                }
            ));
        }

        if let Some(sw) = imp.builder.object::<adw::SwitchRow>("auto_cycle_toggle") {
            sw.connect_active_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |sw| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    obj.update_config(|cfg| cfg.auto_cycle = Some(sw.is_active()));
                }
            ));
        }

        if let Some(spin) = imp.builder.object::<adw::SpinRow>("cycle_interval") {
            spin.connect_value_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |spin| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    obj.update_config(|cfg| cfg.cycle_interval = Some(spin.value() as i32));
                }
            ));
        }

        self.refresh_settings();
    }

    fn refresh_settings(&self) {
        let imp = self.imp();
        *imp.is_refreshing.borrow_mut() = true;

        let config = self.load_config();

        if let Some(sw) = imp.builder.object::<adw::SwitchRow>("auto_cycle_toggle") {
            sw.set_active(config.auto_cycle.unwrap_or(false));
        }
        if let Some(spin) = imp.builder.object::<adw::SpinRow>("cycle_interval") {
            spin.set_value(config.cycle_interval.unwrap_or(30) as f64);
        }

        self.refresh_sources_rows(&config);
        self.load_wallpapers(&config);

        *imp.is_refreshing.borrow_mut() = false;
    }
}

impl WallpapersPage {
    fn load_config(&self) -> WallpaperConfig {
        let home = std::env::var("HOME").unwrap_or_default();
        let path = PathBuf::from(home).join(".config/hypr/wallpaper-manager.json");
        if let Ok(content) = fs::read_to_string(path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            WallpaperConfig::default()
        }
    }

    fn save_config(&self, config: &WallpaperConfig) {
        let home = std::env::var("HOME").unwrap_or_default();
        let path = PathBuf::from(home).join(".config/hypr/wallpaper-manager.json");
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string_pretty(config) {
            let _ = fs::write(path, content);
        }
    }

    fn update_config<F>(&self, f: F)
    where
        F: FnOnce(&mut WallpaperConfig),
    {
        let mut config = self.load_config();
        f(&mut config);
        self.save_config(&config);
    }

    fn refresh_sources_rows(&self, config: &WallpaperConfig) {
        let imp = self.imp();
        if let Some(expander) = imp.builder.object::<adw::ExpanderRow>("sources_expander") {
            // Clear existing rows
            for row in imp.source_rows.borrow().iter() {
                expander.remove(row);
            }
            imp.source_rows.borrow_mut().clear();

            for dir in &config.wallpaper_dirs {
                let row = adw::ActionRow::builder()
                    .title(
                        Path::new(dir)
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or(dir),
                    )
                    .subtitle(dir)
                    .build();

                let del_btn = gtk::Button::builder()
                    .icon_name("user-trash-symbolic")
                    .valign(gtk::Align::Center)
                    .css_classes(vec!["flat".to_string()])
                    .build();

                let dir_clone = dir.clone();
                del_btn.connect_clicked(glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    move |_| {
                        obj.update_config(|cfg| {
                            cfg.wallpaper_dirs.retain(|d| d != &dir_clone);
                        });
                        obj.refresh_settings();
                    }
                ));

                row.add_suffix(&del_btn);
                expander.add_row(&row);
                imp.source_rows.borrow_mut().push(row.upcast());
            }
        }
    }

    fn load_wallpapers(&self, config: &WallpaperConfig) {
        let imp = self.imp();
        if let Some(flowbox) = imp.builder.object::<gtk::FlowBox>("flowbox") {
            flowbox.remove_all();

            let mut images = Vec::new();
            for dir_str in &config.wallpaper_dirs {
                let dir = Path::new(dir_str);
                if dir.exists()
                    && let Ok(entries) = fs::read_dir(dir)
                {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file()
                            && let Some(ext) = path.extension().and_then(|s| s.to_str())
                        {
                            let ext = ext.to_lowercase();
                            if ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "webp" {
                                images.push(path);
                            }
                        }
                    }
                }
            }

            images.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

            for img in images {
                let card = self.create_wallpaper_card(img);
                flowbox.append(&card);
            }
        }
    }

    fn create_wallpaper_card(&self, path: PathBuf) -> gtk::Button {
        let btn = gtk::Button::builder()
            .css_classes(vec!["card".to_string()])
            .valign(gtk::Align::Start)
            .build();

        let box_layout = gtk::Box::new(gtk::Orientation::Vertical, 6);

        let picture = gtk::Picture::new();
        picture.set_content_fit(gtk::ContentFit::Cover);
        picture.set_size_request(240, 135);

        let path_clone = path.clone();
        #[allow(deprecated)]
        if let Ok(pixbuf) = gdk_pixbuf::Pixbuf::from_file_at_scale(&path, 240, -1, true) {
            let texture = gtk::gdk::Texture::for_pixbuf(&pixbuf);
            picture.set_paintable(Some(&texture));
        }

        box_layout.append(&picture);

        let label = gtk::Label::builder()
            .label(path.file_name().and_then(|s| s.to_str()).unwrap_or(""))
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .max_width_chars(20)
            .build();
        box_layout.append(&label);

        btn.set_child(Some(&box_layout));

        btn.connect_clicked(move |_| {
            let _ = std::process::Command::new("hypr-wallpaper-set")
                .arg(&path_clone)
                .spawn();
        });

        btn
    }

    fn on_add_source(&self) {
        let dialog = gtk::FileDialog::builder().title("Select Directory").build();

        dialog.select_folder(
            None::<&gtk::Window>,
            None::<&gio::Cancellable>,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |res| {
                    if let Ok(folder) = res
                        && let Some(path) = folder.path()
                    {
                        let path_str = path.to_string_lossy().to_string();
                        obj.update_config(|cfg| {
                            if !cfg.wallpaper_dirs.contains(&path_str) {
                                cfg.wallpaper_dirs.push(path_str);
                            }
                        });
                        obj.refresh_settings();
                    }
                }
            ),
        );
    }

    fn on_import(&self) {
        let dialog = gtk::FileDialog::builder()
            .title("Select Wallpaper Image")
            .build();

        let filter = gtk::FileFilter::new();
        filter.set_name(Some("Images"));
        filter.add_mime_type("image/*");
        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&filter);
        dialog.set_filters(Some(&filters));

        dialog.open(
            None::<&gtk::Window>,
            None::<&gio::Cancellable>,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |res| {
                    if let Ok(file) = res
                        && let Some(src_path) = file.path()
                    {
                        let home = std::env::var("HOME").unwrap_or_default();
                        let dest_dir = PathBuf::from(home).join("Pictures/Wallpapers");
                        let _ = fs::create_dir_all(&dest_dir);

                        if let Some(filename) = src_path.file_name() {
                            let dest_path = dest_dir.join(filename);
                            if fs::copy(&src_path, &dest_path).is_ok() {
                                obj.refresh_settings();
                            }
                        }
                    }
                }
            ),
        );
    }
}

pub mod imp {
    use super::*;

    pub struct WallpapersPage {
        pub builder: gtk::Builder,
        pub is_refreshing: RefCell<bool>,
        pub source_rows: RefCell<Vec<gtk::Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WallpapersPage {
        const NAME: &'static str = "WallpapersPage";
        type Type = super::WallpapersPage;
        type ParentType = adw::PreferencesPage;

        fn new() -> Self {
            let builder =
                gtk::Builder::from_string(include_str!("../../resources/pages/wallpapers.ui"));
            Self {
                builder,
                is_refreshing: RefCell::new(false),
                source_rows: RefCell::new(Vec::new()),
            }
        }
    }

    impl Default for WallpapersPage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ObjectImpl for WallpapersPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            if let Some(group) = self
                .builder
                .object::<adw::PreferencesGroup>("settings_group")
            {
                obj.add(&group);
            }
            if let Some(group) = self
                .builder
                .object::<adw::PreferencesGroup>("sources_group")
            {
                obj.add(&group);
            }
            if let Some(group) = self
                .builder
                .object::<adw::PreferencesGroup>("selection_group")
            {
                obj.add(&group);
            }
        }
    }

    impl WidgetImpl for WallpapersPage {}
    impl PreferencesPageImpl for WallpapersPage {}
}
