use crate::pages::PageSettings;
use crate::pages::components::border_widget::BorderWidget;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk4 as gtk;
use hypr_config_manager::ThemeStore;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

glib::wrapper! {
    pub struct AppearancePage(ObjectSubclass<imp::AppearancePage>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl AppearancePage {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn debug_log(&self, msg: &str) {
        if std::env::args().any(|x| x == "--debug") {
            println!("[AppearancePage] {}", msg);
        }
    }
}

impl PageSettings for AppearancePage {
    fn setup_ui(&self) {
        let imp = self.imp();
        let home = std::env::var("HOME").unwrap_or_default();

        self.debug_log("Setting up UI models and signals");

        // Populate Combo Models
        self.populate_combo_model(
            "gtk_theme",
            &[
                "/usr/share/themes",
                &format!("{}/.themes", home),
                &format!("{}/.local/share/themes", home),
            ],
        );
        self.populate_combo_model(
            "gtk_icon_theme",
            &[
                "/usr/share/icons",
                &format!("{}/.icons", home),
                &format!("{}/.local/share/icons", home),
            ],
        );
        self.populate_combo_model(
            "gtk_cursor_theme",
            &[
                "/usr/share/icons",
                &format!("{}/.icons", home),
                &format!("{}/.local/share/icons", home),
            ],
        );

        // Connect Signals
        self.connect_spin_signal("gaps_in", "layout.gaps_in");
        self.connect_spin_signal("gaps_out", "layout.gaps_out");
        self.connect_spin_signal("border_size", "layout.border_size");
        self.connect_spin_signal("rounding", "layout.rounding");

        self.connect_switch_signal("shadow_enabled", "layout.shadow_enabled");
        self.connect_spin_signal("shadow_range", "layout.shadow_range");
        self.connect_spin_signal("shadow_power", "layout.shadow_power");
        self.connect_entry_signal("shadow_offset", "layout.shadow_offset");

        self.connect_switch_signal("glow_enabled", "layout.glow_enabled");
        self.connect_spin_signal("glow_range", "layout.glow_range");

        self.connect_switch_signal("dim_inactive", "layout.dim_inactive");
        self.connect_spin_signal("dim_strength", "layout.dim_strength");

        self.connect_switch_signal("blur_enabled", "layout.blur_enabled");
        self.connect_spin_signal("blur_size", "layout.blur_size");
        self.connect_spin_signal("blur_passes", "layout.blur_passes");
        self.connect_spin_signal("blur_vibrancy", "layout.blur_vibrancy");

        self.connect_spin_signal("gdk_scale", "gtk.gdk_scale");
        self.connect_spin_signal("gdk_dpi_scale", "gtk.gdk_dpi_scale");
        self.connect_spin_signal("qt_scale_factor", "gtk.qt_scale_factor");
        self.connect_spin_signal("text_scaling_factor", "gtk.text_scaling_factor");

        self.connect_combo_signal("gtk_color_scheme", "gtk.color_scheme");
        self.connect_combo_signal("gtk_theme", "gtk.theme");
        self.connect_combo_signal("gtk_icon_theme", "gtk.icon_theme");
        self.connect_combo_signal("gtk_cursor_theme", "cursor.theme");
        self.connect_combo_signal("gtk_accent_color", "gtk.accent_color");
        // Invert the switch logic: Enable Hardware Cursor = true -> no_hardware = false
        if let Some(row) = imp.builder.object::<adw::SwitchRow>("cursor_no_hardware") {
            row.connect_active_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |w| {
                    if *obj.imp().is_refreshing.borrow() {
                        return;
                    }
                    let val = !w.is_active();
                    ThemeStore::global()
                        .stage_update("cursor.no_hardware", yaml_serde::Value::Bool(val));
                }
            ));
        }

        self.connect_switch_signal("animations_enabled", "layout.animations_enabled");

        // Animation style custom logic
        if let Some(row) = imp.builder.object::<adw::ComboRow>("animation_style") {
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
                        let val = string_obj.string().to_string().to_lowercase();
                        ThemeStore::global().stage_update(
                            "layout.animation_style",
                            yaml_serde::Value::String(val.clone()),
                        );

                        if let Some(group) = imp
                            .builder
                            .object::<adw::PreferencesGroup>("custom_animations_group")
                        {
                            group.set_visible(val == "custom");
                        }
                    }
                }
            ));
        }

        self.connect_switch_signal("anim_windows_enabled", "layout.animation_windows_enabled");
        self.connect_switch_signal(
            "anim_windows_out_enabled",
            "layout.animation_windows_out_enabled",
        );
        self.connect_switch_signal(
            "anim_workspaces_enabled",
            "layout.animation_workspaces_enabled",
        );
        self.connect_switch_signal("anim_border_enabled", "layout.animation_border_enabled");
        self.connect_switch_signal("anim_fade_enabled", "layout.animation_fade_enabled");

        self.connect_entry_signal("anim_windows_curve", "layout.animation_windows_curve");
        self.connect_spin_signal("anim_windows_speed", "layout.animation_windows_speed");
        self.connect_fade_spin_signal("anim_fade_speed", "layout.animation_fade_speed");
        self.connect_spin_signal("anim_workspaces_speed", "layout.animation_workspaces_speed");

        // Dynamically add color entries
        if let Some(borders_group) = imp.builder.object::<adw::PreferencesGroup>("borders_group") {
            self.create_and_bind_border_widget(
                &borders_group,
                "Active Border Color",
                "layout.active_border",
                true,
            );
            self.create_and_bind_border_widget(
                &borders_group,
                "Inactive Border Color",
                "layout.inactive_border",
                true,
            );
            self.create_and_bind_border_widget(
                &borders_group,
                "No Group Border",
                "layout.nogroup_border",
                true,
            );
            self.create_and_bind_border_widget(
                &borders_group,
                "No Group Border Active",
                "layout.nogroup_border_active",
                true,
            );
        }

        if let Some(shadow_group) = imp.builder.object::<adw::PreferencesGroup>("shadow_group") {
            self.create_and_bind_border_widget(
                &shadow_group,
                "Shadow Color",
                "layout.shadow_color",
                false,
            );
        }

        if let Some(glow_group) = imp.builder.object::<adw::PreferencesGroup>("glow_group") {
            self.create_and_bind_border_widget(
                &glow_group,
                "Glow Color",
                "layout.glow_color",
                false,
            );
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

        self.debug_log(&format!("Refreshing settings for theme: {}", theme));

        // Gaps & Borders
        self.update_spin_row("gaps_in", data.get_raw_f64("layout.gaps_in", 5.0));
        self.update_spin_row("gaps_out", data.get_raw_f64("layout.gaps_out", 10.0));
        self.update_spin_row("border_size", data.get_raw_f64("layout.border_size", 2.0));
        self.update_spin_row("rounding", data.get_raw_f64("layout.rounding", 8.0));

        // Shadow
        self.update_switch_row(
            "shadow_enabled",
            data.get_raw_bool("layout.shadow_enabled", true),
        );
        self.update_spin_row("shadow_range", data.get_raw_f64("layout.shadow_range", 4.0));
        self.update_spin_row("shadow_power", data.get_raw_f64("layout.shadow_power", 3.0));
        self.update_entry_row(
            "shadow_offset",
            &data.get_raw_str("layout.shadow_offset", "0 0"),
        );

        // Glow
        self.update_switch_row(
            "glow_enabled",
            data.get_raw_bool("layout.glow_enabled", false),
        );
        self.update_spin_row("glow_range", data.get_raw_f64("layout.glow_range", 10.0));

        // Dimming
        self.update_switch_row(
            "dim_inactive",
            data.get_raw_bool("layout.dim_inactive", false),
        );
        self.update_spin_row("dim_strength", data.get_raw_f64("layout.dim_strength", 0.2));

        // Blur
        self.update_switch_row(
            "blur_enabled",
            data.get_raw_bool("layout.blur_enabled", true),
        );
        self.update_spin_row("blur_size", data.get_raw_f64("layout.blur_size", 3.0));
        self.update_spin_row("blur_passes", data.get_raw_f64("layout.blur_passes", 1.0));
        self.update_spin_row(
            "blur_vibrancy",
            data.get_raw_f64("layout.blur_vibrancy", 0.1696),
        );

        // Toolkit Scaling
        self.update_spin_row("gdk_scale", data.get_raw_f64("gtk.gdk_scale", 1.0));
        self.update_spin_row("gdk_dpi_scale", data.get_raw_f64("gtk.gdk_dpi_scale", 1.0));
        self.update_spin_row(
            "qt_scale_factor",
            data.get_raw_f64("gtk.qt_scale_factor", 1.0),
        );
        self.update_spin_row(
            "text_scaling_factor",
            data.get_raw_f64("gtk.text_scaling_factor", 1.0),
        );

        // GTK
        self.update_combo_row(
            "gtk_color_scheme",
            &data.get_raw_str("gtk.color_scheme", "default"),
        );
        self.update_combo_row("gtk_theme", &data.get_raw_str("gtk.theme", "Adwaita"));
        self.update_combo_row(
            "gtk_icon_theme",
            &data.get_raw_str("gtk.icon_theme", "Adwaita"),
        );
        self.update_combo_row(
            "gtk_cursor_theme",
            &data.get_raw_str("cursor.theme", "Adwaita"),
        );
        self.update_combo_row(
            "gtk_accent_color",
            &data.get_raw_str("gtk.accent_color", "blue"),
        );
        // Invert the switch logic: Enable Hardware Cursor = true -> no_hardware = false
        self.update_switch_row(
            "cursor_no_hardware",
            !data.get_raw_bool("cursor.no_hardware", false),
        );

        // Animations
        self.update_switch_row(
            "animations_enabled",
            data.get_raw_bool("layout.animations_enabled", true),
        );
        let anim_style = data.get_raw_str("layout.animation_style", "Fluid");
        if let Some(row) = self
            .imp()
            .builder
            .object::<adw::ComboRow>("animation_style")
            && let Some(model) = row.model()
            && let Some(string_list) = model.downcast_ref::<gtk::StringList>()
        {
            for i in 0..string_list.n_items() {
                if let Some(item) = string_list.item(i)
                    && let Some(string_obj) = item.downcast_ref::<gtk::StringObject>()
                    && string_obj.string().as_str() == anim_style
                {
                    row.set_selected(i);
                    break;
                }
            }
        }
        if let Some(group) = imp
            .builder
            .object::<adw::PreferencesGroup>("custom_animations_group")
        {
            group.set_visible(anim_style == "Custom");
        }

        // Specific Animations
        self.update_switch_row(
            "anim_windows_enabled",
            data.get_raw_bool("layout.animation_windows_enabled", true),
        );
        self.update_switch_row(
            "anim_windows_out_enabled",
            data.get_raw_bool("layout.animation_windows_out_enabled", true),
        );
        self.update_switch_row(
            "anim_workspaces_enabled",
            data.get_raw_bool("layout.animation_workspaces_enabled", true),
        );
        self.update_switch_row(
            "anim_border_enabled",
            data.get_raw_bool("layout.animation_border_enabled", true),
        );
        self.update_switch_row(
            "anim_fade_enabled",
            data.get_raw_bool("layout.animation_fade_enabled", true),
        );

        // Custom Animations
        self.update_entry_row(
            "anim_windows_curve",
            &data.get_raw_str("layout.animation_windows_curve", ""),
        );
        self.update_spin_row(
            "anim_windows_speed",
            data.get_raw_f64("layout.animation_windows_speed", 5.0),
        );
        self.update_spin_row(
            "anim_fade_speed",
            data.get_raw_f64("layout.animation_fade_speed", 5.0),
        );
        self.update_spin_row(
            "anim_workspaces_speed",
            data.get_raw_f64("layout.animation_workspaces_speed", 5.0),
        );

        // Dynamic rows
        for row in imp.dynamic_rows.borrow().iter() {
            let key = row.widget_name();
            if !key.is_empty()
                && let Some(entry) = row.downcast_ref::<adw::EntryRow>()
            {
                entry.set_text(&data.get_raw_str(&key, ""));
            }
        }
        // Update Border Widgets
        for bw in imp.border_widgets.borrow().iter() {
            let key = &bw.theme_key;
            bw.set_value(&data.get_raw_str(key, ""));
        }

        *imp.is_refreshing.borrow_mut() = false;
    }
}

