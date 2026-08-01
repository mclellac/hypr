use crate::pages::PageSettings;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::ThemeStore;
use libadwaita as adw;
use std::cell::RefCell;

glib::wrapper! {
    pub struct SystemPage(ObjectSubclass<imp::SystemPage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PageSettings for SystemPage {
    fn setup_ui(&self) {
        let imp = self.imp();

        // Bind spin rows
        self.connect_spin_signal("idle_timeout", "hypr.idle.timeout");
        self.connect_spin_signal("lock_timeout", "hypr.idle.lock_timeout");
        self.connect_spin_signal("dpms_timeout", "hypr.idle.dpms_timeout");

        if let Some(toggle) = imp.builder.object::<adw::SwitchRow>("gamemode_toggle") {
            toggle.connect_active_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    if row.is_active() {
                        let _ = std::process::Command::new("hypr-gamemode")
                            .arg("enable")
                            .spawn();
                    } else {
                        let _ = std::process::Command::new("hypr-gamemode")
                            .arg("disable")
                            .spawn();
                    }
                }
            ));
        }

        if let Some(toggle) = imp.builder.object::<adw::SwitchRow>("zenmode_toggle") {
            toggle.connect_active_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    if row.is_active() {
                        let _ = std::process::Command::new("hypr-zenmode")
                            .arg("enable")
                            .spawn();
                    } else {
                        let _ = std::process::Command::new("hypr-zenmode")
                            .arg("disable")
                            .spawn();
                    }
                }
            ));
        }

        if let Some(combo) = imp.builder.object::<adw::ComboRow>("power_profile") {
            combo.connect_selected_item_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    if let Some(item) = row.selected_item()
                        && let Some(string_obj) = item.downcast_ref::<gtk::StringObject>()
                    {
                        let val = string_obj.string().to_string();
                        let _ = std::process::Command::new("hypr-powerprofiles-set")
                            .arg(&val)
                            .spawn();
                    }
                }
            ));
        }

        // Buttons
        self.setup_button("about_btn", "hypr-launch-about");
        self.setup_button("update_btn", "hypr-update");
        self.setup_button("doctor_btn", "hypr-doctor");
        self.setup_button("clean_btn", "hypr-clean");
        self.setup_button("reset_defaults_btn", "hypr-reset-sudo");

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

        if let Some(row) = imp.builder.object::<adw::SpinRow>("idle_timeout") {
            row.set_value(data.hypr.idle.timeout as f64);
        }
        if let Some(row) = imp.builder.object::<adw::SpinRow>("lock_timeout") {
            row.set_value(data.hypr.idle.lock_timeout as f64);
        }
        if let Some(row) = imp.builder.object::<adw::SpinRow>("dpms_timeout") {
            row.set_value(data.hypr.idle.dpms_timeout as f64);
        }

        let is_gamemode = std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default())
            .join(".local/state/hypr/game_mode")
            .exists();
        if let Some(toggle) = imp.builder.object::<adw::SwitchRow>("gamemode_toggle") {
            toggle.set_active(is_gamemode);
        }

        let is_zenmode = std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default())
            .join(".local/state/hypr/zen_mode")
            .exists();
        if let Some(toggle) = imp.builder.object::<adw::SwitchRow>("zenmode_toggle") {
            toggle.set_active(is_zenmode);
        }

        let current_profile = std::process::Command::new("powerprofilesctl")
            .arg("get")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "balanced".to_string());

        if let Some(row) = imp.builder.object::<adw::ComboRow>("power_profile")
            && let Some(model) = row.model()
            && let Some(string_list) = model.downcast_ref::<gtk::StringList>()
        {
            for i in 0..string_list.n_items() {
                if string_list.string(i).unwrap() == current_profile {
                    row.set_selected(i);
                    break;
                }
            }
        }

        *imp.is_refreshing.borrow_mut() = false;
    }
}

impl SystemPage {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn setup_button(&self, id: &str, command: &'static str) {
        let imp = self.imp();
        if let Some(btn) = imp.builder.object::<gtk::Button>(id) {
            btn.connect_clicked(move |_| {
                let term = std::env::var("TERMINAL").unwrap_or_else(|_| "alacritty".to_string());

                let bash_cmd = format!("{}; echo 'Press any key to close...'; read -n 1", command);

                let mut cmd = std::process::Command::new(&term);
                if term.contains("gnome-terminal") {
                    cmd.arg("--").arg("bash").arg("-c").arg(&bash_cmd);
                } else {
                    cmd.arg("-e").arg("bash").arg("-c").arg(&bash_cmd);
                }

                let _ = cmd.spawn();
            });
        }
    }

    fn connect_spin_signal(&self, id: &str, theme_key: &'static str) {
        let imp = self.imp();
        crate::bind_signal!(self, imp.builder, adw::SpinRow, id, theme_key);
    }
}

pub mod imp {
    use super::*;

    pub struct SystemPage {
        pub builder: gtk::Builder,
        pub is_refreshing: RefCell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SystemPage {
        const NAME: &'static str = "SystemPage";
        type Type = super::SystemPage;
        type ParentType = adw::PreferencesPage;

        fn new() -> Self {
            let builder =
                gtk::Builder::from_string(include_str!("../../resources/pages/system.ui"));
            Self {
                builder,
                is_refreshing: RefCell::new(false),
            }
        }
    }

    impl Default for SystemPage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ObjectImpl for SystemPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            let group_ids = [
                "app_group",
                "maintenance_group",
                "lifestyle_group",
                "power_group",
                "config_group",
            ];

            for id in group_ids {
                if let Some(group) = self.builder.object::<adw::PreferencesGroup>(id) {
                    obj.add(&group);
                }
            }
        }
    }

    impl WidgetImpl for SystemPage {}
    impl PreferencesPageImpl for SystemPage {}
}
