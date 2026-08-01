use crate::pages::PageSettings;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::ThemeStore;
use hypr_config_manager::models::MonitorConfig;
use libadwaita as adw;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::process::Command;

glib::wrapper! {
    pub struct DisplaysPage(ObjectSubclass<imp::DisplaysPage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct HyprctlMonitor {
    name: String,
    description: String,
    width: i32,
    height: i32,
    refresh_rate: f64,
    scale: f64,
    available_modes: Vec<String>,
}

fn get_system_monitors() -> Vec<HyprctlMonitor> {
    if let Ok(output) = Command::new("hyprctl").arg("monitors").arg("-j").output()
        && output.status.success()
        && let Ok(monitors) = serde_json::from_slice::<Vec<HyprctlMonitor>>(&output.stdout)
    {
        return monitors;
    }
    Vec::new()
}

#[derive(Debug, Clone)]
struct DisplayInfo {
    output: String,
    description: String,
    mode: String,
    position: String,
    scale: f32,
    available_modes: Vec<String>,
    is_connected: bool,
}

impl DisplaysPage {
    fn get_display_infos(&self) -> Vec<DisplayInfo> {
        let system_monitors = get_system_monitors();

        let store = ThemeStore::global();
        let theme = store.current_theme();
        let theme_config = store
            .get_theme_data(&theme)
            .unwrap_or_else(|_| std::sync::Arc::new(hypr_config_manager::ThemeConfig::default()));

        let mut display_infos = Vec::new();
        let mut processed_outputs = std::collections::HashSet::new();

        // Process all configured monitors in theme.yaml
        for conf in &theme_config.monitors {
            if processed_outputs.contains(&conf.output) {
                continue;
            }

            let system_opt = system_monitors.iter().find(|m| m.name == conf.output);
            if let Some(sys) = system_opt {
                display_infos.push(DisplayInfo {
                    output: conf.output.clone(),
                    description: sys.description.clone(),
                    mode: if conf.mode.is_empty() {
                        "preferred".to_string()
                    } else {
                        conf.mode.clone()
                    },
                    position: if conf.position.is_empty() {
                        "auto".to_string()
                    } else {
                        conf.position.clone()
                    },
                    scale: conf.scale,
                    available_modes: sys.available_modes.clone(),
                    is_connected: true,
                });
            } else {
                display_infos.push(DisplayInfo {
                    output: conf.output.clone(),
                    description: "Disconnected".to_string(),
                    mode: if conf.mode.is_empty() {
                        "preferred".to_string()
                    } else {
                        conf.mode.clone()
                    },
                    position: if conf.position.is_empty() {
                        "auto".to_string()
                    } else {
                        conf.position.clone()
                    },
                    scale: conf.scale,
                    available_modes: vec![conf.mode.clone()],
                    is_connected: false,
                });
            }
            processed_outputs.insert(conf.output.clone());
        }

        // Process active/connected system monitors that are not configured
        for sys in &system_monitors {
            if processed_outputs.contains(&sys.name) {
                continue;
            }
            display_infos.push(DisplayInfo {
                output: sys.name.clone(),
                description: sys.description.clone(),
                mode: "preferred".to_string(),
                position: "auto".to_string(),
                scale: sys.scale as f32,
                available_modes: sys.available_modes.clone(),
                is_connected: true,
            });
            processed_outputs.insert(sys.name.clone());
        }

        display_infos
    }

    fn update_monitor_value<F>(&self, output: &str, f: F)
    where
        F: FnOnce(&mut MonitorConfig),
    {
        let store = ThemeStore::global();
        let theme = store.current_theme();
        let theme_config = store
            .get_theme_data(&theme)
            .unwrap_or_else(|_| std::sync::Arc::new(hypr_config_manager::ThemeConfig::default()));

        let mut monitors = theme_config.monitors.clone();

        let monitor_opt = monitors.iter_mut().find(|m| m.output == output);
        if let Some(m) = monitor_opt {
            f(m);
        } else {
            let mut new_monitor = MonitorConfig {
                output: output.to_string(),
                mode: "preferred".to_string(),
                position: "auto".to_string(),
                scale: 1.0,
                bitdepth: 0,
            };
            f(&mut new_monitor);
            monitors.push(new_monitor);
        }

        store.stage_update("monitors", yaml_serde::to_value(&monitors).unwrap());
    }

    fn delete_monitor_config(&self, output: &str) {
        let store = ThemeStore::global();
        let theme = store.current_theme();
        let theme_config = store
            .get_theme_data(&theme)
            .unwrap_or_else(|_| std::sync::Arc::new(hypr_config_manager::ThemeConfig::default()));

        let mut monitors = theme_config.monitors.clone();
        monitors.retain(|m| m.output != output);

        store.stage_update("monitors", yaml_serde::to_value(&monitors).unwrap());

        self.refresh_settings();
    }

    fn prompt_add_monitor(&self) {
        let entry = gtk::Entry::builder()
            .placeholder_text("Output Name (e.g. HDMI-A-1)")
            .build();

        let dialog = adw::AlertDialog::builder()
            .heading("Add Monitor Rule")
            .body("Enter the output name of the monitor:")
            .extra_child(&entry)
            .build();

        dialog.add_response("cancel", "Cancel");
        dialog.add_response("add", "Add");
        dialog.set_default_response(Some("add"));
        dialog.set_close_response("cancel");

        let root = self.root();
        let parent_window = root.as_ref().and_then(|r| r.downcast_ref::<gtk::Window>());

        dialog.connect_response(
            None,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |_, response: &str| {
                    if response == "add" {
                        let output_name = entry.text().to_string();
                        if !output_name.trim().is_empty() {
                            obj.update_monitor_value(&output_name, |_| {});
                            obj.refresh_settings();
                        }
                    }
                }
            ),
        );

        dialog.present(parent_window);
    }
}

impl PageSettings for DisplaysPage {
    fn setup_ui(&self) {
        let imp = self.imp();

        if let Some(toggle) = imp.builder.object::<adw::SwitchRow>("night_light_toggle") {
            toggle.connect_active_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    if row.is_active() {
                        let temp = if let Some(spin) =
                            obj.imp().builder.object::<adw::SpinRow>("night_light_temp")
                        {
                            spin.value() as i32
                        } else {
                            6000
                        };
                        let _ = Command::new("hypr-refresh-hyprsunset")
                            .arg(temp.to_string())
                            .spawn();
                    } else {
                        let _ = Command::new("pkill").arg("hyprsunset").spawn();
                    }
                }
            ));
        }

        if let Some(spin) = imp.builder.object::<adw::SpinRow>("night_light_temp") {
            spin.connect_value_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = row.value() as i32;
                    use yaml_serde;
                    ThemeStore::global().stage_update(
                        "hypr.sunset.temperature",
                        yaml_serde::Value::Number(val.into()),
                    );
                }
            ));
        }

        if let Some(toggle) = imp.builder.object::<adw::SwitchRow>("hotplug_toggle") {
            toggle.connect_active_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |row| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    if row.is_active() {
                        let _ = Command::new("hypr-monitor-watch").spawn();
                    } else {
                        let _ = Command::new("pkill")
                            .arg("-f")
                            .arg("hypr-monitor-watch")
                            .spawn();
                    }
                }
            ));
        }

        if let Some(btn) = imp.builder.object::<gtk::Button>("add_monitor_btn") {
            btn.connect_clicked(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |_| {
                    obj.prompt_add_monitor();
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

        // Check running services
        let is_night_light_running = Command::new("pgrep")
            .arg("-f")
            .arg("hyprsunset")
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false);
        if let Some(toggle) = imp.builder.object::<adw::SwitchRow>("night_light_toggle") {
            toggle.set_active(is_night_light_running);
        }

        if let Some(spin) = imp.builder.object::<adw::SpinRow>("night_light_temp") {
            spin.set_value(data.get_raw_f64("hypr.sunset.temperature", 6000.0));
        }

        let is_hotplug_running = Command::new("pgrep")
            .arg("-f")
            .arg("hypr-monitor-watch")
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false);
        if let Some(toggle) = imp.builder.object::<adw::SwitchRow>("hotplug_toggle") {
            toggle.set_active(is_hotplug_running);
        }

        // Re-populate Monitors list
        if let Some(group) = imp
            .builder
            .object::<adw::PreferencesGroup>("monitors_group")
        {
            let mut rows = imp.monitor_rows.borrow_mut();
            for row in rows.iter() {
                group.remove(row);
            }
            rows.clear();

            let display_infos = self.get_display_infos();
            for info in display_infos {
                let title = if info.is_connected {
                    info.output.clone()
                } else {
                    format!("{} (Disconnected)", info.output)
                };

                let expander = adw::ExpanderRow::builder()
                    .title(&title)
                    .subtitle(&info.description)
                    .build();

                // Add delete button
                let delete_btn = gtk::Button::builder()
                    .icon_name("user-trash-symbolic")
                    .valign(gtk::Align::Center)
                    .css_classes(vec!["flat".to_string()])
                    .tooltip_text("Delete monitor config")
                    .build();

                let output_name = info.output.clone();
                delete_btn.connect_clicked(glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    move |_| {
                        obj.delete_monitor_config(&output_name);
                    }
                ));
                expander.add_suffix(&delete_btn);

                // Mode selection combo row
                let mode_row = adw::ComboRow::builder()
                    .title("Resolution &amp; Refresh Rate")
                    .use_markup(true)
                    .build();

                let mut mode_options = vec!["preferred".to_string()];
                for m in &info.available_modes {
                    if m != "preferred" && !mode_options.contains(m) {
                        mode_options.push(m.clone());
                    }
                }
                mode_options.push("Custom...".to_string());

                let string_list = gtk::StringList::new(
                    &mode_options.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                );
                mode_row.set_model(Some(&string_list));

                // 1b. Custom Mode Entry Row
                let custom_mode_row = adw::EntryRow::builder()
                    .title("Custom Resolution / Mode")
                    .build();

                // Select active mode
                let current_mode = info.mode.clone();
                let is_custom =
                    current_mode != "preferred" && !info.available_modes.contains(&current_mode);

                if is_custom {
                    mode_row.set_selected((mode_options.len() - 1) as u32);
                    custom_mode_row.set_text(&current_mode);
                    custom_mode_row.set_sensitive(true);
                } else {
                    if let Some(idx) = mode_options.iter().position(|m| m == &current_mode) {
                        mode_row.set_selected(idx as u32);
                    } else {
                        mode_row.set_selected(0);
                    }
                    custom_mode_row.set_text("");
                    custom_mode_row.set_sensitive(false);
                }

                let output_name_mode = info.output.clone();
                let mode_options_clone = mode_options.clone();
                let custom_row_weak = custom_mode_row.downgrade();
                mode_row.connect_selected_item_notify(glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    move |row| {
                        if *obj.imp().is_refreshing.borrow() {
                            return;
                        }
                        let idx = row.selected();
                        let is_last = idx as usize == mode_options_clone.len() - 1;
                        if let Some(custom_row) = custom_row_weak.upgrade() {
                            custom_row.set_sensitive(is_last);
                            if is_last {
                                let custom_text = custom_row.text().to_string();
                                if !custom_text.trim().is_empty() {
                                    obj.update_monitor_value(&output_name_mode, |m| {
                                        m.mode = custom_text.clone();
                                    });
                                }
                            } else {
                                if let Some(selected_mode) = mode_options_clone.get(idx as usize) {
                                    obj.update_monitor_value(&output_name_mode, |m| {
                                        m.mode = selected_mode.clone();
                                    });
                                }
                            }
                        }
                    }
                ));

                let output_name_custom = info.output.clone();
                custom_mode_row.connect_text_notify(glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    move |row| {
                        if *obj.imp().is_refreshing.borrow() {
                            return;
                        }
                        if !row.is_sensitive() {
                            return;
                        }
                        let text = row.text().to_string();
                        if !text.trim().is_empty() {
                            obj.update_monitor_value(&output_name_custom, |m| {
                                m.mode = text;
                            });
                        }
                    }
                ));

                expander.add_row(&mode_row);
                expander.add_row(&custom_mode_row);

                // Position row (EntryRow)
                let position_row = adw::EntryRow::builder()
                    .title("Position")
                    .text(&info.position)
                    .build();

                let output_name_pos = info.output.clone();
                position_row.connect_text_notify(glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    move |row| {
                        if *obj.imp().is_refreshing.borrow() {
                            return;
                        }
                        let text = row.text().to_string();
                        obj.update_monitor_value(&output_name_pos, |m| {
                            m.position = text;
                        });
                    }
                ));
                expander.add_row(&position_row);

                // Scale row (SpinRow)
                let scale_adj = gtk::Adjustment::new(info.scale as f64, 0.5, 10.0, 0.05, 0.5, 0.0);
                let scale_row = adw::SpinRow::builder()
                    .title("Scale")
                    .subtitle("Display scaling factor")
                    .adjustment(&scale_adj)
                    .digits(2)
                    .build();

                let output_name_scale = info.output.clone();
                scale_row.connect_value_notify(glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    move |row| {
                        if *obj.imp().is_refreshing.borrow() {
                            return;
                        }
                        let val = row.value() as f32;
                        obj.update_monitor_value(&output_name_scale, |m| {
                            m.scale = val;
                        });
                    }
                ));
                expander.add_row(&scale_row);

                group.add(&expander);
                rows.push(expander.upcast());
            }
        }

        *imp.is_refreshing.borrow_mut() = false;
    }
}

impl DisplaysPage {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

pub mod imp {
    use super::*;

    pub struct DisplaysPage {
        pub builder: gtk::Builder,
        pub is_refreshing: RefCell<bool>,
        pub monitor_rows: RefCell<Vec<gtk::Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DisplaysPage {
        const NAME: &'static str = "DisplaysPage";
        type Type = super::DisplaysPage;
        type ParentType = adw::PreferencesPage;

        fn new() -> Self {
            let builder =
                gtk::Builder::from_string(include_str!("../../resources/pages/displays.ui"));
            Self {
                builder,
                is_refreshing: RefCell::new(false),
                monitor_rows: RefCell::new(Vec::new()),
            }
        }
    }

    impl Default for DisplaysPage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ObjectImpl for DisplaysPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            let group_ids = ["global_group", "monitors_group", "actions_group"];
            for id in group_ids {
                if let Some(group) = self.builder.object::<adw::PreferencesGroup>(id) {
                    obj.add(&group);
                }
            }
        }
    }

    impl WidgetImpl for DisplaysPage {}
    impl PreferencesPageImpl for DisplaysPage {}
}