impl AppearancePage {
    fn create_and_bind_border_widget(
        &self,
        group: &adw::PreferencesGroup,
        title: &str,
        theme_key: &str,
        allow_gradient: bool,
    ) {
        let bw = Rc::new(BorderWidget::new(title, theme_key, allow_gradient));
        for row in bw.get_rows() {
            group.add(row);
        }
        self.imp().border_widgets.borrow_mut().push(bw);
    }

    fn populate_combo_model(&self, id: &str, paths: &[&str]) {
        let imp = self.imp();
        if let Some(row) = imp.builder.object::<adw::ComboRow>(id) {
            let mut items = std::collections::HashSet::new();
            for path in paths {
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir()
                            && let Some(name) = entry.file_name().to_str()
                        {
                            items.insert(name.to_string());
                        }
                    }
                }
            }

            let mut sorted_items: Vec<_> = items.into_iter().collect();
            sorted_items.sort();

            let model =
                gtk::StringList::new(&sorted_items.iter().map(|s| s.as_str()).collect::<Vec<_>>());
            row.set_model(Some(&model));
        }
    }

    fn connect_spin_signal(&self, id: &str, theme_key: &str) {
        let imp = self.imp();
        crate::bind_signal!(self, imp.builder, adw::SpinRow, id, theme_key);
    }

    fn connect_fade_spin_signal(&self, id: &str, theme_key: &str) {
        let imp = self.imp();
        crate::bind_signal!(
            self,
            imp.builder,
            adw::SpinRow,
            id,
            theme_key,
            connect_value_notify,
            |w: &adw::SpinRow| w.value() as i32,
            yaml_serde::Value::Number
        );
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
                        ThemeStore::global()
                            .stage_update(&theme_key, yaml_serde::Value::String(val));
                    }
                }
            ));
        }
    }

    fn update_combo_row(&self, id: &str, val: &str) {
        if let Some(row) = self.imp().builder.object::<adw::ComboRow>(id)
            && let Some(model) = row.model()
            && let Some(string_list) = model.downcast_ref::<gtk::StringList>()
        {
            for i in 0..string_list.n_items() {
                if string_list.string(i).unwrap().to_lowercase() == val.to_lowercase() {
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

    fn update_entry_row(&self, id: &str, val: &str) {
        if let Some(row) = self.imp().builder.object::<adw::EntryRow>(id) {
            row.set_text(val);
        }
    }
}

pub mod imp {
    use super::*;

    pub struct AppearancePage {
        pub builder: gtk::Builder,
        pub is_refreshing: Rc<RefCell<bool>>,
        pub dynamic_rows: RefCell<Vec<adw::EntryRow>>,
        pub border_widgets: RefCell<Vec<Rc<BorderWidget>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppearancePage {
        const NAME: &'static str = "AppearancePage";
        type Type = super::AppearancePage;
        type ParentType = adw::PreferencesPage;

        fn new() -> Self {
            let builder =
                gtk::Builder::from_string(include_str!("../../resources/pages/appearance.ui"));
            Self {
                builder,
                is_refreshing: Rc::new(RefCell::new(false)),
                dynamic_rows: RefCell::new(Vec::new()),
                border_widgets: RefCell::new(Vec::new()),
            }
        }
    }

    impl Default for AppearancePage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ObjectImpl for AppearancePage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            let group_ids = [
                "gtk_settings_group",
                "scaling_group",
                "borders_group",
                "shadow_group",
                "glow_group",
                "dimming_group",
                "blur_group",
                "animations_group",
                "custom_animations_group",
            ];

            for id in group_ids {
                if let Some(group) = self.builder.object::<adw::PreferencesGroup>(id) {
                    obj.add(&group);
                }
            }
        }
    }

    impl WidgetImpl for AppearancePage {}
    impl PreferencesPageImpl for AppearancePage {}
}
