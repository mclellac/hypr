use crate::pages::PageSettings;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::ThemeStore;
use libadwaita as adw;
use std::process::Command;
use std::thread;

glib::wrapper! {
    pub struct PluginsPage(ObjectSubclass<imp::PluginsPage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PageSettings for PluginsPage {
    fn setup_ui(&self) {
        self.refresh_settings();
    }

    fn refresh_settings(&self) {
        let imp = self.imp();
        *imp.is_refreshing.borrow_mut() = true;

        // Clear dynamic community plugin rows safely using group.remove
        for row in imp.community_rows.borrow_mut().drain(..) {
            imp.community_group.remove(&row);
        }

        if let Some(row) = imp.loading_row.borrow_mut().take() {
            imp.community_group.remove(&row);
        }

        let store = ThemeStore::global();
        let current_theme = store.current_theme();
        let data = store
            .get_theme_data(&current_theme)
            .unwrap_or_else(|_| std::sync::Arc::new(hypr_config_manager::ThemeConfig::default()));

        // Update Hyprglass controls
        let hyprglass_enabled = data.get_raw_bool("plugins.hyprglass.enabled", false);
        let glass_opacity = data.get_raw_f64("plugins.hyprglass.glass_opacity", 0.85);
        let blur_strength = data.get_raw_f64("plugins.hyprglass.blur_strength", 2.0);
        let blur_iterations = data.get_raw_i32("plugins.hyprglass.blur_iterations", 3);
        let refraction_strength = data.get_raw_f64("plugins.hyprglass.refraction_strength", 0.6);
        let chromatic_aberration = data.get_raw_f64("plugins.hyprglass.chromatic_aberration", 0.5);
        let fresnel_strength = data.get_raw_f64("plugins.hyprglass.fresnel_strength", 0.6);
        let specular_strength = data.get_raw_f64("plugins.hyprglass.specular_strength", 0.8);
        let edge_thickness = data.get_raw_f64("plugins.hyprglass.edge_thickness", 0.06);
        let brightness = data.get_raw_f64("plugins.hyprglass.brightness", 0.9);
        let contrast = data.get_raw_f64("plugins.hyprglass.contrast", 1.0);
        let manage_window_blur = data.get_raw_bool("plugins.hyprglass.manage_window_blur", true);
        let layer_waybar = data.get_raw_bool("plugins.hyprglass.layer_waybar", true);
        let layer_walker = data.get_raw_bool("plugins.hyprglass.layer_walker", true);
        let layer_mako = data.get_raw_bool("plugins.hyprglass.layer_mako", true);

        imp.hyprglass_switch.set_active(hyprglass_enabled);
        imp.glass_opacity_spin.set_value(glass_opacity);
        imp.blur_strength_spin.set_value(blur_strength);
        imp.blur_iterations_spin.set_value(blur_iterations as f64);
        imp.refraction_spin.set_value(refraction_strength);
        imp.chromatic_spin.set_value(chromatic_aberration);
        imp.fresnel_spin.set_value(fresnel_strength);
        imp.specular_spin.set_value(specular_strength);
        imp.edge_thickness_spin.set_value(edge_thickness);
        imp.brightness_spin.set_value(brightness);
        imp.contrast_spin.set_value(contrast);
        imp.manage_blur_switch.set_active(manage_window_blur);
        imp.layer_waybar_switch.set_active(layer_waybar);
        imp.layer_walker_switch.set_active(layer_walker);
        imp.layer_mako_switch.set_active(layer_mako);

        // Community plugins async loading
        let loading_row = adw::ActionRow::builder()
            .title("Loading community plugins...")
            .hexpand(true)
            .halign(gtk::Align::Fill)
            .build();

        imp.community_group.add(&loading_row);
        *imp.loading_row.borrow_mut() = Some(loading_row.clone());

        let obj = self.downgrade();
        let loading_row_clone = loading_row.clone();

        let (sender, receiver) = std::sync::mpsc::channel::<Option<String>>();

        glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            let obj = match obj.upgrade() {
                Some(o) => o,
                None => return glib::ControlFlow::Break,
            };
            let imp = obj.imp();

            let mut keep_going = true;
            match receiver.try_recv() {
                Ok(output) => {
                    if let Some(current_loading) = imp.loading_row.borrow_mut().take() {
                        if current_loading == loading_row_clone {
                            imp.community_group.remove(&current_loading);
                        } else {
                            *imp.loading_row.borrow_mut() = Some(current_loading);
                        }
                    }

                    if let Some(out) = output {
                        let mut current_name = String::new();
                        let mut current_desc = String::new();
                        let mut current_status = false;
                        let mut found_any = false;

                        for line in out.lines() {
                            let line = line.trim();
                            if line.is_empty() {
                                continue;
                            }

                            if line.starts_with("Plugin")
                                || (line.starts_with("->") && line.contains("Plugin"))
                            {
                                if !current_name.is_empty() && current_name != "hyprglass" {
                                    obj.add_community_plugin_row(
                                        &current_name,
                                        &current_desc,
                                        current_status,
                                    );
                                    found_any = true;
                                }

                                let parts: Vec<&str> = line.splitn(2, "Plugin").collect();
                                if parts.len() == 2 {
                                    let name_part = parts[1].trim();
                                    current_name =
                                        if let Some(stripped) = name_part.strip_prefix(':') {
                                            stripped.trim().to_string()
                                        } else {
                                            name_part.to_string()
                                        };
                                    current_desc.clear();
                                    current_status = false;
                                }
                            } else if let Some((key, val)) = line.split_once(':') {
                                let key = key.trim().to_lowercase();
                                let val = val.trim();
                                if key == "description" {
                                    current_desc = val.to_string();
                                } else if key == "status" {
                                    current_status = val.to_lowercase() == "enabled";
                                }
                            }
                        }

                        if !current_name.is_empty() && current_name != "hyprglass" {
                            obj.add_community_plugin_row(
                                &current_name,
                                &current_desc,
                                current_status,
                            );
                            found_any = true;
                        }

                        if !found_any {
                            let row = adw::ActionRow::builder()
                                .title("No additional community plugins installed")
                                .subtitle("Install community plugins via `hypr-plugin install &lt;repo&gt;`")
                                .hexpand(true)
                                .halign(gtk::Align::Fill)
                                .build();
                            imp.community_group.add(&row);
                            imp.community_rows.borrow_mut().push(row.upcast());
                        }
                    } else {
                        let row = adw::ActionRow::builder()
                            .title("Error listing community plugins")
                            .hexpand(true)
                            .halign(gtk::Align::Fill)
                            .build();
                        imp.community_group.add(&row);
                        imp.community_rows.borrow_mut().push(row.upcast());
                    }
                    keep_going = false;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    keep_going = false;
                }
            }

            if keep_going {
                glib::ControlFlow::Continue
            } else {
                glib::ControlFlow::Break
            }
        });

        thread::spawn(move || {
            let output = Command::new("hyprpm").arg("list").output();
            match output {
                Ok(out) if out.status.success() => {
                    let _ = sender.send(Some(String::from_utf8_lossy(&out.stdout).to_string()));
                }
                Ok(out) => {
                    let _ = sender.send(Some(String::from_utf8_lossy(&out.stderr).to_string()));
                }
                Err(_) => {
                    let _ = sender.send(None);
                }
            }
        });

        *imp.is_refreshing.borrow_mut() = false;
    }
}

