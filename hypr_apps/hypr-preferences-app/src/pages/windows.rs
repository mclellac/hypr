use crate::pages::PageSettings;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::ThemeStore;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

glib::wrapper! {
    pub struct WindowsPage(ObjectSubclass<imp::WindowsPage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl WindowsPage {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn debug_log(&self, msg: &str) {
        if std::env::args().any(|x| x == "--debug") {
            println!("[WindowsPage] {}", msg);
        }
    }
}

impl PageSettings for WindowsPage {
    fn setup_ui(&self) {
        self.debug_log("Setting up UI models and signals");

        // Connect Signals
        self.connect_spin_signal("opacity_active", "hypr.window.opacity.active");
        self.connect_spin_signal("opacity_inactive", "hypr.window.opacity.inactive");

        self.connect_expander_signal(
            "workspace_browsers_row",
            "hypr.window.workspace_assignment.browsers",
        );
        self.connect_expander_signal(
            "workspace_coding_row",
            "hypr.window.workspace_assignment.coding",
        );
        self.connect_expander_signal(
            "workspace_communications_row",
            "hypr.window.workspace_assignment.communications",
        );
        self.connect_expander_signal(
            "workspace_media_row",
            "hypr.window.workspace_assignment.media",
        );

        self.connect_combo_signal("animation_browsers", "hypr.window.animation.browsers");
        self.connect_combo_signal("animation_coding", "hypr.window.animation.coding");
        self.connect_combo_signal(
            "animation_communications",
            "hypr.window.animation.communications",
        );
        self.connect_combo_signal("animation_media", "hypr.window.animation.media");

        self.connect_size_signal("size_floating", "hypr.window.size.floating");
        self.connect_size_signal("size_dialog", "hypr.window.size.dialog");
        self.connect_size_signal("size_editor", "hypr.window.size.editor");
        self.connect_size_signal("size_creator", "hypr.window.size.creator");

        self.connect_switch_signal("suppress_maximize", "hypr.window.suppress_maximize");

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

        self.debug_log(&format!("Refreshing settings for theme: {}", theme));

        self.update_spin_row(
            "opacity_active",
            data.get_raw_f64("hypr.window.opacity.active", 1.0),
        );
        self.update_spin_row(
            "opacity_inactive",
            data.get_raw_f64("hypr.window.opacity.inactive", 1.0),
        );

        self.update_expander_row(
            "workspace_browsers_row",
            data.get_raw_bool("hypr.window.workspace_assignment.browsers", false),
        );
        self.update_expander_row(
            "workspace_coding_row",
            data.get_raw_bool("hypr.window.workspace_assignment.coding", false),
        );
        self.update_expander_row(
            "workspace_communications_row",
            data.get_raw_bool("hypr.window.workspace_assignment.communications", false),
        );
        self.update_expander_row(
            "workspace_media_row",
            data.get_raw_bool("hypr.window.workspace_assignment.media", false),
        );

        self.update_combo_row(
            "animation_browsers",
            &data.get_raw_str("hypr.window.animation.browsers", "default"),
        );
        self.update_combo_row(
            "animation_coding",
            &data.get_raw_str("hypr.window.animation.coding", "default"),
        );
        self.update_combo_row(
            "animation_communications",
            &data.get_raw_str("hypr.window.animation.communications", "default"),
        );
        self.update_combo_row(
            "animation_media",
            &data.get_raw_str("hypr.window.animation.media", "default"),
        );

        self.update_size_row(
            "size_floating",
            &data.get_raw_str("hypr.window.size.floating", "800 600"),
        );
        self.update_size_row(
            "size_dialog",
            &data.get_raw_str("hypr.window.size.dialog", "800 600"),
        );
        self.update_size_row(
            "size_editor",
            &data.get_raw_str("hypr.window.size.editor", "800 600"),
        );
        self.update_size_row(
            "size_creator",
            &data.get_raw_str("hypr.window.size.creator", "800 600"),
        );

        self.update_switch_row(
            "suppress_maximize",
            data.get_raw_bool("hypr.window.suppress_maximize", true),
        );

        *imp.is_refreshing.borrow_mut() = false;
    }
}

impl WindowsPage {
    fn connect_size_signal(&self, id_prefix: &str, theme_key: &str) {
        let theme_key = theme_key.to_string();
        let imp = self.imp();
        let width_id = format!("{}_width", id_prefix);
        let height_id = format!("{}_height", id_prefix);

        if let Some(row) = imp.builder.object::<adw::SpinRow>(&width_id) {
            let theme_key_clone = theme_key.clone();
            let height_id_clone = height_id.clone();
            row.connect_value_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    let imp = obj.imp();
                    if *imp.is_refreshing.borrow() {
                        return;
                    }
                    let w = row.value() as i32;
                    let h =
                        if let Some(h_row) = imp.builder.object::<adw::SpinRow>(&height_id_clone) {
                            h_row.value() as i32
                        } else {
                            600
                        };
                    ThemeStore::global().stage_update(
                        &theme_key_clone,
                        yaml_serde::Value::String(format!("{} {}", w, h)),
                    );
                }
            ));
        }

        if let Some(row) = imp.builder.object::<adw::SpinRow>(&height_id) {
            let theme_key_clone = theme_key.clone();
            let width_id_clone = width_id.clone();
            row.connect_value_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    let imp = obj.imp();
                    if *imp.is_refreshing.borrow() {
                        return;
                    }
                    let h = row.value() as i32;
                    let w = if let Some(w_row) = imp.builder.object::<adw::SpinRow>(&width_id_clone)
                    {
                        w_row.value() as i32
                    } else {
                        800
                    };
                    ThemeStore::global().stage_update(
                        &theme_key_clone,
                        yaml_serde::Value::String(format!("{} {}", w, h)),
                    );
                }
            ));
        }
    }

    fn connect_spin_signal(&self, id: &str, theme_key: &str) {
        let imp = self.imp();
        crate::bind_signal!(self, imp.builder, adw::SpinRow, id, theme_key);
    }

    fn connect_switch_signal(&self, id: &str, theme_key: &str) {
        let imp = self.imp();
        crate::bind_signal!(
            self,
            imp.builder,
            adw::SwitchRow,
            id,
            theme_key,
            connect_active_notify,
            |w: &adw::SwitchRow| w.is_active(),
            yaml_serde::Value::Bool
        );
    }

    fn connect_expander_signal(&self, id: &str, theme_key: &str) {
        let theme_key = theme_key.to_string();
        let imp = self.imp();
        if let Some(row) = imp.builder.object::<adw::ExpanderRow>(id) {
            row.connect_enable_expansion_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    let imp = obj.imp();
                    if *imp.is_refreshing.borrow() {
                        return;
                    }
                    let val = row.enables_expansion();
                    ThemeStore::global().stage_update(&theme_key, yaml_serde::Value::Bool(val));
                }
            ));
        }
    }

    fn connect_combo_signal(&self, id: &str, theme_key: &str) {
        let theme_key = theme_key.to_string();
        let imp = self.imp();
        if let Some(row) = imp.builder.object::<adw::ComboRow>(id) {
            row.connect_selected_item_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    let imp = obj.imp();
                    if *imp.is_refreshing.borrow() {
                        return;
                    }
                    if let Some(item) = row.selected_item()
                        && let Some(string_obj) = item.downcast_ref::<gtk::StringObject>()
                    {
                        let val = string_obj.string().to_string();
                        ThemeStore::global()
                            .stage_update(&theme_key, yaml_serde::Value::String(val));
                    }
                }
            ));
        }
    }

    fn update_expander_row(&self, id: &str, val: bool) {
        if let Some(row) = self.imp().builder.object::<adw::ExpanderRow>(id) {
            row.set_enable_expansion(val);
        }
    }

    fn update_size_row(&self, id_prefix: &str, val: &str) {
        let parts: Vec<&str> = val.split_whitespace().collect();
        let width = parts
            .first()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(800.0);
        let height = parts
            .get(1)
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(600.0);

        let width_id = format!("{}_width", id_prefix);
        let height_id = format!("{}_height", id_prefix);

        if let Some(row) = self.imp().builder.object::<adw::SpinRow>(&width_id) {
            row.set_value(width);
        }
        if let Some(row) = self.imp().builder.object::<adw::SpinRow>(&height_id) {
            row.set_value(height);
        }
    }

    fn update_combo_row(&self, id: &str, val: &str) {
        if let Some(row) = self.imp().builder.object::<adw::ComboRow>(id)
            && let Some(model) = row.model()
            && let Some(string_list) = model.downcast_ref::<gtk::StringList>()
        {
            for i in 0..string_list.n_items() {
                if string_list.string(i).unwrap() == val {
                    row.set_selected(i);
                    return;
                }
            }
        }
    }

    fn update_spin_row(&self, id: &str, val: f64) {
        if let Some(row) = self.imp().builder.object::<adw::SpinRow>(id) {
            row.set_value(val);
        }
    }

    fn update_switch_row(&self, id: &str, val: bool) {
        if let Some(row) = self.imp().builder.object::<adw::SwitchRow>(id) {
            row.set_active(val);
        }
    }
}

pub mod imp {
    use super::*;

    pub struct WindowsPage {
        pub builder: gtk::Builder,
        pub is_refreshing: Rc<RefCell<bool>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WindowsPage {
        const NAME: &'static str = "WindowsPage";
        type Type = super::WindowsPage;
        type ParentType = adw::PreferencesPage;

        fn new() -> Self {
            let builder =
                gtk::Builder::from_string(include_str!("../../resources/pages/windows.ui"));
            Self {
                builder,
                is_refreshing: Rc::new(RefCell::new(false)),
            }
        }
    }

    impl Default for WindowsPage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ObjectImpl for WindowsPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            let group_ids = [
                "aesthetics_group",
                "semantic_group",
                "sizes_group",
                "behavior_group",
            ];
            for id in group_ids {
                if let Some(group) = self.builder.object::<adw::PreferencesGroup>(id) {
                    obj.add(&group);
                }
            }
        }
    }

    impl WidgetImpl for WindowsPage {}
    impl PreferencesPageImpl for WindowsPage {}
}
