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
    pub struct InputPage(ObjectSubclass<imp::InputPage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl InputPage {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn debug_log(&self, msg: &str) {
        if std::env::args().any(|x| x == "--debug") {
            println!("[InputPage] {}", msg);
        }
    }
}

impl PageSettings for InputPage {
    fn setup_ui(&self) {
        self.debug_log("Setting up UI models and signals");

        // Connect Signals
        self.connect_spin_signal("sensitivity", "input.sensitivity");
        self.connect_spin_signal("scroll_factor", "input.scroll_factor");
        self.connect_combo_signal("accel_profile", "input.accel_profile");
        self.connect_combo_signal("follow_mouse", "input.follow_mouse");

        self.connect_switch_signal("touchpad_enabled", "input.touchpad_enabled");
        self.connect_switch_signal("natural_scroll", "input.touchpad_natural_scroll");
        self.connect_switch_signal("touchpad_tap_to_click", "input.touchpad_tap_to_click");
        self.connect_spin_signal("touchpad_scroll_factor", "input.touchpad_scroll_factor");

        self.connect_combo_signal("main_mod", "input.main_mod");
        self.connect_entry_signal("kb_layout", "input.kb_layout");
        self.connect_entry_signal("kb_variant", "input.kb_variant");
        self.connect_entry_signal("kb_model", "input.kb_model");
        self.connect_entry_signal("kb_options", "input.kb_options");
        self.connect_entry_signal("kb_rules", "input.kb_rules");

        self.connect_switch_signal("solaar_enabled", "input.solaar_enabled");

        // Buttons
        self.setup_button("kb_editor_btn", "hypr-keybindings-editor");
        self.setup_button("xcompose_btn", "hypr-restart-xcompose");

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

        self.update_spin_row("sensitivity", data.get_raw_f64("input.sensitivity", 0.0));
        self.update_spin_row(
            "scroll_factor",
            data.get_raw_f64("input.scroll_factor", 1.0),
        );
        self.update_combo_row(
            "accel_profile",
            &data.get_raw_str("input.accel_profile", "flat"),
        );
        self.update_combo_row(
            "follow_mouse",
            &data.get_raw_i32("input.follow_mouse", 1).to_string(),
        );

        self.update_switch_row(
            "touchpad_enabled",
            data.get_raw_bool("input.touchpad_enabled", true),
        );
        self.update_switch_row(
            "natural_scroll",
            data.get_raw_bool("input.touchpad_natural_scroll", true),
        );
        self.update_switch_row(
            "touchpad_tap_to_click",
            data.get_raw_bool("input.touchpad_tap_to_click", true),
        );
        self.update_spin_row(
            "touchpad_scroll_factor",
            data.get_raw_f64("input.touchpad_scroll_factor", 1.0),
        );

        self.update_combo_row("main_mod", &data.get_raw_str("input.main_mod", "SUPER"));
        self.update_entry_row("kb_layout", &data.get_raw_str("input.kb_layout", "us"));
        self.update_entry_row("kb_variant", &data.get_raw_str("input.kb_variant", ""));
        self.update_entry_row("kb_model", &data.get_raw_str("input.kb_model", ""));
        self.update_entry_row("kb_options", &data.get_raw_str("input.kb_options", ""));
        self.update_entry_row("kb_rules", &data.get_raw_str("input.kb_rules", ""));

        self.update_switch_row(
            "solaar_enabled",
            data.get_raw_bool("input.solaar_enabled", false),
        );

        *imp.is_refreshing.borrow_mut() = false;
    }
}

impl InputPage {
    fn setup_button(&self, id: &str, command: &'static str) {
        let imp = self.imp();
        if let Some(btn) = imp.builder.object::<gtk::Button>(id) {
            btn.connect_clicked(move |_| {
                let _ = std::process::Command::new(command).spawn();
            });
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

    fn connect_entry_signal(&self, id: &str, theme_key: &str) {
        let imp = self.imp();
        crate::bind_signal!(
            self,
            imp.builder,
            adw::EntryRow,
            id,
            theme_key,
            connect_text_notify,
            |w: &adw::EntryRow| w.text().to_string(),
            yaml_serde::Value::String
        );
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
                        // follow_mouse requires an int
                        if theme_key == "input.follow_mouse" {
                            let first_part = val.split(' ').next().unwrap_or("");
                            if let Ok(num) = first_part.parse::<i32>() {
                                ThemeStore::global().stage_update(
                                    &theme_key,
                                    yaml_serde::Value::Number(num.into()),
                                );
                                return;
                            }
                        }
                        ThemeStore::global()
                            .stage_update(&theme_key, yaml_serde::Value::String(val));
                    }
                }
            ));
        }
    }

    fn update_entry_row(&self, id: &str, val: &str) {
        if let Some(row) = self.imp().builder.object::<adw::EntryRow>(id) {
            row.set_text(val);
        }
    }

    fn update_combo_row(&self, id: &str, val: &str) {
        if let Some(row) = self.imp().builder.object::<adw::ComboRow>(id)
            && let Some(model) = row.model()
            && let Some(string_list) = model.downcast_ref::<gtk::StringList>()
        {
            for i in 0..string_list.n_items() {
                let item_str = string_list.string(i).unwrap();
                if item_str == val || item_str.starts_with(&(val.to_owned() + " ")) {
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

    pub struct InputPage {
        pub builder: gtk::Builder,
        pub is_refreshing: Rc<RefCell<bool>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InputPage {
        const NAME: &'static str = "InputPage";
        type Type = super::InputPage;
        type ParentType = adw::PreferencesPage;

        fn new() -> Self {
            let builder = gtk::Builder::from_string(include_str!("../../resources/pages/input.ui"));
            Self {
                builder,
                is_refreshing: Rc::new(RefCell::new(false)),
            }
        }
    }

    impl Default for InputPage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ObjectImpl for InputPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            let group_ids = ["kb_group", "mouse_group", "touchpad_group", "tools_group"];
            for id in group_ids {
                if let Some(group) = self.builder.object::<adw::PreferencesGroup>(id) {
                    obj.add(&group);
                }
            }
        }
    }

    impl WidgetImpl for InputPage {}
    impl PreferencesPageImpl for InputPage {}
}