impl PluginsPage {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn add_community_plugin_row(&self, name: &str, desc: &str, enabled: bool) {
        let imp = self.imp();
        let row = adw::SwitchRow::builder()
            .title(glib::markup_escape_text(name))
            .subtitle(glib::markup_escape_text(desc))
            .active(enabled)
            .hexpand(true)
            .halign(gtk::Align::Fill)
            .build();

        let name_clone = name.to_string();
        row.connect_active_notify(move |row: &adw::SwitchRow| {
            let active = row.is_active();
            let action = if active { "enable" } else { "disable" };
            let name = name_clone.clone();

            thread::spawn(move || {
                let _ = Command::new("hypr-plugin").arg(action).arg(&name).output();
            });
        });

        imp.community_group.add(&row);
        imp.community_rows.borrow_mut().push(row.upcast());
    }
}

pub mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct PluginsPage {
        pub is_refreshing: RefCell<bool>,

        pub featured_group: adw::PreferencesGroup,
        pub hyprglass_expander: adw::ExpanderRow,
        pub hyprglass_switch: adw::SwitchRow,

        pub glass_opacity_spin: adw::SpinRow,
        pub blur_strength_spin: adw::SpinRow,
        pub blur_iterations_spin: adw::SpinRow,
        pub refraction_spin: adw::SpinRow,
        pub chromatic_spin: adw::SpinRow,
        pub fresnel_spin: adw::SpinRow,
        pub specular_spin: adw::SpinRow,
        pub edge_thickness_spin: adw::SpinRow,
        pub brightness_spin: adw::SpinRow,
        pub contrast_spin: adw::SpinRow,
        pub manage_blur_switch: adw::SwitchRow,

        pub layer_waybar_switch: adw::SwitchRow,
        pub layer_walker_switch: adw::SwitchRow,
        pub layer_mako_switch: adw::SwitchRow,

        pub community_group: adw::PreferencesGroup,
        pub community_rows: RefCell<Vec<gtk::Widget>>,
        pub loading_row: RefCell<Option<adw::ActionRow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PluginsPage {
        const NAME: &'static str = "PluginsPage";
        type Type = super::PluginsPage;
        type ParentType = adw::PreferencesPage;
    }

    impl ObjectImpl for PluginsPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.set_title("Plugins");
            obj.set_icon_name(Some("system-run-symbolic"));

            // Featured Plugins Group
            self.featured_group.set_title("Featured Plugins");
            self.featured_group.set_description(Some("Official and curated Hyprland plugins with full configuration."));

            self.hyprglass_switch.set_title("Hyprglass");
            self.hyprglass_switch.set_subtitle("Liquid glassmorphism, surface refraction, and edge glow effects");
            self.hyprglass_switch.set_hexpand(true);
            self.hyprglass_switch.set_halign(gtk::Align::Fill);

            self.hyprglass_expander.set_title("Hyprglass Settings");
            self.hyprglass_expander.set_subtitle("Fine-tune glass opacity, blur, refraction, glow, and layers");
            self.hyprglass_expander.set_hexpand(true);
            self.hyprglass_expander.set_halign(gtk::Align::Fill);

            self.glass_opacity_spin.set_title("Glass Opacity");
            self.glass_opacity_spin.set_adjustment(Some(&gtk::Adjustment::new(0.85, 0.1, 1.0, 0.05, 0.1, 0.0)));
            self.glass_opacity_spin.set_digits(2);
            self.glass_opacity_spin.set_hexpand(true);
            self.glass_opacity_spin.set_halign(gtk::Align::Fill);

            self.blur_strength_spin.set_title("Blur Strength Scale");
            self.blur_strength_spin.set_adjustment(Some(&gtk::Adjustment::new(2.0, 0.5, 5.0, 0.5, 1.0, 0.0)));
            self.blur_strength_spin.set_digits(1);
            self.blur_strength_spin.set_hexpand(true);
            self.blur_strength_spin.set_halign(gtk::Align::Fill);

            self.blur_iterations_spin.set_title("Blur Iterations (Passes)");
            self.blur_iterations_spin.set_adjustment(Some(&gtk::Adjustment::new(3.0, 1.0, 5.0, 1.0, 1.0, 0.0)));
            self.blur_iterations_spin.set_hexpand(true);
            self.blur_iterations_spin.set_halign(gtk::Align::Fill);

            self.refraction_spin.set_title("Edge Refraction Strength");
            self.refraction_spin.set_adjustment(Some(&gtk::Adjustment::new(0.6, 0.0, 1.0, 0.05, 0.1, 0.0)));
            self.refraction_spin.set_digits(2);
            self.refraction_spin.set_hexpand(true);
            self.refraction_spin.set_halign(gtk::Align::Fill);

            self.chromatic_spin.set_title("Chromatic Aberration");
            self.chromatic_spin.set_adjustment(Some(&gtk::Adjustment::new(0.5, 0.0, 1.0, 0.05, 0.1, 0.0)));
            self.chromatic_spin.set_digits(2);
            self.chromatic_spin.set_hexpand(true);
            self.chromatic_spin.set_halign(gtk::Align::Fill);

            self.fresnel_spin.set_title("Fresnel Edge Glow");
            self.fresnel_spin.set_adjustment(Some(&gtk::Adjustment::new(0.6, 0.0, 1.0, 0.05, 0.1, 0.0)));
            self.fresnel_spin.set_digits(2);
            self.fresnel_spin.set_hexpand(true);
            self.fresnel_spin.set_halign(gtk::Align::Fill);

            self.specular_spin.set_title("Specular Highlight");
            self.specular_spin.set_adjustment(Some(&gtk::Adjustment::new(0.8, 0.0, 1.0, 0.05, 0.1, 0.0)));
            self.specular_spin.set_digits(2);
            self.specular_spin.set_hexpand(true);
            self.specular_spin.set_halign(gtk::Align::Fill);

            self.edge_thickness_spin.set_title("Bezel Edge Thickness");
            self.edge_thickness_spin.set_adjustment(Some(&gtk::Adjustment::new(0.06, 0.01, 0.15, 0.01, 0.02, 0.0)));
            self.edge_thickness_spin.set_digits(2);
            self.edge_thickness_spin.set_hexpand(true);
            self.edge_thickness_spin.set_halign(gtk::Align::Fill);

            self.brightness_spin.set_title("Glass Brightness");
            self.brightness_spin.set_adjustment(Some(&gtk::Adjustment::new(0.9, 0.5, 1.5, 0.05, 0.1, 0.0)));
            self.brightness_spin.set_digits(2);
            self.brightness_spin.set_hexpand(true);
            self.brightness_spin.set_halign(gtk::Align::Fill);

            self.contrast_spin.set_title("Glass Contrast");
            self.contrast_spin.set_adjustment(Some(&gtk::Adjustment::new(1.0, 0.5, 1.5, 0.05, 0.1, 0.0)));
            self.contrast_spin.set_digits(2);
            self.contrast_spin.set_hexpand(true);
            self.contrast_spin.set_halign(gtk::Align::Fill);

            self.manage_blur_switch.set_title("Auto Manage Window Blur");
            self.manage_blur_switch.set_subtitle("Automatically set noblur on glassed windows");
            self.manage_blur_switch.set_hexpand(true);
            self.manage_blur_switch.set_halign(gtk::Align::Fill);

            self.layer_waybar_switch.set_title("Waybar Surface Glass");
            self.layer_waybar_switch.set_subtitle("Apply liquid glass refraction to Waybar");
            self.layer_waybar_switch.set_hexpand(true);
            self.layer_waybar_switch.set_halign(gtk::Align::Fill);

            self.layer_walker_switch.set_title("Walker Surface Glass");
            self.layer_walker_switch.set_subtitle("Apply liquid glass refraction to Walker menu");
            self.layer_walker_switch.set_hexpand(true);
            self.layer_walker_switch.set_halign(gtk::Align::Fill);

            self.layer_mako_switch.set_title("Mako Notifications Glass");
            self.layer_mako_switch.set_subtitle("Apply liquid glass refraction to Mako notifications");
            self.layer_mako_switch.set_hexpand(true);
            self.layer_mako_switch.set_halign(gtk::Align::Fill);

            self.hyprglass_expander.add_row(&self.glass_opacity_spin);
            self.hyprglass_expander.add_row(&self.blur_strength_spin);
            self.hyprglass_expander.add_row(&self.blur_iterations_spin);
            self.hyprglass_expander.add_row(&self.refraction_spin);
            self.hyprglass_expander.add_row(&self.chromatic_spin);
            self.hyprglass_expander.add_row(&self.fresnel_spin);
            self.hyprglass_expander.add_row(&self.specular_spin);
            self.hyprglass_expander.add_row(&self.edge_thickness_spin);
            self.hyprglass_expander.add_row(&self.brightness_spin);
            self.hyprglass_expander.add_row(&self.contrast_spin);
            self.hyprglass_expander.add_row(&self.manage_blur_switch);
            self.hyprglass_expander.add_row(&self.layer_waybar_switch);
            self.hyprglass_expander.add_row(&self.layer_walker_switch);
            self.hyprglass_expander.add_row(&self.layer_mako_switch);

            self.featured_group.add(&self.hyprglass_switch);
            self.featured_group.add(&self.hyprglass_expander);

            // Community Plugins Group
            self.community_group.set_title("Installed Community Plugins");
            self.community_group.set_description(Some("Plugins detected on your system via hyprpm list."));

            obj.add(&self.featured_group);
            obj.add(&self.community_group);

            // Signal connections
            self.hyprglass_switch.connect_active_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let active = w.is_active();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.enabled",
                        yaml_serde::Value::from(active),
                    );
                    let action = if active { "enable" } else { "disable" };
                    thread::spawn(move || {
                        let _ = Command::new("hypr-plugin").arg(action).arg("hyprglass").output();
                    });
                }
            ));

            self.glass_opacity_spin.connect_value_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.glass_opacity",
                        yaml_serde::Value::from(val),
                    );
                }
            ));

            self.blur_strength_spin.connect_value_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.blur_strength",
                        yaml_serde::Value::from(val),
                    );
                }
            ));

            self.blur_iterations_spin.connect_value_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value() as i32;
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.blur_iterations",
                        yaml_serde::Value::Number(val.into()),
                    );
                }
            ));

            self.refraction_spin.connect_value_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.refraction_strength",
                        yaml_serde::Value::from(val),
                    );
                }
            ));

            self.chromatic_spin.connect_value_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.chromatic_aberration",
                        yaml_serde::Value::from(val),
                    );
                }
            ));

            self.fresnel_spin.connect_value_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.fresnel_strength",
                        yaml_serde::Value::from(val),
                    );
                }
            ));

            self.specular_spin.connect_value_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.specular_strength",
                        yaml_serde::Value::from(val),
                    );
                }
            ));

            self.edge_thickness_spin.connect_value_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.edge_thickness",
                        yaml_serde::Value::from(val),
                    );
                }
            ));

            self.brightness_spin.connect_value_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.brightness",
                        yaml_serde::Value::from(val),
                    );
                }
            ));

            self.contrast_spin.connect_value_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = w.value();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.contrast",
                        yaml_serde::Value::from(val),
                    );
                }
            ));

            self.manage_blur_switch.connect_active_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let active = w.is_active();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.manage_window_blur",
                        yaml_serde::Value::from(active),
                    );
                }
            ));

            self.layer_waybar_switch.connect_active_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let active = w.is_active();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.layer_waybar",
                        yaml_serde::Value::from(active),
                    );
                }
            ));

            self.layer_walker_switch.connect_active_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let active = w.is_active();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.layer_walker",
                        yaml_serde::Value::from(active),
                    );
                }
            ));

            self.layer_mako_switch.connect_active_notify(glib::clone!(
                #[weak]
                obj,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let active = w.is_active();
                    ThemeStore::global().stage_update(
                        "plugins.hyprglass.layer_mako",
                        yaml_serde::Value::from(active),
                    );
                }
            ));
        }
    }

    impl WidgetImpl for PluginsPage {}
    impl PreferencesPageImpl for PluginsPage {}
}
